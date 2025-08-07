use crate::{
  buffer::{Buf, BufExt, BufMut, BufMutExt, UnknownBuffer, WriteBuf},
  decode::{BytesSlice, Decode},
  encode::{Encode, PartialEncode},
  error::{DecodeError, EncodeError},
  flavors::groto::{Context, Groto, LengthDelimited},
};

macro_rules! decode_impl {
  ($src:ident, $ty:ty) => {{
    use $crate::__private::buffer::BufExt;

    let remaining = $src.remaining();
    let (len_size, len) = $src.read_varint::<u32>()?;

    let len = len as usize;
    let total = len_size + len;

    if len > $src.remaining() {
      return ::core::result::Result::Err(
        $crate::__private::error::DecodeError::insufficient_data_with_requested(remaining, total),
      );
    }

    let output = <$ty>::from($src.prefix(len));
    $src.advance(len);
    ::core::result::Result::Ok((total, output))
  }};
}

impl Encode<LengthDelimited, Groto> for [u8] {
  #[inline]
  fn encode_raw<B>(
    &self,
    _: &Context,
    buf: impl Into<WriteBuf<B>>,
  ) -> Result<usize, EncodeError<Groto>>
  where
    B: BufMut,
  {
    let mut buf: WriteBuf<B> = buf.into();
    buf.try_write_slice(self).map_err(Into::into)
  }

  #[inline]
  fn encoded_raw_len(&self, _: &Context) -> usize {
    self.len()
  }

  #[inline]
  fn encode<B>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<B>>,
  ) -> Result<usize, EncodeError<Groto>>
  where
    B: BufMut,
  {
    let mut buf: WriteBuf<B> = buf.into();
    let remaining = buf.mutable();
    let this_len = self.len();

    let len_size = buf.write_varint::<u32>(&(this_len as u32)).map_err(|e| {
      EncodeError::from_varint_error(e).propagate_buffer_info(
        || <Self as Encode<LengthDelimited, Groto>>::encoded_len(self, context),
        || remaining,
      )
    })?;

    <Self as Encode<LengthDelimited, Groto>>::encode_raw(self, context, buf.prefix_mut(this_len))
      .map(|write| {
        buf.advance_mut(write);
        len_size + write
      })
      .map_err(|e| {
        e.propagate_buffer_info(
          || <Self as Encode<LengthDelimited, Groto>>::encoded_len(self, context),
          || remaining,
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
    buf: impl Into<WriteBuf<B>>,
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError<Groto>>
  where
    B: BufMut,
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
    buf: impl Into<WriteBuf<B>>,
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError<Groto>>
  where
    B: BufMut,
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
  RB: Buf,
{
  fn encode_raw<B>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<B>>,
  ) -> Result<usize, EncodeError<Groto>>
  where
    B: BufMut,
  {
    <[u8] as Encode<LengthDelimited, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    <[u8] as Encode<LengthDelimited, Groto>>::encoded_raw_len(self, context)
  }

  fn encode<B>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<B>>,
  ) -> Result<usize, EncodeError<Groto>>
  where
    B: BufMut,
  {
    <[u8] as Encode<LengthDelimited, Groto>>::encode(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <[u8] as Encode<LengthDelimited, Groto>>::encoded_len(self, context)
  }
}

impl<RB> PartialEncode<LengthDelimited, Groto> for BytesSlice<RB>
where
  RB: Buf,
{
  fn partial_encode_raw<B>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<B>>,
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError<Groto>>
  where
    B: BufMut,
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
    buf: impl Into<WriteBuf<B>>,
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError<Groto>>
  where
    B: BufMut,
  {
    <[u8] as PartialEncode<LengthDelimited, Groto>>::partial_encode(self, context, buf, selector)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <[u8] as PartialEncode<LengthDelimited, Groto>>::partial_encoded_len(self, context, selector)
  }
}

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for BytesSlice<RB> {
  fn decode(_: &'de Context, mut src: RB) -> Result<(usize, BytesSlice<RB>), DecodeError<Groto>>
  where
    BytesSlice<RB>: Sized + 'de,
    RB: Buf + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    let remaining = src.remaining();
    let (len_size, len) = src.read_varint::<u32>()?;

    let len = len as usize;
    let total = len_size + len;

    if len > src.remaining() {
      return Err(DecodeError::insufficient_data_with_requested(
        remaining, total,
      ));
    }

    Ok((total, BytesSlice::new(src.segment(..len))))
  }
}

bidi_equivalent!(:<RB: Buf>: impl<str, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);

#[cfg(any(feature = "std", feature = "alloc"))]
mod arc;
#[cfg(any(feature = "std", feature = "alloc"))]
mod boxed;
#[cfg(any(feature = "std", feature = "alloc"))]
mod rc;
mod triomphe;
