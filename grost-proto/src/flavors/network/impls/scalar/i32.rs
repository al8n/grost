use core::num::NonZeroI32;

use crate::{
  buffer::{Buf, Buffer},
  decode::Decode,
  decoded_state, default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::network::{Context, Error, Fixed32, Network, Unknown, Varint},
  partial_encode_scalar, selectable, try_from_bridge,
};

default_wire_format!(Network: i32 as Varint);
selectable!(@scalar Network: i32, NonZeroI32);
decoded_state!(@scalar &'a Network: i32 as Fixed32, NonZeroI32 as Fixed32, i32 as Varint, NonZeroI32 as Varint);
flatten_state!(i32, NonZeroI32);

impl Encode<Network, Fixed32> for i32 {
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
    <Self as Encode<Network, Fixed32>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Network, Fixed32>>::encode(self, context, buf)
  }
}

impl Encode<Network, Varint> for i32 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_i32_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_i32_varint_len(*self)
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Network, Varint>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Network, Varint>>::encode(self, context, buf)
  }
}

partial_encode_scalar!(Network: i32 as Fixed32, i32 as Varint);

impl<'de, UB> Decode<'de, Network, Fixed32, Self, UB> for i32 {
  fn decode<B>(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: Buf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    let chunk = src.chunk();
    if src.len() < 4 {
      return Err(Error::buffer_underflow());
    }

    Ok((4, i32::from_le_bytes(chunk[..4].try_into().unwrap())))
  }
}

impl<'de, UB> Decode<'de, Network, Varint, Self, UB> for i32 {
  fn decode<B>(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: Buf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    varing::decode_i32_varint(src.chunk()).map_err(Into::into)
  }
}

try_from_bridge!(
  Network: i32 {
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
