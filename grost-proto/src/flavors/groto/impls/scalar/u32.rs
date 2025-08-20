use core::num::NonZeroU32;

use crate::{
  buffer::{Chunk, ChunkExt, ChunkMut, ChunkMutExt, ChunkWriter, UnknownBuffer},
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, DecodeError, EncodeError, Fixed32, Groto, Varint},
  partial_encode_scalar, partial_identity, partial_ref_state, partial_state, ref_state, selectable,
  try_from_bridge,
};

default_scalar_wire_format!(Groto: u32 as Varint; NonZeroU32 as Varint);
selectable!(@scalar Groto: u32, NonZeroU32);
ref_state!(@scalar &'a Groto:
  u32 as Fixed32,
  NonZeroU32 as Fixed32,
  u32 as Varint,
  NonZeroU32 as Varint,
);
partial_ref_state!(@scalar &'a Groto:
  u32 as Fixed32,
  NonZeroU32 as Fixed32,
  u32 as Varint,
  NonZeroU32 as Varint,
);
partial_state!(@scalar Groto: u32, NonZeroU32);
flatten_state!(u32, NonZeroU32);
partial_identity!(@scalar Groto: u32, NonZeroU32);
partial_encode_scalar!(Groto: u32 as Fixed32, u32 as Varint);

impl Encode<Fixed32, Groto> for u32 {
  fn encode_raw<B>(&self, _: &Context, buf: impl Into<ChunkWriter<B>>) -> Result<usize, EncodeError>
  where
    B: ChunkMut,
  {
    let mut buf: ChunkWriter<B> = buf.into();
    buf.try_write_u32_le(*self).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    4
  }

  fn encode<B>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<B>>,
  ) -> Result<usize, EncodeError>
  where
    B: ChunkMut,
  {
    <Self as Encode<Fixed32, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Fixed32, Groto>>::encoded_raw_len(self, context)
  }
}

impl Encode<Varint, Groto> for u32 {
  fn encode_raw<B>(&self, _: &Context, buf: impl Into<ChunkWriter<B>>) -> Result<usize, EncodeError>
  where
    B: ChunkMut,
  {
    let mut buf: ChunkWriter<B> = buf.into();
    buf.write_varint(self).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    varing::encoded_u32_varint_len(*self)
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

impl<'de, RB, B> Decode<'de, Fixed32, RB, B, Groto> for u32 {
  fn decode(_: &Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Chunk,
    B: UnknownBuffer<RB, Groto>,
  {
    src
      .try_read_u32_le()
      .map(|val| (4, val))
      .map_err(Into::into)
  }
}

impl<'de, RB, B> Decode<'de, Varint, RB, B, Groto> for u32 {
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
  Groto: u32 {
    NonZeroU32 as Fixed32 {
      try_from: |v: u32| NonZeroU32::new(v).ok_or_else(|| DecodeError::other("value cannot be zero"));
      to: |v: &NonZeroU32| v.get();
    },
    NonZeroU32 as Varint {
      try_from: |v: u32| NonZeroU32::new(v).ok_or_else(|| DecodeError::other("value cannot be zero"));
      to: |v: &NonZeroU32| v.get();
    }
  },
);
