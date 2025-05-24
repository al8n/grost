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

impl<'a, UB> crate::decode::Decode<'a, Network, LengthDelimited, &'a str, UB> for String {
  fn decode<B>(context: &<Network as crate::flavors::Flavor>::Context, src: B) -> Result<(usize, &'a str), <Network as crate::flavors::Flavor>::Error>
  where
    &'a str: Sized + 'a,
    B: crate::buffer::Buf<'a>,
    UB: crate::buffer::Buffer<<Network as crate::flavors::Flavor>::Unknown<B>> + 'a
  {
    <&str as crate::decode::Decode<'a, Network, LengthDelimited, &'a str, UB>>::decode(context, src)
  }
}