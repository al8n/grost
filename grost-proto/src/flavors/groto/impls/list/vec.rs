use std::vec::Vec;

use crate::{buffer::Chunk, decode::BytesSlice, flavors::groto::LengthDelimited};

mod packed;
mod repeated;

flatten_state!(Vec<T>);
partial_state!(Vec<T> => Vec<T::Output>);

partial_ref_state!(@bytes Vec<u8>);
partial_ref_state!(@packed Vec<T>);
partial_ref_state!(@repeated Vec<T>);

ref_state!(@bytes Vec<u8>);
ref_state!(@packed Vec<T>);
ref_state!(@repeated Vec<T>);

default_wire_format!(@bytes Vec<u8>);
default_wire_format!(@packed Vec<T>);
default_wire_format!(@repeated Vec<T>);

encode_list!(@bytes Vec<u8>);
encode_list!(@packed Vec<T>);
encode_list!(@repeated Vec<T>);
encode_list!(@borrow Vec<&'b T>);

length!(Vec<T>);
selectable!(Vec<T>);

// Vec<u8> is the same as encode BytesSlice<RB>
bidi_equivalent!(:<RB: Chunk>: impl<Vec<u8>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
// Vec<u8> is the same as encode [u8]
bidi_equivalent!(impl <Vec<u8>, LengthDelimited> for <[u8], LengthDelimited>);
