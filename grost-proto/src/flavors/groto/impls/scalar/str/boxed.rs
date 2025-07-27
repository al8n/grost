use std::boxed::Box;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  decode::Decode,
  flavors::groto::{Context, Error, Groto, LengthDelimited, impls::decode_str},
};

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for Box<str> {
  fn decode(_: &'de Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    decode_str(&src).map(|(read, s)| (read, Box::from(s)))
  }
}

bidi_equivalent!(impl<str, LengthDelimited> for <Box<str>, LengthDelimited>);
