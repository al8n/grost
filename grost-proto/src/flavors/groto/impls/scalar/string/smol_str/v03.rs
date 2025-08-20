use crate::{
  buffer::{Chunk, UnknownBuffer},
  convert::{PartialIdentity, PartialTryFromRef, TryFromPartialRef, TryFromRef},
  decode::{Decode, Str},
  default_string_wire_format, encode_bridge, flatten_state,
  flavors::groto::{Context, DecodeError, Groto, LengthDelimited, impls::decode_str},
  partial_ref_state, partial_state, ref_state, selectable,
  state::{Partial, PartialRef, Ref, State},
};
use smol_str_0_3::SmolStr;

default_string_wire_format!(Groto: SmolStr as LengthDelimited);

selectable!(@scalar Groto:SmolStr);

encode_bridge!(
  Groto: str {
    SmolStr as LengthDelimited {
      convert: SmolStr::as_str;
    },
  },
);

flatten_state!(SmolStr);
ref_state!(
  &'a Groto:
    SmolStr as LengthDelimited => Str<__GROST_READ_BUF__>,
);
partial_ref_state!(
  &'a Groto:
    SmolStr as LengthDelimited => Str<__GROST_READ_BUF__>,
);
partial_state!(
  Groto: SmolStr => SmolStr
);
bidi_equivalent!(impl <str, LengthDelimited> for <SmolStr, LengthDelimited>);
bidi_equivalent!(:<RB: Chunk>: impl<SmolStr, LengthDelimited> for <Str<RB>, LengthDelimited>);

impl<'a, RB, B> Decode<'a, LengthDelimited, RB, B, Groto> for SmolStr {
  fn decode(_: &'a Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'a,
    RB: Chunk,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let res = decode_str(&mut src).map(|(read, s)| (read, SmolStr::new(s)))?;
    src.advance(res.0);
    Ok(res)
  }
}

impl<'de, RB, B> TryFromPartialRef<'de, LengthDelimited, RB, B, Groto> for SmolStr {
  fn try_from_partial_ref(
    _: &'de Context,
    input: <Self as State<PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output,
  ) -> Result<Self, DecodeError>
  where
    Self: Sized,
    <Self as State<PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output: Sized,
    RB: Chunk,
    B: UnknownBuffer<RB, Groto>,
  {
    Ok(SmolStr::new(input.as_ref()))
  }
}

impl<'de, RB, B> TryFromRef<'de, LengthDelimited, RB, B, Groto> for SmolStr
where
  RB: Chunk,
  B: UnknownBuffer<RB, Groto>,
{
  fn try_from_ref(
    _: &'de Context,
    input: <Self as State<Ref<'de, LengthDelimited, RB, B, Groto>>>::Output,
  ) -> Result<Self, DecodeError>
  where
    Self: Sized,
  {
    Ok(SmolStr::new(input.as_ref()))
  }
}

impl<'de, RB, B> PartialTryFromRef<'de, LengthDelimited, RB, B, Groto> for SmolStr
where
  RB: Chunk + 'de,
{
  fn partial_try_from_ref(
    _: &'de Context,
    input: <Self as State<PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output,
    _: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, DecodeError>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output: Sized,
  {
    Ok(SmolStr::new(input.as_ref()))
  }
}

impl PartialIdentity<Groto> for SmolStr {
  fn partial_identity<'a>(
    input: &'a mut <Self as State<Partial<Groto>>>::Output,
    _: &'a bool,
  ) -> &'a mut Self
  where
    Self: Sized,
  {
    input
  }
}
