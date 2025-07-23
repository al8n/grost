use tinyvec_1::TinyVec;

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

flatten_state!(:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);

default_wire_format!(@bytes :<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>);
default_wire_format!(@packed :<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
default_wire_format!(@repeated :<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);

partial_ref_state!(@bytes :<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>);
partial_ref_state!(@packed :<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
partial_ref_state!(@repeated :<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);

ref_state!(@bytes :<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>);
ref_state!(@packed :<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
ref_state!(@repeated :<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);

encode_list!(@bytes :<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>);
encode_list!(@packed :<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
encode_list!(@repeated :<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
encode_list!(@borrow :<A: tinyvec_1::Array<Item = &'b T>>: TinyVec<A>);

length!(:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
selectable!(:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);

bidi_equivalent!(:<RB: ReadBuf, A: tinyvec_1::Array<Item = u8>>: impl<TinyVec<A>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
bidi_equivalent!(:<A: tinyvec_1::Array<Item = u8>>: impl <TinyVec<A>, LengthDelimited> for <[u8], LengthDelimited>);

impl<T, const N: usize> State<Partial<Groto>> for TinyVec<[T; N]>
where
  T: State<Partial<Groto>> + Default,
  T::Output: Sized + Default,
{
  type Output = TinyVec<[T::Output; N]>;
}
