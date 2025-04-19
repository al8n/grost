use super::{buffer::Buffer, flavors::Flavor};

/// A buffer that stores the [`Unknown`] data type.
///
/// This trait is used for forward and backward compatibility for structs.
pub trait UnknownBuffer<F: Flavor, B: ?Sized> {
  /// Creates a new buffer.
  fn new() -> Self;

  /// Pushes the unknown data type to the buffer, if the buffer is full,
  /// the given value will be returned back.
  fn push(&mut self, value: Unknown<F, B>) -> Option<Unknown<F, B>>
  where
    B: Sized;

  /// Returns the capacity of the buffer.
  fn capacity(&self) -> usize;

  /// Returns the length of the buffer.
  fn len(&self) -> usize;

  /// Returns a slice of the unknown data type.
  fn as_slice(&self) -> &[Unknown<F, B>]
  where
    B: Sized;

  /// Returns `true` if the buffer is empty.
  fn is_empty(&self) -> bool {
    self.len() == 0
  }
}

impl<B: ?Sized, F: Flavor> UnknownBuffer<F, B> for () {
  fn new() -> Self {}

  fn push(&mut self, value: Unknown<F, B>) -> Option<Unknown<F, B>>
  where
    B: Sized,
  {
    Some(value)
  }

  fn capacity(&self) -> usize {
    0
  }

  fn len(&self) -> usize {
    0
  }

  fn as_slice(&self) -> &[Unknown<F, B>]
  where
    B: Sized,
  {
    &[]
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::vec::Vec;

  impl<F: Flavor, B> UnknownBuffer<F, B> for Vec<Unknown<F, B>> {
    fn new() -> Self {
      Vec::new()
    }

    fn push(&mut self, value: Unknown<F, B>) -> Option<Unknown<F, B>> {
      self.push(value);
      None
    }

    fn capacity(&self) -> usize {
      self.capacity()
    }

    fn len(&self) -> usize {
      self.len()
    }

    fn as_slice(&self) -> &[Unknown<F, B>]
    where
      B: Sized,
    {
      self.as_slice()
    }

    fn is_empty(&self) -> bool {
      self.is_empty()
    }
  }
};

#[cfg(feature = "heapless_0_8")]
const _: () = {
  use heapless_0_8::Vec;

  impl<F: Flavor, B, const N: usize> UnknownBuffer<F, B> for Vec<Unknown<F, B>, N> {
    fn new() -> Self {
      Vec::new()
    }

    fn push(&mut self, value: Unknown<F, B>) -> Option<Unknown<F, B>> {
      self.push(value).err()
    }

    fn capacity(&self) -> usize {
      self.capacity()
    }

    fn as_slice(&self) -> &[Unknown<F, B>]
    where
      B: Sized,
    {
      self.as_ref()
    }

    fn len(&self) -> usize {
      self.as_slice().len()
    }

    fn is_empty(&self) -> bool {
      self.is_empty()
    }
  }
};

/// The trait for encoding unknown data type.
pub trait UnknownEncode<F: Flavor, B: ?Sized>: sealed::Sealed {
  /// Encodes the unknown data type.
  fn encode(&self, dst: &mut [u8]) -> Result<usize, F::EncodeError>
  where
    B: Buffer;

  /// Returns the length of the encoded data.
  fn encoded_len(&self) -> usize
  where
    B: Buffer;
}

/// The trait for encoding unknown data type.
pub trait UnknownDecode<F: Flavor, B: ?Sized>: sealed::Sealed {
  /// Decodes the unknown data type.
  fn decode(wire_type: F::WireType, tag: F::Tag, buf: B) -> Result<(usize, Self), F::DecodeError>
  where
    B: Buffer + Sized,
    Self: Sized;
}

mod sealed {
  pub trait Sealed {}

  impl<B: ?Sized, F: super::Flavor> Sealed for super::Unknown<F, B> {}
}

/// The unknown type, used for forward and backward compatibility.
/// The data is stored as a byte array, including the wire type and the tag,
/// and the data.
///
/// When the older version trying to decode the newer version, for the tag that
/// is not recognized, it will be stored in this variant. And when the older version
/// trying to forward the data, the data stored in this variant will be forwarded
/// as is.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Unknown<F: Flavor, B: ?Sized> {
  tag: F::Tag,
  wire_type: F::WireType,
  data_offset: usize,
  data: B,
}

impl<B: ?Sized, F: Flavor> Unknown<F, B> {
  /// Creates a new unknown
  #[inline]
  pub const fn new(tag: F::Tag, wire_type: F::WireType, data_offset: usize, data: B) -> Self
  where
    B: Sized,
  {
    Self {
      tag,
      wire_type,
      data_offset,
      data,
    }
  }

  /// Returns the tag of the unknown data type.
  #[inline]
  pub const fn tag(&self) -> F::Tag {
    self.tag
  }

  /// Returns the wire type of the unknown data type.
  #[inline]
  pub const fn wire_type(&self) -> F::WireType {
    self.wire_type
  }

  /// Returns the actual data of the unknown data type.
  ///
  /// Note: The data does not include the wire type and the tag.
  /// If you want to access the original data, use [`raw`] instead.
  #[inline]
  pub fn data(&self) -> &[u8]
  where
    B: Buffer,
  {
    &self.data.as_bytes()[self.data_offset..]
  }

  /// Returns the raw data of the unknown data type.
  ///
  /// Note: The data includes the wire type and the tag.
  /// If you want to access the actual data, use [`data`] instead.
  #[inline]
  pub fn raw(&self) -> &[u8]
  where
    B: Buffer,
  {
    self.data.as_bytes()
  }

  // /// Encodes the unknown data type.
  // pub fn encode(&self, dst: &mut [u8]) -> Result<usize, EncodeError>
  // where
  //   B: AsRef<[u8]>,
  // {
  //   let data = self.data.as_ref();
  //   let data_len = data.len();
  //   let dst_len = dst.len();
  //   if data_len > dst_len {
  //     return Err(EncodeError::insufficient_buffer(data_len, dst_len));
  //   }

  //   dst[..data_len].copy_from_slice(data);
  //   Ok(data_len)
  // }

  // /// Decodes the unknown data type.
  // pub fn decode(identifier: Identifier, buf: B) -> Result<(usize, Self), DecodeError>
  // where
  //   B: Buffer + Sized,
  // {

  // }

  /// Returns the length of the encoded data.
  #[inline]
  pub fn encoded_len(&self) -> usize
  where
    B: AsRef<[u8]>,
  {
    self.data.as_ref().len()
  }

  /// Converts the unknown data type to an borrowed type.
  #[inline]
  pub const fn as_ref(&self) -> Unknown<F, &B> {
    Unknown {
      tag: self.tag,
      wire_type: self.wire_type,
      data_offset: self.data_offset,
      data: &self.data,
    }
  }
}

impl<F, B> Unknown<F, B>
where
  B: Buffer,
  F: Flavor,
{
  /// Converts the `Unknown<B>` to `Unknown<N>`.
  pub fn map<'a, N>(&'a self) -> Unknown<F, N>
  where
    N: From<&'a [u8]>,
  {
    Unknown {
      tag: self.tag,
      wire_type: self.wire_type,
      data_offset: self.data_offset,
      data: N::from(self.raw()),
    }
  }

  /// Converts the `Unknown<B>` to `Unknown<N>`.
  pub fn consume_map<N>(self) -> Unknown<F, N>
  where
    N: From<B>,
  {
    Unknown {
      tag: self.tag,
      wire_type: self.wire_type,
      data_offset: self.data_offset,
      data: N::from(self.data),
    }
  }
}
