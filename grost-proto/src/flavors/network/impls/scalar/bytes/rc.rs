#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::flavors::network::Network;
  use bytes_1::Bytes;
  use std::rc::Rc;

  use crate::{into_target, type_owned, type_ref};

  bytes_bridge!(Network: Rc<[u8]> {
    from_slice: |val: &[u8]| Rc::<[u8]>::from(val);
    as_slice: AsRef::as_ref;
  
    type EncodedOwned = Bytes;
  },);

  into_target!(Network: Bytes => Rc<[u8]> {
    |val: Bytes| Ok(Rc::from(val.as_ref()))
  });
  into_target!(Network: &[u8] => Rc<[u8]> {
    |val: &[u8]| Ok(Rc::from(val))
  });
  into_target!(@self Network: Rc<[u8]>);
  type_ref!(@mapping Network: &[u8] => Rc<[u8]> {
    |val: &[u8]| Ok(Rc::from(val))
  });
  type_owned!(@mapping Network: Bytes => Rc<[u8]> {
    |val: &Bytes| Ok(Rc::from(val.as_ref()))
  });
  type_owned!(@clone Network: Rc<[u8]>);
};
