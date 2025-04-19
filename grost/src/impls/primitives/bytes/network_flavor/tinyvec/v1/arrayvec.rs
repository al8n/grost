use tinyvec_1::{Array, ArrayVec};

use super::larger_than_array_capacity;
use crate::{
  Decode, DecodeOwned, Encode, IntoTarget, Message, PartialMessage, TypeOwned, TypeRef, Wirable,
  buffer::Buffer,
  flavors::network::{Context, DecodeError, EncodeError, Network, WireType},
  unknown::UnknownBuffer,
};

impl<A> Wirable<Network> for ArrayVec<A>
where
  A: Array<Item = u8>,
{
  const WIRE_TYPE: WireType = {
    match A::CAPACITY {
      0 => WireType::Zst,
      _ => WireType::LengthDelimited,
    }
  };
}

impl<A> Encode<Network> for ArrayVec<A>
where
  A: Array<Item = u8>,
{
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    if A::CAPACITY == 0 {
      return Ok(0);
    }

    self.as_slice().encode(context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    if A::CAPACITY == 0 {
      return 0;
    }

    self.as_slice().encoded_len(context)
  }
}

impl<'de, A> Decode<'de, Network, Self> for ArrayVec<A>
where
  A: Array<Item = u8>,
{
  fn decode<B>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownBuffer<Network, &'de [u8]>,
  {
    decode_to_array(src)
  }
}

impl<A> DecodeOwned<Network, Self> for ArrayVec<A>
where
  A: Array<Item = u8> + 'static,
{
  fn decode_owned<B, UB>(context: &Context, src: B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: Buffer + 'static,
    UB: UnknownBuffer<Network, B> + 'static,
  {
    <Self as Decode<'_, Network, Self>>::decode::<()>(context, src.as_bytes())
  }
}

impl<A> PartialMessage<Network> for ArrayVec<A>
where
  A: Array<Item = u8> + Clone,
{
  type UnknownBuffer<B: ?Sized> = ();

  type Encoded<'a>
    = &'a [A::Item]
  where
    Self: Sized + 'a;

  type Borrowed<'a>
    = &'a Self
  where
    Self: 'a;

  type EncodedOwned
    = Self
  where
    Self: Sized + 'static;
}

impl<A> Message<Network> for ArrayVec<A>
where
  A: Array<Item = u8> + Clone,
{
  type Partial = Self;

  type Encoded<'a>
    = &'a [A::Item]
  where
    Self: Sized + 'a;

  type Borrowed<'a>
    = &'a Self
  where
    Self: 'a;

  type EncodedOwned
    = Self
  where
    Self: Sized + 'static;
}

impl<A> IntoTarget<Network, Self> for ArrayVec<A>
where
  A: Array,
{
  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

impl<A> TypeOwned<Network, Self> for ArrayVec<A>
where
  A: Array,
  A::Item: Clone,
{
  fn to(&self) -> Result<Self, DecodeError> {
    let mut arr = ArrayVec::new();
    arr.extend_from_slice(self.as_slice());
    Ok(arr)
  }
}

impl<A> IntoTarget<Network, ArrayVec<A>> for &[A::Item]
where
  A: Array,
  A::Item: Clone,
{
  fn into_target(self) -> Result<ArrayVec<A>, DecodeError> {
    decode_to_array(self).map(|(_, arr)| arr)
  }
}

impl<A> TypeRef<Network, ArrayVec<A>> for &[A::Item]
where
  A: Array,
  A::Item: Clone,
{
  fn to(&self) -> Result<ArrayVec<A>, DecodeError> {
    decode_to_array(self).map(|(_, arr)| arr)
  }
}

#[inline]
fn decode_to_array<A>(src: &[A::Item]) -> Result<(usize, ArrayVec<A>), DecodeError>
where
  A: Array,
  A::Item: Clone,
{
  if A::CAPACITY == 0 {
    return Ok((0, ArrayVec::new()));
  }

  if src.len() > A::CAPACITY {
    return Err(larger_than_array_capacity::<A>());
  }

  let mut arr = ArrayVec::new();
  arr.extend_from_slice(src);

  Ok((src.len(), arr))
}
