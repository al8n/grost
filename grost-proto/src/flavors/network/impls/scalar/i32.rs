use core::num::NonZeroI32;

use crate::{
  buffer::Buffer,
  decode::{Decode, DecodeOwned},
  encode::Encode,
  flavors::network::{Context, DecodeError, EncodeError, Network, Unknown, WireType},
  message, partial_encode_primitives, try_from_bridge,
};

impl Encode<Network> for i32 {
  fn encode(&self, _: &Context, wire_type: WireType, buf: &mut [u8]) -> Result<usize, EncodeError> {
    match wire_type {
      WireType::Varint => varing::encode_i32_varint_to(*self, buf).map_err(Into::into),
      WireType::Fixed32 => {
        if buf.len() < 4 {
          return Err(EncodeError::insufficient_buffer(4, buf.len()));
        }

        buf[..4].copy_from_slice(self.to_le_bytes().as_slice());
        Ok(4)
      }
      WireType::Fixed64 => {
        if buf.len() < 8 {
          return Err(EncodeError::insufficient_buffer(8, buf.len()));
        }

        let (low, high) = buf[..8].split_at_mut(4);
        low.copy_from_slice(&self.to_le_bytes());
        high.copy_from_slice(&[0, 0, 0, 0]);
        Ok(8)
      }
      WireType::Fixed128 => {
        if buf.len() < 16 {
          return Err(EncodeError::insufficient_buffer(16, buf.len()));
        }

        let (low, high) = buf[..16].split_at_mut(4);
        low.copy_from_slice(&self.to_le_bytes());
        high.copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        Ok(16)
      }
      val => Err(EncodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        val,
      )),
    }
  }

  fn encoded_len(&self, _: &Context, wire_type: WireType) -> Result<usize, EncodeError> {
    Ok(match wire_type {
      WireType::Varint => varing::encoded_i32_varint_len(*self),
      WireType::Fixed32 => 4,
      WireType::Fixed64 => 8,
      WireType::Fixed128 => 16,
      val => {
        return Err(EncodeError::unsupported_wire_type(
          core::any::type_name::<Self>(),
          val,
        ));
      }
    })
  }

  fn encoded_length_delimited_len(
    &self,
    context: &Context,
    wire_type: WireType,
  ) -> Result<usize, EncodeError> {
    self.encoded_len(context, wire_type)
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    self.encode(context, wire_type, buf)
  }
}

partial_encode_primitives!(Network: i32);

impl<'de> Decode<'de, Network, Self> for i32 {
  fn decode<UB>(
    _: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    decode_i32(wire_type, src)
  }

  fn decode_length_delimited<UB>(
    ctx: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    Self::decode::<UB>(ctx, wire_type, src)
  }
}

impl DecodeOwned<Network, Self> for i32 {
  fn decode_owned<B, UB>(
    _: &Context,
    wire_type: WireType,
    src: B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: Buffer<Unknown<B>> + 'static,
  {
    decode_i32(wire_type, src.as_bytes())
  }

  fn decode_length_delimited_owned<B, UB>(
    context: &Context,
    wire_type: WireType,
    src: B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: Buffer<Unknown<B>> + 'static,
  {
    Self::decode_owned::<B, UB>(context, wire_type, src)
  }
}

fn decode_i32(wire_type: WireType, src: &[u8]) -> Result<(usize, i32), DecodeError> {
  match wire_type {
    WireType::Varint => varing::decode_i32_varint(src).map_err(Into::into),
    WireType::Fixed32 => {
      if src.len() < 4 {
        return Err(DecodeError::buffer_underflow());
      }

      Ok((4, i32::from_le_bytes(src[..4].try_into().unwrap())))
    }
    WireType::Fixed64 => {
      if src.len() < 8 {
        return Err(DecodeError::buffer_underflow());
      }
      Ok((8, i32::from_le_bytes(src[..4].try_into().unwrap())))
    }
    WireType::Fixed128 => {
      if src.len() < 16 {
        return Err(DecodeError::buffer_underflow());
      }

      Ok((16, i32::from_le_bytes(src[..4].try_into().unwrap())))
    }
    _ => Err(DecodeError::unsupported_wire_type(
      core::any::type_name::<i32>(),
      wire_type,
    )),
  }
}

message!(Network: i32);

try_from_bridge!(
  Network: i32 {
    NonZeroI32 {
      try_from: |v: i32| NonZeroI32::new(v).ok_or_else(|| crate::error::DecodeError::custom("value cannot be zero"));
      to: |v: &NonZeroI32| v.get();
    }
  },
);
