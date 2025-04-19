#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use crate::flavors::network::{Network, WireType};
  use smol_str_0_3::SmolStr;
  use std::string::String;

  str_bridge!(Network:(WireType::LengthDelimited):String {
    from_str: |val: &str| Ok(String::from(val));
    to_str: String::as_str;
  
    type EncodedOwned = SmolStr {
      from_ref: |s: &SmolStr| Ok(String::from(s.as_str()));
      from: |val: SmolStr| Ok(val.into());
    }
  },);
};
