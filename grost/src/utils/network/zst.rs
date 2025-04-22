use crate::flavors::network::{Context, DecodeError, EncodeError, Identifier, WireType};

pub fn encoded_zst_len(ctx: &Context) -> usize {
  if let Some(tag) = ctx.tag() {
    let identifier = Identifier::new(WireType::Zst, tag);
    identifier.encoded_len()
  } else {
    0
  }
}

pub fn encode_zst(ctx: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
  if let Some(tag) = ctx.tag() {
    let identifier = Identifier::new(WireType::Zst, tag);
    identifier.encode_to(buf)
  } else {
    Ok(0)
  }
}

pub fn decode_zst<T, F>(ctx: &Context, src: &[u8], f: F) -> Result<(usize, T), DecodeError>
where
  F: FnOnce() -> Result<(usize, T), DecodeError>,
{
  if let Some(tag) = ctx.tag() {
    let identifier = Identifier::new(WireType::Zst, tag);
    let (offset, decoded_identifier) = Identifier::decode(src)?;
    if identifier != decoded_identifier {
      return Err(DecodeError::identifier_mismatch(
        identifier.into_components(),
        decoded_identifier.into_components(),
      ));
    }
    f().map(|(_, v)| (offset, v))
  } else {
    f().map(|(_, v)| (0, v))
  }
}
