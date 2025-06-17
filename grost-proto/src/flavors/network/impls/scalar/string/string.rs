use crate::{
  decode::{PartialTransform, Transform},
  decoded_state, flatten_state,
  flavors::{Network, network::LengthDelimited},
  selectable,
};
use std::string::String;

selectable!(@scalar Network:String);
decoded_state!(
  &'a Network: String as LengthDelimited => &'a str
);
flatten_state!(String);

str_bridge!(Network: String {
  from_str: |val: &str| String::from(val);
  as_str: AsRef::as_ref;
},);

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
