#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::flavors::network::Network;
  use bytes_1::Bytes;
  use std::rc::Rc;

  use crate::{into_target, type_owned, type_ref};

  into_target!(Network: Bytes => Rc<[u8]> {
    |val: Bytes| Ok(Rc::from(val.as_ref()))
  });
  into_target!(Network: &[u8] => Rc<[u8]> {
    |val: &[u8]| Ok(Rc::from(val))
  });
  type_ref!( Network: &[u8] => Rc<[u8]> {
    |val: &[u8]| Ok(Rc::from(val))
  });
  type_owned!( Network: Bytes => Rc<[u8]> {
    |val: &Bytes| Ok(Rc::from(val.as_ref()))
  });
  bytes_message!(Rc<[u8]> => Bytes);
};
