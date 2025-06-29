use core::num::NonZeroU32;

use crate::{
  buffer::{Buffer, ReadBuf},
  decode::Decode,
  default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed32, Groto, Unknown, Varint},
  identity_transform, partial_encode_scalar, partial_ref_state, partial_state, selectable,
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
identity_transform!(
  Groto {
    u32 as Fixed32,
    u32 as Varint,
    NonZeroU32 as Fixed32,
    NonZeroU32 as Varint,
  }
);
identity_partial_transform!(
  Groto {
    u32 as Fixed32,
    u32 as Varint,
    NonZeroU32 as Fixed32,
    NonZeroU32 as Varint,
  }
);

impl Encode<Groto, Fixed32> for u32 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 4 {
      return Err(Error::insufficient_buffer(4, buf.len()));
    }

    buf[..4].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(4)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    4
  }
}

impl Encode<Groto, Varint> for u32 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_u32_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_u32_varint_len(*self)
  }
}

partial_encode_scalar!(Groto: u32 as Fixed32, u32 as Varint);

impl<'de, B, UB> Decode<'de, Groto, Fixed32, Self, B, UB> for u32 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
  {
    let src = src.as_bytes();
    if src.len() < 4 {
      return Err(Error::buffer_underflow());
    }

    Ok((4, u32::from_le_bytes(src[..4].try_into().unwrap())))
  }
}

impl<'de, B, UB> Decode<'de, Groto, Varint, Self, B, UB> for u32 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
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
