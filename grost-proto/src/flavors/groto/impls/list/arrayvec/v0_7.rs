use arrayvec_0_7::ArrayVec;

use crate::{
  buffer::ReadBuf,
  decode::BytesSlice,
  encode::{Encode, PartialEncode},
  flavors::{
    Borrowed, Groto, Packed, WireFormat,
    groto::{Error, LengthDelimited},
  },
  state::State,
};

mod packed;
mod repeated;

default_wire_format!(@bytes ArrayVec<u8, N> [const N: usize]);
default_wire_format!(@packed ArrayVec<T, N> [const N: usize]);
default_wire_format!(@repeated ArrayVec<T, N> [const N: usize]);

list!(@length ArrayVec<T, N> [const N: usize]);
list!(@flatten_state ArrayVec<T, N> [const N: usize]);
list!(@partial_state ArrayVec<T, N> [const N: usize] => ArrayVec<T::Output, N>);
list!(@selectable ArrayVec<T, N> [const N: usize]);
list!(
  @decode_to_packed_decoder(try_from_bytes) ArrayVec<u8, N> [const N: usize] {
    |bytes| ArrayVec::try_from(bytes).map_err(|_| crate::__private::larger_than_array_capacity::<Groto, N>())
  }
);
list!(
  @encode(packed) ArrayVec<T, N> [const N: usize]
);
list!(
  @encode(borrow) ArrayVec<&'b T, N> [const N: usize]
);
list!(
  @encode(bytes) ArrayVec<u8, N> [const N: usize]
);
bidi_equivalent!(:<RB: ReadBuf>: [const N: usize] impl<ArrayVec<u8, N>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
bidi_equivalent!([const N: usize] impl <ArrayVec<u8, N>, LengthDelimited> for <[u8], LengthDelimited>);

bidi_equivalent!(@encode :<T: Encode<W, Groto>, W: WireFormat<Groto>>:[const N: usize] impl <ArrayVec<T, N>, Packed<W>> for <[T], Packed<W>>);
bidi_equivalent!(@partial_encode :<T: PartialEncode<W, Groto>, W: WireFormat<Groto>>:[const N: usize] impl <ArrayVec<T, N>, Packed<W>> for <[T], Packed<W>>);

bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <ArrayVec<T, N>, Packed<W>>);
bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <ArrayVec<T, N>, Packed<W>>);

bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize] impl <ArrayVec<&'a T, N>, Borrowed<'a, Packed<W>>> for <ArrayVec<T, N>, Packed<W>>);
bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize] impl <ArrayVec<&'a T, N>, Borrowed<'a, Packed<W>>> for <ArrayVec<T, N>, Packed<W>>);
