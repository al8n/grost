use tinyvec_1::{Array, ArrayVec};

use crate::{
  Decoded, Flatten, State,
  decode::{Decode, DecodeOwned},
  encode::{Encode, PartialEncode},
  flavors::{
    Network, Selectable,
    network::{Context, DecodeError, EncodeError, LengthDelimited, Unknown},
  },
  partial_encode_scalar,
};

use super::larger_than_array_capacity;

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

impl<'a, A> State<Decoded<'a, Network, LengthDelimited>> for ArrayVec<A>
where
  A: Array<Item = u8>,
{
  type Input = &'a [u8];
  type Output = &'a [u8];
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
