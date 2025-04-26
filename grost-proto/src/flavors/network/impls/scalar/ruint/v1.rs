use ruint_1::Uint;

use crate::network_varint;

network_varint!(
  Uint<BITS, LMITS> [const BITS: usize, const LMITS: usize]
);
