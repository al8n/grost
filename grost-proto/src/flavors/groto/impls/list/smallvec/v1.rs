use smallvec_1::SmallVec;

use crate::{buffer::Chunk, decode::BytesSlice, flavors::groto::LengthDelimited};

mod packed;
mod repeated;

flatten_state!(SmallVec<[T; N]> [const N: usize]);
partial_state!(SmallVec<[T; N]> [const N: usize] => SmallVec<[T::Output; N]>);

partial_ref_state!(@bytes SmallVec<[u8; N]> [const N: usize]);
partial_ref_state!(@packed SmallVec<[T; N]> [const N: usize]);
partial_ref_state!(@repeated SmallVec<[T; N]> [const N: usize]);

ref_state!(@bytes SmallVec<[u8; N]> [const N: usize]);
ref_state!(@packed SmallVec<[T; N]> [const N: usize]);
ref_state!(@repeated SmallVec<[T; N]> [const N: usize]);

default_wire_format!(@bytes SmallVec<[u8; N]> [const N: usize]);
default_wire_format!(@packed SmallVec<[T; N]> [const N: usize]);
default_wire_format!(@repeated SmallVec<[T; N]> [const N: usize]);

encode_list!(@bytes SmallVec<[u8; N]> [const N: usize]);
encode_list!(@packed SmallVec<[T; N]> [const N: usize]);
encode_list!(@repeated SmallVec<[T; N]> [const N: usize]);
encode_list!(@borrow SmallVec<[&'b T; N]> [const N: usize]);

length!(SmallVec<[T; N]> [const N: usize]);
selectable!(SmallVec<[T; N]> [const N: usize]);

bidi_equivalent!(:<RB: Chunk>: [const N: usize] impl<SmallVec<[u8; N]>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
bidi_equivalent!([const N: usize] impl <SmallVec<[u8; N]>, LengthDelimited> for <[u8], LengthDelimited>);
