use crate::{
  buffer::{ReadBuf, UnknownBuffer}, convert::{Partial, PartialIdentity, PartialRef, PartialTryFromRef, Ref, TryFromPartialRef, TryFromRef}, decode::{Decode, Str}, decode_bridge, default_string_wire_format, encode_bridge, flatten_state, flavors::groto::{Context, Error, Groto, LengthDelimited}, partial_ref_state, partial_state, ref_state, selectable, state::State
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

decode_bridge!(
  Groto: &'de str => Str<RB> {
    SmolStr as LengthDelimited {
      convert: |src: &str| SmolStr::new(src);
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
bidi_equivalent!(:<RB: ReadBuf>: impl<SmolStr, LengthDelimited> for <Str<RB>, LengthDelimited>);

impl<'a, RB, B> Decode<'a, LengthDelimited, RB, B, Groto> for SmolStr {
  fn decode(
    context: &'a <Groto as crate::flavors::Flavor>::Context,
    src: RB,
  ) -> Result<(usize, Str<RB>), <Groto as crate::flavors::Flavor>::Error>
  where
    Str<RB>: Sized + 'a,
    RB: crate::buffer::ReadBuf,
    B: crate::buffer::UnknownBuffer<RB, Groto> + 'a,
  {
    <&'a str as Decode<'a, LengthDelimited, RB, B, Groto>>::decode(context, src)
      .map(|(read, val)| (read, Self::new(val)))
  }
}

impl<'de, RB, B> TryFromPartialRef<'de, RB, B, LengthDelimited, Groto> for SmolStr {
  fn try_from_partial_ref(
    _: &'de Context,
    input: <Self as State<PartialRef<'de, RB, B, LengthDelimited, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'de, RB, B, LengthDelimited, Groto>>>::Output: Sized,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    Ok(SmolStr::new(input.as_ref()))
  }
}

impl<'de, RB, B> TryFromRef<'de, RB, B, LengthDelimited, Groto> for SmolStr
where
  RB: ReadBuf,
  B: UnknownBuffer<RB, Groto>,
{
  fn try_from_ref(
    _: &'de Context,
    input: <Self as State<Ref<'de, RB, B, LengthDelimited, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
  {
    Ok(SmolStr::new(input.as_ref()))
  }
}

impl<'de, RB, B> PartialTryFromRef<'de, RB, B, LengthDelimited, Groto> for SmolStr
where
  RB: ReadBuf + 'de,
{
  fn partial_try_from_ref(
    _: &'de Context,
    input: <Self as State<PartialRef<'de, RB, B, LengthDelimited, Groto>>>::Output,
    _: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, <Groto as crate::flavors::Flavor>::Error>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'de, RB, B, LengthDelimited, Groto>>>::Output: Sized,
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
