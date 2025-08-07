use core::num::NonZeroU16;

use crate::{
  buffer::{Buf, BufExt, BufMut, BufMutExt, UnknownBuffer, WriteBuf},
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, DecodeError, EncodeError, Fixed16, Groto, Varint},
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
  fn encode_raw<B>(&self, _: &Context, buf: impl Into<WriteBuf<B>>) -> Result<usize, EncodeError>
  where
    B: BufMut,
  {
    let mut buf: WriteBuf<B> = buf.into();
    buf.try_write_u16_le(*self).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    2
  }

  fn encode<B>(&self, context: &Context, buf: impl Into<WriteBuf<B>>) -> Result<usize, EncodeError>
  where
    B: BufMut,
  {
    <Self as Encode<Fixed16, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Fixed16, Groto>>::encoded_raw_len(self, context)
  }
}

impl Encode<Varint, Groto> for u16 {
  fn encode_raw<B>(&self, _: &Context, buf: impl Into<WriteBuf<B>>) -> Result<usize, EncodeError>
  where
    B: BufMut,
  {
    let mut buf: WriteBuf<B> = buf.into();
    buf.write_varint(self).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    varing::encoded_u16_varint_len(*self)
  }

  fn encode<B>(&self, context: &Context, buf: impl Into<WriteBuf<B>>) -> Result<usize, EncodeError>
  where
    B: BufMut,
  {
    <Self as Encode<Varint, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Varint, Groto>>::encoded_raw_len(self, context)
  }
}

impl<'de, RB, B> Decode<'de, Fixed16, RB, B, Groto> for u16 {
  fn decode(_: &Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Buf,
    B: UnknownBuffer<RB, Groto>,
  {
    src
      .try_read_u16_le()
      .map(|val| (2, val))
      .map_err(Into::into)
  }
}

impl<'de, RB, B> Decode<'de, Varint, RB, B, Groto> for u16 {
  fn decode(_: &Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Buf,
    B: UnknownBuffer<RB, Groto>,
  {
    src.read_varint().map_err(Into::into)
  }
}

try_from_bridge!(
  Groto: u16 {
    NonZeroU16 as Fixed16 {
      try_from: |v: u16| NonZeroU16::new(v).ok_or_else(|| DecodeError::other("value cannot be zero"));
      to: |v: &NonZeroU16| v.get();
    },
    NonZeroU16 as Varint {
      try_from: |v: u16| NonZeroU16::new(v).ok_or_else(|| DecodeError::other("value cannot be zero"));
      to: |v: &NonZeroU16| v.get();
    }
  },
);
