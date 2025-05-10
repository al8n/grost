use core::num::NonZeroU16;

use crate::{
  buffer::Buffer,
  decode::Decode,
  decode_owned_scalar, default_wire_format,
  encode::Encode,
  encoded_state,
  flavors::network::{Context, DecodeError, EncodeError, Fixed16, Network, Unknown, Varint},
  message, partial_encode_scalar, referenceable_scalar, selectable, try_from_bridge,
};

default_wire_format!(Network: u16 as Varint);
selectable!(@scalar Network: u16, NonZeroU16);
referenceable_scalar!(Network: u16 as Fixed16, NonZeroU16 as Fixed16, u16 as Varint, NonZeroU16 as Varint);
encoded_state!(@scalar &'a Network: u16 as Fixed16, NonZeroU16 as Fixed16, u16 as Varint, NonZeroU16 as Varint);

impl Encode<Network, Fixed16> for u16 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    if buf.len() < 2 {
      return Err(EncodeError::insufficient_buffer(2, buf.len()));
    }

    buf[..2].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(2)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    2
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Network, Fixed16>>::encoded_len(self, context)
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Network, Fixed16>>::encode(self, context, buf)
  }
}

impl Encode<Network, Varint> for u16 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    varing::encode_u16_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_u16_varint_len(*self)
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

partial_encode_scalar!(Network: u16 as Fixed16, u16 as Varint);

impl<'de> Decode<'de, Network, Fixed16, Self> for u16 {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    if src.len() < 2 {
      return Err(DecodeError::buffer_underflow());
    }

    Ok((2, u16::from_le_bytes(src[..2].try_into().unwrap())))
  }

  fn decode_length_delimited<UB>(
    ctx: &Context,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <Self as Decode<'_, Network, Fixed16, Self>>::decode::<UB>(ctx, src)
  }
}

impl<'de> Decode<'de, Network, Varint, Self> for u16 {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    varing::decode_u16_varint(src).map_err(Into::into)
  }

  fn decode_length_delimited<UB>(
    ctx: &Context,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <Self as Decode<'_, Network, Fixed16, Self>>::decode::<UB>(ctx, src)
  }
}

decode_owned_scalar!(Network: u16 as Fixed16, u16 as Varint);
message!(Network: u16 as Fixed16, u16 as Varint);

try_from_bridge!(
  Network: u16 {
    NonZeroU16 as Fixed16 {
      try_from: |v: u16| NonZeroU16::new(v).ok_or_else(|| DecodeError::custom("value cannot be zero"));
      to: |v: &NonZeroU16| v.get();
    },
    NonZeroU16 as Varint {
      try_from: |v: u16| NonZeroU16::new(v).ok_or_else(|| DecodeError::custom("value cannot be zero"));
      to: |v: &NonZeroU16| v.get();
    }
  },
);
