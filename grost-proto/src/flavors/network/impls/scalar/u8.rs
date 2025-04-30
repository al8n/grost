use core::num::NonZeroU8;

use crate::{
  buffer::Buffer,
  decode::{Decode, DecodeOwned},
  encode::Encode,
  flavors::network::{Context, DecodeError, EncodeError, Network, Unknown, WireType},
  message, partial_encode_primitives, try_from_bridge,
};

impl Encode<Network> for u8 {
  fn encode(&self, _: &Context, wire_type: WireType, buf: &mut [u8]) -> Result<usize, EncodeError> {
    match wire_type {
      WireType::Varint => varing::encode_u8_varint_to(*self, buf).map_err(Into::into),
      WireType::Byte => {
        if buf.is_empty() {
          return Err(EncodeError::insufficient_buffer(1, buf.len()));
        }

        buf[0] = *self;
        Ok(1)
      }
      WireType::Fixed16 => {
        if buf.len() < 2 {
          return Err(EncodeError::insufficient_buffer(2, buf.len()));
        }

        buf[..2].copy_from_slice(&[*self, 0]);
        Ok(2)
      }
      WireType::Fixed32 => {
        if buf.len() < 4 {
          return Err(EncodeError::insufficient_buffer(4, buf.len()));
        }

        buf[..4].copy_from_slice(&[*self, 0, 0, 0]);
        Ok(4)
      }
      WireType::Fixed64 => {
        if buf.len() < 8 {
          return Err(EncodeError::insufficient_buffer(8, buf.len()));
        }

        buf[..8].copy_from_slice(&[*self, 0, 0, 0, 0, 0, 0, 0, 0]);
        Ok(8)
      }
      WireType::Fixed128 => {
        if buf.len() < 16 {
          return Err(EncodeError::insufficient_buffer(16, buf.len()));
        }

        let mut tmp = [0; 16];
        tmp[0] = *self;

        buf[..16].copy_from_slice(&tmp);
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
      WireType::Varint => varing::encoded_u8_varint_len(*self),
      WireType::Byte => 1,
      WireType::Fixed16 => 2,
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

partial_encode_primitives!(Network: u8);

impl<'de> Decode<'de, Network, Self> for u8 {
  fn decode<UB>(
    _: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    decode_u8(wire_type, src)
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

impl DecodeOwned<Network, Self> for u8 {
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
    decode_u8(wire_type, src.as_bytes())
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

fn decode_u8(wire_type: WireType, src: &[u8]) -> Result<(usize, u8), DecodeError> {
  match wire_type {
    WireType::Varint => varing::decode_u8_varint(src).map_err(Into::into),
    WireType::Byte => {
      if src.is_empty() {
        return Err(DecodeError::buffer_underflow());
      }

      let value = src[0];
      Ok((1, value))
    }
    WireType::Fixed16 => {
      if src.len() < 2 {
        return Err(DecodeError::buffer_underflow());
      }

      let mut tmp = [0; 2];
      tmp.copy_from_slice(&src[..2]);
      Ok((2, tmp[0]))
    }
    WireType::Fixed32 => {
      if src.len() < 4 {
        return Err(DecodeError::buffer_underflow());
      }

      let mut tmp = [0; 4];
      tmp.copy_from_slice(&src[..4]);
      Ok((4, tmp[0]))
    }
    WireType::Fixed64 => {
      if src.len() < 8 {
        return Err(DecodeError::buffer_underflow());
      }

      let mut tmp = [0; 8];
      tmp.copy_from_slice(&src[..8]);
      Ok((8, tmp[0]))
    }
    WireType::Fixed128 => {
      if src.len() < 16 {
        return Err(DecodeError::buffer_underflow());
      }

      let mut tmp = [0; 16];
      tmp.copy_from_slice(&src[..16]);
      Ok((16, tmp[0]))
    }
    _ => Err(DecodeError::unsupported_wire_type(
      core::any::type_name::<u8>(),
      wire_type,
    )),
  }
}

message!(Network: u8);

try_from_bridge!(
  Network: u8 {
    NonZeroU8 {
      try_from: |v: u8| NonZeroU8::new(v).ok_or_else(|| crate::error::DecodeError::custom("value cannot be zero"));
      to: |v: &NonZeroU8| v.get();
    }
  },
);
