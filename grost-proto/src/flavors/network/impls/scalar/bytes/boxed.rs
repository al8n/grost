#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::flavors::network::Network;
  use bytes_1::Bytes;
  use std::boxed::Box;

  use crate::{bytes_bridge, into_target, type_owned, type_ref};

  bytes_bridge!(Network: Box<[u8]> {
    from_slice: |val: &[u8]| Box::<[u8]>::from(val);
    as_slice: AsRef::as_ref;
  
    type EncodedOwned = Bytes {
      from_ref: |s: &Bytes| Ok(Box::<[u8]>::from(s.as_ref()));
      from: |s: Bytes| Ok(Box::from(s.as_ref()));
    }
  },);

  into_target!(Network: Bytes => Box<[u8]> {
    |val: Bytes| Ok(Box::from(val.as_ref()))
  });
  into_target!(Network: &[u8] => Box<[u8]> {
    |val: &[u8]| Ok(Box::from(val))
  });
  into_target!(@self Network: Box<[u8]>);
  type_ref!(@mapping Network: &[u8] => Box<[u8]> {
    |val: &[u8]| Ok(Box::from(val))
  });
  type_owned!(@mapping Network: Bytes => Box<[u8]> {
    |val: &Bytes| Ok(Box::from(val.as_ref()))
  });
  type_owned!(@clone Network: Box<[u8]>);
  bytes_message!(Box<[u8]> => Bytes);
};
