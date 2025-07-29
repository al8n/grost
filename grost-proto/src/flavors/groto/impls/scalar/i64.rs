use core::num::NonZeroI64;

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
partial_identity!(@scalar Groto: i64, NonZeroI64);

impl Encode<Fixed64, Groto> for i64 {
  fn encode_raw<B>(&self, _: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    buf
      .write_i64_le_checked(*self)
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

impl Encode<Varint, Groto> for i64 {
  fn encode_raw<B>(&self, _: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    buf.write_i64_varint(*self).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    varing::encoded_i64_varint_len(*self)
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

partial_encode_scalar!(Groto: i64 as Fixed64, i64 as Varint);

impl<'de, RB, B> Decode<'de, Fixed64, RB, B, Groto> for i64 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    let src = src.remaining_slice();
    if src.len() < 8 {
      return Err(Error::buffer_underflow());
    }

    Ok((8, i64::from_le_bytes(src[..8].try_into().unwrap())))
  }
}

impl<'de, RB, B> Decode<'de, Varint, RB, B, Groto> for i64 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    varing::decode_i64_varint(src.remaining_slice()).map_err(Into::into)
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
