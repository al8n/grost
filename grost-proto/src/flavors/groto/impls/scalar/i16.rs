use crate::{
  buffer::{Buffer, ReadBuf},
  decode::Decode,
  default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed16, Groto, Unknown, Varint},
  groto_identity_transform, partial_encode_scalar, partial_ref_state, partial_state, selectable,
  try_from_bridge,
};
use core::num::NonZeroI16;

default_wire_format!(Groto: i16 as Varint; NonZeroI16 as Varint);
selectable!(@scalar Groto: i16, NonZeroI16);
partial_ref_state!(@scalar &'a Groto:
  i16 as Fixed16,
  NonZeroI16 as Fixed16,
  i16 as Varint,
  NonZeroI16 as Varint,
);
partial_state!(@scalar Groto: i16, NonZeroI16);
flatten_state!(i16, NonZeroI16);
groto_identity_transform!(
  i16 as Fixed16,
  i16 as Varint,
  NonZeroI16 as Fixed16,
  NonZeroI16 as Varint,
);
identity_partial_transform!(
  Groto {
    i16 as Fixed16,
    i16 as Varint,
    NonZeroI16 as Fixed16,
    NonZeroI16 as Varint,
  }
);

impl Encode<Fixed16, Groto> for i16 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.len() < 2 {
      return Err(Error::insufficient_buffer(2, buf.len()));
    }

    buf[..2].copy_from_slice(self.to_le_bytes().as_slice());
    Ok(2)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    2
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Fixed16, Groto>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Fixed16, Groto>>::encode(self, context, buf)
  }
}

impl Encode<Varint, Groto> for i16 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_i16_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_i16_varint_len(*self)
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Varint, Groto>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Varint, Groto>>::encode(self, context, buf)
  }
}

partial_encode_scalar!(Groto: i16 as Fixed16, i16 as Varint);

impl<'de, RB, B> Decode<'de, Self, Fixed16, RB, B, Groto> for i16 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: Buffer<Unknown<RB>> + 'de,
  {
    let src = src.as_bytes();
    if src.len() < 2 {
      return Err(Error::buffer_underflow());
    }

    Ok((2, i16::from_le_bytes(src[..2].try_into().unwrap())))
  }
}

impl<'de, RB, B> Decode<'de, Self, Varint, RB, B, Groto> for i16 {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: Buffer<Unknown<RB>> + 'de,
  {
    varing::decode_i16_varint(src.as_bytes()).map_err(Into::into)
  }
}

try_from_bridge!(
  Groto: i16 {
    NonZeroI16 as Fixed16 {
      try_from: |v: i16| NonZeroI16::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroI16| v.get();
    },
    NonZeroI16 as Varint {
      try_from: |v: i16| NonZeroI16::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroI16| v.get();
    }
  },
);
