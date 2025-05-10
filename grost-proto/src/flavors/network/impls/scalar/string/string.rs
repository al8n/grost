use crate::{
  default_wire_format, encoded_state,
  flavors::{Network, network::LengthDelimited},
  selectable,
};

selectable!(@scalar Network:String);
encoded_state!(
  &'a Network: String as LengthDelimited => &'a str
);

#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use crate::{
    flavors::network::{LengthDelimited, Network},
    referenceable,
  };
  use smol_str_0_3::SmolStr;
  use std::string::String;

  use crate::{into_target, type_owned, type_ref};

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
    Network: String as LengthDelimited => &'a str
  );
  type_owned!(Network: SmolStr => String {
    |val: &SmolStr| Ok(String::from(val.clone()))
  });
};
