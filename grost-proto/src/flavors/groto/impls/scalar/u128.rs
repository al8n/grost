use core::num::NonZeroU128;

use crate::{
  buffer::{Buf, BufExt, BufMut, BufMutExt, UnknownBuffer, WriteBuf},
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, DecodeError, EncodeError, Fixed128, Groto, Varint},
  partial_encode_scalar, partial_identity, partial_ref_state, partial_state, ref_state, selectable,
  try_from_bridge,
};

default_scalar_wire_format!(Groto: u128 as Varint; NonZeroU128 as Varint);
selectable!(@scalar Groto: u128, NonZeroU128);
ref_state!(@scalar &'a Groto:
  u128 as Fixed128,
  NonZeroU128 as Fixed128,
  u128 as Varint,
  NonZeroU128 as Varint,
);
partial_ref_state!(@scalar &'a Groto:
  u128 as Fixed128,
  NonZeroU128 as Fixed128,
  u128 as Varint,
  NonZeroU128 as Varint,
);
partial_state!(@scalar Groto: u128, NonZeroU128);
flatten_state!(u128, NonZeroU128);
partial_identity!(@scalar Groto: u128, NonZeroU128);
partial_encode_scalar!(Groto: u128 as Fixed128, u128 as Varint);

impl Encode<Fixed128, Groto> for u128 {
  fn encode_raw<B>(&self, _: &Context, buf: impl Into<WriteBuf<B>>) -> Result<usize, EncodeError>
  where
    B: BufMut,
  {
    let mut buf: WriteBuf<B> = buf.into();
    buf.try_write_u128_le(*self).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    16
  }

  fn encode<B>(&self, context: &Context, buf: impl Into<WriteBuf<B>>) -> Result<usize, EncodeError>
  where
    B: BufMut,
  {
    <Self as Encode<Fixed128, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Fixed128, Groto>>::encoded_raw_len(self, context)
  }
}

impl Encode<Varint, Groto> for u128 {
  fn encode_raw<B>(&self, _: &Context, buf: impl Into<WriteBuf<B>>) -> Result<usize, EncodeError>
  where
    B: BufMut,
  {
    let mut buf: WriteBuf<B> = buf.into();
    buf.write_varint(self).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    varing::encoded_u128_varint_len(*self)
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

impl<'de, RB, B> Decode<'de, Fixed128, RB, B, Groto> for u128 {
  fn decode(_: &Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Buf,
    B: UnknownBuffer<RB, Groto>,
  {
    src
      .try_read_u128_le()
      .map_err(Into::into)
      .map(|val| (16, val))
  }
}

impl<'de, RB, B> Decode<'de, Varint, RB, B, Groto> for u128 {
  fn decode(_: &Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Buf,
    B: UnknownBuffer<RB, Groto>,
  {
    src
      .read_varint()
      .map_err(Into::into)
      .map(|(len, value)| (len, value))
  }
}

try_from_bridge!(
  Groto: u128 {
    NonZeroU128 as Fixed128 {
      try_from: |v: u128| NonZeroU128::new(v).ok_or_else(|| DecodeError::other("value cannot be zero"));
      to: |v: &NonZeroU128| v.get();
    },
    NonZeroU128 as Varint {
      try_from: |v: u128| NonZeroU128::new(v).ok_or_else(|| DecodeError::other("value cannot be zero"));
      to: |v: &NonZeroU128| v.get();
    }
  },
);
