pub use context::Context;
pub use error::{DecodeError, EncodeError};
pub use identifier::Identifier;
pub use tag::Tag;
pub use unknown::Unknown;
pub use wire_type::*;

use super::Flavor;
use crate::buffer::BytesBuffer;

mod context;
mod error;
mod identifier;
mod impls;
mod tag;
mod wire_type;

/// The unknown data types
mod unknown;
/// The network flavor
#[derive(
  Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display,
)]
#[display("Network")]
pub struct Network;

impl Flavor for Network {
  type Context = Context;
  type Identifier = Identifier;
  type WireType = WireType;
  type Tag = Tag;
  type EncodeError = EncodeError;
  type DecodeError = DecodeError;
  type Unknown<B> = Unknown<B>;

  const NAME: &'static str = "Network";

  fn encode_unknown<B>(
    _: &Self::Context,
    value: &Self::Unknown<B>,
    buf: &mut [u8],
  ) -> Result<usize, Self::EncodeError>
  where
    B: BytesBuffer + Sized,
  {
    let value_bytes = value.raw();
    let value_len = value_bytes.len();
    if value_len > buf.len() {
      return Err(EncodeError::insufficient_buffer(value_len, buf.len()));
    }

    buf[..value_len].copy_from_slice(value_bytes);
    Ok(value_len)
  }

  fn encoded_unknown_len<B>(_: &Self::Context, value: &Self::Unknown<B>) -> usize
  where
    B: BytesBuffer,
  {
    value.raw().len()
  }

  fn decode_unknown<B>(
    _: &Self::Context,
    buf: B,
  ) -> Result<(usize, Self::Unknown<B>), Self::DecodeError>
  where
    B: BytesBuffer + Sized,
  {
    let src = buf.as_bytes();
    let (identifier_len, identifier) = Identifier::decode(src)?;
    let (wire_type, tag) = identifier.into_components();
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
      ($size:literal, $offset:ident, $buf_len:ident) => {{
        let end = $offset + $size;
        if end > $buf_len {
          return Err(DecodeError::buffer_underflow());
        }

        Ok((
          end,
          Unknown::new(tag, wire_type, $offset, slice!(end, $buf_len, buf)),
        ))
      }};
    }

    let mut offset = identifier_len;
    let buf_len = src.len();
    match identifier.wire_type() {
      WireType::LengthDelimited => {
        if offset >= buf_len {
          return Err(DecodeError::buffer_underflow());
        }

        let (size_len, size) = varing::decode_u32_varint(&src[offset..])?;
        offset += size_len;
        let end = offset + size as usize;

        if end > buf_len {
          return Err(DecodeError::buffer_underflow());
        }

        Ok((
          end,
          Unknown::new(tag, wire_type, offset, slice!(end, buf_len, buf)),
        ))
      }
      WireType::Varint => {
        if offset >= buf_len {
          return Err(DecodeError::buffer_underflow());
        }

        let size_len = varing::consume_varint(&src[offset..])?;
        let end = offset + size_len;
        Ok((
          end,
          Unknown::new(tag, wire_type, offset, slice!(end, buf_len, buf)),
        ))
      }
      WireType::Fixed8 => consume_fixed!(1, offset, buf_len),
      WireType::Fixed16 => consume_fixed!(2, offset, buf_len),
      WireType::Fixed32 => consume_fixed!(4, offset, buf_len),
      WireType::Fixed64 => consume_fixed!(8, offset, buf_len),
      WireType::Fixed128 => consume_fixed!(16, offset, buf_len),
      WireType::Zst => Ok((
        offset,
        Unknown::new(tag, wire_type, offset, slice!(offset, buf_len, buf)),
      )),
    }
  }
}
