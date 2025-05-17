use crate::{
  decoded_state, default_wire_format,
  flavors::{Network, network::LengthDelimited},
};

use std::vec::Vec;

decoded_state!(
  &'a Network: Vec<u8> as LengthDelimited => &'a [u8]
);
default_wire_format!(Network: Vec<u8> as LengthDelimited);

#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::flavors::network::Network;
  use bytes_1::Bytes;

  use crate::{into_target, type_owned, type_ref};

  bytes_bridge!(Network: Vec<u8> {
    from_slice: |val: &[u8]| val.to_vec();
    as_slice: AsRef::as_ref;

    type EncodedOwned = Bytes;
  },);

  into_target!(Network: Bytes => Vec<u8> {
    |val: Bytes| Ok(val.into())
  });
  into_target!(Network: &[u8] => Vec<u8> {
    |val: &[u8]| Ok(val.to_vec())
  });

  type_ref!(Network: &[u8] => Vec<u8> {
    |val: &[u8]| Ok(val.to_vec())
  });
  type_owned!(Network: Bytes => Vec<u8> {
    |val: &Bytes| Ok(val.to_vec())
  });
};
