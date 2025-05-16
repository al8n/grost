#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use crate::flavors::network::Network;
  use smol_str_0_3::SmolStr;
  use std::rc::Rc;

  use crate::{into_target, type_owned, type_ref};

  into_target!(Network: SmolStr => Rc<str> {
    |val: SmolStr| Ok(Rc::from(val.as_ref()))
  });
  into_target!(Network: &str => Rc<str> {
    |val: &str| Ok(Rc::from(val))
  });
  type_ref!(Network: &str => Rc<str> {
    |val: &str| Ok(Rc::from(val))
  });
  type_owned!(Network: SmolStr => Rc<str> {
    |val: &SmolStr| Ok(Rc::from(val.as_ref()))
  });
  str_message!(Rc<str> => SmolStr);
};
