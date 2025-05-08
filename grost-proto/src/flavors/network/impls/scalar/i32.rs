use core::num::NonZeroI32;

use crate::{
  buffer::Buffer,
  decode::Decode,
  decode_owned_scalar, default_wire_format,
  encode::Encode,
  flavors::network::{Context, DecodeError, EncodeError, Fixed32, Network, Unknown, Varint},
  message, partial_encode_scalar, referenceable_scalar, selectable_bridge, selectable_scalar,
  try_from_bridge,
};

default_wire_format!(Network: i32 as Varint);
selectable_scalar!(Network: i32);
referenceable_scalar!(Network: i32 as Fixed32, NonZeroI32 as Fixed32, i32 as Varint, NonZeroI32 as Varint);

impl Encode<Network, Fixed32> for i32 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    if buf.len() < 4 {
      return Err(EncodeError::insufficient_buffer(4, buf.len()));
    }

    buf[..4].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(4)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    4
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Network, Fixed32>>::encoded_len(self, context)
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Network, Fixed32>>::encode(self, context, buf)
  }
}

impl Encode<Network, Varint> for i32 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    varing::encode_i32_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_i32_varint_len(*self)
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Network, Varint>>::encoded_len(self, context)
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Network, Varint>>::encode(self, context, buf)
  }
}

partial_encode_scalar!(Network: i32 as Fixed32, i32 as Varint);

impl<'de> Decode<'de, Network, Fixed32, Self> for i32 {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    if src.len() < 4 {
      return Err(DecodeError::buffer_underflow());
    }

    Ok((4, i32::from_le_bytes(src[..4].try_into().unwrap())))
  }

  fn decode_length_delimited<UB>(
    ctx: &Context,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <Self as Decode<'_, Network, Fixed32, Self>>::decode::<UB>(ctx, src)
  }
}

impl<'de> Decode<'de, Network, Varint, Self> for i32 {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    varing::decode_i32_varint(src).map_err(Into::into)
  }

  fn decode_length_delimited<UB>(
    ctx: &Context,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <Self as Decode<'_, Network, Fixed32, Self>>::decode::<UB>(ctx, src)
  }
}

decode_owned_scalar!(Network: i32 as Fixed32, i32 as Varint);
message!(Network: i32 as Fixed32, i32 as Varint);

selectable_bridge!(Network:i32[NonZeroI32]);
try_from_bridge!(
  Network: i32 {
    NonZeroI32 as Fixed32 {
      try_from: |v: i32| NonZeroI32::new(v).ok_or_else(|| DecodeError::custom("value cannot be zero"));
      to: |v: &NonZeroI32| v.get();
    },
    NonZeroI32 as Varint {
      try_from: |v: i32| NonZeroI32::new(v).ok_or_else(|| DecodeError::custom("value cannot be zero"));
      to: |v: &NonZeroI32| v.get();
    }
  },
);
