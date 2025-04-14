#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

pub use buffer::Buffer;
pub use error::{DecodeError, EncodeError};

pub use impls::*;
pub use selection_set::SelectionSet;
pub use tag::Tag;

pub use unknown::*;
pub use wire_type::WireType;

#[cfg(feature = "bytes_1")]
pub use bytes_1 as bytes;

#[cfg(feature = "smol_str_0_3")]
pub use smol_str_0_3 as smol_str;

#[cfg(feature = "tinyvec_1")]
pub use tinyvec_1 as tinyvec;

mod buffer;
/// The error module contains all the error types used in the `Grost`.
mod error;
#[macro_use]
mod macros;
/// Traits implemented for primitive types and common types.
mod impls;
mod selection_set;
mod tag;
mod unknown;
mod utils;
mod wire_type;

/// A message type that can be encoded and decoded.
///
/// This trait defines how output types can be encoded, decoded,
/// borrowed, and converted between different representations.
///
/// * `Encoded<'a>` - A encoded representation with lifetime 'a
/// * `Borrowed<'a>` - A borrowed view with lifetime 'a
/// * `EncodedOwned` - An owned encoded representation
pub trait Message: Encode {
  /// A encoded representation of this type with lifetime 'a.
  ///
  /// This type can be converted back to the original type and decoded from raw bytes.
  type Encoded<'a>: Copy + TypeRef<Self> + Encode + Decode<'a>
  where
    Self: Sized + 'a;

  /// A borrowed view of this type with lifetime 'a.
  ///
  /// This type provides a non-owned view that can be created from a reference
  /// and encoded when needed.
  type Borrowed<'a>: Copy + TypeBorrowed<'a, Self> + Encode
  where
    Self: 'a;

  /// An owned encoded representation of this type.
  type EncodedOwned: Clone + TypeOwned<Self> + Encode + DecodeOwned
  where
    Self: Sized + 'static;
}

/// A trait for consuming `Self` and converting it to `T`.
pub trait IntoTarget<T> {
  /// Consumes this type and converts it to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn into_target(self) -> Result<T, DecodeError>;
}

/// A trait for types that can be converted to another type.
///
/// This trait enables bidirectional conversion between encoded
/// representations and their corresponding decoded types.
///
/// * `T` - The target type to convert to
pub trait TypeRef<T>: IntoTarget<T> {
  /// Converts a reference of this type to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn to(&self) -> Result<T, DecodeError>;
}

/// A trait for types that can be converted to another type.
///
/// This trait enables bidirectional conversion between encoded
/// representations and their corresponding decoded types.
///
/// * `T` - The target type to convert to
pub trait TypeBorrowed<'a, T: ?Sized> {
  /// Converts a reference of this type to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn from_borrow(val: &'a T) -> Self;
}

impl<'a, T: ?Sized> TypeBorrowed<'a, T> for &'a T {
  fn from_borrow(val: &'a T) -> Self {
    val
  }
}

/// A trait for types that can be converted to another type.
///
/// This trait enables bidirectional conversion between encoded
/// representations and their corresponding decoded types.
///
/// * `T` - The target type to convert to
pub trait TypeOwned<T>: IntoTarget<T> {
  /// Converts a reference of this type to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn to(&self) -> Result<T, DecodeError>;
}

/// A trait for types that can be encoded with a specific wire format.
///
/// This trait defines the wire encoding format to be used when serializing a type.
/// It serves as a foundation for serialization traits by specifying how data should
/// be represented on the wire.
pub trait Wirable {
  /// The wire type of the data, which determines how the data is encoded.
  ///
  /// Defaults to [`WireType::LengthDelimited`].
  const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

/// A trait for types that can be decoded from bytes with a lifetime.
///
/// This trait provides methods to decode data from byte slices,
/// with support for both direct and length-prefixed decoding.
///
/// * `'de` - The lifetime of the input data
pub trait Decode<'de>: Wirable {
  /// Decodes an instance of this type from a byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode<B>(src: &'de [u8], unknown_buffer: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownRefBuffer<'de>;

  /// Decodes a length-prefixed instance of this type from a byte buffer.
  ///
  /// The function first reads a length prefix, then uses that to determine
  /// how many bytes to consume for the actual data.
  fn decode_length_prefix<B>(
    src: &'de [u8],
    unknown_buffer: &mut B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownRefBuffer<'de>,
  {
    if Self::WIRE_TYPE != WireType::LengthDelimited {
      return Self::decode(src, unknown_buffer);
    }

    let (mut offset, len) = varing::decode_u32_varint(src)?;
    let len = len as usize;
    if len + offset > src.len() {
      return Err(DecodeError::buffer_underflow());
    }

    let src = &src[offset..offset + len];
    let (bytes_read, value) = Self::decode(src, unknown_buffer)?;

    #[cfg(debug_assertions)]
    debug_assert_read_eq::<Self>(bytes_read, len);

    offset += bytes_read;
    Ok((offset, value))
  }
}

/// A marker trait for types that can be decoded without borrowing data.
///
/// Types implementing this trait can be decoded into owned values
/// without maintaining a borrow of the original data.
///
/// This is useful for deserialization scenarios where the input data
/// may not outlive the decoded value.
pub trait DecodeOwned: Decode<'static> + 'static {
  /// Decodes an instance of this type from a byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  #[cfg(any(feature = "std", feature = "alloc"))]
  #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
  fn decode_from_bytes<U>(
    src: bytes::Bytes,
    unknown_buffer: &mut U,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    U: UnknownBuffer<bytes::Bytes>;

  /// Decodes a length-prefixed instance of this type from a byte buffer.
  ///
  /// The function first reads a length prefix, then uses that to determine
  /// how many bytes to consume for the actual data.
  #[cfg(any(feature = "std", feature = "alloc"))]
  #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
  fn decode_length_prefix_from_bytes<U>(
    src: bytes::Bytes,
    unknown_buffer: &mut U,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    U: UnknownBuffer<bytes::Bytes>,
  {
    if Self::WIRE_TYPE != WireType::LengthDelimited {
      return Self::decode_from_bytes(src, unknown_buffer);
    }

    let (mut offset, len) = varing::decode_u32_varint(&src)?;
    let len = len as usize;
    if len + offset > src.len() {
      return Err(DecodeError::buffer_underflow());
    }

    let src = src.slice(offset..offset + len);
    let (bytes_read, value) = Self::decode_from_bytes(src, unknown_buffer)?;

    #[cfg(debug_assertions)]
    debug_assert_read_eq::<Self>(bytes_read, len);

    offset += bytes_read;
    Ok((offset, value))
  }
}

/// A trait for serializing data to binary format with support for various wire types.
///
/// This trait provides methods to encode data into binary representations,
/// calculate required buffer sizes, and handle length-delimited encoding.
pub trait Encode: Wirable {
  /// Encodes the message into the provided buffer.
  ///
  /// Returns the number of bytes written to the buffer or an error if the operation fails.
  ///
  /// [`Encode::encoded_len`] can be used to determine the required buffer size.
  fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError>;

  /// Returns the number of bytes needed to encode the message.
  ///
  /// This is used to determine the buffer size required for encoding.
  fn encoded_len(&self) -> usize;

  /// Returns the encoded length of the data including the length delimiter prefix.
  ///
  /// For `WireType::LengthDelimited`, this includes the varint-encoded length
  /// prefix followed by the actual data length. For other wire types, this is
  /// equivalent to [`Encode::encoded_len`].
  fn encoded_len_with_prefix(&self) -> usize {
    let len = self.encoded_len();
    match Self::WIRE_TYPE {
      WireType::LengthDelimited => varing::encoded_u32_varint_len(len as u32) + len,
      _ => len,
    }
  }

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_vec(&self) -> Result<std::vec::Vec<u8>, error::EncodeError> {
    let mut buf = std::vec![0; self.encoded_len()];
    self.encode(&mut buf)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_bytes(&self) -> Result<bytes::Bytes, EncodeError> {
    self.encode_to_vec().map(Into::into)
  }

  /// Encodes the message with a length-delimiter prefix to a buffer.
  ///
  /// For `WireType::LengthDelimited`, this prepends a varint-encoded length
  /// before the message data. For other wire types, this behaves the same as [`Encode::encode`].
  ///
  /// An error will be returned if the buffer does not have sufficient capacity.
  fn encode_with_prefix(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
    if Self::WIRE_TYPE != WireType::LengthDelimited {
      return self.encode(buf);
    }

    let len = self.encoded_len();
    if len > u32::MAX as usize {
      return Err(EncodeError::TooLarge);
    }

    let mut offset = 0;
    offset += varing::encode_u32_varint_to(len as u32, buf)?;
    offset += self.encode(&mut buf[offset..])?;

    #[cfg(debug_assertions)]
    debug_assert_write_eq::<Self>(offset, self.encoded_len_with_prefix());

    Ok(offset)
  }

  /// Encodes the message with a length-delimiter into a new [`std::vec::Vec<u8>`].
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_vec_with_prefix(&self) -> Result<::std::vec::Vec<u8>, EncodeError> {
    let len = self.encoded_len_with_prefix();
    let mut vec = ::std::vec![0; len];
    self.encode_with_prefix(&mut vec).map(|_| vec)
  }

  /// Encodes the message with a length-delimiter into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_bytes_with_prefix(&self) -> Result<bytes::Bytes, EncodeError> {
    self.encode_to_vec_with_prefix().map(Into::into)
  }
}

/// A trait for serializing data to binary format with support for various wire types.
///
/// This trait provides methods to encode data into binary representations,
/// calculate required buffer sizes, and handle length-delimited encoding.
pub trait PartialEncode: Wirable {
  /// The selection type for the message, which determines which fields to include
  /// in the encoded output.
  type Selection;

  /// Encodes the message into the provided buffer.
  ///
  /// Returns the number of bytes written to the buffer or an error if the operation fails.
  ///
  /// [`Encode::encoded_len`] can be used to determine the required buffer size.
  fn partial_encode(
    &self,
    buf: &mut [u8],
    selection: &Self::Selection,
  ) -> Result<usize, EncodeError>;

  /// Returns the number of bytes needed to encode the message.
  ///
  /// This is used to determine the buffer size required for encoding.
  fn partial_encoded_len(&self, selection: &Self::Selection) -> usize;

  /// Returns the encoded length of the data including the length delimiter prefix.
  ///
  /// For `WireType::LengthDelimited`, this includes the varint-encoded length
  /// prefix followed by the actual data length. For other wire types, this is
  /// equivalent to [`Encode::encoded_len`].
  fn partial_encoded_len_with_prefix(&self, selection: &Self::Selection) -> usize {
    let len = self.partial_encoded_len(selection);
    match Self::WIRE_TYPE {
      WireType::LengthDelimited => varing::encoded_u32_varint_len(len as u32) + len,
      _ => len,
    }
  }

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_to_vec(
    &self,
    selection: &Self::Selection,
  ) -> Result<std::vec::Vec<u8>, error::EncodeError> {
    let mut buf = std::vec![0; self.partial_encoded_len(selection)];
    self.partial_encode(&mut buf, selection)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_to_bytes(
    &self,
    selection: &Self::Selection,
  ) -> Result<bytes::Bytes, EncodeError> {
    self.partial_encode_to_vec(selection).map(Into::into)
  }

  /// Encodes the message with a length-delimiter prefix to a buffer.
  ///
  /// For `WireType::LengthDelimited`, this prepends a varint-encoded length
  /// before the message data. For other wire types, this behaves the same as [`Encode::encode`].
  ///
  /// An error will be returned if the buffer does not have sufficient capacity.
  fn partial_encode_with_prefix(
    &self,
    buf: &mut [u8],
    selection: &Self::Selection,
  ) -> Result<usize, EncodeError> {
    if Self::WIRE_TYPE != WireType::LengthDelimited {
      return self.partial_encode(buf, selection);
    }

    let len = self.partial_encoded_len(selection);
    if len > u32::MAX as usize {
      return Err(EncodeError::TooLarge);
    }

    let mut offset = 0;
    offset += varing::encode_u32_varint_to(len as u32, buf)?;
    offset += self.partial_encode(&mut buf[offset..], selection)?;

    #[cfg(debug_assertions)]
    debug_assert_write_eq::<Self>(offset, self.partial_encoded_len_with_prefix(selection));

    Ok(offset)
  }

  /// Encodes the message with a length-delimiter into a new [`std::vec::Vec<u8>`].
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_to_vec_with_prefix(
    &self,
    selection: &Self::Selection,
  ) -> Result<::std::vec::Vec<u8>, EncodeError> {
    let len = self.partial_encoded_len_with_prefix(selection);
    let mut vec = ::std::vec![0; len];
    self
      .partial_encode_with_prefix(&mut vec, selection)
      .map(|_| vec)
  }

  /// Encodes the message with a length-delimiter into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_to_bytes_with_prefix(
    &self,
    selection: &Self::Selection,
  ) -> Result<bytes::Bytes, EncodeError> {
    self
      .partial_encode_to_vec_with_prefix(selection)
      .map(Into::into)
  }
}

impl<T> Wirable for &T
where
  T: Wirable + ?Sized,
{
  const WIRE_TYPE: WireType = T::WIRE_TYPE;
}

impl<T> Encode for &T
where
  T: Encode + ?Sized,
{
  fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
    (*self).encode(buf)
  }

  fn encoded_len(&self) -> usize {
    (*self).encoded_len()
  }
}

impl<T> PartialEncode for &T
where
  T: PartialEncode + ?Sized,
{
  type Selection = T::Selection;

  fn partial_encode(
    &self,
    buf: &mut [u8],
    selection: &Self::Selection,
  ) -> Result<usize, EncodeError> {
    (*self).partial_encode(buf, selection)
  }

  fn partial_encoded_len(&self, selection: &Self::Selection) -> usize {
    (*self).partial_encoded_len(selection)
  }
}

/// Merge wire type and tag into a byte.
#[inline]
pub const fn merge(ty: WireType, tag: Tag) -> u32 {
  (tag.get() << 3) | (ty as u8 as u32)
}

/// Split a merged `u32` into wire type and tag.
#[inline]
pub const fn split(val: u32) -> (WireType, Tag) {
  let wire_type = val & 0b111; // Get last 3 bits for wire type
  let tag = val >> 3; // Shift right to get the tag
  // Using from_u8_unchecked since we know wire_type is within 0-7
  (WireType::from_u8_unchecked(wire_type as u8), Tag(tag))
}

/// Skip a field in the buffer.
#[inline]
pub const fn skip(src: &[u8]) -> Result<usize, DecodeError> {
  let buf_len = src.len();
  if buf_len == 0 {
    return Ok(0);
  }

  let mut offset = 0;
  let (wire_type, _) = match varing::decode_u32_varint(src) {
    Ok((bytes_read, val)) => {
      offset += bytes_read;
      split(val)
    }
    Err(e) => return Err(DecodeError::from_varint_error(e)),
  };

  let (_, src) = src.split_at(offset);
  let val = match wire_type {
    WireType::Varint => match varing::consume_varint(src) {
      Ok(bytes_read) => offset + bytes_read,
      Err(e) => return Err(DecodeError::from_varint_error(e)),
    },
    WireType::LengthDelimited => {
      // Skip length-delimited field by reading the length and skipping the payload
      if src.is_empty() {
        return Err(DecodeError::buffer_underflow());
      }

      match varing::decode_u32_varint(src) {
        Ok((bytes_read, length)) => offset + bytes_read + length as usize,
        Err(e) => return Err(DecodeError::from_varint_error(e)),
      }
    }
    WireType::Byte => offset + 1,
    WireType::Fixed16 => offset + 2,
    WireType::Fixed32 => offset + 4,
    WireType::Fixed64 => offset + 8,
    WireType::Fixed128 => offset + 16,
    WireType::Zst => offset,
  };

  if val > buf_len {
    return Ok(buf_len);
  }

  Ok(val)
}

#[doc(hidden)]
#[cfg(debug_assertions)]
#[inline]
pub fn debug_assert_write_eq<T: ?Sized>(actual: usize, expected: usize) {
  debug_assert_eq!(
    actual,
    expected,
    "{}: expect writting {expected} bytes, but actual write {actual} bytes",
    core::any::type_name::<T>()
  );
}

#[doc(hidden)]
#[cfg(debug_assertions)]
#[inline]
pub fn debug_assert_read_eq<T: ?Sized>(actual: usize, expected: usize) {
  debug_assert_eq!(
    actual,
    expected,
    "{}: expect reading {expected} bytes, but actual read {actual} bytes",
    core::any::type_name::<T>()
  );
}

#[doc(hidden)]
pub mod __private {
  pub use super::*;
  pub use either;
  pub use varing;

  #[cfg(feature = "bytes")]
  pub use bytes_1 as bytes;
  #[cfg(feature = "smol_str")]
  pub use smol_str_0_3 as smol_str;

  #[cfg(not(feature = "simdutf8"))]
  pub use ::core::str::from_utf8;
  #[cfg(feature = "simdutf8")]
  pub use simdutf8::basic::from_utf8;

  #[cfg(feature = "quickcheck")]
  pub use quickcheck;
  #[cfg(feature = "arbitrary")]
  pub use arbitrary;

  pub use memchr;

  pub use thiserror;

  #[cfg(not(any(feature = "std", feature = "alloc")))]
  pub fn larger_than_str_capacity<const N: usize>() -> crate::DecodeError {
    crate::DecodeError::custom("cannot decode string with length greater than the capacity")
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  pub fn larger_than_str_capacity<const N: usize>() -> crate::DecodeError {
    crate::DecodeError::custom(std::format!(
      "cannot decode string with length greater than the capacity {N}"
    ))
  }

  #[cfg(not(any(feature = "std", feature = "alloc")))]
  pub fn larger_than_array_capacity<const N: usize>() -> crate::DecodeError {
    crate::DecodeError::custom("cannot decode array with length greater than the capacity")
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  pub fn larger_than_array_capacity<const N: usize>() -> crate::DecodeError {
    crate::DecodeError::custom(std::format!(
      "cannot decode array with length greater than the capacity {N}"
    ))
  }
}
