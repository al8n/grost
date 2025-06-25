use crate::{
  buffer::ReadBuf, decode::{PartialTransform, Str, Transform}, flatten_state, flavors::{network::LengthDelimited, Network}, identity_partial_transform, identity_transform, partial_ref_state, partial_state, selectable
};
use std::string::String;

selectable!(@scalar Network:String);
partial_ref_state!(
  &'a Network: String as LengthDelimited => Str<__GROST_READ_BUF__>
);
partial_state!(
  Network: String as LengthDelimited => String
);
flatten_state!(String);

str_bridge!(Network: String {
  from_str: |val: &str| String::from(val);
  as_str: AsRef::as_ref;
},);

identity_transform!(
  Network {
    String as LengthDelimited,
  }
);
identity_partial_transform!(
  Network {
    String as LengthDelimited,
  }
);

impl Transform<Network, LengthDelimited, &str> for String {
  fn transform(input: &str) -> Result<Self, <Network as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    Ok(String::from(input))
  }
}

impl PartialTransform<Network, LengthDelimited, &str> for String {
  fn partial_transform(
    input: &str,
    selector: &bool,
  ) -> Result<Option<Self>, <Network as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    if *selector {
      <Self as Transform<Network, LengthDelimited, &str>>::transform(input).map(Some)
    } else {
      Ok(None)
    }
  }
}

impl<B> Transform<Network, LengthDelimited, Str<B>> for String
where
  B: ReadBuf,
{
  fn transform(input: Str<B>) -> Result<Self, <Network as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    Ok(String::from(input.as_ref()))
  }
}

impl<B> PartialTransform<Network, LengthDelimited, Str<B>> for String
where
  B: ReadBuf,
{
  fn partial_transform(
    input: Str<B>,
    selector: &bool,
  ) -> Result<Option<Self>, <Network as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    if *selector {
      <Self as Transform<Network, LengthDelimited, Str<B>>>::transform(input).map(Some)
    } else {
      Ok(None)
    }
  }
}
