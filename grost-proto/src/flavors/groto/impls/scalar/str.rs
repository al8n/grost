use crate::{
  buffer::{Buf, BufMut, WriteBuf},
  decode::{BytesSlice, Decode, Str},
  default_string_wire_format,
  encode::{Encode, PartialEncode},
  encode_bridge, flatten_state,
  flavors::groto::{Context, DecodeError, EncodeError, Groto, LengthDelimited},
  partial_ref_state, partial_state, ref_state, selectable,
};

default_string_wire_format!(
  Groto: str as LengthDelimited
);
selectable!(@scalar Groto:str);
partial_state!(@scalar Groto: str);
partial_ref_state!(&'a Groto:
  str as LengthDelimited => Str<__GROST_READ_BUF__>,
);
ref_state!(&'a Groto:
  str as LengthDelimited => Str<__GROST_READ_BUF__>,
);
flatten_state!(str);

encode_bridge!(
  Groto: [u8] {
    str as LengthDelimited {
      convert: str::as_bytes;
    },
  },
);

impl<RB> Encode<LengthDelimited, Groto> for Str<RB>
where
  RB: Buf,
{
  fn encode_raw<B>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<B>>,
  ) -> Result<usize, EncodeError>
  where
    B: BufMut,
  {
    <str as Encode<LengthDelimited, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    <str as Encode<LengthDelimited, Groto>>::encoded_raw_len(self, context)
  }

  fn encode<B>(&self, context: &Context, buf: impl Into<WriteBuf<B>>) -> Result<usize, EncodeError>
  where
    B: BufMut,
  {
    <str as Encode<LengthDelimited, Groto>>::encode(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <str as Encode<LengthDelimited, Groto>>::encoded_len(self, context)
  }
}

impl<RB> PartialEncode<LengthDelimited, Groto> for Str<RB>
where
  RB: Buf,
{
  fn partial_encode_raw<B>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<B>>,
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError>
  where
    B: BufMut,
  {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encode_raw(self, context, buf, selector)
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encoded_raw_len(self, context, selector)
  }

  fn partial_encode<B>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<B>>,
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError>
  where
    B: BufMut,
  {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encode(self, context, buf, selector)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encoded_len(self, context, selector)
  }
}

impl<'de: 'a, 'a, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for Str<RB> {
  fn decode(context: &'de Context, src: RB) -> Result<(usize, Str<RB>), DecodeError>
  where
    Str<RB>: Sized + 'de,
    RB: crate::buffer::Buf + 'de,
    B: crate::buffer::UnknownBuffer<RB, Groto> + 'de,
  {
    <BytesSlice<RB> as Decode<'de, LengthDelimited, RB, B, Groto>>::decode(context, src).and_then(
      |(read, val)| {
        Str::try_new(val.into_inner())
          .map_err(|_| DecodeError::other("invalid UTF-8"))
          .map(|s| (read, s))
      },
    )
  }
}

bidi_equivalent!(:<RB: Buf>: impl<str, LengthDelimited> for <Str<RB>, LengthDelimited>);

#[cfg(any(feature = "std", feature = "alloc"))]
mod arc;
#[cfg(any(feature = "std", feature = "alloc"))]
mod boxed;
#[cfg(any(feature = "std", feature = "alloc"))]
mod rc;
mod triomphe;
