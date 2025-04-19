use crate::flavors::network::{Network, WireType};
use ::smol_str_0_3::SmolStr;

str_bridge!(Network:(WireType::LengthDelimited):SmolStr {
  from_str: |val| Ok(SmolStr::new(val));
  to_str: SmolStr::as_str;

  type EncodedOwned = SmolStr {
    from_ref: |val: &SmolStr| Ok(val.clone());
    from: |val: SmolStr| Ok(val);
  }
},);
