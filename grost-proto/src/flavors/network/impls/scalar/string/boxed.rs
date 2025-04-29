#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use crate::flavors::network::Network;
  use smol_str_0_3::SmolStr;
  use std::boxed::Box;

  use crate::{into_target, type_owned, type_ref};

  str_bridge!(Network: Box<str> {
    from_str: |val: &str| Box::<str>::from(val);
    as_str: AsRef::as_ref;
  
    type EncodedOwned = SmolStr;
  },);

  into_target!(Network: SmolStr => Box<str> {
    |val: SmolStr| Ok(Box::from(val.as_ref()))
  });
  into_target!(Network: &str => Box<str> {
    |val: &str| Ok(Box::from(val))
  });
  into_target!(@self Network: Box<str>);
  type_ref!(@mapping Network: &str => Box<str> {
    |val: &str| Ok(Box::from(val))
  });
  type_owned!(@mapping Network: SmolStr => Box<str> {
    |val: &SmolStr| Ok(Box::from(val.as_ref()))
  });
  type_owned!(@clone Network: Box<str>);
};
