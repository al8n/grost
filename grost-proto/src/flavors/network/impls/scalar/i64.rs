use core::num::NonZeroI64;

use crate::{
  buffer::Buffer,
  decode::Decode,
  decode_owned_scalar, decoded_state, default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::network::{Context, Error, Fixed64, Network, Unknown, Varint},
  partial_encode_scalar, selectable, try_from_bridge,
};

default_wire_format!(Network: i64 as Varint);
selectable!(@scalar Network: i64, NonZeroI64);
decoded_state!(@scalar &'a Network: i64 as Fixed64, NonZeroI64 as Fixed64, i64 as Varint, NonZeroI64 as Varint);
flatten_state!(i64, NonZeroI64);

impl Encode<Network, Fixed64> for i64 {
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
    <Self as Encode<Network, Fixed64>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Network, Fixed64>>::encode(self, context, buf)
  }
}

impl Encode<Network, Varint> for i64 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_i64_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_i64_varint_len(*self)
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Network, Varint>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Network, Varint>>::encode(self, context, buf)
  }
}

partial_encode_scalar!(Network: i64 as Fixed64, i64 as Varint);

impl<'de, B> Decode<'de, Network, Fixed64, Self, B> for i64 {
  fn decode(_: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    if src.len() < 8 {
      return Err(Error::buffer_underflow());
    }

    Ok((8, i64::from_le_bytes(src[..8].try_into().unwrap())))
  }
}

impl<'de, B> Decode<'de, Network, Varint, Self, B> for i64 {
  fn decode(_: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    varing::decode_i64_varint(src).map_err(Into::into)
  }
}

decode_owned_scalar!(Network: i64 as Fixed64, i64 as Varint);
try_from_bridge!(
  Network: i64 {
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
