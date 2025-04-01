
use tinyvec_1::{TinyVec, Array};

use crate::{bytes::Bytes, DecodeError, Deserialize, DeserializeOwned, IntoTarget, Message, Serialize, TypeOwned, TypeRef, Wirable, WireType};

impl<A> Wirable for TinyVec<A>
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

impl<A> Serialize for TinyVec<A>
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

impl<'de, A> Deserialize<'de> for TinyVec<A>
where
  A: Array<Item = u8>,
{
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: crate::UnknownRefBuffer<'de>
  {
    decode_to_array(src)
  }
}

impl<A> DeserializeOwned for TinyVec<A>
where
  A: Array<Item = u8> + 'static,
{
  fn decode_from_bytes<U>(
    src: bytes_1::Bytes,
    _: &mut U,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    U: crate::UnknownBuffer<bytes_1::Bytes>
  {
    decode_to_array(src.as_ref())
  }
}

impl<A> Message for TinyVec<A>
where
  A: Array<Item = u8> + Clone,
{
  type Serialized<'a> = &'a [A::Item]
  where
    Self: Sized + 'a;

  type Borrowed<'a> = &'a Self
  where
    Self: 'a;

  type SerializedOwned = Bytes
  where
    Self: Sized + 'static;
}

impl<A> IntoTarget<Self> for TinyVec<A>
where
  A: Array,
{
  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

impl<A> TypeOwned<Self> for TinyVec<A>
where
  A: Array,
  A::Item: Clone,
{
  fn to(&self) -> Result<Self, DecodeError> {
    Ok(Self::from(self.as_slice()))
  }
}

impl<A> IntoTarget<TinyVec<A>> for &[A::Item]
where
  A: Array,
  A::Item: Clone,
{
  fn into_target(self) -> Result<TinyVec<A>, DecodeError> {
    Ok(TinyVec::from(self))
  }
}

impl<A> TypeRef<TinyVec<A>> for &[A::Item]
where
  A: Array,
  A::Item: Clone,
{
  fn to(&self) -> Result<TinyVec<A>, DecodeError> {
    Ok(TinyVec::from(*self))
  }
}

impl<A> IntoTarget<TinyVec<A>> for Bytes 
where
  A: Array<Item = u8>,
{
  fn into_target(self) -> Result<TinyVec<A>, DecodeError> {
    Ok(TinyVec::from(self.as_ref()))
  }
}

impl<A> TypeOwned<TinyVec<A>> for Bytes
where
  A: Array<Item = u8>,
  A::Item: Clone,
{
  fn to(&self) -> Result<TinyVec<A>, DecodeError> {
    decode_to_array(self).map(|(_, arr)| arr)
  }
}

#[inline]
fn decode_to_array<A>(src: &[A::Item]) -> Result<(usize, TinyVec<A>), DecodeError>
where
  A: Array,
  A::Item: Clone,
{
  if A::CAPACITY == 0 {
    return Ok((0, TinyVec::new()));
  }

  Ok((src.len(), TinyVec::from(src)))
}
