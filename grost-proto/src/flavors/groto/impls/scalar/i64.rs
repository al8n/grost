use core::num::NonZeroI64;

use crate::{
  buffer::{Chunk, ChunkExt, ChunkMut, ChunkMutExt, ChunkWriter, UnknownBuffer},
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, DecodeError, EncodeError, Fixed64, Groto, Varint},
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
  fn encode_raw<B>(&self, _: &Context, buf: impl Into<ChunkWriter<B>>) -> Result<usize, EncodeError>
  where
    B: ChunkMut,
  {
    let mut buf: ChunkWriter<B> = buf.into();
    buf.try_write_i64_le(*self).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    8
  }

  fn encode<B>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<B>>,
  ) -> Result<usize, EncodeError>
  where
    B: ChunkMut,
  {
    <Self as Encode<Fixed64, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Fixed64, Groto>>::encoded_raw_len(self, context)
  }
}

impl Encode<Varint, Groto> for i64 {
  fn encode_raw<B>(&self, _: &Context, buf: impl Into<ChunkWriter<B>>) -> Result<usize, EncodeError>
  where
    B: ChunkMut,
  {
    let mut buf: ChunkWriter<B> = buf.into();
    buf.write_varint(self).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    varing::encoded_i64_varint_len(*self)
  }

  fn encode<B>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<B>>,
  ) -> Result<usize, EncodeError>
  where
    B: ChunkMut,
  {
    <Self as Encode<Varint, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Varint, Groto>>::encoded_raw_len(self, context)
  }
}

partial_encode_scalar!(Groto: i64 as Fixed64, i64 as Varint);

impl<'de, RB, B> Decode<'de, Fixed64, RB, B, Groto> for i64 {
  fn decode(_: &Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Chunk,
    B: UnknownBuffer<RB, Groto>,
  {
    src
      .try_read_i64_le()
      .map(|val| (8, val))
      .map_err(Into::into)
  }
}

impl<'de, RB, B> Decode<'de, Varint, RB, B, Groto> for i64 {
  fn decode(_: &Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Chunk,
    B: UnknownBuffer<RB, Groto>,
  {
    src.read_varint().map_err(Into::into)
  }
}

try_from_bridge!(
  Groto: i64 {
    NonZeroI64 as Fixed64 {
      try_from: |v: i64| NonZeroI64::new(v).ok_or_else(|| DecodeError::other("value cannot be zero"));
      to: |v: &NonZeroI64| v.get();
    },
    NonZeroI64 as Varint {
      try_from: |v: i64| NonZeroI64::new(v).ok_or_else(|| DecodeError::other("value cannot be zero"));
      to: |v: &NonZeroI64| v.get();
    }
  },
);
