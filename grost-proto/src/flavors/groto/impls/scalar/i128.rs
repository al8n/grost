use core::num::NonZeroI128;

use crate::{
  buffer::{ReadBuf, UnknownBuffer, WriteBuf},
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed128, Groto, Varint},
  partial_encode_scalar, partial_identity, partial_ref_state, partial_state, ref_state, selectable,
  try_from_bridge,
};

default_scalar_wire_format!(Groto: i128 as Varint; NonZeroI128 as Varint);
selectable!(@scalar Groto: i128, NonZeroI128);
ref_state!(@scalar &'a Groto:
  i128 as Varint,
  i128 as Fixed128,
  NonZeroI128 as Varint,
  NonZeroI128 as Fixed128,
);
partial_ref_state!(@scalar &'a Groto:
  i128 as Varint,
  i128 as Fixed128,
  NonZeroI128 as Varint,
  NonZeroI128 as Fixed128,
);
partial_state!(@scalar Groto: i128, NonZeroI128);
flatten_state!(i128, NonZeroI128);
partial_identity!(@scalar Groto: i128, NonZeroI128);

impl Encode<Fixed128, Groto> for i128 {
  fn encode_raw<B>(&self, _: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    buf
      .write_i128_le_checked(*self)
      .ok_or_else(|| Error::insufficient_buffer(16, buf.len()))
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    16
  }

  fn encode<B>(&self, context: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    <Self as Encode<Fixed128, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Fixed128, Groto>>::encoded_raw_len(self, context)
  }
}

impl Encode<Varint, Groto> for i128 {
  fn encode_raw<B>(&self, _: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: crate::buffer::WriteBuf + ?Sized,
  {
    buf.write_i128_varint(*self).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    varing::encoded_i128_varint_len(*self)
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

partial_encode_scalar!(Groto: i128 as Fixed128, i128 as Varint);

impl<'de, RB, B> Decode<'de, Fixed128, RB, B, Groto> for i128 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    let src = src.remaining_slice();
    if src.len() < 16 {
      return Err(Error::buffer_underflow());
    }

    Ok((16, i128::from_le_bytes(src[..16].try_into().unwrap())))
  }
}

impl<'de, RB, B> Decode<'de, Varint, RB, B, Groto> for i128 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: UnknownBuffer<RB, Groto>,
  {
    varing::decode_i128_varint(src.remaining_slice()).map_err(Into::into)
  }
}

try_from_bridge!(
  Groto: i128 {
    NonZeroI128 as Fixed128 {
      try_from: |v: i128| NonZeroI128::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroI128| v.get();
    },
    NonZeroI128 as Varint {
      try_from: |v: i128| NonZeroI128::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroI128| v.get();
    }
  },
);
