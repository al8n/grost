#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(
  not(any(feature = "std", feature = "alloc")),
  not(feature = "heapless")
))]
compile_error!("`heapless` feature must be enabled when both `std` and `alloc` are disabled");

pub use buffer::Buffer;
pub use error::{DecodeError, EncodeError};
pub use impls::*;
pub use selection_set::SelectionSet;
pub use tag::Tag;
#[cfg(any(feature = "std", feature = "alloc"))]
pub use tinyvec;
pub use unknown::*;
pub use wire_type::WireType;

#[cfg(feature = "bytes_1")]
pub use bytes_1 as bytes;

mod buffer;
/// The error module contains all the error types used in the Graph RPC.
mod error;
mod impls;
mod selection_set;
mod tag;
mod unknown;
mod wire_type;

/// A trait defining the core type components for Graph RPC queries.
///
/// This trait specifies the associated types that work together to form
/// a complete Graph RPC query system, including selection, output, filtering,
/// and ordering capabilities.
pub trait Type {
  /// The selection type for a Graph RPC query.
  ///
  /// This type defines which fields should be returned in the query results.
  /// It must be serializable for transmission and clonable for manipulation.
  type Selection: OutputType + Clone;

  /// The output type for a Graph RPC query.
  ///
  /// This represents the structured data returned from a query operation.
  type Output: OutputType;

  /// The filter type for a Graph RPC query.
  ///
  /// This type defines criteria to determine which results should be included
  /// or excluded from the query results.
  type Filter: OutputType + Clone;

  /// The order by type for a Graph RPC query.
  ///
  /// This type defines how results should be sorted when returned.
  /// It must be clonable for efficient manipulation.
  type OrderBy: OutputType + Clone;
}

/// A trait for types that can be used as output from Graph RPC queries.
///
/// This trait defines how output types can be serialized, deserialized,
/// borrowed, and converted between different representations.
///
/// * `Serialized<'a>` - A serialized representation with lifetime 'a
/// * `Borrowed<'a>` - A borrowed view with lifetime 'a
/// * `SerializedOwned` - An owned serialized representation
pub trait OutputType: Serialize {
  /// A serialized representation of this type with lifetime 'a.
  ///
  /// This type can be converted back to the original type and deserialized from raw bytes.
  type Serialized<'a>: Copy + TypeRef<Self> + Serialize + Deserialize<'a>
  where
    Self: Sized + 'a;

  /// A borrowed view of this type with lifetime 'a.
  ///
  /// This type provides a non-owned view that can be created from a reference
  /// and serialized when needed.
  type Borrowed<'a>: Copy + From<&'a Self> + Serialize
  where
    Self: 'a;

  /// An owned serialized representation of this type.
  type SerializedOwned: Clone + TypeOwned<Self> + Serialize + DeserializeOwned
  where
    Self: Sized + 'static;
}

/// A trait for types that can be converted to another type.
///
/// This trait enables bidirectional conversion between serialized
/// representations and their corresponding deserialized types.
///
/// * `T` - The target type to convert to
pub trait TypeRef<T> {
  /// Converts a reference of this type to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn to_target(&self) -> Result<T, DecodeError>;

  /// Consumes this type and converts it to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn into_target(self) -> Result<T, DecodeError>;
}

/// A trait for types that can be converted to another type.
///
/// This trait enables bidirectional conversion between serialized
/// representations and their corresponding deserialized types.
///
/// * `T` - The target type to convert to
pub trait TypeOwned<T> {
  /// Converts a reference of this type to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn to_target(&self) -> Result<T, DecodeError>;

  /// Consumes this type and converts it to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn into_target(self) -> Result<T, DecodeError>;
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

/// A trait for types that can be deserialized from bytes with a lifetime.
///
/// This trait provides methods to decode data from byte slices,
/// with support for both direct and length-prefixed decoding.
///
/// * `'de` - The lifetime of the input data
pub trait Deserialize<'de>: Wirable {
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

/// A marker trait for types that can be deserialized without borrowing data.
///
/// Types implementing this trait can be deserialized into owned values
/// without maintaining a borrow of the original data.
///
/// This is useful for deserialization scenarios where the input data
/// may not outlive the deserialized value.
pub trait DeserializeOwned: Deserialize<'static> + 'static {
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
pub trait Serialize: Wirable {
  /// Encodes the message into the provided buffer.
  ///
  /// Returns the number of bytes written to the buffer or an error if the operation fails.
  ///
  /// [`Serialize::encoded_len`] can be used to determine the required buffer size.
  fn encode(&self, tag: Tag, buf: &mut [u8]) -> Result<usize, EncodeError>;

  /// Returns the number of bytes needed to encode the message.
  ///
  /// This is used to determine the buffer size required for encoding.
  fn encoded_len(&self, tag: Tag) -> usize;

  /// Returns the encoded length of the data including the length delimiter prefix.
  ///
  /// For `WireType::LengthDelimited`, this includes the varint-encoded length
  /// prefix followed by the actual data length. For other wire types, this is
  /// equivalent to [`Serialize::encoded_len`].
  fn encoded_len_with_prefix(&self, tag: Tag) -> usize {
    let len = self.encoded_len(tag);
    match Self::WIRE_TYPE {
      WireType::LengthDelimited => varing::encoded_u32_varint_len(len as u32) + len,
      _ => len,
    }
  }

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_vec(&self, tag: Tag) -> Result<std::vec::Vec<u8>, error::EncodeError> {
    let mut buf = std::vec![0; self.encoded_len(tag)];
    self.encode(tag, &mut buf)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_bytes(&self, tag: Tag) -> Result<bytes::Bytes, EncodeError> {
    self.encode_to_vec(tag).map(Into::into)
  }

  /// Encodes the message with a length-delimiter prefix to a buffer.
  ///
  /// For `WireType::LengthDelimited`, this prepends a varint-encoded length
  /// before the message data. For other wire types, this behaves the same as [`Serialize::encode`].
  ///
  /// An error will be returned if the buffer does not have sufficient capacity.
  fn encode_with_prefix(&self, tag: Tag, buf: &mut [u8]) -> Result<usize, EncodeError> {
    if Self::WIRE_TYPE != WireType::LengthDelimited {
      return self.encode(tag, buf);
    }

    let len = self.encoded_len(tag);
    if len > u32::MAX as usize {
      return Err(EncodeError::TooLarge);
    }

    let mut offset = 0;
    offset += varing::encode_u32_varint_to(len as u32, buf)?;
    offset += self.encode(tag, &mut buf[offset..])?;

    #[cfg(debug_assertions)]
    debug_assert_write_eq::<Self>(offset, self.encoded_len_with_prefix(tag));

    Ok(offset)
  }

  /// Encodes the message with a length-delimiter into a new [`std::vec::Vec<u8>`].
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_vec_with_prefix(&self, tag: Tag) -> Result<::std::vec::Vec<u8>, EncodeError> {
    let len = self.encoded_len_with_prefix(tag);
    let mut vec = ::std::vec![0; len];
    self.encode_with_prefix(tag, &mut vec).map(|_| vec)
  }

  /// Encodes the message with a length-delimiter into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_bytes_with_prefix(&self, tag: Tag) -> Result<bytes::Bytes, EncodeError> {
    self.encode_to_vec_with_prefix(tag).map(Into::into)
  }
}

impl<'a, T> Wirable for &'a T
where
  T: Wirable + ?Sized,
{
  const WIRE_TYPE: WireType = T::WIRE_TYPE;
}

impl<'a, T> Serialize for &'a T
where
  T: Serialize + ?Sized,
{
  fn encode(&self, tag: Tag, buf: &mut [u8]) -> Result<usize, EncodeError> {
    (*self).encode(tag, buf)
  }

  fn encoded_len(&self, tag: Tag) -> usize {
    (*self).encoded_len(tag)
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
    WireType::Merged => offset,
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
