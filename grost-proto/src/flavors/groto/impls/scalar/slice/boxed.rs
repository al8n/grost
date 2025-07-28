use std::boxed::Box;

use crate::{
  buffer::{ReadBuf, UnknownBuffer, WriteBuf},
  decode::Decode,
  flavors::groto::{Context, Error, Groto, LengthDelimited},
};

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for Box<[u8]> {
  fn decode(_: &'de Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    decode_impl!(src, Box<[u8]>)
  }
}

bidi_equivalent!(impl<[u8], LengthDelimited> for <Box<[u8]>, LengthDelimited>);
