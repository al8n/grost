use core::num::NonZeroI128;

use crate::{
  buffer::{Buffer, ReadBuf},
  decode::Decode,
  default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed128, Groto, Unknown, Varint},
  identity_transform, partial_encode_scalar, partial_ref_state, partial_state, selectable,
  try_from_bridge,
};

default_wire_format!(Groto: i128 as Varint);
selectable!(@scalar Groto: i128, NonZeroI128);
partial_ref_state!(@scalar &'a Groto:
  i128 as Varint,
  i128 as Fixed128,
  NonZeroI128 as Varint,
  NonZeroI128 as Fixed128,
);
partial_state!(@scalar Groto: i128, NonZeroI128);
flatten_state!(i128, NonZeroI128);
identity_transform!(
  Groto {
    i128 as Fixed128,
    i128 as Varint,
    NonZeroI128 as Fixed128,
    NonZeroI128 as Varint,
  }
);
identity_partial_transform!(
  Groto {
    i128 as Fixed128,
    i128 as Varint,
    NonZeroI128 as Fixed128,
    NonZeroI128 as Varint,
  }
);

impl Encode<Groto, Fixed128> for i128 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 16 {
      return Err(Error::insufficient_buffer(16, buf.len()));
    }

    buf[..16].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(16)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    16
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Groto, Fixed128>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Groto, Fixed128>>::encode(self, context, buf)
  }
}

impl Encode<Groto, Varint> for i128 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_i128_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_i128_varint_len(*self)
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Groto, Varint>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Groto, Varint>>::encode(self, context, buf)
  }
}

partial_encode_scalar!(Groto: i128 as Fixed128, i128 as Varint);

impl<'de, B, UB> Decode<'de, Groto, Fixed128, Self, B, UB> for i128 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
  {
    let src = src.as_bytes();
    if src.len() < 16 {
      return Err(Error::buffer_underflow());
    }

    Ok((16, i128::from_le_bytes(src[..16].try_into().unwrap())))
  }
}

impl<'de, B, UB> Decode<'de, Groto, Varint, Self, B, UB> for i128 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
  {
    varing::decode_i128_varint(src.as_bytes()).map_err(Into::into)
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
