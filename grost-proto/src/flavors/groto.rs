pub use context::Context;
pub use error::{Error, ParseTagError};
pub use identifier::Identifier;
pub use impls::PackedDecoder;
pub use tag::Tag;
pub use unknown::Unknown;
pub use wire_type::*;

use super::Flavor;

use crate::buffer::ReadBuf;

mod context;
mod error;
mod identifier;
mod impls;
mod tag;
mod wire_type;

/// The unknown data types
mod unknown;

/// The groto flavor
#[derive(
  Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display,
)]
#[display("Groto")]
pub struct Groto;

impl Flavor for Groto {
  type Context = Context;
  type Identifier = Identifier;
  type WireType = WireType;
  type Tag = Tag;
  type Error = Error;
  type Unknown<B> = Unknown<B>;

  const NAME: &'static str = "Groto";

  fn encode_unknown<'a, B>(
    _: &Self::Context,
    value: &Self::Unknown<B>,
    buf: &mut [u8],
  ) -> Result<usize, Self::Error>
  where
    B: ReadBuf + 'a,
  {
    let value_bytes = value.raw();
    let value_len = value_bytes.len();
    if value_len > buf.len() {
      return Err(Error::insufficient_buffer(value_len, buf.len()));
    }

    buf[..value_len].copy_from_slice(value_bytes);
    Ok(value_len)
  }

  fn encoded_unknown_len<'a, B>(_: &Self::Context, value: &Self::Unknown<B>) -> usize
  where
    B: ReadBuf + 'a,
  {
    value.raw().len()
  }

  fn decode_unknown<'de, B>(
    _: &Self::Context,
    buf: B,
  ) -> Result<(usize, Self::Unknown<B>), Self::Error>
  where
    B: ReadBuf + 'de,
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
          return Err(Error::buffer_underflow());
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
          return Err(Error::buffer_underflow());
        }

        let (size_len, size) = varing::decode_u32_varint(&src[offset..])?;
        offset += size_len;
        let end = offset + size as usize;

        if end > buf_len {
          return Err(Error::buffer_underflow());
        }

        Ok((
          end,
          Unknown::new(tag, wire_type, offset, slice!(end, buf_len, buf)),
        ))
      }
      WireType::Varint => {
        if offset >= buf_len {
          return Err(Error::buffer_underflow());
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
      WireType::Fixed256 => consume_fixed!(32, offset, buf_len),
    }
  }

  fn skip<'de, B>(
    _: &Self::Context,
    wire_type: Self::WireType,
    buf: B,
  ) -> Result<usize, Self::Error>
  where
    B: ReadBuf + 'de,
  {
    Ok(match wire_type {
      WireType::Varint => varing::consume_varint(buf.as_bytes())?,
      WireType::LengthDelimited => {
        let (size_len, size) = varing::decode_u32_varint(buf.as_bytes())?;
        let size = size as usize;
        if size > buf.len() {
          return Err(Error::buffer_underflow());
        }
        size_len + size
      }
      WireType::Fixed8 => 1,
      WireType::Fixed16 => 2,
      WireType::Fixed32 => 4,
      WireType::Fixed64 => 8,
      WireType::Fixed128 => 16,
      WireType::Fixed256 => 32,
    })
  }
}
