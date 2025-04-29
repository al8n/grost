#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use crate::flavors::network::Network;
  use smol_str_0_3::SmolStr;
  use std::rc::Rc;

  use crate::{into_target, type_owned, type_ref};

  str_bridge!(Network: Rc<str> {
    from_str: |val: &str| Rc::<str>::from(val);
    as_str: AsRef::as_ref;
  
    type EncodedOwned = SmolStr;
  },);

  into_target!(Network: SmolStr => Rc<str> {
    |val: SmolStr| Ok(Rc::from(val.as_ref()))
  });
  into_target!(Network: &str => Rc<str> {
    |val: &str| Ok(Rc::from(val))
  });
  into_target!(@self Network: Rc<str>);
  type_ref!(@mapping Network: &str => Rc<str> {
    |val: &str| Ok(Rc::from(val))
  });
  type_owned!(@mapping Network: SmolStr => Rc<str> {
    |val: &SmolStr| Ok(Rc::from(val.as_ref()))
  });
  type_owned!(@clone Network: Rc<str>);
};
