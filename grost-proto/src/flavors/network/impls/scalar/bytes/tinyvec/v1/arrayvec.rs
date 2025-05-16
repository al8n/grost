use tinyvec_1::{Array, ArrayVec};

use crate::{
  Encoded, Flatten, IntoTarget, Message, PartialMessage, State, TypeRef,
  decode::{Decode, DecodeOwned},
  encode::{Encode, PartialEncode},
  flavors::{
    Network, Selectable,
    network::{Context, DecodeError, EncodeError, LengthDelimited, Unknown},
  },
  partial_encode_scalar,
};

use super::larger_than_array_capacity;

impl<A, S> State<Flatten<S>> for ArrayVec<A>
where
  A: Array,
  S: ?Sized,
{
  type Input = Self;
  type Output = Self;
}

impl<A> Encode<Network, LengthDelimited> for ArrayVec<A>
where
  A: Array<Item = u8>,
{
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let this_len = self.len();
    let buf_len = buf.len();
    if buf_len < this_len {
      return Err(EncodeError::insufficient_buffer(this_len, buf_len));
    }

    buf[..this_len].copy_from_slice(self.as_slice());
    Ok(this_len)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    self.len()
  }

  fn encoded_length_delimited_len(&self, _: &Context) -> usize {
    let this_len = self.len();
    let len_size = varing::encoded_u32_varint_len(this_len as u32);
    len_size + this_len
  }

  fn encode_length_delimited(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let this_len = self.len();
    let mut offset = varing::encode_u32_varint_to(this_len as u32, buf)?;
    let buf_len = buf.len();
    if buf_len < offset + this_len {
      return Err(EncodeError::insufficient_buffer(this_len + offset, buf_len));
    }

    buf[offset..offset + this_len].copy_from_slice(self.as_slice());
    offset += this_len;
    Ok(offset)
  }
}

impl<A> Selectable<Network, LengthDelimited> for ArrayVec<A>
where
  A: Array<Item = u8>,
{
  type Selector = bool;
}

impl<A> PartialEncode<Network, LengthDelimited> for ArrayVec<A>
where
  A: Array<Item = u8>,
{
  partial_encode_scalar!(@impl Network as LengthDelimited);
}

impl<'de, A> Decode<'de, Network, LengthDelimited, Self> for ArrayVec<A>
where
  A: Array<Item = u8>,
{
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    decode_to_array(src)
  }

  fn decode_length_delimited<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    decode_length_delimited_to_array(src)
  }
}

impl<A> DecodeOwned<Network, LengthDelimited, Self> for ArrayVec<A>
where
  A: Array<Item = u8> + 'static,
{
  fn decode_owned<B, UB>(context: &Context, src: B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    Self::decode::<()>(context, src.as_bytes())
  }

  fn decode_length_delimited_owned<B, UB>(
    context: &Context,

    src: B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    Self::decode_length_delimited::<()>(context, src.as_bytes())
  }
}

impl<A> PartialMessage<Network, LengthDelimited> for ArrayVec<A>
where
  A: Array<Item = u8> + Clone,
{
  type UnknownBuffer<B> = ();

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

impl<'a, A> State<Encoded<'a, Network, LengthDelimited>> for ArrayVec<A>
where
  A: Array<Item = u8>,
{
  type Input = &'a [u8];
  type Output = &'a [u8];
}

impl<A> Message<Network, LengthDelimited> for ArrayVec<A>
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

#[inline]
fn decode_length_delimited_to_array<A>(src: &[u8]) -> Result<(usize, ArrayVec<A>), DecodeError>
where
  A: Array<Item = u8>,
  A::Item: Clone,
{
  if A::CAPACITY == 0 {
    return Ok((0, ArrayVec::new()));
  }

  let (offset, data_len) = varing::decode_u32_varint(src)?;
  if data_len as usize > A::CAPACITY {
    return Err(larger_than_array_capacity::<A>());
  }
  let total = offset + data_len as usize;

  let mut arr = ArrayVec::new();
  arr.extend_from_slice(&src[offset..total]);

  Ok((total, arr))
}
