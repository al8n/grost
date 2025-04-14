#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

pub use error::*;
pub use identifier::Identifier;
pub use tag::Tag;
pub use wire_type::WireType;

mod error;
mod identifier;
mod tag;
mod wire_type;

/// Skip a field from the Graph protocol buffer
#[inline]
pub const fn skip(src: &[u8]) -> Result<usize, DecodeError> {
  let buf_len = src.len();
  if buf_len == 0 {
    return Ok(0);
  }

  let mut offset = 0;
  let (bytes_read, identifier) = match Identifier::decode_inner(src) {
    Ok((bytes_read, identifier)) => (bytes_read, identifier),
    Err(e) => return Err(DecodeError::from_varint_error(e)),
  };
  offset += bytes_read;

  let (_, src) = src.split_at(offset);
  let val = match identifier.wire_type() {
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
