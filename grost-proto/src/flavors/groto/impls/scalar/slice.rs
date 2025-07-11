use crate::{
  buffer::{Buffer, ReadBuf},
  decode::{BytesSlice, Decode},
  encode::{Encode, PartialEncode},
  flavors::{
    Flavor,
    groto::{Context, Error, Groto, LengthDelimited, Unknown},
  },
};

impl<RB> Encode<LengthDelimited, Groto> for BytesSlice<RB>
where
  RB: ReadBuf,
{
  fn encode_raw(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
  ) -> Result<usize, <Groto as Flavor>::Error> {
    <[u8] as Encode<LengthDelimited, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_raw_len(&self, context: &<Groto as Flavor>::Context) -> usize {
    <[u8] as Encode<LengthDelimited, Groto>>::encoded_raw_len(self, context)
  }

  fn encode(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
  ) -> Result<usize, <Groto as Flavor>::Error> {
    <[u8] as Encode<LengthDelimited, Groto>>::encode(self, context, buf)
  }

  fn encoded_len(&self, context: &<Groto as Flavor>::Context) -> usize {
    <[u8] as Encode<LengthDelimited, Groto>>::encoded_len(self, context)
  }
}

impl<RB> PartialEncode<LengthDelimited, Groto> for BytesSlice<RB>
where
  RB: ReadBuf,
{
  fn partial_encode_raw(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, <Groto as Flavor>::Error> {
    <[u8] as PartialEncode<LengthDelimited, Groto>>::partial_encode_raw(
      self, context, buf, selector,
    )
  }

  fn partial_encoded_raw_len(
    &self,
    context: &<Groto as Flavor>::Context,
    selector: &Self::Selector,
  ) -> usize {
    <[u8] as PartialEncode<LengthDelimited, Groto>>::partial_encoded_raw_len(
      self, context, selector,
    )
  }

  fn partial_encode(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, <Groto as Flavor>::Error> {
    <[u8] as PartialEncode<LengthDelimited, Groto>>::partial_encode(self, context, buf, selector)
  }

  fn partial_encoded_len(
    &self,
    context: &<Groto as Flavor>::Context,
    selector: &Self::Selector,
  ) -> usize {
    <[u8] as PartialEncode<LengthDelimited, Groto>>::partial_encoded_len(self, context, selector)
  }
}

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

// impl<'de, RB, B> Decode<'de, BytesSlice<RB>, LengthDelimited, RB, B, Groto> for &'de [u8] {
//   fn decode(context: &'de Context, src: RB) -> Result<(usize, BytesSlice<RB>), Error>
//   where
//     Self: Sized + 'de,
//     RB: ReadBuf + 'de,
//     B: Buffer<Unknown<RB>> + 'de,
//   {
//     <[u8] as Decode<'de, BytesSlice<RB>, LengthDelimited, RB, B, Groto>>::decode(context, src)
//   }
// }

bidi_equivalent!(:<RB: ReadBuf>: impl<[u8], LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
