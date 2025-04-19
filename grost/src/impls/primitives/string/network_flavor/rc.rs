#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use crate::flavors::network::{Network, WireType};
  use smol_str_0_3::SmolStr;
  use std::rc::Rc;

  str_bridge!(Network:(WireType::LengthDelimited):Rc<str> {
    from_str: |val: &str| Ok(Rc::<str>::from(val));
    to_str: AsRef::as_ref;
  
    type EncodedOwned = SmolStr {
      from_ref: |s: &SmolStr| Ok(Rc::<str>::from(s.as_str()));
      from: |s: SmolStr| Ok(Rc::from(s.as_str()));
    }
  },);
};
