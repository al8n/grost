use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  convert::{
    Partial, PartialIdentity, PartialRef, PartialTryFromRef, Ref, TryFromPartialRef, TryFromRef,
  },
  decode::Str,
  flatten_state,
  flavors::{
    Groto,
    groto::{Context, Error, LengthDelimited},
  },
  partial_ref_state, partial_state, ref_state, selectable,
  state::State,
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

impl<'de, RB, B> TryFromPartialRef<'de, RB, B, LengthDelimited, Groto> for String {
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
    Ok(input.to_string())
  }
}

impl<'de, RB, B> TryFromRef<'de, RB, B, LengthDelimited, Groto> for String
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
    Ok(input.to_string())
  }
}

impl<'de, RB, B> PartialTryFromRef<'de, RB, B, LengthDelimited, Groto> for String
where
  RB: ReadBuf + 'de,
{
  fn partial_try_from_ref(
    context: &'de Context,
    input: <Self as State<PartialRef<'de, RB, B, LengthDelimited, Groto>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, <Groto as crate::flavors::Flavor>::Error>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'de, RB, B, LengthDelimited, Groto>>>::Output: Sized,
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
