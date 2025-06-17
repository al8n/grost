use crate::{
  buffer::{Buffer, ReadBuf},
  decode::{Decode, PartialDecode},
  flavors::network::{Context, Error, LengthDelimited, Network, Unknown},
};

impl<'de, UB> Decode<'de, Network, LengthDelimited, &'de [u8], UB> for [u8] {
  fn decode<B>(_: &'de Context, src: B) -> Result<(usize, &'de [u8]), Error>
  where
    &'de [u8]: Sized + 'de,
    B: ReadBuf<'de>,
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

    Ok((total, &bytes[len_size..total]))
  }
}

impl<'de, UB> PartialDecode<'de, Network, LengthDelimited, &'de [u8], UB> for [u8] {
  fn partial_decode<B>(
    ctx: &'de Context,
    src: B,
    selector: &bool,
  ) -> Result<(usize, Option<&'de [u8]>), Error>
  where
    &'de [u8]: Sized + 'de,
    B: ReadBuf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    if *selector {
      return <[u8] as Decode<'de, Network, LengthDelimited, &'de [u8], UB>>::decode(ctx, src)
        .map(|(len, val)| (len, Some(val)));
    }

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

    Ok((total, None))
  }
}

impl<'de, UB> Decode<'de, Network, LengthDelimited, Self, UB> for &'de [u8] {
  fn decode<B>(context: &'de Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    <[u8] as Decode<'de, Network, LengthDelimited, Self, UB>>::decode(context, src)
  }
}

impl<'de, UB> PartialDecode<'de, Network, LengthDelimited, Self, UB> for &'de [u8] {
  fn partial_decode<B>(
    context: &'de Context,
    src: B,
    selector: &'de bool,
  ) -> Result<(usize, Option<Self>), Error>
  where
    Self: Sized + 'de,
    B: ReadBuf<'de>,
    UB: Buffer<Unknown<B>> + 'de,
  {
    <[u8] as PartialDecode<'de, Network, LengthDelimited, Self, UB>>::partial_decode(
      context, src, selector,
    )
  }
}
