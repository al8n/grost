pub use context::Context;
pub use error::Error;
pub use identifier::Identifier;
pub use impls::{PackedDecoder, PackedEntriesDecoder, RepeatedDecoder};
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
      WireType::Nullable => {
        if offset + 1 >= buf_len {
          return Err(Error::buffer_underflow());
        }

        let marker = src[offset];
        offset += 1;
        if marker == 0 {
          // This is a zero byte indicating absence, so we skip it
          return Ok((
            offset,
            Unknown::new(tag, wire_type, offset, slice!(offset, buf_len, buf)),
          ));
        }
        if offset + 1 >= buf_len {
          return Err(Error::buffer_underflow());
        }

        let wire_type = WireType::try_from_u8(src[offset])?;
        offset += 1;

        match wire_type {
          WireType::Nullable => Err(Error::custom("unexpected nested nullable wire type")),
          WireType::Varint => {
            let size_len = varing::consume_varint(&src[offset..])?;
            let end = offset + size_len;
            Ok((
              end,
              Unknown::new(tag, wire_type, offset, slice!(end, buf_len, buf)),
            ))
          }
          WireType::LengthDelimited => {
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
          WireType::Fixed8 => consume_fixed!(1, offset, buf_len),
          WireType::Fixed16 => consume_fixed!(2, offset, buf_len),
          WireType::Fixed32 => consume_fixed!(4, offset, buf_len),
          WireType::Fixed64 => consume_fixed!(8, offset, buf_len),
          WireType::Fixed128 => consume_fixed!(16, offset, buf_len),
        }
      }
      WireType::Fixed8 => consume_fixed!(1, offset, buf_len),
      WireType::Fixed16 => consume_fixed!(2, offset, buf_len),
      WireType::Fixed32 => consume_fixed!(4, offset, buf_len),
      WireType::Fixed64 => consume_fixed!(8, offset, buf_len),
      WireType::Fixed128 => consume_fixed!(16, offset, buf_len),
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
    // TODO(al8n): advance the buffer?
    skip_helper(wire_type, buf.as_bytes())
  }
}

fn skip_helper(wire_type: WireType, buf: &[u8]) -> Result<usize, Error> {
  let buf_len = buf.len();

  macro_rules! try_skip_fixed {
    ($bytes:literal) => {{
      if buf_len < $bytes {
        return Err(Error::buffer_underflow());
      }

      $bytes
    }};
  }

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
    WireType::Nullable => {
      if buf_len <= 1 {
        return Err(Error::buffer_underflow());
      }
      let marker = buf[0];
      if marker == 0 {
        // This is a zero byte indicating absence, so we skip it
        return Ok(1);
      }

      if buf_len <= 2 {
        return Err(Error::buffer_underflow());
      }
      let wire_type = WireType::try_from_u8(buf[1])?;
      if wire_type.is_nullable() {
        return Err(Error::custom("unexpected nested nullable wire type"));
      }
      return skip_helper(wire_type, &buf[2..]).map(|len| len + 2);
    }
    WireType::Fixed8 => try_skip_fixed!(1),
    WireType::Fixed16 => try_skip_fixed!(2),
    WireType::Fixed32 => try_skip_fixed!(4),
    WireType::Fixed64 => try_skip_fixed!(8),
    WireType::Fixed128 => try_skip_fixed!(16),
  })
}
