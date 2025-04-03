use crate::smol_str::SmolStr;
use std::string::String;

str_bridge!(String {
  from_str: |val: &str| Ok(String::from(val));
  to_str: String::as_str;

  type EncodedOwned = SmolStr {
    from_ref: |s: &SmolStr| Ok(String::from(s.as_str()));
    from: |val: SmolStr| Ok(val.into());
  }
},);
