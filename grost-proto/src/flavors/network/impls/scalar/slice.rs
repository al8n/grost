use crate::{
  buffer::Buffer,
  decode::Decode,
  encode::Encode,
  flavors::network::{Context, DecodeError, EncodeError, Network, Unknown, WireType},
  partial_encode_primitives,
};

impl Encode<Network> for [u8] {
  fn encode(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    if let WireType::LengthDelimited = wire_type {
      let this_len = self.len();
      let maximum_msg_len = context.maximum_message_size();
      if this_len > maximum_msg_len {
        return Err(EncodeError::too_large(this_len, maximum_msg_len));
      }

      let buf_len = buf.len();

      if this_len > buf_len {
        return Err(EncodeError::insufficient_buffer(this_len, buf_len));
      }

      buf[..this_len].copy_from_slice(self);
      Ok(this_len)
    } else {
      Err(EncodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wire_type,
      ))
    }
  }

  fn encoded_len(&self, context: &Context, wire_type: WireType) -> Result<usize, EncodeError> {
    if let WireType::LengthDelimited = wire_type {
      let this_len = self.len();
      let maximum_msg_len = context.maximum_message_size();
      if this_len > maximum_msg_len {
        return Err(EncodeError::too_large(this_len, maximum_msg_len));
      }

      Ok(this_len)
    } else {
      Err(EncodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wire_type,
      ))
    }
  }

  fn encoded_length_delimited_len(
    &self,
    context: &Context,
    wire_type: WireType,
  ) -> Result<usize, EncodeError> {
    if let WireType::LengthDelimited = wire_type {
      let this_len = self.len();
      let maximum_msg_len = context.maximum_message_size();
      if this_len > maximum_msg_len {
        return Err(EncodeError::too_large(this_len, maximum_msg_len));
      }
      let encoded_len = varing::encoded_u32_varint_len(this_len as u32);
      if this_len + encoded_len > maximum_msg_len {
        return Err(EncodeError::too_large(
          this_len + encoded_len,
          maximum_msg_len,
        ));
      }

      Ok(this_len + encoded_len)
    } else {
      Err(EncodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wire_type,
      ))
    }
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    if let WireType::LengthDelimited = wire_type {
      let this_len = self.len();
      let maximum_msg_len = context.maximum_message_size();
      if this_len > maximum_msg_len {
        return Err(EncodeError::too_large(this_len, maximum_msg_len));
      }

      let buf_len = buf.len();
      if this_len > buf_len {
        return Err(EncodeError::insufficient_buffer(this_len, buf_len));
      }

      let offset = varing::encode_u32_varint_to(this_len as u32, buf)?;
      let encoded_len = offset + this_len;
      if encoded_len > buf_len {
        return Err(EncodeError::insufficient_buffer(encoded_len, buf_len));
      }

      buf[offset..encoded_len].copy_from_slice(self);
      Ok(this_len)
    } else {
      Err(EncodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wire_type,
      ))
    }
  }
}

partial_encode_primitives!(Network: [u8]);

impl<'de> Decode<'de, Network, &'de [u8]> for [u8] {
  fn decode<UB>(
    _: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, &'de [u8]), DecodeError>
  where
    &'de [u8]: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    if let WireType::LengthDelimited = wire_type {
      let src_len = src.len();
      if src_len == 0 {
        return Err(DecodeError::buffer_underflow());
      }

      Ok((src_len, src))
    } else {
      Err(DecodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wire_type,
      ))
    }
  }

  fn decode_length_delimited<UB>(
    _: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, &'de [u8]), DecodeError>
  where
    &'de [u8]: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    if let WireType::LengthDelimited = wire_type {
      let src_len = src.len();
      if src_len == 0 {
        return Err(DecodeError::buffer_underflow());
      }

      let (size_len, size) = varing::decode_u32_varint(src)?;
      let end = size_len + size as usize;
      if end > src_len {
        return Err(DecodeError::buffer_underflow());
      }

      Ok((end, &src[size_len..end]))
    } else {
      Err(DecodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wire_type,
      ))
    }
  }
}

impl<'de> Decode<'de, Network, Self> for &'de [u8] {
  fn decode<UB>(
    context: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <[u8]>::decode::<UB>(context, wire_type, src)
  }

  fn decode_length_delimited<UB>(
    context: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <[u8]>::decode_length_delimited::<UB>(context, wire_type, src)
  }
}
