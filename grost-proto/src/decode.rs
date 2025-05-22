use crate::flavors::WireFormat;

use super::{
  buffer::{Buffer, BytesBuffer},
  error::Error,
  flavors::Flavor,
};

/// A trait for types that can be decoded from bytes with a lifetime.
///
/// This trait provides methods to decode data from byte slices,
/// with support for both direct and length-prefixed decoding.
///
/// * `'de` - The lifetime of the input data
pub trait Decode<'de, F, W, O, B = ()>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Decodes an instance of this type from a byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode(context: &F::Context, src: &'de [u8]) -> Result<(usize, O), F::Error>
  where
    O: Sized + 'de,
    B: Buffer<F::Unknown<&'de [u8]>> + 'de;

  /// Decodes an instance of this type from a length-delimited byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode_length_delimited(context: &F::Context, src: &'de [u8]) -> Result<(usize, O), F::Error>
  where
    O: Sized + 'de,
    B: Buffer<F::Unknown<&'de [u8]>> + 'de,
  {
    let (len_size, len) = varing::decode_u32_varint(src).map_err(Error::from)?;
    let src_len = src.len();
    let len = len as usize;
    let total = len_size + len;
    if total > src_len {
      return Err(Error::buffer_underflow().into());
    }

    if len_size >= src_len {
      return Err(Error::buffer_underflow().into());
    }

    Self::decode(context, &src[len_size..total])
  }
}

/// A trait for types that can be decoded without borrowing data.
///
/// Types implementing this trait can be decoded into owned values
/// without maintaining a borrow of the original data.
///
/// This is useful for deserialization scenarios where the input data
/// may not outlive the decoded value.
pub trait DecodeOwned<F, W, O, B = ()>: Decode<'static, F, W, O, B> + 'static
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Decodes an instance of this type from a byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode_owned<D>(context: &F::Context, src: D) -> Result<(usize, O), F::Error>
  where
    O: Sized + 'static,
    D: BytesBuffer + 'static,
    B: Buffer<F::Unknown<D>> + 'static;

  /// Decodes an instance of this type from a length-delimited byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode_length_delimited_owned<D>(context: &F::Context, src: D) -> Result<(usize, O), F::Error>
  where
    O: Sized + 'static,
    D: BytesBuffer + 'static,
    B: Buffer<F::Unknown<D>> + 'static,
  {
    let bytes = src.as_bytes();
    let (len_size, len) = varing::decode_u32_varint(bytes).map_err(Error::from)?;
    let src_len = src.len();
    let len = len as usize;
    let total = len_size + len;
    if total > src_len {
      return Err(Error::buffer_underflow().into());
    }

    if len_size >= src_len {
      return Err(Error::buffer_underflow().into());
    }

    Self::decode_owned::<D>(context, src.slice(len_size..total))
  }
}

// pub struct Decoded<T, B, UB> {
//   data: T,
//   unknown: Option<UB>,
//   _m: core::marker::PhantomData<B>,
// }

// impl<T, B, UB> Decoded<T, B, UB> {
//   /// Creates a new `Decoded` instance with the given data.
//   #[inline]
//   pub const fn new(data: T) -> Self {
//     Self {
//       data,
//       unknown: None,
//       _m: core::marker::PhantomData,
//     }
//   }

//   /// Gets the data of the decoded message.
//   #[inline]
//   pub const fn data(&self) -> &T {
//     &self.data
//   }

//   /// Gets the unknown data of the decoded message.
//   #[inline]
//   pub const fn unknown_ref(&self) -> Option<&UB> {
//     self.unknown.as_ref()
//   }

//   /// Returns the mutable reference to the unknown data of the decoded message.
//   #[inline]
//   pub const fn unknown_mut(&mut self) -> Option<&mut UB> {
//     self.unknown.as_mut()
//   }

//   /// Sets the unknown data of the decoded message.
//   #[inline]
//   pub fn set_unknown(&mut self, unknown: UB) -> &mut Self {
//     self.unknown = Some(unknown);
//     self
//   }

//   /// Sets the unknown data of the decoded message to `None`.
//   #[inline]
//   pub fn with_unknown(mut self) -> Self {
//     self.unknown = None;
//     self
//   }

//   /// Clears the unknown data of the decoded message.
//   #[inline]
//   pub fn clear_unknown(&mut self) -> &mut Self {
//     self.unknown = None;
//     self
//   }

//   /// Clears the unknown data of the decoded message.
//   #[inline]
//   pub fn without_unknown(mut self) -> Self {
//     self.unknown = None;
//     self
//   }

//   /// Clears the unknown data of the decoded message and returns the data.
//   #[inline]
//   pub const fn take_unknown(&mut self) -> Option<UB> {
//     self.unknown.take()
//   }
// }

#[cfg(any(feature = "std", feature = "alloc", feature = "triomphe_0_1"))]
macro_rules! deref_decode_impl {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<'de, F, W, O, B, T> Decode<'de, F, W, O, B> for $ty
      where
        F: Flavor + ?Sized,
        W: WireFormat<F>,
        T: Decode<'de, F, W, O, B> + ?Sized,
      {
        fn decode(context: &<F as Flavor>::Context, src: &'de [u8]) -> Result<(usize, O), <F as Flavor>::Error>
        where
          O: Sized + 'de,
          B: Buffer<<F as Flavor>::Unknown<&'de [u8]>> + 'de
        {
          T::decode(context, src)
        }

        fn decode_length_delimited(
          context: &<F as Flavor>::Context,
          src: &'de [u8],
        ) -> Result<(usize, O), <F as Flavor>::Error>
        where
          O: Sized + 'de,
          B: Buffer<<F as Flavor>::Unknown<&'de [u8]>> + 'de
        {
          T::decode_length_delimited(context, src)
        }
      }
    )*
  };
}

#[cfg(any(feature = "std", feature = "alloc", feature = "triomphe_0_1"))]
macro_rules! deref_decode_owned_impl {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<F, W, O, B, T> DecodeOwned<F, W, O, B> for $ty
      where
        F: Flavor + ?Sized,
        W: WireFormat<F>,
        T: DecodeOwned<F, W, O, B> + ?Sized,
      {
        fn decode_owned<D>(context: &<F as Flavor>::Context, src: D) -> Result<(usize, O), <F as Flavor>::Error>
        where
          O: Sized + 'static,
          D: BytesBuffer + 'static,
          B: Buffer<<F as Flavor>::Unknown<D>> + 'static
        {
          T::decode_owned(context, src)
        }

        fn decode_length_delimited_owned<D>(
          context: &<F as Flavor>::Context,
          src: D,
        ) -> Result<(usize, O), <F as Flavor>::Error>
        where
          O: Sized + 'static,
          D: BytesBuffer + 'static,
          B: Buffer<<F as Flavor>::Unknown<D>> + 'static
        {
          T::decode_length_delimited_owned(context, src)
        }
      }
    )*
  };
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::{boxed::Box, rc::Rc, sync::Arc};

  deref_decode_impl!(Box<T>, Rc<T>, Arc<T>);
  deref_decode_owned_impl!(Box<T>, Rc<T>, Arc<T>);
};

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  use triomphe_0_1::Arc;

  deref_decode_impl!(Arc<T>);
  deref_decode_owned_impl!(Arc<T>);
};
