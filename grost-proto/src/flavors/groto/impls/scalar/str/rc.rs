use std::rc::Rc;

use crate::{
  buffer::{Chunk, UnknownBuffer},
  decode::Decode,
  flavors::groto::{Context, DecodeError, Groto, LengthDelimited, impls::decode_str},
};

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for Rc<str> {
  fn decode(_: &'de Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Chunk + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    let res = decode_str(&mut src).map(|(read, s)| (read, Rc::from(s)))?;
    src.advance(res.0);
    Ok(res)
  }
}

bidi_equivalent!(impl<str, LengthDelimited> for <Rc<str>, LengthDelimited>);
