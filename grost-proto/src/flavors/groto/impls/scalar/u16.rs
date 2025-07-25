use core::num::NonZeroU16;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed16, Groto, Varint},
  partial_encode_scalar, partial_identity, partial_ref_state, partial_state, ref_state, selectable,
  try_from_bridge,
};

default_scalar_wire_format!(Groto: u16 as Varint; NonZeroU16 as Varint);
selectable!(@scalar Groto: u16, NonZeroU16);
ref_state!(@scalar &'a Groto:
  u16 as Fixed16,
  NonZeroU16 as Fixed16,
  u16 as Varint,
  NonZeroU16 as Varint,
);
partial_ref_state!(@scalar &'a Groto:
  u16 as Fixed16,
  NonZeroU16 as Fixed16,
  u16 as Varint,
  NonZeroU16 as Varint,
);
partial_state!(@scalar Groto: u16, NonZeroU16);
flatten_state!(u16, NonZeroU16);
partial_identity!(@scalar Groto: u16, NonZeroU16);
partial_encode_scalar!(Groto: u16 as Fixed16, u16 as Varint);

impl Encode<Fixed16, Groto> for u16 {
  fn encode_raw(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 2 {
      return Err(Error::insufficient_buffer(2, buf.len()));
    }

    buf[..2].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(2)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    2
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Fixed16, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Fixed16, Groto>>::encoded_raw_len(self, context)
  }
}

impl Encode<Varint, Groto> for u16 {
  fn encode_raw(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_u16_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    varing::encoded_u16_varint_len(*self)
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Varint, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Varint, Groto>>::encoded_raw_len(self, context)
  }
}

impl<'de, RB, B> Decode<'de, Fixed16, RB, B, Groto> for u16 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    let src = src.as_bytes();
    if src.len() < 2 {
      return Err(Error::buffer_underflow());
    }

    Ok((2, u16::from_le_bytes(src[..2].try_into().unwrap())))
  }
}

impl<'de, RB, B> Decode<'de, Varint, RB, B, Groto> for u16 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    varing::decode_u16_varint(src.as_bytes()).map_err(Into::into)
  }
}

try_from_bridge!(
  Groto: u16 {
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
