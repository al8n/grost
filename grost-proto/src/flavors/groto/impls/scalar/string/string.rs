use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  convert::{
    Partial, PartialIdentity, PartialRef, PartialTransform, PartialTryFromRef, Ref, Transform,
    TryFromPartial, TryFromPartialRef, TryFromRef,
  },
  decode::Str,
  flatten_state,
  flavors::{
    Groto,
    groto::{Error, LengthDelimited},
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

identity_partial_transform!(
  Groto {
    String as LengthDelimited,
  }
);

impl Transform<&str, Self, LengthDelimited, Groto> for String {
  fn transform(input: &str) -> Result<Self, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    Ok(String::from(input))
  }
}

impl PartialTransform<&str, Option<Self>, LengthDelimited, Groto> for String {
  fn partial_transform(
    input: &str,
    selector: &bool,
  ) -> Result<Option<Self>, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    if *selector {
      <Self as Transform<&str, Self, LengthDelimited, Groto>>::transform(input).map(Some)
    } else {
      Ok(None)
    }
  }
}

impl<RB> Transform<Str<RB>, Self, LengthDelimited, Groto> for String
where
  RB: ReadBuf,
{
  fn transform(input: Str<RB>) -> Result<Self, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    Ok(String::from(input.as_ref()))
  }
}

impl<RB> PartialTransform<Str<RB>, Option<Self>, LengthDelimited, Groto> for String
where
  RB: ReadBuf,
{
  fn partial_transform(
    input: Str<RB>,
    selector: &bool,
  ) -> Result<Option<Self>, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    if *selector {
      <Self as Transform<Str<RB>, Self, LengthDelimited, Groto>>::transform(input).map(Some)
    } else {
      Ok(None)
    }
  }
}

bidi_equivalent!(:<RB: ReadBuf>: impl<String, LengthDelimited> for <Str<RB>, LengthDelimited>);
bidi_equivalent!(impl <String, LengthDelimited> for <str, LengthDelimited>);

impl TryFromPartial<LengthDelimited, Groto> for String {
  fn try_from_partial(input: <Self as State<Partial<Groto>>>::Output) -> Result<Self, Error>
  where
    Self: Sized,
  {
    Ok(input)
  }
}

impl<'de, RB, B> TryFromPartialRef<'de, RB, B, LengthDelimited, Groto> for String {
  fn try_from_partial_ref(
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
  RB: ReadBuf,
  B: UnknownBuffer<RB, Groto>,
{
  fn partial_try_from_ref(
    input: <Self as State<PartialRef<'de, RB, B, LengthDelimited, Groto>>>::Output,
    _: &bool,
  ) -> Result<Self, Error>
  where
    Self: Sized,
  {
    Ok(input.to_string())
  }
}

impl PartialIdentity<LengthDelimited, Groto> for String {
  fn partial_identity(input: <Self as State<Partial<Groto>>>::Output, _: &bool) -> Self
  where
    Self: Sized,
  {
    input
  }
}
