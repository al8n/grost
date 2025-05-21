use core::num::NonZeroU32;

use crate::{
  buffer::Buffer,
  decode::Decode,
  decode_owned_scalar, decoded_state, default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::network::{Context, Error, Fixed32, Network, Unknown, Varint},
  partial_encode_scalar, selectable, try_from_bridge,
};

default_wire_format!(Network: u32 as Varint);
selectable!(@scalar Network: u32, NonZeroU32);
decoded_state!(@scalar &'a Network: u32 as Fixed32, NonZeroU32 as Fixed32, u32 as Varint, NonZeroU32 as Varint);
flatten_state!(u32, NonZeroU32);

impl Encode<Network, Fixed32> for u32 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 4 {
      return Err(Error::insufficient_buffer(4, buf.len()));
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

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Network, Fixed32>>::encode(self, context, buf)
  }
}

impl Encode<Network, Varint> for u32 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_u32_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_u32_varint_len(*self)
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Network, Varint>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Network, Varint>>::encode(self, context, buf)
  }
}

partial_encode_scalar!(Network: u32 as Fixed32, u32 as Varint);

impl<'de> Decode<'de, Network, Fixed32, Self> for u32 {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    if src.len() < 4 {
      return Err(Error::buffer_underflow());
    }

    Ok((4, u32::from_le_bytes(src[..4].try_into().unwrap())))
  }

  fn decode_length_delimited<UB>(ctx: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <Self as Decode<'_, Network, Fixed32, Self>>::decode::<UB>(ctx, src)
  }
}

impl<'de> Decode<'de, Network, Varint, Self> for u32 {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    varing::decode_u32_varint(src).map_err(Into::into)
  }

  fn decode_length_delimited<UB>(ctx: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <Self as Decode<'_, Network, Fixed32, Self>>::decode::<UB>(ctx, src)
  }
}

decode_owned_scalar!(Network: u32 as Fixed32, u32 as Varint);

try_from_bridge!(
  Network: u32 {
    NonZeroU32 as Fixed32 {
      try_from: |v: u32| NonZeroU32::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU32| v.get();
    },
    NonZeroU32 as Varint {
      try_from: |v: u32| NonZeroU32::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU32| v.get();
    }
  },
);
