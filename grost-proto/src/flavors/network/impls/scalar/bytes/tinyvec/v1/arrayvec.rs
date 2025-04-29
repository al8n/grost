use tinyvec_1::{Array, ArrayVec};

use crate::{
  IntoTarget, Message, PartialMessage, TypeOwned, TypeRef,
  decode::{Decode, DecodeOwned},
  encode::Encode,
  flavors::{
    Network,
    network::{Context, DecodeError, EncodeError, Unknown, WireType},
  },
};

use super::larger_than_array_capacity;

impl<A> Encode<Network> for ArrayVec<A>
where
  A: Array<Item = u8>,
{
  fn encode(&self, _: &Context, wire_type: WireType, buf: &mut [u8]) -> Result<usize, EncodeError> {
    Ok(match wire_type {
      WireType::Zst if A::CAPACITY == 0 => 0,
      WireType::LengthDelimited => {
        let this_len = self.len();
        let buf_len = buf.len();
        if buf_len < this_len {
          return Err(EncodeError::insufficient_buffer(this_len, buf_len));
        }

        buf[..this_len].copy_from_slice(self.as_slice());
        this_len
      }
      val => {
        return Err(EncodeError::unsupported_wire_type(
          core::any::type_name::<Self>(),
          val,
        ));
      }
    })
  }

  fn encoded_len(&self, _: &Context, wire_type: WireType) -> Result<usize, EncodeError> {
    Ok(match wire_type {
      WireType::Zst if A::CAPACITY == 0 => 0,
      WireType::LengthDelimited => self.len(),
      val => {
        return Err(EncodeError::unsupported_wire_type(
          core::any::type_name::<Self>(),
          val,
        ));
      }
    })
  }

  fn encoded_length_delimited_len(
    &self,
    _: &Context,
    wire_type: WireType,
  ) -> Result<usize, EncodeError> {
    Ok(match wire_type {
      WireType::Zst if A::CAPACITY == 0 => 0,
      WireType::LengthDelimited => {
        let this_len = self.len();
        let len_size = varing::encoded_u32_varint_len(this_len as u32);
        len_size + this_len
      }
      val => {
        return Err(EncodeError::unsupported_wire_type(
          core::any::type_name::<Self>(),
          val,
        ));
      }
    })
  }

  fn encode_length_delimited(
    &self,
    _: &Context,
    wire_type: WireType,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    Ok(match wire_type {
      WireType::Zst if A::CAPACITY == 0 => 0,
      WireType::LengthDelimited => {
        let this_len = self.len();
        let mut offset = varing::encode_u32_varint_to(this_len as u32, buf)?;
        let buf_len = buf.len();
        if buf_len < offset + this_len {
          return Err(EncodeError::insufficient_buffer(this_len + offset, buf_len));
        }

        buf[offset..offset + this_len].copy_from_slice(self.as_slice());
        offset += this_len;
        offset
      }
      _ => {
        return Err(EncodeError::unsupported_wire_type(
          core::any::type_name::<Self>(),
          wire_type,
        ));
      }
    })
  }
}

impl<'de, A> Decode<'de, Network, Self> for ArrayVec<A>
where
  A: Array<Item = u8>,
{
  fn decode<UB>(
    _: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    match wire_type {
      WireType::Zst if A::CAPACITY == 0 => Ok((0, ArrayVec::new())),
      WireType::LengthDelimited => decode_to_array(src),
      val => Err(DecodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        val,
      )),
    }
  }

  fn decode_length_delimited<UB>(
    _: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    match wire_type {
      WireType::Zst if A::CAPACITY == 0 => Ok((0, ArrayVec::new())),
      WireType::LengthDelimited => decode_length_delimited_to_array(src),
      val => Err(DecodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        val,
      )),
    }
  }
}

impl<A> DecodeOwned<Network, Self> for ArrayVec<A>
where
  A: Array<Item = u8> + 'static,
{
  fn decode_owned<B, UB>(
    context: &Context,
    wire_type: WireType,
    src: B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    Self::decode::<()>(context, wire_type, src.as_bytes())
  }

  fn decode_length_delimited_owned<B, UB>(
    context: &Context,
    wire_type: WireType,
    src: B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    Self::decode_length_delimited::<()>(context, wire_type, src.as_bytes())
  }
}

impl<A> PartialMessage<Network> for ArrayVec<A>
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
