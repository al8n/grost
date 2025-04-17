use grost_types::{DecodeError, EncodeError, Identifier, WireType};
use varing::Varint;

use crate::Context;

pub fn encode_varint<V: Varint>(
  ctx: &Context,
  val: &V,
  buf: &mut [u8],
) -> Result<usize, EncodeError> {
  if let Some(tag) = ctx.tag() {
    let mut offset = 0;
    let identifier = Identifier::new(WireType::Varint, tag);
    offset += identifier
      .encode_to(buf)
      .map_err(|e| e.update(encoded_varint_len(ctx, val), buf.len()))?;
    let buf_len = buf.len();
    if offset >= buf_len {
      return Err(EncodeError::insufficient_buffer(
        encoded_varint_len(ctx, val),
        buf_len,
      ));
    }

    offset += Varint::encode(val, &mut buf[offset..])
      .map_err(|_| EncodeError::insufficient_buffer(encoded_varint_len(ctx, val), buf_len))?;
    Ok(offset)
  } else {
    Varint::encode(val, buf).map_err(::core::convert::Into::into)
  }
}

pub fn encoded_varint_len<V: Varint>(ctx: &Context, val: &V) -> usize {
  if let Some(tag) = ctx.tag() {
    let identifier = Identifier::new(WireType::Varint, tag);
    identifier.encoded_len() + Varint::encoded_len(val)
  } else {
    Varint::encoded_len(val)
  }
}

pub fn decode_varint<V: Varint>(ctx: &Context, src: &[u8]) -> Result<(usize, V), DecodeError> {
  if let Some(tag) = ctx.tag() {
    let identifier = Identifier::new(WireType::Varint, tag);
    let (mut offset, decoded_identifier) = Identifier::decode(src)?;
    if identifier != decoded_identifier {
      return Err(DecodeError::identifier_mismatch(
        identifier,
        decoded_identifier,
      ));
    }

    if offset >= src.len() {
      return Err(DecodeError::buffer_underflow());
    }

    let (readed, value) = varing::Varint::decode(&src[offset..])?;
    offset += readed;
    Result::Ok((offset, value))
  } else {
    varing::Varint::decode(src).map_err(::core::convert::Into::into)
  }
}
