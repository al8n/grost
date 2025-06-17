use core::num::NonZeroU8;

use crate::{
  buffer::{Buffer, ReadBuf},
  decode::Decode,
  decoded_state, default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::network::{Context, Error, Fixed8, Network, Unknown, Varint},
  identity_partial_transform, identity_transform, partial_encode_scalar, selectable,
  try_from_bridge,
};

default_wire_format!(Network: u8 as Fixed8);
selectable!(@scalar Network: u8, NonZeroU8);
decoded_state!(@scalar &'a Network: u8 as Fixed8, NonZeroU8 as Fixed8, u8 as Varint, NonZeroU8 as Varint);
flatten_state!(u8, NonZeroU8);

impl Encode<Network, Fixed8> for u8 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if buf.is_empty() {
      return Err(Error::insufficient_buffer(1, buf.len()));
    }

    buf[0] = *self;
    Ok(1)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    1
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Network, Fixed8>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Network, Fixed8>>::encode(self, context, buf)
  }
}

impl Encode<Network, Varint> for u8 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_u8_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_u8_varint_len(*self)
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Network, Varint>>::encoded_len(self, context)
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Network, Varint>>::encode(self, context, buf)
  }
}

partial_encode_scalar!(Network: u8 as Fixed8, u8 as Varint);

identity_transform!(Network {
  u8 as Fixed8,
  u8 as Varint,
  NonZeroU8 as Fixed8,
  NonZeroU8 as Varint,
});
identity_partial_transform!(Network {
  u8 as Fixed8,
  u8 as Varint,
  NonZeroU8 as Fixed8,
  NonZeroU8 as Varint,
});

impl<'de, UB> Decode<'de, Network, Fixed8, Self, UB> for u8 {
  fn decode<B>(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    if src.is_empty() {
      return Err(Error::buffer_underflow());
    }

    let value = src.as_bytes()[0];
    Ok((1, value))
  }
}

impl<'de, UB> Decode<'de, Network, Varint, Self, UB> for u8 {
  fn decode<B>(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    varing::decode_u8_varint(src.as_bytes()).map_err(Into::into)
  }
}

// decode_owned_scalar!(Network: u8 as Fixed8, u8 as Varint);
try_from_bridge!(
  Network: u8 {
    NonZeroU8 as Fixed8 {
      try_from: |v: u8| NonZeroU8::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU8| v.get();
    },
    NonZeroU8 as Varint {
      try_from: |v: u8| NonZeroU8::new(v).ok_or_else(|| Error::custom("value cannot be zero"));
      to: |v: &NonZeroU8| v.get();
    },
  },
);
