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

  bytes_bridge!(Network: Vec<u8> {
    from_slice: |val: &[u8]| val.to_vec();
    as_slice: AsRef::as_ref;
  },);
};
