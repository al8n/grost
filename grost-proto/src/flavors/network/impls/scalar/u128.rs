use core::num::NonZeroU128;

use crate::{
  buffer::Buffer, decode::Decode, decode_owned_scalar, default_wire_format, encode::Encode, flavors::network::{Context, DecodeError, EncodeError, Fixed128, Network, Unknown, Varint}, message, partial_encode_scalar, selectable_bridge, selectable_scalar, try_from_bridge
};

default_wire_format!(Network: u128 as Varint);
selectable_scalar!(Network: u128);
impl Encode<Network, Fixed128> for u128 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    if buf.len() < 16 {
      return Err(EncodeError::insufficient_buffer(16, buf.len()));
    }

    buf[..16].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(16)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    16
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Network, Fixed128>>::encoded_len(self, context)
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Network, Fixed128>>::encode(self, context, buf)
  }
}

impl Encode<Network, Varint> for u128 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    varing::encode_u128_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_u128_varint_len(*self)
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

partial_encode_scalar!(Network: u128 as Fixed128, u128 as Varint);

impl<'de> Decode<'de, Network, Fixed128, Self> for u128 {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    if src.len() < 16 {
      return Err(DecodeError::buffer_underflow());
    }

    Ok((16, u128::from_le_bytes(src[..16].try_into().unwrap())))
  }

  fn decode_length_delimited<UB>(
    ctx: &Context,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <Self as Decode<'_, Network, Fixed128, Self>>::decode::<UB>(ctx, src)
  }
}

impl<'de> Decode<'de, Network, Varint, Self> for u128 {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    varing::decode_u128_varint(src).map_err(Into::into)
  }

  fn decode_length_delimited<UB>(
    ctx: &Context,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <Self as Decode<'_, Network, Fixed128, Self>>::decode::<UB>(ctx, src)
  }
}

decode_owned_scalar!(Network: u128 as Fixed128, u128 as Varint);
message!(Network: u128 as Fixed128, u128 as Varint);

selectable_bridge!(Network: u128 [NonZeroU128]);
try_from_bridge!(
  Network: u128 {
    NonZeroU128 as Fixed128 {
      try_from: |v: u128| NonZeroU128::new(v).ok_or_else(|| DecodeError::custom("value cannot be zero"));
      to: |v: &NonZeroU128| v.get();
    },
    NonZeroU128 as Varint {
      try_from: |v: u128| NonZeroU128::new(v).ok_or_else(|| DecodeError::custom("value cannot be zero"));
      to: |v: &NonZeroU128| v.get();
    }
  },
);
