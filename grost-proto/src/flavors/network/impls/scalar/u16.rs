use core::num::NonZeroU16;

use crate::{
  buffer::{Buffer, ReadBuf},
  decode::Decode,
  decoded_state, default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::network::{Context, Error, Fixed16, Network, Unknown, Varint},
  identity_partial_transform, identity_transform, partial_encode_scalar, selectable,
  try_from_bridge,
};

default_wire_format!(Network: u16 as Varint);
selectable!(@scalar Network: u16, NonZeroU16);
decoded_state!(@scalar &'a Network: u16 as Fixed16, NonZeroU16 as Fixed16, u16 as Varint, NonZeroU16 as Varint);
flatten_state!(u16, NonZeroU16);
identity_transform!(Network {
  u16 as Fixed16,
  u16 as Varint,
  NonZeroU16 as Fixed16,
  NonZeroU16 as Varint,
});
identity_partial_transform!(Network {
  u16 as Fixed16,
  u16 as Varint,
  NonZeroU16 as Fixed16,
  NonZeroU16 as Varint,
});

impl Encode<Network, Fixed16> for u16 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 2 {
      return Err(Error::insufficient_buffer(2, buf.len()));
    }

    buf[..2].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(2)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    2
  }
}

impl Encode<Network, Varint> for u16 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_u16_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_u16_varint_len(*self)
  }
}

partial_encode_scalar!(Network: u16 as Fixed16, u16 as Varint);

impl<'de, B, UB> Decode<'de, Network, Fixed16, Self, B, UB> for u16 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
  {
    let src = src.as_bytes();
    if src.len() < 2 {
      return Err(Error::buffer_underflow());
    }

    Ok((2, u16::from_le_bytes(src[..2].try_into().unwrap())))
  }
}

impl<'de, B, UB> Decode<'de, Network, Varint, Self, B, UB> for u16 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
  {
    varing::decode_u16_varint(src.as_bytes()).map_err(Into::into)
  }
}

try_from_bridge!(
  Network: u16 {
    NonZeroU16 as Fixed16 {
      try_from: |v: u16| NonZeroU16::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU16| v.get();
    },
    NonZeroU16 as Varint {
      try_from: |v: u16| NonZeroU16::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU16| v.get();
    }
  },
);
