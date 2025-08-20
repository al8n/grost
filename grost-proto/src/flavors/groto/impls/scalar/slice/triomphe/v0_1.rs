use triomphe_0_1::Arc;

use crate::{
  buffer::{Chunk, UnknownBuffer},
  decode::Decode,
  flavors::groto::{Context, DecodeError, Groto, LengthDelimited},
};

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for Arc<[u8]> {
  fn decode(_: &'de Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Chunk + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    decode_impl!(src, Arc<[u8]>)
  }
}

bidi_equivalent!(impl<[u8], LengthDelimited> for <Arc<[u8]>, LengthDelimited>);
