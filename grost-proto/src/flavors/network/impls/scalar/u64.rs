use core::num::NonZeroU64;

use crate::{
  buffer::Buffer,
  decode::Decode,
  decode_owned_scalar, decoded_state, default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::network::{Context, DecodeError, EncodeError, Fixed64, Network, Unknown, Varint},
  message, partial_encode_scalar, selectable, try_from_bridge,
};

default_wire_format!(Network: u64 as Varint);
selectable!(@scalar Network: u64, NonZeroU64);
decoded_state!(@scalar &'a Network: u64 as Fixed64, NonZeroU64 as Fixed64, u64 as Varint, NonZeroU64 as Varint);
flatten_state!(u64, NonZeroU64);

impl Encode<Network, Fixed64> for u64 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    if buf.len() < 8 {
      return Err(EncodeError::insufficient_buffer(8, buf.len()));
    }

    buf[..8].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(8)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    8
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Network, Fixed64>>::encoded_len(self, context)
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Network, Fixed64>>::encode(self, context, buf)
  }
}

impl Encode<Network, Varint> for u64 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    varing::encode_u64_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_u64_varint_len(*self)
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

partial_encode_scalar!(Network: u64 as Fixed64, u64 as Varint);

impl<'de> Decode<'de, Network, Fixed64, Self> for u64 {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    if src.len() < 8 {
      return Err(DecodeError::buffer_underflow());
    }

    Ok((8, u64::from_le_bytes(src[..8].try_into().unwrap())))
  }

  fn decode_length_delimited<UB>(
    ctx: &Context,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <Self as Decode<'_, Network, Fixed64, Self>>::decode::<UB>(ctx, src)
  }
}

impl<'de> Decode<'de, Network, Varint, Self> for u64 {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    varing::decode_u64_varint(src).map_err(Into::into)
  }

  fn decode_length_delimited<UB>(
    ctx: &Context,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <Self as Decode<'_, Network, Fixed64, Self>>::decode::<UB>(ctx, src)
  }
}

decode_owned_scalar!(Network: u64 as Fixed64, u64 as Varint);
message!(Network: u64 as Fixed64, u64 as Varint);

try_from_bridge!(
  Network: u64 {
    NonZeroU64 as Fixed64 {
      try_from: |v: u64| NonZeroU64::new(v).ok_or_else(|| DecodeError::custom("value cannot be zero"));
      to: |v: &NonZeroU64| v.get();
    },
    NonZeroU64 as Varint {
      try_from: |v: u64| NonZeroU64::new(v).ok_or_else(|| DecodeError::custom("value cannot be zero"));
      to: |v: &NonZeroU64| v.get();
    }
  },
);
