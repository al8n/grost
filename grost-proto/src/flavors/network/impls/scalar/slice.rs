use crate::{
  buffer::{Buf, Buffer},
  decode::Decode,
  flavors::network::{Context, Error, LengthDelimited, Network, Unknown},
};

impl<'de, UB> Decode<'de, Network, LengthDelimited, &'de [u8], UB> for [u8] {
  fn decode<B>(_: &Context, src: B) -> Result<(usize, &'de [u8]), Error>
  where
    &'de [u8]: Sized + 'de,
    B: Buf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    let src_len = src.len();
    if src_len == 0 {
      return Err(Error::buffer_underflow());
    }

    Ok((src_len, src.as_bytes()))
  }
}

impl<'de, UB> Decode<'de, Network, LengthDelimited, Self, UB> for &'de [u8] {
  fn decode<B>(context: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: Buf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    <[u8] as Decode<'de, Network, LengthDelimited, Self, UB>>::decode(context, src)
  }
}
