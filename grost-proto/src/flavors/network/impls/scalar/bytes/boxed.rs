#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::flavors::network::Network;
  use bytes_1::Bytes;
  use std::boxed::Box;

  use crate::{into_target, type_owned, type_ref};

  into_target!(Network: Bytes => Box<[u8]> {
    |val: Bytes| Ok(Box::from(val.as_ref()))
  });
  into_target!(Network: &[u8] => Box<[u8]> {
    |val: &[u8]| Ok(Box::from(val))
  });
  type_ref!( Network: &[u8] => Box<[u8]> {
    |val: &[u8]| Ok(Box::from(val))
  });
  type_owned!( Network: Bytes => Box<[u8]> {
    |val: &Bytes| Ok(Box::from(val.as_ref()))
  });
  bytes_message!(Box<[u8]> => Bytes);
};
