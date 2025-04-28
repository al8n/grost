#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::flavors::network::Network;
  use bytes_1::Bytes;
  use std::sync::Arc;

  use crate::{bytes_bridge, into_target, type_owned, type_ref};

  bytes_bridge!(Network: Arc<[u8]> {
    from_slice: |val: &[u8]| Arc::<[u8]>::from(val);
    as_slice: AsRef::as_ref;
  
    type EncodedOwned = Bytes {
      from_ref: |s: &Bytes| Ok(Arc::<[u8]>::from(s.as_ref()));
      from: |s: Bytes| Ok(Arc::from(s.as_ref()));
    }
  },);

  into_target!(Network: Bytes => Arc<[u8]> {
    |val: Bytes| Ok(Arc::from(val.as_ref()))
  });
  into_target!(Network: &[u8] => Arc<[u8]> {
    |val: &[u8]| Ok(Arc::from(val))
  });
  into_target!(@self Network: Arc<[u8]>);
  type_ref!(@mapping Network: &[u8] => Arc<[u8]> {
    |val: &[u8]| Ok(Arc::from(val))
  });
  type_owned!(@mapping Network: Bytes => Arc<[u8]> {
    |val: &Bytes| Ok(Arc::from(val.as_ref()))
  });
  type_owned!(@clone Network: Arc<[u8]>);
  bytes_message!(Arc<[u8]> => Bytes);
};
