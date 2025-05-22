use crate::{
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
