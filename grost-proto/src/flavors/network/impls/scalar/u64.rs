use core::num::NonZeroU64;

use crate::{
  buffer::{Buf, Buffer},
  decode::Decode,
  decoded_state, default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::network::{Context, Error, Fixed64, Network, Unknown, Varint},
  partial_encode_scalar, selectable, try_from_bridge,
};

default_wire_format!(Network: u64 as Varint);
selectable!(@scalar Network: u64, NonZeroU64);
decoded_state!(@scalar &'a Network: u64 as Fixed64, NonZeroU64 as Fixed64, u64 as Varint, NonZeroU64 as Varint);
flatten_state!(u64, NonZeroU64);

impl Encode<Network, Fixed64> for u64 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 8 {
      return Err(Error::insufficient_buffer(8, buf.len()));
    }

    buf[..8].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(8)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    8
  }
}

impl Encode<Network, Varint> for u64 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_u64_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_u64_varint_len(*self)
  }
}

partial_encode_scalar!(Network: u64 as Fixed64, u64 as Varint);

impl<'de, UB> Decode<'de, Network, Fixed64, Self, UB> for u64 {
  fn decode<B>(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: Buf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    let src = src.chunk();
    if src.len() < 8 {
      return Err(Error::buffer_underflow());
    }

    Ok((8, u64::from_le_bytes(src[..8].try_into().unwrap())))
  }
}

impl<'de, UB> Decode<'de, Network, Varint, Self, UB> for u64 {
  fn decode<B>(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: Buf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    varing::decode_u64_varint(src.chunk()).map_err(Into::into)
  }
}

try_from_bridge!(
  Network: u64 {
    NonZeroU64 as Fixed64 {
      try_from: |v: u64| NonZeroU64::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU64| v.get();
    },
    NonZeroU64 as Varint {
      try_from: |v: u64| NonZeroU64::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU64| v.get();
    }
  },
);
