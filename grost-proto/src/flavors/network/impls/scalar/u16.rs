use crate::{
  buffer::Buffer,
  decode::{Decode, DecodeOwned},
  encode::Encode,
  flavors::network::{Context, DecodeError, EncodeError, Network, Unknown, WireType},
  message, partial_encode_primitives,
};

impl Encode<Network> for u16 {
  fn encode(&self, _: &Context, wire_type: WireType, buf: &mut [u8]) -> Result<usize, EncodeError> {
    match wire_type {
      WireType::Varint => varing::encode_u16_varint_to(*self, buf).map_err(Into::into),
      WireType::Fixed16 => {
        if buf.len() < 2 {
          return Err(EncodeError::insufficient_buffer(2, buf.len()));
        }

        buf[..2].copy_from_slice(self.to_le_bytes().as_slice());
        Ok(2)
      }
      WireType::Fixed32 => {
        if buf.len() < 4 {
          return Err(EncodeError::insufficient_buffer(4, buf.len()));
        }

        let (low, high) = buf[..4].split_at_mut(2);
        low.copy_from_slice(&self.to_le_bytes());
        high.copy_from_slice(&[0, 0]);
        Ok(4)
      }
      WireType::Fixed64 => {
        if buf.len() < 8 {
          return Err(EncodeError::insufficient_buffer(8, buf.len()));
        }

        let (low, high) = buf[..8].split_at_mut(2);
        low.copy_from_slice(&self.to_le_bytes());
        high.copy_from_slice(&[0, 0, 0, 0, 0, 0]);
        Ok(8)
      }
      WireType::Fixed128 => {
        if buf.len() < 16 {
          return Err(EncodeError::insufficient_buffer(16, buf.len()));
        }

        let (low, high) = buf[..16].split_at_mut(2);
        low.copy_from_slice(&self.to_le_bytes());
        high.copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
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
      WireType::Varint => varing::encoded_u16_varint_len(*self),
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

partial_encode_primitives!(Network: u16);

impl<'de> Decode<'de, Network, Self> for u16 {
  fn decode<UB>(
    _: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    decode_u16(wire_type, src)
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

impl DecodeOwned<Network, Self> for u16 {
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
    decode_u16(wire_type, src.as_bytes())
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

fn decode_u16(wire_type: WireType, src: &[u8]) -> Result<(usize, u16), DecodeError> {
  match wire_type {
    WireType::Varint => varing::decode_u16_varint(src).map_err(Into::into),
    WireType::Fixed16 => {
      if src.len() < 2 {
        return Err(DecodeError::buffer_underflow());
      }

      Ok((2, u16::from_le_bytes(src[..2].try_into().unwrap())))
    }
    WireType::Fixed32 => {
      if src.len() < 4 {
        return Err(DecodeError::buffer_underflow());
      }

      Ok((4, u16::from_le_bytes(src[..2].try_into().unwrap())))
    }
    WireType::Fixed64 => {
      if src.len() < 8 {
        return Err(DecodeError::buffer_underflow());
      }
      Ok((8, u16::from_le_bytes(src[..2].try_into().unwrap())))
    }
    WireType::Fixed128 => {
      if src.len() < 16 {
        return Err(DecodeError::buffer_underflow());
      }

      Ok((16, u16::from_le_bytes(src[..2].try_into().unwrap())))
    }
    _ => Err(DecodeError::unsupported_wire_type(
      core::any::type_name::<u16>(),
      wire_type,
    )),
  }
}

message!(Network: u16);
