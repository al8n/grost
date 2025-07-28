use crate::{
  buffer::{ReadBuf, UnknownBuffer, WriteBuf},
  decode::{BytesSlice, Decode},
  encode::{Encode, PartialEncode},
  flavors::groto::{Context, Error, Groto, LengthDelimited},
};

macro_rules! decode_impl {
  ($src:ident, $ty:ty) => {{
    let bytes = $src.as_bytes();
    let (len_size, len) = $crate::__private::varing::decode_u32_varint(bytes)
      .map_err($crate::__private::flavors::groto::Error::from)?;

    let len = len as usize;
    let total = len_size + len;

    if len_size >= bytes.len() {
      return ::core::result::Result::Err(
        $crate::__private::flavors::groto::Error::buffer_underflow(),
      );
    }

    if total > bytes.len() {
      return ::core::result::Result::Err(
        $crate::__private::flavors::groto::Error::buffer_underflow(),
      );
    }

    ::core::result::Result::Ok((total, <$ty>::from(&bytes[len_size..total])))
  }};
}

impl Encode<LengthDelimited, Groto> for [u8] {
  #[inline]
  fn encode_raw<B>(&self, _: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: WriteBuf + ?Sized,
  {
    buf
      .write_slice_checked(self)
      .ok_or_else(|| Error::insufficient_buffer(self.len(), buf.len()))
  }

  #[inline]
  fn encoded_raw_len(&self, _: &Context) -> usize {
    self.len()
  }

  #[inline]
  fn encode<B>(&self, context: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: WriteBuf + ?Sized,
  {
    let buf_len = buf.len();
    let this_len = self.len();
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(
        <Self as Encode<LengthDelimited, Groto>>::encoded_len(self, context),
        buf_len,
      ));
    }

    let len_size = buf.write_u32_varint(this_len as u32).map_err(|e| {
      Error::from_varint_encode_error(e).update(
        <Self as Encode<LengthDelimited, Groto>>::encoded_len(self, context),
        buf_len,
      )
    })?;

    <Self as Encode<LengthDelimited, Groto>>::encode_raw(self, context, buf.suffix_mut(len_size))
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
  fn partial_encode_raw<B>(
    &self,
    context: &Context,
    buf: &mut B,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    B: WriteBuf + ?Sized,
  {
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
  fn partial_encode<B>(
    &self,
    context: &Context,
    buf: &mut B,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    B: WriteBuf + ?Sized,
  {
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
  fn encode_raw<B>(&self, context: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: WriteBuf + ?Sized,
  {
    <[u8] as Encode<LengthDelimited, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    <[u8] as Encode<LengthDelimited, Groto>>::encoded_raw_len(self, context)
  }

  fn encode<B>(&self, context: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: WriteBuf + ?Sized,
  {
    <[u8] as Encode<LengthDelimited, Groto>>::encode(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <[u8] as Encode<LengthDelimited, Groto>>::encoded_len(self, context)
  }
}

impl<RB> PartialEncode<LengthDelimited, Groto> for BytesSlice<RB>
where
  RB: ReadBuf,
{
  fn partial_encode_raw<B>(
    &self,
    context: &Context,
    buf: &mut B,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    B: WriteBuf + ?Sized,
  {
    <[u8] as PartialEncode<LengthDelimited, Groto>>::partial_encode_raw(
      self, context, buf, selector,
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <[u8] as PartialEncode<LengthDelimited, Groto>>::partial_encoded_raw_len(
      self, context, selector,
    )
  }

  fn partial_encode<B>(
    &self,
    context: &Context,
    buf: &mut B,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    B: WriteBuf + ?Sized,
  {
    <[u8] as PartialEncode<LengthDelimited, Groto>>::partial_encode(self, context, buf, selector)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <[u8] as PartialEncode<LengthDelimited, Groto>>::partial_encoded_len(self, context, selector)
  }
}

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for BytesSlice<RB> {
  fn decode(_: &'de Context, src: RB) -> Result<(usize, BytesSlice<RB>), Error>
  where
    BytesSlice<RB>: Sized + 'de,
    RB: ReadBuf + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
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

bidi_equivalent!(:<RB: ReadBuf>: impl<str, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);

#[cfg(any(feature = "std", feature = "alloc"))]
mod arc;
#[cfg(any(feature = "std", feature = "alloc"))]
mod boxed;
#[cfg(any(feature = "std", feature = "alloc"))]
mod rc;
mod triomphe;
