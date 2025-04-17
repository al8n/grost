use super::{Buffer, Context, DecodeError, UnknownBuffer, Wirable, WireType, debug_assert_read_eq};

/// A trait for types that can be decoded from bytes with a lifetime.
///
/// This trait provides methods to decode data from byte slices,
/// with support for both direct and length-prefixed decoding.
///
/// * `'de` - The lifetime of the input data
pub trait Decode<'de, O>: Wirable {
  /// Decodes an instance of this type from a byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode<UB>(context: &Context, src: &'de [u8]) -> Result<(usize, O), DecodeError>
  where
    Self: Sized + 'de,
    UB: UnknownBuffer<&'de [u8]> + 'de;
}

// Decodes a length-prefixed instance of this type from a byte buffer.
// ///
// /// The function first reads a length prefix, then uses that to determine
// /// how many bytes to consume for the actual data.
// fn decode_length_prefix<B>(
//   src: &'de [u8],
//   unknown_buffer: &mut B,
// ) -> Result<(usize, Self), DecodeError>
// where
//   Self: Sized + 'de,
//   B: UnknownRefBuffer<'de>,
// {
//   if Self::WIRE_TYPE != WireType::LengthDelimited {
//     return Self::decode(context, src, unknown_buffer);
//   }

//   let (mut offset, len) = varing::decode_u32_varint(src)?;
//   let len = len as usize;
//   if len + offset > src.len() {
//     return Err(DecodeError::buffer_underflow());
//   }

//   let src = &src[offset..offset + len];
//   let (bytes_read, value) = Self::decode(src, unknown_buffer)?;

//   #[cfg(debug_assertions)]
//   debug_assert_read_eq::<Self>(bytes_read, len);

//   offset += bytes_read;
//   Ok((offset, value))
// }

/// A marker trait for types that can be decoded without borrowing data.
///
/// Types implementing this trait can be decoded into owned values
/// without maintaining a borrow of the original data.
///
/// This is useful for deserialization scenarios where the input data
/// may not outlive the decoded value.
pub trait DecodeOwned<O>: Decode<'static, O> + 'static {
  /// Decodes an instance of this type from a byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode_owned<B, UB>(context: &Context, src: B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: Buffer + 'static,
    UB: UnknownBuffer<B> + 'static;
}

// /// Decodes a length-prefixed instance of this type from a byte buffer.
// ///
// /// The function first reads a length prefix, then uses that to determine
// /// how many bytes to consume for the actual data.
// #[cfg(any(feature = "std", feature = "alloc"))]
// #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
// fn decode_length_prefix_from_bytes<U>(
//   src: super::bytes::Bytes,
//   unknown_buffer: &mut U,
// ) -> Result<(usize, Self), DecodeError>
// where
//   Self: Sized + 'static,
//   U: UnknownBuffer<super::bytes::Bytes>,
// {
//   if Self::WIRE_TYPE != WireType::LengthDelimited {
//     return Self::decode_from_bytes(src, unknown_buffer);
//   }

//   let (mut offset, len) = varing::decode_u32_varint(&src)?;
//   let len = len as usize;
//   if len + offset > src.len() {
//     return Err(DecodeError::buffer_underflow());
//   }

//   let src = src.slice(offset..offset + len);
//   let (bytes_read, value) = Self::decode_from_bytes(src, unknown_buffer)?;

//   #[cfg(debug_assertions)]
//   debug_assert_read_eq::<Self>(bytes_read, len);

//   offset += bytes_read;
//   Ok((offset, value))
// }
