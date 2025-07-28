use crate::{
  buffer::{ReadBuf, UnknownBuffer, WriteBuf},
  convert::{PartialIdentity, PartialTryFromRef, TryFromPartialRef, TryFromRef},
  decode::{Decode, Str},
  flatten_state,
  flavors::{
    Groto,
    groto::{Context, Error, LengthDelimited, impls::decode_str},
  },
  partial_ref_state, partial_state, ref_state, selectable,
  state::{Partial, PartialRef, Ref, State},
};
use std::string::String;

selectable!(@scalar Groto:String);
ref_state!(
  &'a Groto:
    String as LengthDelimited => Str<__GROST_READ_BUF__>,
);
partial_ref_state!(
  &'a Groto:
    String as LengthDelimited => Str<__GROST_READ_BUF__>,
);
partial_state!(
  Groto: String => String
);
flatten_state!(String);

str_bridge!(Groto: String {
  from_str: |val: &str| String::from(val);
  as_str: AsRef::as_ref;
},);

bidi_equivalent!(:<RB: ReadBuf>: impl<String, LengthDelimited> for <Str<RB>, LengthDelimited>);
bidi_equivalent!(impl <String, LengthDelimited> for <str, LengthDelimited>);

impl<'de, RB, B> TryFromPartialRef<'de, LengthDelimited, RB, B, Groto> for String {
  fn try_from_partial_ref(
    _: &'de Context,
    input: <Self as State<PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output: Sized,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    Ok(input.to_string())
  }
}

impl<'de, RB, B> TryFromRef<'de, LengthDelimited, RB, B, Groto> for String
where
  RB: ReadBuf,
  B: UnknownBuffer<RB, Groto>,
{
  fn try_from_ref(
    _: &'de Context,
    input: <Self as State<Ref<'de, LengthDelimited, RB, B, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
  {
    Ok(input.to_string())
  }
}

impl<'de, RB, B> PartialTryFromRef<'de, LengthDelimited, RB, B, Groto> for String
where
  RB: ReadBuf + 'de,
{
  fn partial_try_from_ref(
    _: &'de Context,
    input: <Self as State<PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output,
    _: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, <Groto as crate::flavors::Flavor>::Error>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output: Sized,
  {
    Ok(input.to_string())
  }
}

impl PartialIdentity<Groto> for String {
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

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for String {
  fn decode(_: &'de Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized,
    RB: ReadBuf + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    decode_str(&src).map(|(read, s)| (read, s.to_string()))
  }
}
