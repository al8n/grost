use triomphe_0_1::Arc;

use crate::{
  buffer::{Buf, UnknownBuffer},
  decode::Decode,
  flavors::groto::{Context, DecodeError, Groto, LengthDelimited, impls::decode_str},
};

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for Arc<str> {
  fn decode(_: &'de Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Buf + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    let res = decode_str(&mut src).map(|(read, s)| (read, Arc::from(s)))?;
    src.advance(res.0);
    Ok(res)
  }
}

bidi_equivalent!(impl<str, LengthDelimited> for <Arc<str>, LengthDelimited>);
