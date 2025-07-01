use crate::{
  buffer::{Buffer, ReadBuf},
  decode::{BytesSlice, Decode},
  flavors::groto::{Context, Error, Groto, LengthDelimited, Unknown},
};

impl<'de, RB, B> Decode<'de, BytesSlice<RB>, LengthDelimited, RB, B, Groto> for [u8] {
  fn decode(_: &'de Context, src: RB) -> Result<(usize, BytesSlice<RB>), Error>
  where
    BytesSlice<B>: Sized + 'de,
    RB: ReadBuf,
    B: Buffer<Unknown<RB>> + 'de,
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

impl<'de, RB, B> Decode<'de, BytesSlice<RB>, LengthDelimited, RB, B, Groto> for &'de [u8] {
  fn decode(context: &'de Context, src: RB) -> Result<(usize, BytesSlice<RB>), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf + 'de,
    B: Buffer<Unknown<RB>> + 'de,
  {
    <[u8] as Decode<'de, BytesSlice<RB>, LengthDelimited, RB, B, Groto>>::decode(context, src)
  }
}
