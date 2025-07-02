use core::num::NonZeroU8;

use crate::{
  buffer::{Buffer, ReadBuf},
  decode::Decode,
  default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed8, Groto, Unknown, Varint},
  groto_identity_transform, partial_encode_scalar, partial_ref_state, partial_state, selectable,
  try_from_bridge,
};

default_wire_format!(Groto: u8 as Fixed8; NonZeroU8 as Fixed8);
selectable!(@scalar Groto: u8, NonZeroU8);
partial_ref_state!(@scalar &'a Groto:
  u8 as Fixed8,
  NonZeroU8 as Fixed8,
  u8 as Varint,
  NonZeroU8 as Varint,
);
partial_state!(@scalar Groto: u8, NonZeroU8);
flatten_state!(u8, NonZeroU8);

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

partial_encode_scalar!(Groto: u8 as Fixed8, u8 as Varint);

groto_identity_transform!(
  u8 as Fixed8,
  u8 as Varint,
  NonZeroU8 as Fixed8,
  NonZeroU8 as Varint,
);
identity_partial_transform!(Groto {
  u8 as Fixed8,
  u8 as Varint,
  NonZeroU8 as Fixed8,
  NonZeroU8 as Varint,
});

impl<'de, RB, B> Decode<'de, Self, Fixed8, RB, B, Groto> for u8 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: Buffer<Unknown<RB>> + 'de,
  {
    if src.is_empty() {
      return Err(Error::buffer_underflow());
    }

    let value = src.as_bytes()[0];
    Ok((1, value))
  }
}

impl<'de, RB, B> Decode<'de, Self, Varint, RB, B, Groto> for u8 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: Buffer<Unknown<RB>> + 'de,
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
