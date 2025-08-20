use crate::{
  buffer::Chunk,
  decode::BytesSlice,
  flavors::groto::{DecodeError, EncodeError, LengthDelimited},
  selection::Selector,
  state::{Partial, PartialRef, Ref, State},
};

mod packed;
mod repeated;

#[cfg(not(any(feature = "std", feature = "alloc")))]
pub fn larger_than_array_capacity<const CAP: usize>() -> DecodeError {
  DecodeError::other("cannot allocate array with length greater than the capacity")
}

#[cfg(any(feature = "std", feature = "alloc"))]
pub fn larger_than_array_capacity<const CAP: usize>() -> DecodeError {
  DecodeError::other(std::format!(
    "cannot allocate array with length greater than the capacity {CAP}",
  ))
}

flatten_state!([T; N] [const N: usize]);
partial_state!([T; N] [const N: usize] => [T::Output; N]);

partial_ref_state!(@bytes [u8; N] [const N: usize]);
partial_ref_state!(@packed [T; N] [const N: usize]);
partial_ref_state!(@repeated [T; N] [const N: usize]);

ref_state!(@bytes [u8; N] [const N: usize]);
ref_state!(@packed [T; N] [const N: usize]);
ref_state!(@repeated [T; N] [const N: usize]);

default_wire_format!(@bytes [u8; N] [const N: usize]);
default_wire_format!(@packed [T; N] [const N: usize]);
default_wire_format!(@repeated [T; N] [const N: usize]);

encode_list!(@bytes [u8; N] [const N: usize]);
encode_list!(@packed [T; N] [const N: usize]);
encode_list!(@repeated [T; N] [const N: usize]);
encode_list!(@borrow [&'b T; N] [const N: usize]);

selectable!([T; N] [const N: usize]);

impl<T, const N: usize> crate::encode::Length for [T; N] {
  fn length(&self) -> usize {
    N
  }
}

// TODO(al8n): change this to single direction equivalent
bidi_equivalent!(:<RB: Chunk>: [const N: usize] impl<[u8; N], LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
bidi_equivalent!([const N: usize] impl <[u8; N], LengthDelimited> for <[u8], LengthDelimited>);
