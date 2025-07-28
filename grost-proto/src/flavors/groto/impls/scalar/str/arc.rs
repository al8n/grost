use std::sync::Arc;

use crate::{
  buffer::{ReadBuf, UnknownBuffer, WriteBuf},
  convert::PartialTryFromRef,
  decode::Decode,
  flavors::groto::{Context, Error, Groto, LengthDelimited, impls::decode_str},
};

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for Arc<str> {
  fn decode(_: &'de Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    decode_str(&src).map(|(read, s)| (read, Arc::from(s)))
  }
}

bidi_equivalent!(impl<str, LengthDelimited> for <Arc<str>, LengthDelimited>);

impl<'de, RB, B> PartialTryFromRef<'de, LengthDelimited, RB, B, Groto> for Arc<str>
where
  RB: ReadBuf + 'de,
{
  fn partial_try_from_ref(
    _: &'de Context,
    input: <Self as crate::state::State<crate::state::PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output,
    _: &Self::Selector,
  ) -> Result<<Self as crate::state::State<crate::state::Partial<Groto>>>::Output, <Groto as crate::flavors::Flavor>::Error>
  where
    <Self as crate::state::State<crate::state::Partial<Groto>>>::Output: Sized,
    <Self as crate::state::State<crate::state::PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output: Sized
  {
    Ok(Arc::from(input.as_ref()))
  }
}
