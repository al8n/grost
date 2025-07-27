use crate::groto_varint;
use bnum_0_13::*;

groto_varint!(
  @scalar
  BUintD8<N> [const N: usize],
  BUintD16<N> [const N: usize],
  BUintD32<N> [const N: usize],
  BUint<N> [const N: usize],
  BIntD8<N> [const N: usize],
  BIntD16<N> [const N: usize],
  BIntD32<N> [const N: usize],
  BInt<N> [const N: usize],
);
