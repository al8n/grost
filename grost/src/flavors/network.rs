use grost_proto::{
  buffer::Buffer,
  flavors::Flavor,
  unknown::{Unknown, UnknownDecode, UnknownEncode},
};

pub use context::Context;
pub use error::*;
pub use grost_proto::{Tag, flavors::network::WireType};
pub use identifier::Identifier;

mod context;
mod error;
mod identifier;

/// The network flavor
#[derive(Debug, Default, Clone, Copy)]
pub struct Network;

impl Flavor for Network {
  type Context = Context;
  type WireType = WireType;
  type EncodeError = EncodeError;
  type DecodeError = DecodeError;
}

impl<B: ?Sized> UnknownDecode<Network, B> for Unknown<Network, B> {
  fn decode(wire_type: WireType, tag: Tag, buf: B) -> Result<(usize, Self), DecodeError>
  where
    B: Buffer + Sized,
    Self: Sized,
  {
    let identifier = Identifier::new(wire_type, tag);
    macro_rules! slice {
      ($end:ident, $buf_len:ident, $buf:ident) => {{
        if $end == $buf_len {
          $buf
        } else {
          $buf.slice(..$end)
        }
      }};
    }

    macro_rules! consume_fixed {
      ($size:literal, $identifier_len:ident) => {{
        let end = $identifier_len + $size;
        let buf_len = buf.len();
        if end > buf_len {
          return Err(DecodeError::buffer_underflow());
        }

        Ok((
          end,
          Self::new(tag, wire_type, $identifier_len, slice!(end, buf_len, buf)),
        ))
      }};
    }

    let src = buf.as_bytes();
    let identifier_len = identifier.encoded_len();
    let mut offset = 0;
    match identifier.wire_type() {
      WireType::LengthDelimited => {
        if identifier_len >= src.len() {
          return Err(DecodeError::buffer_underflow());
        }

        let (size_len, size) = varing::decode_u32_varint(&src[identifier_len..])?;
        offset += size_len;
        let end = offset + size as usize;

        if end > src.len() {
          return Err(DecodeError::buffer_underflow());
        }

        let data = buf.slice(..end);
        Ok((
          end,
          Self::new(tag, wire_type, identifier_len + size_len, data),
        ))
      }
      WireType::Varint => {
        let buf_len = buf.len();
        if identifier_len >= buf_len {
          return Err(DecodeError::buffer_underflow());
        }

        let size_len = varing::consume_varint(&src[identifier_len..])?;
        let end = identifier_len + size_len;
        Ok((
          end,
          Self::new(tag, wire_type, identifier_len, slice!(end, buf_len, buf)),
        ))
      }
      WireType::Byte => consume_fixed!(1, identifier_len),
      WireType::Fixed16 => consume_fixed!(2, identifier_len),
      WireType::Fixed32 => consume_fixed!(4, identifier_len),
      WireType::Fixed64 => consume_fixed!(8, identifier_len),
      WireType::Fixed128 => consume_fixed!(16, identifier_len),
      WireType::Zst => consume_fixed!(0, identifier_len),
    }
  }
}

impl<B: ?Sized> UnknownEncode<Network, B> for Unknown<Network, B> {
  fn encode(&self, dst: &mut [u8]) -> Result<usize, EncodeError>
  where
    B: Buffer,
  {
    let data = self.raw();
    let data_len = data.len();
    let dst_len = dst.len();
    if data_len > dst_len {
      return Err(EncodeError::insufficient_buffer(data_len, dst_len));
    }

    dst[..data_len].copy_from_slice(data);
    Ok(data_len)
  }

  fn encoded_len(&self) -> usize
  where
    B: Buffer,
  {
    self.raw().len()
  }
}

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
