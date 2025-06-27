use crate::{
  buffer::{Buffer, ReadBuf},
  decode::Decode,
  default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::groto::{Context, Error, Fixed16, Groto, Unknown, Varint},
  identity_transform, partial_encode_scalar, partial_ref_state, partial_state, selectable,
  try_from_bridge,
};
use core::num::NonZeroI16;

default_wire_format!(Groto: i16 as Varint);
selectable!(@scalar Groto: i16, NonZeroI16);
partial_ref_state!(@scalar &'a Groto:
  i16 as Fixed16,
  NonZeroI16 as Fixed16,
  i16 as Varint,
  NonZeroI16 as Varint,
);
partial_state!(@scalar Groto: i16, NonZeroI16);
flatten_state!(i16, NonZeroI16);
identity_transform!(
  Groto {
    i16 as Fixed16,
    i16 as Varint,
    NonZeroI16 as Fixed16,
    NonZeroI16 as Varint,
  }
);
identity_partial_transform!(
  Groto {
    i16 as Fixed16,
    i16 as Varint,
    NonZeroI16 as Fixed16,
    NonZeroI16 as Varint,
  }
);

impl Encode<Groto, Fixed16> for i16 {
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
    <Self as Encode<Groto, Fixed16>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Groto, Fixed16>>::encode(self, context, buf)
  }
}

impl Encode<Groto, Varint> for i16 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_i16_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_i16_varint_len(*self)
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Groto, Varint>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Groto, Varint>>::encode(self, context, buf)
  }
}

partial_encode_scalar!(Groto: i16 as Fixed16, i16 as Varint);

impl<'de, B, UB> Decode<'de, Groto, Fixed16, Self, B, UB> for i16 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
  {
    let src = src.as_bytes();
    if src.len() < 2 {
      return Err(Error::buffer_underflow());
    }

    Ok((2, i16::from_le_bytes(src[..2].try_into().unwrap())))
  }
}

impl<'de, B, UB> Decode<'de, Groto, Varint, Self, B, UB> for i16 {
  fn decode(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
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
