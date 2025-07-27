use core::num::NonZeroU8;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed8, Groto, Varint},
  partial_encode_scalar, partial_identity, partial_ref_state, partial_state, ref_state, selectable,
  try_from_bridge,
};

default_scalar_wire_format!(Groto: u8 as Fixed8; NonZeroU8 as Fixed8);
selectable!(@scalar Groto: u8, NonZeroU8);
ref_state!(@scalar &'a Groto:
  u8 as Fixed8,
  NonZeroU8 as Fixed8,
  u8 as Varint,
  NonZeroU8 as Varint,
);
partial_ref_state!(@scalar &'a Groto:
  u8 as Fixed8,
  NonZeroU8 as Fixed8,
  u8 as Varint,
  NonZeroU8 as Varint,
);
partial_state!(@scalar Groto: u8, NonZeroU8);
flatten_state!(u8, NonZeroU8);
partial_identity!(@scalar Groto: u8, NonZeroU8);
partial_encode_scalar!(Groto: u8 as Fixed8, u8 as Varint);

impl Encode<Fixed8, Groto> for u8 {
  fn encode_raw(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.is_empty() {
      return Err(Error::insufficient_buffer(1, buf.len()));
    }

    buf[0] = *self;
    Ok(1)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    1
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Fixed8, Groto>>::encoded_raw_len(self, context)
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Fixed8, Groto>>::encode_raw(self, context, buf)
  }
}

impl Encode<Varint, Groto> for u8 {
  fn encode_raw(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_u8_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    varing::encoded_u8_varint_len(*self)
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Varint, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Varint, Groto>>::encoded_raw_len(self, context)
  }
}

impl<'de, RB, B> Decode<'de, Fixed8, RB, B, Groto> for u8 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    if src.is_empty() {
      return Err(Error::buffer_underflow());
    }

    let value = src.as_bytes()[0];
    Ok((1, value))
  }
}

impl<'de, RB, B> Decode<'de, Varint, RB, B, Groto> for u8 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    varing::decode_u8_varint(src.as_bytes()).map_err(Into::into)
  }
}

// decode_owned_scalar!(Groto: u8 as Fixed8, u8 as Varint);
try_from_bridge!(
  Groto: u8 {
    NonZeroU8 as Fixed8 {
      try_from: |v: u8| NonZeroU8::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU8| v.get();
    },
    NonZeroU8 as Varint {
      try_from: |v: u8| NonZeroU8::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU8| v.get();
    },
  },
);
