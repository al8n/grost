use tinyvec_1::ArrayVec;

use crate::{
  buffer::ReadBuf,
  decode::BytesSlice,
  flavors::{Groto, groto::LengthDelimited},
  state::Partial,
  state::State,
};

use super::{DefaultEncode, DefaultPartialEncode};

mod packed;
mod repeated;

flatten_state!(:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);

default_wire_format!(@bytes :<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A>);
default_wire_format!(@packed :<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
default_wire_format!(@repeated :<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);

partial_ref_state!(@bytes :<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A>);
partial_ref_state!(@packed :<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
partial_ref_state!(@repeated :<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);

ref_state!(@bytes :<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A>);
ref_state!(@packed :<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
ref_state!(@repeated :<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);

encode_list!(@bytes :<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A>);
encode_list!(@packed :<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
encode_list!(@repeated :<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
encode_list!(@borrow :<A: tinyvec_1::Array<Item = &'b T>>: ArrayVec<A>);

length!(:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
selectable!(:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
// list!(
//   @decode_to_packed_decoder(try_from_bytes):<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A> {
//     |bytes| ArrayVec::try_from(bytes).map_err(|_| super::larger_than_array_capacity::<A>())
//   }
// );

bidi_equivalent!(:<RB: ReadBuf, A: tinyvec_1::Array<Item = u8>>: impl<ArrayVec<A>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
bidi_equivalent!(:<A: tinyvec_1::Array<Item = u8>>: impl <ArrayVec<A>, LengthDelimited> for <[u8], LengthDelimited>);

impl<T, const N: usize> State<Partial<Groto>> for ArrayVec<[T; N]>
where
  T: State<Partial<Groto>>,
  T::Output: Sized,
{
  type Output = ArrayVec<[T::Output; N]>;
}
