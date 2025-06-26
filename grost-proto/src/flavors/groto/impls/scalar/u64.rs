use core::num::NonZeroU64;

use crate::{
  buffer::{Buffer, ReadBuf},
  decode::Decode,
  default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed64, Groto, Unknown, Varint},
  identity_transform, partial_encode_scalar, partial_ref_state, partial_state, selectable,
  try_from_bridge,
};

default_wire_format!(Groto: u64 as Varint);
selectable!(@scalar Groto: u64, NonZeroU64);
partial_ref_state!(@scalar &'a Groto: u64 as Fixed64, NonZeroU64 as Fixed64, u64 as Varint, NonZeroU64 as Varint);
partial_state!(@scalar Groto: u64, NonZeroU64);
flatten_state!(u64, NonZeroU64);
identity_transform!(
  Groto {
    u64 as Fixed64,
    u64 as Varint,
    NonZeroU64 as Fixed64,
    NonZeroU64 as Varint,
  }
);
identity_partial_transform!(
  Groto {
    u64 as Fixed64,
    u64 as Varint,
    NonZeroU64 as Fixed64,
    NonZeroU64 as Varint,
  }
);

impl Encode<Groto, Fixed64> for u64 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 8 {
      return Err(Error::insufficient_buffer(8, buf.len()));
    }

    buf[..8].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(8)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    8
  }
}

impl Encode<Groto, Varint> for u64 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_u64_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_u64_varint_len(*self)
  }
}

partial_encode_scalar!(Groto: u64 as Fixed64, u64 as Varint);

impl<'de, B, UB> Decode<'de, Groto, Fixed64, Self, B, UB> for u64 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
  {
    let src = src.as_bytes();
    if src.len() < 8 {
      return Err(Error::buffer_underflow());
    }

    Ok((8, u64::from_le_bytes(src[..8].try_into().unwrap())))
  }
}

impl<'de, B, UB> Decode<'de, Groto, Varint, Self, B, UB> for u64 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
  {
    varing::decode_u64_varint(src.as_bytes()).map_err(Into::into)
  }
}

try_from_bridge!(
  Groto: u64 {
    NonZeroU64 as Fixed64 {
      try_from: |v: u64| NonZeroU64::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU64| v.get();
    },
    NonZeroU64 as Varint {
      try_from: |v: u64| NonZeroU64::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU64| v.get();
    }
  },
);
