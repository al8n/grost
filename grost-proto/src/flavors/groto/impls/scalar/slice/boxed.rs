use std::boxed::Box;

use crate::{
  buffer::{Chunk, UnknownBuffer},
  decode::Decode,
  flavors::groto::{Context, DecodeError, Groto, LengthDelimited},
};

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for Box<[u8]> {
  fn decode(_: &'de Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Chunk + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    decode_impl!(src, Box<[u8]>)
  }
}

bidi_equivalent!(impl<[u8], LengthDelimited> for <Box<[u8]>, LengthDelimited>);
