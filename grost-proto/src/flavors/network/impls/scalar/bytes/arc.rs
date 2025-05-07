#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::flavors::network::Network;
  use bytes_1::Bytes;
  use std::sync::Arc;

  use crate::{into_target, type_owned, type_ref};

  into_target!(Network: Bytes => Arc<[u8]> {
    |val: Bytes| Ok(Arc::from(val.as_ref()))
  });
  into_target!(Network: &[u8] => Arc<[u8]> {
    |val: &[u8]| Ok(Arc::from(val))
  });
  type_ref!( Network: &[u8] => Arc<[u8]> {
    |val: &[u8]| Ok(Arc::from(val))
  });
  type_owned!( Network: Bytes => Arc<[u8]> {
    |val: &Bytes| Ok(Arc::from(val.as_ref()))
  });
  bytes_message!(Arc<[u8]> => Bytes);
};
