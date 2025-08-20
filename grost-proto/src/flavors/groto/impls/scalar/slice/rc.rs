use std::rc::Rc;

use crate::{
  buffer::{Chunk, UnknownBuffer},
  decode::Decode,
  flavors::groto::{Context, DecodeError, Groto, LengthDelimited},
};

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for Rc<[u8]> {
  fn decode(_: &'de Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Chunk + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    decode_impl!(src, Rc<[u8]>)
  }
}

bidi_equivalent!(impl<[u8], LengthDelimited> for <Rc<[u8]>, LengthDelimited>);
