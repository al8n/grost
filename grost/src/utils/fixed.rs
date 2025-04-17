use grost_types::{DecodeError, EncodeError, Identifier, WireType};

use crate::{Context, Wirable};

pub fn encode_fixed<V, F, const N: usize>(
  ctx: &Context,
  val: &V,
  buf: &mut [u8],
  f: F,
) -> Result<usize, EncodeError>
where
  F: FnOnce(&V, &mut [u8]) -> Result<(), EncodeError>,
  V: Wirable,
{
  if let Some(tag) = ctx.tag() {
    let mut offset = 0;
    let identifier = Identifier::new(V::WIRE_TYPE, tag);
    offset += identifier
      .encode_to(buf)
      .map_err(|e| e.update(encoded_fixed_len::<N>(ctx), buf.len()))?;
    let buf_len = buf.len();
    if offset >= buf_len {
      return Err(EncodeError::insufficient_buffer(
        encoded_fixed_len::<N>(ctx),
        buf_len,
      ));
    }

    f(val, &mut buf[offset..])
      .map_err(|_| EncodeError::insufficient_buffer(encoded_fixed_len::<N>(ctx), buf_len))?;
    Ok(offset + N)
  } else {
    let buf_len = buf.len();
    if buf_len < N {
      return Err(EncodeError::insufficient_buffer(N, buf_len));
    }

    f(val, &mut buf[..N]).map(|_| N)
  }
}

pub fn encoded_fixed_len<const N: usize>(ctx: &Context) -> usize {
  if let Some(tag) = ctx.tag() {
    let identifier = Identifier::new(WireType::Varint, tag);
    identifier.encoded_len() + N
  } else {
    N
  }
}

pub fn decode_fixed<V, F, const N: usize>(
  ctx: &Context,
  buf: &[u8],
  f: F,
) -> Result<(usize, V), DecodeError>
where
  F: FnOnce(&[u8]) -> Result<V, DecodeError>,
{
  if let Some(tag) = ctx.tag() {
    let identifier = Identifier::new(WireType::Varint, tag);
    let (offset, decoded_identifier) = Identifier::decode(buf)?;
    if identifier != decoded_identifier {
      return Err(DecodeError::identifier_mismatch(
        identifier,
        decoded_identifier,
      ));
    }

    if offset + N > buf.len() {
      return Err(DecodeError::buffer_underflow());
    }

    f(&buf[offset..offset + N]).map(|v| (offset + N, v))
  } else {
    if buf.len() < N {
      return Err(DecodeError::buffer_underflow());
    }

    f(&buf[..N]).map(|v| (N, v))
  }
}
