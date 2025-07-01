use crate::{
  buffer::ReadBuf,
  convert::{PartialTransform, Transform},
  decode::Str,
  flatten_state,
  flavors::{Groto, groto::LengthDelimited},
  groto_identity_transform, partial_ref_state, partial_state, selectable,
};
use std::string::String;

selectable!(@scalar Groto:String);
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

groto_identity_transform!(String as LengthDelimited,);
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
