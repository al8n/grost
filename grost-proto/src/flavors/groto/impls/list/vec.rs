use std::vec::Vec;

use crate::{buffer::ReadBuf, decode::BytesSlice, flavors::groto::LengthDelimited};

mod packed;
mod repeated;

partial_ref_state!(@bytes Vec<u8>);
partial_ref_state!(@packed Vec<T>);
partial_ref_state!(@repeated Vec<T>);

ref_state!(@bytes Vec<u8>);
ref_state!(@packed Vec<T>);
ref_state!(@repeated Vec<T>);

default_wire_format!(@bytes Vec<u8>);
default_wire_format!(@packed Vec<T>);
default_wire_format!(@repeated Vec<T>);

list!(@length Vec<T>);
list!(@flatten_state Vec<T>);
list!(@partial_state Vec<T> => Vec<T::Output>);
list!(@selectable Vec<T>);
list!(@decode_to_packed_decoder(from_bytes) Vec<u8> {
  Vec::from
});
list!(
  @encode(packed) Vec<T>
);
list!(
  @encode(repeated) Vec<T>
);
list!(
  @encode(borrow) Vec<&'b T>
);
list!(
  @encode(bytes) Vec<u8>
);

// Vec<u8> is the same as encode BytesSlice<RB>
bidi_equivalent!(:<RB: ReadBuf>: impl<Vec<u8>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
// Vec<u8> is the same as encode [u8]
bidi_equivalent!(impl <Vec<u8>, LengthDelimited> for <[u8], LengthDelimited>);
