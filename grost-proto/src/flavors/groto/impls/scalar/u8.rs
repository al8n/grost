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

impl Encode<Groto, Fixed8> for u8 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.is_empty() {
      return Err(Error::insufficient_buffer(1, buf.len()));
    }

    buf[0] = *self;
    Ok(1)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    1
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Groto, Fixed8>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Groto, Fixed8>>::encode(self, context, buf)
  }
}

impl Encode<Groto, Varint> for u8 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_u8_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_u8_varint_len(*self)
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Groto, Varint>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Groto, Varint>>::encode(self, context, buf)
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

impl<'de, B, UB> Decode<'de, Groto, Fixed8, Self, B, UB> for u8 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
  {
    if src.is_empty() {
      return Err(Error::buffer_underflow());
    }

    let value = src.as_bytes()[0];
    Ok((1, value))
  }
}

impl<'de, B, UB> Decode<'de, Groto, Varint, Self, B, UB> for u8 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
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
