#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use crate::flavors::network::Network;
  use smol_str_0_3::SmolStr;
  use std::boxed::Box;

  use crate::{into_target, type_owned, type_ref};

  into_target!(Network: SmolStr => Box<str> {
    |val: SmolStr| Ok(Box::from(val.as_ref()))
  });
  into_target!(Network: &str => Box<str> {
    |val: &str| Ok(Box::from(val))
  });
  type_ref!(Network: &str => Box<str> {
    |val: &str| Ok(Box::from(val))
  });
  type_owned!( Network: SmolStr => Box<str> {
    |val: &SmolStr| Ok(Box::from(val.as_ref()))
  });
  str_message!(Box<str> => SmolStr);
};
