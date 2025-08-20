use std::sync::Arc;

use crate::{
  buffer::{Chunk, UnknownBuffer},
  convert::PartialTryFromRef,
  decode::Decode,
  flavors::groto::{Context, DecodeError, Groto, LengthDelimited, impls::decode_str},
};

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for Arc<str> {
  fn decode(_: &'de Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Chunk + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    let res = decode_str(&mut src).map(|(read, s)| (read, Arc::from(s)))?;
    src.advance(res.0);
    Ok(res)
  }
}

bidi_equivalent!(impl<str, LengthDelimited> for <Arc<str>, LengthDelimited>);

impl<'de, RB, B> PartialTryFromRef<'de, LengthDelimited, RB, B, Groto> for Arc<str>
where
  RB: Chunk + 'de,
{
  fn partial_try_from_ref(
    _: &'de Context,
    input: <Self as crate::state::State<crate::state::PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output,
    _: &Self::Selector,
  ) -> Result<<Self as crate::state::State<crate::state::Partial<Groto>>>::Output, DecodeError>
  where
    <Self as crate::state::State<crate::state::Partial<Groto>>>::Output: Sized,
    <Self as crate::state::State<crate::state::PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output: Sized
  {
    Ok(Arc::from(input.as_ref()))
  }
}
