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
}

impl Encode<Network, Varint> for u32 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_u32_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_u32_varint_len(*self)
  }
}

partial_encode_scalar!(Network: u32 as Fixed32, u32 as Varint);

impl<'de, B> Decode<'de, Network, Fixed32, Self, B> for u32 {
  fn decode(_: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    if src.len() < 4 {
      return Err(Error::buffer_underflow());
    }

    Ok((4, u32::from_le_bytes(src[..4].try_into().unwrap())))
  }
}

impl<'de, B> Decode<'de, Network, Varint, Self, B> for u32 {
  fn decode(_: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    varing::decode_u32_varint(src).map_err(Into::into)
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
