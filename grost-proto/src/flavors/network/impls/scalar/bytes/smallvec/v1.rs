use smallvec_1::SmallVec;

use crate::{
  decoded_state, default_wire_format, flatten_state,
  flavors::{Network, network::LengthDelimited},
};

default_wire_format!(Network: SmallVec<[u8; N]> [const N: usize] as LengthDelimited);
decoded_state!(
  &'a Network: SmallVec<[u8; N]> [const N: usize] as LengthDelimited => &'a [u8]
);
flatten_state!(SmallVec<[u8; N]> [const N: usize]);

#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::flavors::network::Network;
  use smallvec_1::SmallVec;

  bytes_bridge!(Network: SmallVec<[u8; N]> [const N: usize] {
    from_slice: |val: &[u8]| SmallVec::<[u8; N]>::from(val);
    as_slice: AsRef::as_ref;
  },);
};
