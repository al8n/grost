use core::num::NonZeroI32;

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

default_wire_format!(Groto: i32 as Varint);
selectable!(@scalar Groto: i32, NonZeroI32);
partial_ref_state!(@scalar &'a Groto: i32 as Fixed32, NonZeroI32 as Fixed32, i32 as Varint, NonZeroI32 as Varint);
partial_state!(@scalar Groto: i32, NonZeroI32);
flatten_state!(i32, NonZeroI32);
identity_transform!(
  Groto {
    i32 as Fixed32,
    i32 as Varint,
    NonZeroI32 as Fixed32,
    NonZeroI32 as Varint,
  }
);
identity_partial_transform!(
  Groto {
    i32 as Fixed32,
    i32 as Varint,
    NonZeroI32 as Fixed32,
    NonZeroI32 as Varint,
  }
);

impl Encode<Groto, Fixed32> for i32 {
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

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Groto, Fixed32>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Groto, Fixed32>>::encode(self, context, buf)
  }
}

impl Encode<Groto, Varint> for i32 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_i32_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_i32_varint_len(*self)
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Groto, Varint>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Groto, Varint>>::encode(self, context, buf)
  }
}

partial_encode_scalar!(Groto: i32 as Fixed32, i32 as Varint);

impl<'de, B, UB> Decode<'de, Groto, Fixed32, Self, B, UB> for i32 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
  {
    let as_bytes = src.as_bytes();
    if src.len() < 4 {
      return Err(Error::buffer_underflow());
    }

    Ok((4, i32::from_le_bytes(as_bytes[..4].try_into().unwrap())))
  }
}

impl<'de, B, UB> Decode<'de, Groto, Varint, Self, B, UB> for i32 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
  {
    varing::decode_i32_varint(src.as_bytes()).map_err(Into::into)
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
