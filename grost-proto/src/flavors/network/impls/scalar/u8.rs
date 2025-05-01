use core::num::NonZeroU8;

use crate::{
  buffer::Buffer,
  decode::Decode,
  decode_owned_scalar, default_wire_format,
  encode::Encode,
  flavors::network::{Context, DecodeError, EncodeError, Fixed8, Network, Unknown, Varint},
  message, partial_encode_scalar, try_from_bridge,
};

default_wire_format!(Network: u8 as Fixed8);

impl Encode<Network, Fixed8> for u8 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    if buf.is_empty() {
      return Err(EncodeError::insufficient_buffer(1, buf.len()));
    }

    buf[0] = *self;
    Ok(1)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    1
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Network, Fixed8>>::encoded_len(self, context)
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Network, Fixed8>>::encode(self, context, buf)
  }
}

impl Encode<Network, Varint> for u8 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    varing::encode_u8_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_u8_varint_len(*self)
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

partial_encode_scalar!(Network: u8 as Fixed8, u8 as Varint);

impl<'de> Decode<'de, Network, Fixed8, Self> for u8 {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    if src.is_empty() {
      return Err(DecodeError::buffer_underflow());
    }

    let value = src[0];
    Ok((1, value))
  }

  fn decode_length_delimited<UB>(
    ctx: &Context,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <Self as Decode<'_, Network, Fixed8, Self>>::decode::<UB>(ctx, src)
  }
}

impl<'de> Decode<'de, Network, Varint, Self> for u8 {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    varing::decode_u8_varint(src).map_err(Into::into)
  }

  fn decode_length_delimited<UB>(
    ctx: &Context,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <Self as Decode<'_, Network, Fixed8, Self>>::decode::<UB>(ctx, src)
  }
}

decode_owned_scalar!(Network: u8 as Fixed8, u8 as Varint);
message!(Network: u8 as Fixed8, u8 as Varint);

try_from_bridge!(
  Network: u8 {
    NonZeroU8 as Fixed8 {
      try_from: |v: u8| NonZeroU8::new(v).ok_or_else(|| DecodeError::custom("value cannot be zero"));
      to: |v: &NonZeroU8| v.get();
    },
    NonZeroU8 as Varint {
      try_from: |v: u8| NonZeroU8::new(v).ok_or_else(|| DecodeError::custom("value cannot be zero"));
      to: |v: &NonZeroU8| v.get();
    },
  },
);
