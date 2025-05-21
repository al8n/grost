use crate::{
  buffer::Buffer,
  decode::Decode,
  flavors::network::{Context, Error, LengthDelimited, Network, Unknown},
};

impl<'de> Decode<'de, Network, LengthDelimited, &'de [u8]> for [u8] {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, &'de [u8]), Error>
  where
    &'de [u8]: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    let src_len = src.len();
    if src_len == 0 {
      return Err(Error::buffer_underflow());
    }

    Ok((src_len, src))
  }

  fn decode_length_delimited<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, &'de [u8]), Error>
  where
    &'de [u8]: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    let src_len = src.len();
    if src_len == 0 {
      return Err(Error::buffer_underflow());
    }

    let (size_len, size) = varing::decode_u32_varint(src)?;
    let end = size_len + size as usize;
    if end > src_len {
      return Err(Error::buffer_underflow());
    }

    Ok((end, &src[size_len..end]))
  }
}

impl<'de> Decode<'de, Network, LengthDelimited, Self> for &'de [u8] {
  fn decode<UB>(context: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <[u8]>::decode::<UB>(context, src)
  }

  fn decode_length_delimited<UB>(context: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <[u8]>::decode_length_delimited::<UB>(context, src)
  }
}
