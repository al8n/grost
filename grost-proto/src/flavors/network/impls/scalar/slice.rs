use crate::{
  buffer::Buffer,
  decode::Decode,
  default_wire_format,
  encode::Encode,
  flavors::network::{Context, DecodeError, EncodeError, LengthDelimited, Network, Unknown},
  partial_encode_scalar, selectable_scalar,
};

default_wire_format!(
  Network: [u8] as LengthDelimited
);

selectable_scalar!(Network: [u8]);

impl Encode<Network, LengthDelimited> for [u8] {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let this_len = self.len();
    let buf_len = buf.len();

    if this_len > buf_len {
      return Err(EncodeError::insufficient_buffer(this_len, buf_len));
    }

    buf[..this_len].copy_from_slice(self);
    Ok(this_len)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    self.len()
  }

  fn encoded_length_delimited_len(&self, _: &Context) -> usize {
    let this_len = self.len();
    let encoded_len = varing::encoded_u32_varint_len(this_len as u32);
    this_len + encoded_len
  }

  fn encode_length_delimited(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let this_len = self.len();

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
  }
}

partial_encode_scalar!(Network: [u8] as LengthDelimited);

impl<'de> Decode<'de, Network, LengthDelimited, &'de [u8]> for [u8] {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, &'de [u8]), DecodeError>
  where
    &'de [u8]: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    let src_len = src.len();
    if src_len == 0 {
      return Err(DecodeError::buffer_underflow());
    }

    Ok((src_len, src))
  }

  fn decode_length_delimited<UB>(
    _: &Context,
    src: &'de [u8],
  ) -> Result<(usize, &'de [u8]), DecodeError>
  where
    &'de [u8]: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
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
  }
}

impl<'de> Decode<'de, Network, LengthDelimited, Self> for &'de [u8] {
  fn decode<UB>(context: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <[u8]>::decode::<UB>(context, src)
  }

  fn decode_length_delimited<UB>(
    context: &Context,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <[u8]>::decode_length_delimited::<UB>(context, src)
  }
}
