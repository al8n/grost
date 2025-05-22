use crate::{
  buffer::Buffer,
  decode::Decode,
  flavors::network::{Context, Error, LengthDelimited, Network, Unknown},
};

impl<'de, B> Decode<'de, Network, LengthDelimited, &'de [u8], B> for [u8] {
  fn decode(_: &Context, src: &'de [u8]) -> Result<(usize, &'de [u8]), Error>
  where
    &'de [u8]: Sized + 'de,
    B: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    let src_len = src.len();
    if src_len == 0 {
      return Err(Error::buffer_underflow());
    }

    Ok((src_len, src))
  }
}

impl<'de, B> Decode<'de, Network, LengthDelimited, Self, B> for &'de [u8] {
  fn decode(context: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <[u8] as Decode<'de, Network, LengthDelimited, Self, B>>::decode(context, src)
  }
}
