use tinyvec_1::{Array, ArrayVec};

use super::larger_than_array_capacity;
use crate::{
  DecodeError, Decode, DecodeOwned, IntoTarget, Message, Encode, TypeOwned, TypeRef,
  Wirable, WireType,
};

impl<A> Wirable for ArrayVec<A>
where
  A: Array<Item = u8>,
{
  const WIRE_TYPE: WireType = {
    match A::CAPACITY {
      0 => WireType::Merged,
      _ => WireType::LengthDelimited,
    }
  };
}

impl<A> Encode for ArrayVec<A>
where
  A: Array<Item = u8>,
{
  fn encode(&self, buf: &mut [u8]) -> Result<usize, crate::EncodeError> {
    if A::CAPACITY == 0 {
      return Ok(0);
    }

    self.as_slice().encode(buf)
  }

  fn encoded_len(&self) -> usize {
    if A::CAPACITY == 0 {
      return 0;
    }

    self.as_slice().encoded_len()
  }
}

impl<'de, A> Decode<'de> for ArrayVec<A>
where
  A: Array<Item = u8>,
{
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: crate::UnknownRefBuffer<'de>,
  {
    decode_to_array(src)
  }
}

impl<A> DecodeOwned for ArrayVec<A>
where
  A: Array<Item = u8> + 'static,
{
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn decode_from_bytes<U>(src: bytes_1::Bytes, _: &mut U) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    U: crate::UnknownBuffer<bytes_1::Bytes>,
  {
    decode_to_array(src.as_ref())
  }
}

impl<A> Message for ArrayVec<A>
where
  A: Array<Item = u8> + Clone,
{
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

impl<A> IntoTarget<Self> for ArrayVec<A>
where
  A: Array,
{
  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

impl<A> TypeOwned<Self> for ArrayVec<A>
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

impl<A> IntoTarget<ArrayVec<A>> for &[A::Item]
where
  A: Array,
  A::Item: Clone,
{
  fn into_target(self) -> Result<ArrayVec<A>, DecodeError> {
    decode_to_array(self).map(|(_, arr)| arr)
  }
}

impl<A> TypeRef<ArrayVec<A>> for &[A::Item]
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
