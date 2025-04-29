#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::flavors::network::Network;
  use bytes_1::Bytes;
  use triomphe_0_1::Arc;

  use crate::{into_target, type_owned, type_ref};

  bytes_bridge!(Network: Arc<[u8]> {
    from_slice: |val: &[u8]| Arc::<[u8]>::from(val);
    as_slice: AsRef::as_ref;
  
    type EncodedOwned = Bytes;
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
};
