use crate::{
  buffer::{ReadBuf, UnknownBuffer, WriteBuf},
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed16, Groto, Varint},
  partial_encode_scalar, partial_identity, partial_ref_state, partial_state, ref_state, selectable,
  try_from_bridge,
};
use core::num::NonZeroI16;

default_scalar_wire_format!(Groto: i16 as Varint; NonZeroI16 as Varint);
selectable!(@scalar Groto: i16, NonZeroI16);
ref_state!(@scalar &'a Groto:
  i16 as Fixed16,
  NonZeroI16 as Fixed16,
  i16 as Varint,
  NonZeroI16 as Varint,
);
partial_ref_state!(@scalar &'a Groto:
  i16 as Fixed16,
  NonZeroI16 as Fixed16,
  i16 as Varint,
  NonZeroI16 as Varint,
);
partial_state!(@scalar Groto: i16, NonZeroI16);
flatten_state!(i16, NonZeroI16);
partial_identity!(@scalar Groto: i16, NonZeroI16);

impl Encode<Fixed16, Groto> for i16 {
  fn encode_raw<B>(&self, _: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    buf
      .write_i16_le_checked(*self)
      .ok_or_else(|| Error::insufficient_buffer(2, buf.len()))
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    2
  }

  fn encode<B>(&self, ctx: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    <Self as Encode<Fixed16, Groto>>::encode_raw(self, ctx, buf)
  }

  fn encoded_len(&self, ctx: &Context) -> usize {
    <Self as Encode<Fixed16, Groto>>::encoded_raw_len(self, ctx)
  }
}

impl Encode<Varint, Groto> for i16 {
  fn encode_raw<B>(&self, _: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    buf.write_i16_varint(*self).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    varing::encoded_i16_varint_len(*self)
  }

  fn encode<B>(&self, ctx: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    <Self as Encode<Varint, Groto>>::encode_raw(self, ctx, buf)
  }

  fn encoded_len(&self, ctx: &Context) -> usize {
    <Self as Encode<Varint, Groto>>::encoded_raw_len(self, ctx)
  }
}

partial_encode_scalar!(Groto: i16 as Fixed16, i16 as Varint);

impl<'de, RB, B> Decode<'de, Fixed16, RB, B, Groto> for i16 {
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

    Ok((2, i16::from_le_bytes(src[..2].try_into().unwrap())))
  }
}

impl<'de, RB, B> Decode<'de, Varint, RB, B, Groto> for i16 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    varing::decode_i16_varint(src.as_bytes()).map_err(Into::into)
  }
}

try_from_bridge!(
  Groto: i16 {
    NonZeroI16 as Fixed16 {
      try_from: |v: i16| NonZeroI16::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroI16| v.get();
    },
    NonZeroI16 as Varint {
      try_from: |v: i16| NonZeroI16::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroI16| v.get();
    }
  },
);
