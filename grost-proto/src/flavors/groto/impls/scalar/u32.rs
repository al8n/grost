use core::num::NonZeroU32;

use crate::{
  buffer::{Buffer, ReadBuf},
  decode::Decode,
  default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed32, Groto, Unknown, Varint},
  groto_identity_transform, partial_encode_scalar, partial_ref_state, partial_state, selectable,
  try_from_bridge,
};

default_wire_format!(Groto: u32 as Varint; NonZeroU32 as Varint);
selectable!(@scalar Groto: u32, NonZeroU32);
partial_ref_state!(@scalar &'a Groto:
  u32 as Fixed32,
  NonZeroU32 as Fixed32,
  u32 as Varint,
  NonZeroU32 as Varint,
);
partial_state!(@scalar Groto: u32, NonZeroU32);
flatten_state!(u32, NonZeroU32);
groto_identity_transform!(
  u32 as Fixed32,
  u32 as Varint,
  NonZeroU32 as Fixed32,
  NonZeroU32 as Varint,
);
identity_partial_transform!(
  Groto {
    u32 as Fixed32,
    u32 as Varint,
    NonZeroU32 as Fixed32,
    NonZeroU32 as Varint,
  }
);

impl Encode<Fixed32, Groto> for u32 {
  fn encode_raw(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 4 {
      return Err(Error::insufficient_buffer(4, buf.len()));
    }

    buf[..4].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(4)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    4
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Fixed32, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Fixed32, Groto>>::encoded_raw_len(self, context)
  }
}

impl Encode<Varint, Groto> for u32 {
  fn encode_raw(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_u32_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    varing::encoded_u32_varint_len(*self)
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Varint, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Varint, Groto>>::encoded_raw_len(self, context)
  }
}

partial_encode_scalar!(Groto: u32 as Fixed32, u32 as Varint);

impl<'de, RB, B> Decode<'de, Self, Fixed32, RB, B, Groto> for u32 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: Buffer<Unknown<RB>> + 'de,
  {
    let src = src.as_bytes();
    if src.len() < 4 {
      return Err(Error::buffer_underflow());
    }

    Ok((4, u32::from_le_bytes(src[..4].try_into().unwrap())))
  }
}

impl<'de, RB, B> Decode<'de, Self, Varint, RB, B, Groto> for u32 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: Buffer<Unknown<RB>> + 'de,
  {
    varing::decode_u32_varint(src.as_bytes()).map_err(Into::into)
  }
}

try_from_bridge!(
  Groto: u32 {
    NonZeroU32 as Fixed32 {
      try_from: |v: u32| NonZeroU32::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU32| v.get();
    },
    NonZeroU32 as Varint {
      try_from: |v: u32| NonZeroU32::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU32| v.get();
    }
  },
);
