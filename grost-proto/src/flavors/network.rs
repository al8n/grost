pub use context::Context;
pub use error::{DecodeError, EncodeError};
pub use identifier::Identifier;
pub use unknown::Unknown;
pub use wire_type::WireType;

use super::Flavor;
use crate::buffer::BytesBuffer;

mod context;
mod error;
mod identifier;
mod impls;
mod wire_type;

/// The unknown data types
mod unknown;

/// The zero-sized type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display)]
#[display("Zst")]
pub struct Zst;

impl super::WireFormat for Zst {}

/// The length-delimited encoding/decoding wire type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display)]
#[display("LengthDelimited")]
pub struct LengthDelimited;

impl super::WireFormat for LengthDelimited {}

/// The varint encoding/decoding wire type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display)]
#[display("Varint")]
pub struct Varint;

impl super::WireFormat for Varint {}

/// The fixed 8-bit length encoding/decoding wire type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display)]
#[display("Fixed8")]
pub struct Fixed8;

impl super::WireFormat for Fixed8 {}

/// The fixed 16-bit length encoding/decoding wire type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display)]
#[display("Fixed16")]
pub struct Fixed16;

impl super::WireFormat for Fixed16 {}

/// The fixed 32-bit length encoding/decoding wire type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display)]
#[display("Fixed32")]
pub struct Fixed32;

impl super::WireFormat for Fixed32 {}

/// The fixed 64-bit length encoding/decoding wire type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display)]
#[display("Fixed64")]
pub struct Fixed64;

impl super::WireFormat for Fixed64 {}

/// The fixed 128-bit length encoding/decoding wire type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display)]
#[display("Fixed128")]
pub struct Fixed128;

impl super::WireFormat for Fixed128 {}

/// The network flavor
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Network;

impl Flavor for Network {
  type Context = Context;
  type Identifier = Identifier;
  type EncodeError = EncodeError;
  type DecodeError = DecodeError;
  type Unknown<B> = Unknown<B>;

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

  fn decode_unknown<B>(
    _: &Self::Context,
    identifier: Self::Identifier,
    buf: B,
  ) -> Result<(usize, Self::Unknown<B>), Self::DecodeError>
  where
    B: BytesBuffer + Sized,
  {
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
      ($size:literal, $identifier_len:ident) => {{
        let end = $identifier_len + $size;
        let buf_len = buf.len();
        if end > buf_len {
          return Err(DecodeError::buffer_underflow());
        }

        Ok((
          end,
          Unknown::new(tag, wire_type, $identifier_len, slice!(end, buf_len, buf)),
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
          Unknown::new(tag, wire_type, identifier_len + size_len, data),
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
          Unknown::new(tag, wire_type, identifier_len, slice!(end, buf_len, buf)),
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
