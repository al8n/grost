use crate::{
  Wirable,
  flavors::network::{Context, DecodeError, EncodeError, Identifier, Network, WireType},
};

pub fn encode_varint<V, E, EL>(
  ctx: &Context,
  val: &V,
  buf: &mut [u8],
  encode_fn: E,
  encoded_len_fn: EL,
) -> Result<usize, EncodeError>
where
  V: Wirable<Network> + ?Sized,
  E: FnOnce(&V, &mut [u8]) -> Result<usize, EncodeError>,
  EL: FnOnce(&V) -> usize,
{
  if let Some(tag) = ctx.tag() {
    let mut offset = 0;
    let identifier = Identifier::new(WireType::Varint, tag);
    offset += match identifier.encode_to(buf) {
      Ok(write) => write,
      Err(e) => {
        return Err(e.update(encoded_varint_len(ctx, val, encoded_len_fn), buf.len()));
      }
    };

    let buf_len = buf.len();
    if offset >= buf_len {
      return Err(EncodeError::insufficient_buffer(
        encoded_varint_len(ctx, val, encoded_len_fn),
        buf_len,
      ));
    }

    offset += encode_fn(val, &mut buf[offset..]).map_err(|_| {
      EncodeError::insufficient_buffer(encoded_varint_len(ctx, val, encoded_len_fn), buf_len)
    })?;
    Ok(offset)
  } else {
    encode_fn(val, buf)
  }
}

pub fn encoded_varint_len<V, F>(ctx: &Context, val: &V, f: F) -> usize
where
  V: Wirable<Network> + ?Sized,
  F: FnOnce(&V) -> usize,
{
  if let Some(tag) = ctx.tag() {
    let identifier = Identifier::new(WireType::Varint, tag);
    identifier.encoded_len() + f(val)
  } else {
    f(val)
  }
}

pub fn decode_varint<'a, V, F>(
  ctx: &Context,
  src: &'a [u8],
  f: F,
) -> Result<(usize, V), DecodeError>
where
  V: Sized,
  F: FnOnce(&'a [u8]) -> Result<(usize, V), DecodeError>,
{
  if let Some(tag) = ctx.tag() {
    let identifier = Identifier::new(WireType::Varint, tag);
    let (mut offset, decoded_identifier) = Identifier::decode(src)?;
    if identifier != decoded_identifier {
      return Err(DecodeError::identifier_mismatch(
        identifier.into_components(),
        decoded_identifier.into_components(),
      ));
    }

    if offset >= src.len() {
      return Err(DecodeError::buffer_underflow());
    }

    let (readed, value) = f(&src[offset..])?;
    offset += readed;
    Result::Ok((offset, value))
  } else {
    f(src)
  }
}
