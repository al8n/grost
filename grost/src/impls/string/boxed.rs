use crate::smol_str::SmolStr;
use std::boxed::Box;

str_bridge!(Box<str> {
  from_str: |val: &str| Ok(Box::<str>::from(val));
  to_str: AsRef::as_ref;

  type SerializedOwned = SmolStr {
    from_ref: |s: &SmolStr| Ok(Box::<str>::from(s.as_str()));
    from: |s: SmolStr| Ok(Box::from(s.as_str()));
  }
},);
