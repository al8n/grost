use grost_types::Identifier;
use ref_cast::RefCast;
use varing::{encode_u32_varint_to, encoded_u32_varint_len};

use crate::{Encode, error::EncodeError, Tag, Wirable, WireType};

/// A wrapper type for repeated fields.
///
/// This type is used to implement `Encode` and `Decode` traits for collections
/// of items. It is a transparent wrapper around the inner type collection `T`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, RefCast)]
#[repr(transparent)]
pub struct RepeatedEncoder<T: ?Sized>(T);

impl<T: ?Sized> AsRef<T> for RepeatedEncoder<T> {
  fn as_ref(&self) -> &T {
    &self.0
  }
}

impl<T: ?Sized> AsMut<T> for RepeatedEncoder<T> {
  fn as_mut(&mut self) -> &mut T {
    &mut self.0
  }
}

impl<T: ?Sized> RepeatedEncoder<T> {
  /// Creates a new `RepeatedEncoder` instance.
  #[inline]
  pub fn from_ref(inner: &T) -> &Self {
    RefCast::ref_cast(inner)
  }

  /// Returns a reference to the inner collection.
  #[inline]
  pub const fn as_inner(&self) -> &T {
    &self.0
  }
}

impl<T> RepeatedEncoder<T> {
  /// Creates a new `RepeatedEncoder` instance.
  #[inline]
  pub const fn new(inner: T) -> Self {
    Self(inner)
  }

  /// Consumes the `RepeatedEncoder` and returns the inner collection.
  #[inline]
  pub fn into_inner(self) -> T {
    self.0
  }
}

impl<T> Wirable for RepeatedEncoder<T> {}

impl<T> RepeatedEncoder<T>
where
  T: Repeated,
  for<'a> T::Item<'a>: Encode,
{
  /// Encodes the collection into the provided buffer.
  pub fn encode(&self, tag: Tag, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let encoded_len = self.encoded_len(tag);
    if buf.len() < encoded_len {
      return Err(EncodeError::insufficient_buffer(encoded_len, buf.len()));
    }

    let mut offset = 0;
    for item in self.0.iter() {
      match <T::Item<'_> as Wirable>::WIRE_TYPE {
        WireType::Zst => {
          // No encoding needed for zero-size types, because zst are not allowed to store information.
        }
        WireType::Varint => {
          offset += item.encode(&mut buf[offset..])?;
        }
        WireType::LengthDelimited => {
          let merged_len = Identifier::new(WireType::LengthDelimited, tag).encode_to(&mut buf[offset..])?;
          offset += merged_len;
          offset += item.encode_with_prefix(&mut buf[offset..])?;
        }
        WireType::Byte => {
          let written = item.encode(&mut buf[offset..])?;
          debug_assert_eq!(written, 1, "Byte wire type should write exactly 1 byte");
          offset += written;
        }
        WireType::Fixed16 => {
          let written = item.encode(&mut buf[offset..])?;
          debug_assert_eq!(written, 2, "Fixed16 wire type should write exactly 2 bytes");
          offset += written;
        }
        WireType::Fixed32 => {
          let written = item.encode(&mut buf[offset..])?;
          debug_assert_eq!(written, 4, "Fixed32 wire type should write exactly 4 bytes");
          offset += written;
        }
        WireType::Fixed64 => {
          let written = item.encode(&mut buf[offset..])?;
          debug_assert_eq!(written, 8, "Fixed64 wire type should write exactly 8 bytes");
          offset += written;
        }
        WireType::Fixed128 => {
          let written = item.encode(&mut buf[offset..])?;
          debug_assert_eq!(
            written, 16,
            "Fixed128 wire type should write exactly 16 bytes"
          );
          offset += written;
        }
      }
    }

    Ok(offset)
  }

  /// Encodes the collection into the provided buffer with tag, wire type, and length delimited.
  pub fn encode_with_prefix(&self, tag: Tag, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let encoded_len = self.encoded_len(tag);

    match <T::Item<'_> as Wirable>::WIRE_TYPE {
      WireType::LengthDelimited => self.encode(tag, buf),
      _ => {
        let mut written = Identifier::new(WireType::LengthDelimited, tag).encode_to(buf)?;

        if encoded_len > u32::MAX as usize {
          return Err(EncodeError::TooLarge);
        }

        written += encode_u32_varint_to(encoded_len as u32, &mut buf[written..])?;
        written += self.encode(tag, &mut buf[written..])?;
        Ok(written)
      }
    }
  }

  /// Returns the number of bytes required to encode the collection.
  pub fn encoded_len(&self, tag: Tag) -> usize {
    match <T::Item<'_> as Wirable>::WIRE_TYPE {
      WireType::Zst => encoded_u32_varint_len(self.0.len() as u32),
      WireType::Varint => self.0.iter().map(|e| Encode::encoded_len(&e)).sum(),
      WireType::LengthDelimited => {
        let merged_len = Identifier::new(WireType::LengthDelimited, tag).encoded_len();
        self
          .0
          .iter()
          .map(|item| merged_len + item.encoded_len_with_prefix())
          .sum()
      }
      WireType::Byte => self.0.len(),
      WireType::Fixed16 => self.0.len() * 2,
      WireType::Fixed32 => self.0.len() * 4,
      WireType::Fixed64 => self.0.len() * 8,
      WireType::Fixed128 => self.0.len() * 16,
    }
  }

  /// Returns the number of bytes required to encode the collection with tag, wire type, and length delimited.
  pub fn encoded_len_with_prefix(&self, tag: Tag) -> usize {
    let encoded_len = self.encoded_len(tag);

    match <T::Item<'_> as Wirable>::WIRE_TYPE {
      WireType::LengthDelimited => encoded_len,
      _ => {
        let merged_len = Identifier::new(WireType::LengthDelimited, tag).encoded_len();
        let encoded_len_size = encoded_u32_varint_len(encoded_len as u32);
        merged_len + encoded_len_size + encoded_len
      }
    }
  }
}

/// A trait for types which can be encoded as repeated fields.
pub trait Repeated {
  type Item<'a>
  where
    Self: 'a;

  /// Returns the number of items in the collection.
  fn len(&self) -> usize;

  /// Returns `true` if the collection is empty.
  fn is_empty(&self) -> bool;

  /// Returns an iterator over the items in the collection.
  fn iter(&self) -> impl Iterator<Item = Self::Item<'_>>;
}

impl<T: Repeated> Repeated for &T {
  type Item<'a>
    = T::Item<'a>
  where
    Self: 'a;

  #[inline]
  fn len(&self) -> usize {
    T::len(self)
  }

  #[inline]
  fn is_empty(&self) -> bool {
    T::is_empty(self)
  }

  #[inline]
  fn iter(&self) -> impl Iterator<Item = Self::Item<'_>> {
    T::iter(self)
  }
}

macro_rules! impl_repeated {
  ($($ty:ty $([ const $g:ident: usize ])?),+$(,)?) => {
    $(
      impl<T, $(const $g: usize)?> Repeated for $ty {
        type Item<'a> = &'a T
        where
          Self: 'a;

        #[inline]
        fn len(&self) -> usize {
          let s: &[T] = self.as_ref();
          s.len()
        }

        #[inline]
        fn is_empty(&self) -> bool {
          let s: &[T] = self.as_ref();
          s.is_empty()
        }

        #[inline]
        fn iter(&self) -> impl Iterator<Item = Self::Item<'_>> {
          let s: &[T] = self.as_ref();
          s.iter()
        }
      }
    )*
  };
  (@hash_set $($ty:ty $([ const $g:ident: usize ])?),+$(,)?) => {
    $(
      impl<T, S, $(const $g: usize)?> Repeated for $ty {
        type Item<'a> = &'a T
        where
          Self: 'a;

        #[inline]
        fn len(&self) -> usize {
          <$ty>::len(self)
        }

        #[inline]
        fn is_empty(&self) -> bool {
          <$ty>::is_empty(self)
        }

        #[inline]
        fn iter(&self) -> impl Iterator<Item = Self::Item<'_>> {
          <$ty>::iter(self)
        }
      }
    )*
  };
  (@set $($ty:ty $([ const $g:ident: usize ])?),+$(,)?) => {
    $(
      impl<T, $(const $g: usize)?> Repeated for $ty {
        type Item<'a> = &'a T
        where
          Self: 'a;

        #[inline]
        fn len(&self) -> usize {
          <$ty>::len(self)
        }

        #[inline]
        fn is_empty(&self) -> bool {
          <$ty>::is_empty(self)
        }

        #[inline]
        fn iter(&self) -> impl Iterator<Item = Self::Item<'_>> {
          <$ty>::iter(self)
        }
      }
    )*
  };
}

impl_repeated!([T], [T; N] [const N: usize]);

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  impl_repeated!(
    std::vec::Vec<T>,
    std::boxed::Box<[T]>,
    std::sync::Arc<[T]>,
    std::rc::Rc<[T]>,
  );

  impl_repeated!(@set std::collections::BTreeSet<T>);
};

#[cfg(feature = "std")]
const _: () = {
  impl_repeated!(@hash_set std::collections::HashSet<T, S>);
};

#[cfg(feature = "hashbrown_0_15")]
const _: () = {
  impl_repeated!(@hash_set hashbrown_0_15::HashSet<T, S>);
};

#[cfg(feature = "indexmap_2")]
const _: () = {
  impl_repeated!(@hash_set indexmap_2::IndexSet<T, S>);
};

#[cfg(feature = "heapless_0_8")]
const _: () = {
  impl_repeated!(heapless_0_8::Vec<T, N> [const N: usize]);
  impl_repeated!(@hash_set heapless_0_8::IndexSet<T, S, N> [const N: usize]);
};

#[cfg(feature = "smallvec_1")]
const _: () = {
  impl_repeated!(smallvec_1::SmallVec<[T; N]> [const N: usize]);
};

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  impl_repeated!(triomphe_0_1::Arc<[T]>);
};

#[cfg(feature = "tinyvec_1")]
const _: () = {
  #[cfg(any(feature = "std", feature = "alloc"))]
  impl<T: Default, const N: usize> Repeated for tinyvec_1::TinyVec<[T; N]> {
    type Item<'a>
      = &'a T
    where
      Self: 'a;

    fn len(&self) -> usize {
      self.len()
    }

    fn is_empty(&self) -> bool {
      self.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = Self::Item<'_>> {
      self.as_ref().iter()
    }
  }

  impl<T: Default, const N: usize> Repeated for tinyvec_1::ArrayVec<[T; N]> {
    type Item<'a>
      = &'a T
    where
      Self: 'a;

    fn len(&self) -> usize {
      self.len()
    }

    fn is_empty(&self) -> bool {
      self.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = Self::Item<'_>> {
      self.as_ref().iter()
    }
  }
};
