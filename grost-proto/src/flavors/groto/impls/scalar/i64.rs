use core::num::NonZeroI64;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed64, Groto, Varint},
  groto_identity_transform, partial_encode_scalar, partial_ref_state, partial_state, ref_state,
  selectable, try_from_bridge,
};

default_scalar_wire_format!(Groto: i64 as Varint; NonZeroI64 as Varint);
selectable!(@scalar Groto: i64, NonZeroI64);
ref_state!(@scalar &'a Groto:
  i64 as Fixed64,
  NonZeroI64 as Fixed64,
  i64 as Varint,
  NonZeroI64 as Varint,
);
partial_ref_state!(@scalar &'a Groto:
  i64 as Fixed64,
  NonZeroI64 as Fixed64,
  i64 as Varint,
  NonZeroI64 as Varint,
);
partial_state!(@scalar Groto: i64, NonZeroI64);
flatten_state!(i64, NonZeroI64);
groto_identity_transform!(
  i64 as Fixed64,
  i64 as Varint,
  NonZeroI64 as Fixed64,
  NonZeroI64 as Varint,
);
identity_partial_transform!(
  Groto {
    i64 as Fixed64,
    i64 as Varint,
    NonZeroI64 as Fixed64,
    NonZeroI64 as Varint,
  }
);

impl Encode<Fixed64, Groto> for i64 {
  fn encode_raw(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 8 {
      return Err(Error::insufficient_buffer(8, buf.len()));
    }

    buf[..8].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(8)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    8
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Fixed64, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Fixed64, Groto>>::encoded_raw_len(self, context)
  }
}

impl Encode<Varint, Groto> for i64 {
  fn encode_raw(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_i64_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    varing::encoded_i64_varint_len(*self)
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Varint, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Varint, Groto>>::encoded_raw_len(self, context)
  }
}

partial_encode_scalar!(Groto: i64 as Fixed64, i64 as Varint);

impl<'de, RB, B> Decode<'de, Self, Fixed64, RB, B, Groto> for i64 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    let src = src.as_bytes();
    if src.len() < 8 {
      return Err(Error::buffer_underflow());
    }

    Ok((8, i64::from_le_bytes(src[..8].try_into().unwrap())))
  }
}

impl<'de, RB, B> Decode<'de, Self, Varint, RB, B, Groto> for i64 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    varing::decode_i64_varint(src.as_bytes()).map_err(Into::into)
  }
}

try_from_bridge!(
  Groto: i64 {
    NonZeroI64 as Fixed64 {
      try_from: |v: i64| NonZeroI64::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroI64| v.get();
    },
    NonZeroI64 as Varint {
      try_from: |v: i64| NonZeroI64::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroI64| v.get();
    }
  },
);
