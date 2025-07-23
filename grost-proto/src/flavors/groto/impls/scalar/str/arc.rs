use std::sync::Arc;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  convert::PartialTryFromRef,
  decode::Decode,
  flavors::groto::{Context, Error, Groto, LengthDelimited},
};

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for Arc<str> {
  fn decode(_: &'de Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    decode_impl!(src, Arc<str>)
  }
}

bidi_equivalent!(impl<str, LengthDelimited> for <Arc<str>, LengthDelimited>);

impl<'de, RB, B> PartialTryFromRef<'de, RB, B, LengthDelimited, Groto> for Arc<str>
where
  RB: ReadBuf + 'de,
{
  fn partial_try_from_ref(
    context: &'de Context,
    input: <Self as crate::state::State<crate::state::PartialRef<'de, RB, B, LengthDelimited, Groto>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as crate::state::State<crate::state::Partial<Groto>>>::Output, <Groto as crate::flavors::Flavor>::Error>
  where
    <Self as crate::state::State<crate::state::Partial<Groto>>>::Output: Sized,
    <Self as crate::state::State<crate::state::PartialRef<'de, RB, B, LengthDelimited, Groto>>>::Output: Sized
  {
    Ok(Arc::from(input.as_ref()))
  }
}
