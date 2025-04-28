#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::flavors::network::Network;
  use bytes_1::Bytes;
  use std::vec::Vec;

  use crate::{bytes_bridge, into_target, type_owned, type_ref};

  bytes_bridge!(Network: Vec<u8> {
    from_slice: |val: &[u8]| val.to_vec();
    as_slice: AsRef::as_ref;
  
    type EncodedOwned = Bytes {
      from_ref: |s: &Bytes| Ok(Arc::<[u8]>::from(s.as_ref()));
      from: |s: Bytes| Ok(Arc::from(s.as_ref()));
    }
  },);

  into_target!(Network: Bytes => Vec<u8> {
    |val: Bytes| Ok(val.into())
  });
  into_target!(Network: &[u8] => Vec<u8> {
    |val: &[u8]| Ok(val.to_vec())
  });
  into_target!(@self Network: Vec<u8>);
  type_ref!(@mapping Network: &[u8] => Vec<u8> {
    |val: &[u8]| Ok(val.to_vec())
  });
  type_owned!(@mapping Network: Bytes => Vec<u8> {
    |val: &Bytes| Ok(val.to_vec())
  });
  type_owned!(@clone Network: Vec<u8>);
  bytes_message!(Vec<u8> => Bytes);
};
