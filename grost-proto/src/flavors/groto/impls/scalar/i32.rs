use core::num::NonZeroI32;

use crate::{
  buffer::{ReadBuf, UnknownBuffer, WriteBuf},
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed32, Groto, Varint},
  partial_encode_scalar, partial_identity, partial_ref_state, partial_state, ref_state, selectable,
  try_from_bridge,
};

default_scalar_wire_format!(Groto: i32 as Varint; NonZeroI32 as Varint);
selectable!(@scalar Groto: i32, NonZeroI32);
ref_state!(@scalar &'a Groto:
  i32 as Fixed32,
  NonZeroI32 as Fixed32,
  i32 as Varint,
  NonZeroI32 as Varint,
);
partial_ref_state!(@scalar &'a Groto:
  i32 as Fixed32,
  NonZeroI32 as Fixed32,
  i32 as Varint,
  NonZeroI32 as Varint,
);
partial_state!(@scalar Groto: i32, NonZeroI32);
flatten_state!(i32, NonZeroI32);
partial_identity!(@scalar Groto: i32, NonZeroI32);

impl Encode<Fixed32, Groto> for i32 {
  fn encode_raw<B>(&self, _: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    buf
      .write_i32_le_checked(*self)
      .ok_or_else(|| Error::insufficient_buffer(4, buf.len()))
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    4
  }

  fn encode<B>(&self, ctx: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    <Self as Encode<Fixed32, Groto>>::encode_raw(self, ctx, buf)
  }

  fn encoded_len(&self, ctx: &Context) -> usize {
    <Self as Encode<Fixed32, Groto>>::encoded_raw_len(self, ctx)
  }
}

impl Encode<Varint, Groto> for i32 {
  fn encode_raw<B>(&self, _: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    buf.write_i32_varint(*self).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    varing::encoded_i32_varint_len(*self)
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

partial_encode_scalar!(Groto: i32 as Fixed32, i32 as Varint);

impl<'de, RB, B> Decode<'de, Fixed32, RB, B, Groto> for i32 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    let as_bytes = src.remaining_slice();
    if src.remaining() < 4 {
      return Err(Error::buffer_underflow());
    }

    Ok((4, i32::from_le_bytes(as_bytes[..4].try_into().unwrap())))
  }
}

impl<'de, RB, B> Decode<'de, Varint, RB, B, Groto> for i32 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    varing::decode_i32_varint(src.remaining_slice()).map_err(Into::into)
  }
}

try_from_bridge!(
  Groto: i32 {
    NonZeroI32 as Fixed32 {
      try_from: |v: i32| NonZeroI32::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroI32| v.get();
    },
    NonZeroI32 as Varint {
      try_from: |v: i32| NonZeroI32::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroI32| v.get();
    }
  },
);
