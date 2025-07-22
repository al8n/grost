use smallvec_1::SmallVec;

use crate::{buffer::ReadBuf, decode::BytesSlice, flavors::groto::LengthDelimited};

mod packed;
mod repeated;

partial_ref_state!(@bytes SmallVec<[u8; N]> [const N: usize]);
partial_ref_state!(@packed SmallVec<[T; N]> [const N: usize]);
partial_ref_state!(@repeated SmallVec<[T; N]> [const N: usize]);

ref_state!(@bytes SmallVec<[u8; N]> [const N: usize]);
ref_state!(@packed SmallVec<[T; N]> [const N: usize]);
ref_state!(@repeated SmallVec<[T; N]> [const N: usize]);

default_wire_format!(@bytes SmallVec<[u8; N]> [const N: usize]);
default_wire_format!(@packed SmallVec<[T; N]> [const N: usize]);
default_wire_format!(@repeated SmallVec<[T; N]> [const N: usize]);

list!(@length SmallVec<[T; N]> [const N: usize]);
list!(@flatten_state SmallVec<[T; N]> [const N: usize]);
list!(@partial_state SmallVec<[T; N]> [const N: usize] => SmallVec<[T::Output; N]>);
list!(@selectable SmallVec<[T; N]> [const N: usize]);
list!(
  @encode(packed) SmallVec<[T; N]> [const N: usize]
);
list!(
  @encode(repeated) SmallVec<[T; N]> [const N: usize]
);
list!(
  @encode(borrow) SmallVec<[&'b T; N]> [const N: usize]
);
list!(
  @encode(bytes) SmallVec<[u8; N]> [const N: usize]
);
bidi_equivalent!(:<RB: ReadBuf>: [const N: usize] impl<SmallVec<[u8; N]>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
bidi_equivalent!([const N: usize] impl <SmallVec<[u8; N]>, LengthDelimited> for <[u8], LengthDelimited>);
