use core::num::NonZeroU128;

use crate::{
  buffer::{Buffer, ReadBuf},
  decode::Decode,
  decoded_state, default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::network::{Context, Error, Fixed128, Network, Unknown, Varint},
  partial_encode_scalar, selectable, try_from_bridge,
};

default_wire_format!(Network: u128 as Varint);
selectable!(@scalar Network: u128, NonZeroU128);
decoded_state!(@scalar &'a Network: u128 as Fixed128, NonZeroU128 as Fixed128, u128 as Varint, NonZeroU128 as Varint);
flatten_state!(u128, NonZeroU128);

impl Encode<Network, Fixed128> for u128 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 16 {
      return Err(Error::insufficient_buffer(16, buf.len()));
    }

    buf[..16].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(16)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    16
  }
}

impl Encode<Network, Varint> for u128 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_u128_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_u128_varint_len(*self)
  }
}

partial_encode_scalar!(Network: u128 as Fixed128, u128 as Varint);

impl<'de, UB> Decode<'de, Network, Fixed128, Self, UB> for u128 {
  fn decode<B>(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    let src = src.as_bytes();
    if src.len() < 16 {
      return Err(Error::buffer_underflow());
    }

    Ok((16, u128::from_le_bytes(src[..16].try_into().unwrap())))
  }
}

impl<'de, UB> Decode<'de, Network, Varint, Self, UB> for u128 {
  fn decode<B>(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    varing::decode_u128_varint(src.as_bytes()).map_err(Into::into)
  }
}

try_from_bridge!(
  Network: u128 {
    NonZeroU128 as Fixed128 {
      try_from: |v: u128| NonZeroU128::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU128| v.get();
    },
    NonZeroU128 as Varint {
      try_from: |v: u128| NonZeroU128::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU128| v.get();
    }
  },
);
