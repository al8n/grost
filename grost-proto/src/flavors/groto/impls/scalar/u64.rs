use core::num::NonZeroU64;

use crate::{
  buffer::{ReadBuf, UnknownBuffer, WriteBuf},
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed64, Groto, Varint},
  partial_encode_scalar, partial_identity, partial_ref_state, partial_state, ref_state, selectable,
  try_from_bridge,
};

default_scalar_wire_format!(Groto: u64 as Varint; NonZeroU64 as Varint);
selectable!(@scalar Groto: u64, NonZeroU64);
ref_state!(@scalar &'a Groto:
  u64 as Fixed64,
  NonZeroU64 as Fixed64,
  u64 as Varint,
  NonZeroU64 as Varint,
);
partial_ref_state!(@scalar &'a Groto:
  u64 as Fixed64,
  NonZeroU64 as Fixed64,
  u64 as Varint,
  NonZeroU64 as Varint,
);
partial_state!(@scalar Groto: u64, NonZeroU64);
flatten_state!(u64, NonZeroU64);
partial_identity!(@scalar Groto: u64, NonZeroU64);
partial_encode_scalar!(Groto: u64 as Fixed64, u64 as Varint);

impl Encode<Fixed64, Groto> for u64 {
  fn encode_raw<B>(&self, _: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    buf
      .write_u64_le_checked(*self)
      .ok_or_else(|| Error::insufficient_buffer(8, buf.len()))
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    8
  }

  fn encode<B>(&self, context: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    <Self as Encode<Fixed64, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Fixed64, Groto>>::encoded_raw_len(self, context)
  }
}

impl Encode<Varint, Groto> for u64 {
  fn encode_raw<B>(&self, _: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    buf.write_u64_varint(*self).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    varing::encoded_u64_varint_len(*self)
  }

  fn encode<B>(&self, context: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    <Self as Encode<Varint, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Varint, Groto>>::encoded_raw_len(self, context)
  }
}

impl<'de, RB, B> Decode<'de, Fixed64, RB, B, Groto> for u64 {
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

    Ok((8, u64::from_le_bytes(src[..8].try_into().unwrap())))
  }
}

impl<'de, RB, B> Decode<'de, Varint, RB, B, Groto> for u64 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    varing::decode_u64_varint(src.as_bytes()).map_err(Into::into)
  }
}

try_from_bridge!(
  Groto: u64 {
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
