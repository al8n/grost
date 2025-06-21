use crate::{
  buffer::{Buffer, ReadBuf},
  decode::{Decode, BytesSlice},
  flavors::network::{Context, Error, LengthDelimited, Network, Unknown},
};

impl<'de, B, UB> Decode<'de, Network, LengthDelimited, BytesSlice<B>, B, UB> for [u8] {
  fn decode(_: &'de Context, src: B) -> Result<(usize, BytesSlice<B>), Error>
  where
    BytesSlice<B>: Sized + 'de,
    B: ReadBuf,
    UB: Buffer<Unknown<B>> + 'de,
  {
    let bytes = src.as_bytes();
    let (len_size, len) = varing::decode_u32_varint(bytes).map_err(Error::from)?;

    let len = len as usize;
    let total = len_size + len;

    if len_size >= bytes.len() {
      return Err(Error::buffer_underflow());
    }

    if total > bytes.len() {
      return Err(Error::buffer_underflow());
    }

    Ok((total, BytesSlice::new(src.slice(len_size..total))))
  }
}

impl<'de, B, UB> Decode<'de, Network, LengthDelimited, BytesSlice<B>, B, UB> for &'de [u8] {
  fn decode(context: &'de Context, src: B) -> Result<(usize, BytesSlice<B>), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf + 'de,
    UB: Buffer<Unknown<B>> + 'de,
  {
    <[u8] as Decode<'de, Network, LengthDelimited, BytesSlice<B>, B, UB>>::decode(context, src)
  }
}
