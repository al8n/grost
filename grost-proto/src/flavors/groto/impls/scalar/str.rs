use crate::{
  buffer::ReadBuf,
  decode::{BytesSlice, Decode, Str},
  default_string_wire_format,
  encode::{Encode, PartialEncode},
  encode_bridge, flatten_state,
  flavors::groto::{Context, Error, Groto, LengthDelimited},
  partial_ref_state, partial_state, ref_state, selectable,
  selection::Selectable,
};

macro_rules! decode_impl {
  ($src:ident, $ty:ty) => {{
    let bytes = $src.as_bytes();
    let (len_size, len) =
      $crate::__private::varing::decode_u32_varint(bytes).map_err(Error::from)?;

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

    $crate::utils::from_utf8(&bytes[len_size..total])
      .map_err(|_| $crate::__private::flavors::groto::Error::custom("invalid UTF-8"))
      .map(|s| (total, <$ty>::from(s)))
  }};
}

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
  RB: ReadBuf,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <str as Encode<LengthDelimited, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    <str as Encode<LengthDelimited, Groto>>::encoded_raw_len(self, context)
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <str as Encode<LengthDelimited, Groto>>::encode(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <str as Encode<LengthDelimited, Groto>>::encoded_len(self, context)
  }
}

impl<RB> PartialEncode<LengthDelimited, Groto> for Str<RB>
where
  RB: ReadBuf,
{
  fn partial_encode_raw(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encode_raw(self, context, buf, selector)
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encoded_raw_len(self, context, selector)
  }

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encode(self, context, buf, selector)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encoded_len(self, context, selector)
  }
}

impl<'de: 'a, 'a, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for Str<RB> {
  fn decode(context: &'de Context, src: RB) -> Result<(usize, Str<RB>), Error>
  where
    Str<RB>: Sized + 'de,
    RB: crate::buffer::ReadBuf + 'de,
    B: crate::buffer::UnknownBuffer<RB, Groto> + 'de,
  {
    <BytesSlice<RB> as Decode<'de, LengthDelimited, RB, B, Groto>>::decode(context, src).and_then(
      |(read, val)| {
        Str::try_new(val.into_inner())
          .map_err(|_| Error::custom("invalid UTF-8"))
          .map(|s| (read, s))
      },
    )
  }
}

bidi_equivalent!(:<RB: ReadBuf>: impl<str, LengthDelimited> for <Str<RB>, LengthDelimited>);

#[cfg(any(feature = "std", feature = "alloc"))]
mod arc;
#[cfg(any(feature = "std", feature = "alloc"))]
mod boxed;
#[cfg(any(feature = "std", feature = "alloc"))]
mod rc;
mod triomphe;
