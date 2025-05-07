#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use crate::{
    flavors::network::{LengthDelimited, Network},
    referenceable, selectable_bridge,
  };
  use smol_str_0_3::SmolStr;
  use std::string::String;

  use crate::{into_target, type_owned, type_ref};

  selectable_bridge!(Network:str[String]);

  str_bridge!(Network: String {
    from_str: |val: &str| String::from(val);
    as_str: AsRef::as_ref;
  
    type EncodedOwned = SmolStr;
  },);

  into_target!(Network: SmolStr => String {
    |val: SmolStr| Ok(String::from(val))
  });
  into_target!(Network: &str => String {
    |val: &str| Ok(String::from(val))
  });
  type_ref!( Network: &str => String {
    |val: &str| Ok(String::from(val))
  });
  referenceable!(
    Network: String:LengthDelimited => &'a str
  );
  type_owned!( Network: SmolStr => String {
    |val: &SmolStr| Ok(String::from(val.clone()))
  });
};
