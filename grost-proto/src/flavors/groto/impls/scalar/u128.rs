use core::num::NonZeroU128;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed128, Groto, Varint},
  partial_encode_scalar, partial_identity, partial_ref_state, partial_state, ref_state, selectable,
  try_from_bridge,
};

default_scalar_wire_format!(Groto: u128 as Varint; NonZeroU128 as Varint);
selectable!(@scalar Groto: u128, NonZeroU128);
ref_state!(@scalar &'a Groto:
  u128 as Fixed128,
  NonZeroU128 as Fixed128,
  u128 as Varint,
  NonZeroU128 as Varint,
);
partial_ref_state!(@scalar &'a Groto:
  u128 as Fixed128,
  NonZeroU128 as Fixed128,
  u128 as Varint,
  NonZeroU128 as Varint,
);
partial_state!(@scalar Groto: u128, NonZeroU128);
flatten_state!(u128, NonZeroU128);
partial_identity!(@scalar Groto: u128, NonZeroU128);
partial_encode_scalar!(Groto: u128 as Fixed128, u128 as Varint);

impl Encode<Fixed128, Groto> for u128 {
  fn encode_raw(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 16 {
      return Err(Error::insufficient_buffer(16, buf.len()));
    }

    buf[..16].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(16)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    16
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Fixed128, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Fixed128, Groto>>::encoded_raw_len(self, context)
  }
}

impl Encode<Varint, Groto> for u128 {
  fn encode_raw(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_u128_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    varing::encoded_u128_varint_len(*self)
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Varint, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Varint, Groto>>::encoded_raw_len(self, context)
  }
}

impl<'de, RB, B> Decode<'de, Fixed128, RB, B, Groto> for u128 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    let src = src.as_bytes();
    if src.len() < 16 {
      return Err(Error::buffer_underflow());
    }

    Ok((16, u128::from_le_bytes(src[..16].try_into().unwrap())))
  }
}

impl<'de, RB, B> Decode<'de, Varint, RB, B, Groto> for u128 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    varing::decode_u128_varint(src.as_bytes()).map_err(Into::into)
  }
}

try_from_bridge!(
  Groto: u128 {
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
