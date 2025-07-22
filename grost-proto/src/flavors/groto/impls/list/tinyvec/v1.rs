mod packed;
mod repeated;

use tinyvec_1::ArrayVec;

use crate::{
  buffer::ReadBuf,
  convert::Partial,
  decode::BytesSlice,
  encode::{Encode, PartialEncode},
  flavors::{
    Borrowed, Groto, Packed, WireFormat,
    groto::{Error, LengthDelimited},
  },
  state::State,
};

#[cfg(not(any(feature = "std", feature = "alloc")))]
pub fn larger_than_array_capacity<A>() -> Error
where
  A: tinyvec_1::Array<Item = u8>,
{
  Error::custom("cannot decode array with length greater than the capacity")
}

#[cfg(any(feature = "std", feature = "alloc"))]
pub fn larger_than_array_capacity<A>() -> Error
where
  A: tinyvec_1::Array<Item = u8>,
{
  Error::custom(std::format!(
    "cannot decode array with length greater than the capacity {}",
    A::CAPACITY
  ))
}

trait DefaultEncode<W: WireFormat<Groto>>: Encode<W, Groto> + Default {}

impl<T, W> DefaultEncode<W> for T
where
  T: Encode<W, Groto> + Default,
  W: WireFormat<Groto>,
{
}

trait DefaultPartialEncode<W: WireFormat<Groto>>: PartialEncode<W, Groto> + Default {}

impl<T, W> DefaultPartialEncode<W> for T
where
  T: PartialEncode<W, Groto> + Default,
  W: WireFormat<Groto>,
{
}

default_wire_format!(@bytes :<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A>);
default_wire_format!(@packed :<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
default_wire_format!(@repeated :<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);

list!(@length:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
list!(@flatten_state:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
list!(@selectable:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
list!(
  @decode_to_packed_decoder(try_from_bytes):<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A> {
    |bytes| ArrayVec::try_from(bytes).map_err(|_| larger_than_array_capacity::<A>())
  }
);
list!(
  @encode(packed):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>
);
list!(
  @encode(repeated):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>
);
list!(
  @encode(borrow):<A: tinyvec_1::Array<Item = &'b T>>: ArrayVec<A>
);
list!(
  @encode(bytes):<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A>
);
bidi_equivalent!(:<RB: ReadBuf, A: tinyvec_1::Array<Item = u8>>: impl<ArrayVec<A>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
bidi_equivalent!(:<A: tinyvec_1::Array<Item = u8>>: impl <ArrayVec<A>, LengthDelimited> for <[u8], LengthDelimited>);

bidi_equivalent!(@encode :<T: DefaultEncode<W>, W: WireFormat<Groto>>:[const N: usize] impl <ArrayVec<[T; N]>, Packed<W>> for <[T], Packed<W>>);
bidi_equivalent!(@partial_encode :<T: DefaultPartialEncode<W>, W: WireFormat<Groto>>:[const N: usize] impl <ArrayVec<[T; N]>, Packed<W>> for <[T], Packed<W>>);

bidi_equivalent!(@encode 'a:<T: DefaultEncode<W>, W:WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <ArrayVec<[T; N]>, Packed<W>>);
bidi_equivalent!(@partial_encode 'a:<T: DefaultPartialEncode<W>, W: WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <ArrayVec<[T; N]>, Packed<W>>);

impl<T, const N: usize> State<Partial<Groto>> for ArrayVec<[T; N]>
where
  T: State<Partial<Groto>>,
  T::Output: Sized,
{
  type Output = ArrayVec<[T::Output; N]>;
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use tinyvec_1::TinyVec;

  default_wire_format!(@bytes :<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>);
  default_wire_format!(@packed :<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
  default_wire_format!(@repeated :<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);

  list!(@length:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
  list!(@flatten_state:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
  list!(@selectable:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
  list!(
    @decode_to_packed_decoder(from_bytes):<A: tinyvec_1::Array<Item = u8>>: TinyVec<A> {
      TinyVec::from
    }
  );
  list!(
    @encode(packed):<A: tinyvec_1::Array<Item = T>>: TinyVec<A>
  );
  list!(
    @encode(repeated):<A: tinyvec_1::Array<Item = T>>: TinyVec<A>
  );
  list!(
    @encode(borrow):<A: tinyvec_1::Array<Item = &'b T>>: TinyVec<A>
  );
  list!(
    @encode(bytes):<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>
  );
  bidi_equivalent!(:<RB: ReadBuf, A: tinyvec_1::Array<Item = u8>>: impl<TinyVec<A>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
  bidi_equivalent!(:<A: tinyvec_1::Array<Item = u8>>: impl <TinyVec<A>, LengthDelimited> for <[u8], LengthDelimited>);

  bidi_equivalent!(@encode :<T: DefaultEncode<W>, W: WireFormat<Groto>>:[const N: usize] impl <TinyVec<[T; N]>, Packed<W>> for <[T], Packed<W>>);
  bidi_equivalent!(@partial_encode :<T: DefaultPartialEncode<W>, W: WireFormat<Groto>>:[const N: usize] impl <TinyVec<[T; N]>, Packed<W>> for <[T], Packed<W>>);

  bidi_equivalent!(@encode 'a:<T: DefaultEncode<W>, W:WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <TinyVec<[T; N]>, Packed<W>>);
  bidi_equivalent!(@partial_encode 'a:<T: DefaultPartialEncode<W>, W: WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <TinyVec<[T; N]>, Packed<W>>);

  impl<T, const N: usize> State<Partial<Groto>> for TinyVec<[T; N]>
  where
    T: State<Partial<Groto>> + Default,
    T::Output: Sized + Default,
  {
    type Output = TinyVec<[T::Output; N]>;
  }
};
