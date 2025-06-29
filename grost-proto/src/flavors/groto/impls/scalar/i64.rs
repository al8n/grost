use core::num::NonZeroI64;

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

default_wire_format!(Groto: i64 as Varint; NonZeroI64 as Varint);
selectable!(@scalar Groto: i64, NonZeroI64);
partial_ref_state!(@scalar &'a Groto:
  i64 as Fixed64,
  NonZeroI64 as Fixed64,
  i64 as Varint,
  NonZeroI64 as Varint,
);
partial_state!(@scalar Groto: i64, NonZeroI64);
flatten_state!(i64, NonZeroI64);
identity_transform!(
  Groto {
    i64 as Fixed64,
    i64 as Varint,
    NonZeroI64 as Fixed64,
    NonZeroI64 as Varint,
  }
);
identity_partial_transform!(
  Groto {
    i64 as Fixed64,
    i64 as Varint,
    NonZeroI64 as Fixed64,
    NonZeroI64 as Varint,
  }
);

impl Encode<Groto, Fixed64> for i64 {
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

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Groto, Fixed64>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Groto, Fixed64>>::encode(self, context, buf)
  }
}

impl Encode<Groto, Varint> for i64 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_i64_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_i64_varint_len(*self)
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Groto, Varint>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Groto, Varint>>::encode(self, context, buf)
  }
}

partial_encode_scalar!(Groto: i64 as Fixed64, i64 as Varint);

impl<'de, B, UB> Decode<'de, Groto, Fixed64, Self, B, UB> for i64 {
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

    Ok((8, i64::from_le_bytes(src[..8].try_into().unwrap())))
  }
}

impl<'de, B, UB> Decode<'de, Groto, Varint, Self, B, UB> for i64 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
  {
    varing::decode_i64_varint(src.as_bytes()).map_err(Into::into)
  }
}

try_from_bridge!(
  Groto: i64 {
    NonZeroI64 as Fixed64 {
      try_from: |v: i64| NonZeroI64::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroI64| v.get();
    },
    NonZeroI64 as Varint {
      try_from: |v: i64| NonZeroI64::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroI64| v.get();
    }
  },
);
