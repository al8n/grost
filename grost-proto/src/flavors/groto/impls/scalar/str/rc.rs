use std::rc::Rc;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  decode::Decode,
  flavors::groto::{Context, Error, Groto, LengthDelimited},
};

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for Rc<str> {
  fn decode(_: &'de Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    decode_impl!(src, Rc<str>)
  }
}

bidi_equivalent!(impl<str, LengthDelimited> for <Rc<str>, LengthDelimited>);
