use super::{DefaultWireFormat, Flavor, WireFormat};

pub use super::network::Context;
pub use error::*;
pub use identifier::*;
pub use unknown::Unknown;

mod error;
mod identifier;
mod impls;
mod unknown;

/// A flavor for encoding and decoding selectors.
#[derive(
  Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display,
)]
#[display("Selector")]
pub struct SelectorFlavor;

impl Flavor for SelectorFlavor {
  type Identifier = SelectorIdentifier;
  type WireType = SelectorWireType;
  type Tag = SelectorTag;

  type Context = Context;

  type Unknown<B> = Unknown<B>;

  type EncodeError = EncodeError;

  type DecodeError = DecodeError;

  const NAME: &'static str = "Selector";

  fn encode_unknown<B>(
    ctx: &Self::Context,
    value: &Self::Unknown<B>,
    buf: &mut [u8],
  ) -> Result<usize, Self::EncodeError>
  where
    B: crate::buffer::BytesBuffer,
  {
    let maximum_encode_size = ctx.maximum_message_size();

    let value_bytes = value.raw();
    let value_len = value_bytes.len();
    if value_len > maximum_encode_size {
      return Err(EncodeError::insufficient_buffer(
        value_len,
        maximum_encode_size,
      ));
    }

    if value_len > buf.len() {
      return Err(EncodeError::insufficient_buffer(value_len, buf.len()));
    }

    buf[..value_len].copy_from_slice(value_bytes);
    Ok(value_len)
  }

  fn decode_unknown<B>(
    _: &Self::Context,
    buf: B,
  ) -> Result<(usize, Self::Unknown<B>), Self::DecodeError>
  where
    B: crate::buffer::BytesBuffer,
  {
    macro_rules! slice {
      ($end:ident, $buf_len:ident, $buf:ident) => {{
        if $end == $buf_len {
          $buf
        } else {
          $buf.slice(..$end)
        }
      }};
    }

    let src = buf.as_bytes();
    if src.is_empty() {
      return Err(DecodeError::buffer_underflow());
    }

    let identifier = SelectorIdentifier::try_from_u8(src[0])?;
    let mut offset = 1;
    let buf_len = src.len();
    let (wire_type, tag) = identifier.into_components();
    match wire_type {
      SelectorWireType::Zst => Ok((
        offset,
        Unknown::new(tag, wire_type, offset, slice!(offset, buf_len, buf)),
      )),
      SelectorWireType::Varint => {
        let buf_len = buf.len();
        if offset >= buf_len {
          return Err(DecodeError::buffer_underflow());
        }

        let size_len = varing::consume_varint(&src[offset..])?;
        let end = offset + size_len;
        Ok((
          end,
          Unknown::new(
            identifier.tag(),
            identifier.wire_type(),
            offset,
            slice!(end, buf_len, buf),
          ),
        ))
      }
      SelectorWireType::LengthDelimited => {
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
    }
  }
}
