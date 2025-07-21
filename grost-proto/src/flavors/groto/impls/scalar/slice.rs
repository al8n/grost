use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  decode::{BytesSlice, Decode},
  encode::{Encode, PartialEncode},
  flavors::{
    Flavor,
    groto::{Context, Error, Groto, LengthDelimited},
  },
};

impl Encode<LengthDelimited, Groto> for [u8] {
  #[inline]
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let buf_len = buf.len();
    let this_len = self.len();
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(
        <Self as Encode<LengthDelimited, Groto>>::encoded_len(self, context),
        buf_len,
      ));
    }

    buf[..this_len].copy_from_slice(self);
    Ok(this_len)
  }

  #[inline]
  fn encoded_raw_len(&self, _: &<Groto as crate::flavors::Flavor>::Context) -> usize {
    self.len()
  }

  #[inline]
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let buf_len = buf.len();
    let this_len = self.len();
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(
        <Self as Encode<LengthDelimited, Groto>>::encoded_len(self, context),
        buf_len,
      ));
    }

    let len_size = varing::encode_u32_varint_to(this_len as u32, buf).map_err(|e| {
      Error::from_varint_encode_error(e).update(
        <Self as Encode<LengthDelimited, Groto>>::encoded_len(self, context),
        buf_len,
      )
    })?;

    <Self as Encode<LengthDelimited, Groto>>::encode_raw(self, context, &mut buf[len_size..])
      .map(|write| len_size + write)
      .map_err(|e| {
        e.update(
          <Self as Encode<LengthDelimited, Groto>>::encoded_len(self, context),
          buf_len,
        )
      })
  }

  #[inline]
  fn encoded_len(&self, _: &Context) -> usize {
    let len = self.len();
    let len_size = varing::encoded_u32_varint_len(len as u32);
    len_size + len
  }
}

impl PartialEncode<LengthDelimited, Groto> for [u8] {
  #[inline]
  fn partial_encode_raw(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if *selector {
      <Self as Encode<LengthDelimited, Groto>>::encode_raw(self, context, buf)
    } else {
      Ok(0)
    }
  }

  #[inline]
  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if *selector {
      <Self as Encode<LengthDelimited, Groto>>::encoded_raw_len(self, context)
    } else {
      0
    }
  }

  #[inline]
  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if *selector {
      <Self as Encode<LengthDelimited, Groto>>::encode(self, context, buf)
    } else {
      Ok(0)
    }
  }

  #[inline]
  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if *selector {
      <Self as Encode<LengthDelimited, Groto>>::encoded_len(self, context)
    } else {
      0
    }
  }
}

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
    B: UnknownBuffer<RB, Groto>,
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
//     B: UnknownBuffer<RB, Groto>,
//   {
//     <[u8] as Decode<'de, BytesSlice<RB>, LengthDelimited, RB, B, Groto>>::decode(context, src)
//   }
// }

bidi_equivalent!(:<RB: ReadBuf>: impl<[u8], LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
