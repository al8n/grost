use arrayvec_0_7::ArrayVec;

use crate::{
  buffer::Chunk,
  decode::BytesSlice,
  flavors::groto::{Error, LengthDelimited},
};

mod packed;
mod repeated;

#[cfg(not(any(feature = "std", feature = "alloc")))]
pub fn larger_than_array_capacity<const CAP: usize>() -> Error {
  Error::custom("cannot allocate array with length greater than the capacity")
}

#[cfg(any(feature = "std", feature = "alloc"))]
pub fn larger_than_array_capacity<const CAP: usize>() -> Error {
  Error::custom(std::format!(
    "cannot allocate array with length greater than the capacity {CAP}",
  ))
}

flatten_state!(ArrayVec<T, N> [const N: usize]);
partial_state!(ArrayVec<T, N> [const N: usize] => ArrayVec<T::Output, N>);

partial_ref_state!(@bytes ArrayVec<u8, N> [const N: usize]);
partial_ref_state!(@packed ArrayVec<T, N> [const N: usize]);
partial_ref_state!(@repeated ArrayVec<T, N> [const N: usize]);

ref_state!(@bytes ArrayVec<u8, N> [const N: usize]);
ref_state!(@packed ArrayVec<T, N> [const N: usize]);
ref_state!(@repeated ArrayVec<T, N> [const N: usize]);

default_wire_format!(@bytes ArrayVec<u8, N> [const N: usize]);
default_wire_format!(@packed ArrayVec<T, N> [const N: usize]);
default_wire_format!(@repeated ArrayVec<T, N> [const N: usize]);

encode_list!(@bytes ArrayVec<u8, N> [const N: usize]);
encode_list!(@packed ArrayVec<T, N> [const N: usize]);
encode_list!(@repeated ArrayVec<T, N> [const N: usize]);
encode_list!(@borrow ArrayVec<&'b T, N> [const N: usize]);

length!(ArrayVec<T, N> [const N: usize]);
selectable!(ArrayVec<T, N> [const N: usize]);

bidi_equivalent!(:<RB: Chunk>: [const N: usize] impl<ArrayVec<u8, N>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
bidi_equivalent!([const N: usize] impl <ArrayVec<u8, N>, LengthDelimited> for <[u8], LengthDelimited>);
