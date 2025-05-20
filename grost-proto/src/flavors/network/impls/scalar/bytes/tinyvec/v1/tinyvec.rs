use tinyvec_1::TinyVec;

use crate::{
  decoded_state,
  flavors::{Network, network::LengthDelimited},
};

decoded_state!(
  &'a Network: TinyVec<[u8; N]> [const N: usize] as LengthDelimited => &'a [u8]
);
