use core::num::NonZeroU32;

use crate::{
  buffer::{Buffer, ReadBuf},
  decode::Decode,
  decoded_state, default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::network::{Context, Error, Fixed32, Network, Unknown, Varint},
  identity_transform, partial_decode_scalar, partial_encode_scalar, selectable, try_from_bridge,
};

default_wire_format!(Network: u32 as Varint);
selectable!(@scalar Network: u32, NonZeroU32);
decoded_state!(@scalar &'a Network: u32 as Fixed32, NonZeroU32 as Fixed32, u32 as Varint, NonZeroU32 as Varint);
flatten_state!(u32, NonZeroU32);
identity_transform!(
  Network {
    u32 as Fixed32,
    u32 as Varint,
    NonZeroU32 as Fixed32,
    NonZeroU32 as Varint,
  }
);

impl Encode<Network, Fixed32> for u32 {
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

impl Encode<Network, Varint> for u32 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    varing::encode_u32_varint_to(*self, buf).map_err(Into::into)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    varing::encoded_u32_varint_len(*self)
  }
}

partial_encode_scalar!(Network: u32 as Fixed32, u32 as Varint);
partial_decode_scalar!(Network:
  u32 as Fixed32 => |_, src: &'de [u8]| {
    if src.is_empty() {
      return Err(Error::buffer_underflow());
    }

    Ok((4, None))
  },
  u32 as Varint => |_, src: &'de [u8]| {
    if src.is_empty() {
      return Err(Error::buffer_underflow());
    }

    varing::consume_varint(src).map(|val| (val, None)).map_err(Into::into)
  },
);

impl<'de, UB> Decode<'de, Network, Fixed32, Self, UB> for u32 {
  fn decode<B>(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    let src = src.as_bytes();
    if src.len() < 4 {
      return Err(Error::buffer_underflow());
    }

    Ok((4, u32::from_le_bytes(src[..4].try_into().unwrap())))
  }
}

impl<'de, UB> Decode<'de, Network, Varint, Self, UB> for u32 {
  fn decode<B>(_: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    varing::decode_u32_varint(src.as_bytes()).map_err(Into::into)
  }
}

try_from_bridge!(
  Network: u32 {
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
