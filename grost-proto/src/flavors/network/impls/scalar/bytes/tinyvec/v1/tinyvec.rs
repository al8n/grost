use tinyvec_1::TinyVec;

use crate::{
  decoded_state, default_wire_format, flatten_state,
  flavors::{Network, network::LengthDelimited},
};

default_wire_format!(Network: TinyVec<[u8; N]> [const N: usize] as LengthDelimited);
decoded_state!(
  &'a Network: TinyVec<[u8; N]> [const N: usize] as LengthDelimited => &'a [u8]
);
flatten_state!(TinyVec<[u8; N]> [const N: usize]);

#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::flavors::network::Network;
  use ::tinyvec_1::TinyVec;

  bytes_bridge!(Network: TinyVec<[u8; N]> [const N: usize] {
    from_slice: |val: &[u8]| TinyVec::<[u8; N]>::from(val);
    as_slice: AsRef::as_ref;
  },);
};
