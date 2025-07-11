use crate::{
  buffer::ReadBuf,
  decode::{BytesSlice, Decode, Str},
  default_string_wire_format,
  encode::{Encode, PartialEncode},
  encode_bridge, flatten_state,
  flavors::{
    Flavor,
    groto::{Context, Error, Groto, LengthDelimited, Unknown},
  },
  partial_ref_state, selectable,
};

default_string_wire_format!(
  Groto: str as LengthDelimited
);
selectable!(@scalar Groto:str);
partial_ref_state!(&'a Groto:
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
  fn encode_raw(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
  ) -> Result<usize, <Groto as Flavor>::Error> {
    <str as Encode<LengthDelimited, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_raw_len(&self, context: &<Groto as Flavor>::Context) -> usize {
    <str as Encode<LengthDelimited, Groto>>::encoded_raw_len(self, context)
  }

  fn encode(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
  ) -> Result<usize, <Groto as Flavor>::Error> {
    <str as Encode<LengthDelimited, Groto>>::encode(self, context, buf)
  }

  fn encoded_len(&self, context: &<Groto as Flavor>::Context) -> usize {
    <str as Encode<LengthDelimited, Groto>>::encoded_len(self, context)
  }
}

impl<RB> PartialEncode<LengthDelimited, Groto> for Str<RB>
where
  RB: ReadBuf,
{
  fn partial_encode_raw(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, <Groto as Flavor>::Error> {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encode_raw(self, context, buf, selector)
  }

  fn partial_encoded_raw_len(
    &self,
    context: &<Groto as Flavor>::Context,
    selector: &Self::Selector,
  ) -> usize {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encoded_raw_len(self, context, selector)
  }

  fn partial_encode(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, <Groto as Flavor>::Error> {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encode(self, context, buf, selector)
  }

  fn partial_encoded_len(
    &self,
    context: &<Groto as Flavor>::Context,
    selector: &Self::Selector,
  ) -> usize {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encoded_len(self, context, selector)
  }
}

impl<'de, RB, B> Decode<'de, Str<RB>, LengthDelimited, RB, B, Groto> for str {
  fn decode(context: &'de Context, src: RB) -> Result<(usize, Str<RB>), Error>
  where
    Str<B>: Sized + 'de,
    RB: crate::buffer::ReadBuf + 'de,
    B: crate::buffer::Buffer<Unknown<RB>> + 'de,
  {
    <[u8] as Decode<'de, BytesSlice<RB>, LengthDelimited, RB, B, Groto>>::decode(context, src)
      .and_then(|(read, val)| {
        Str::try_new(val.into_inner())
          .map_err(|_| Error::custom("invalid UTF-8"))
          .map(|s| (read, s))
      })
  }
}

macro_rules! impl_ {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<'de, RB, B> Decode<'de, Str<RB>, LengthDelimited, RB, B, Groto> for $ty {
        fn decode(
          context: &'de Context,
          src: RB,
        ) -> Result<(usize, Str<RB>), Error>
        where
          Str<B>: Sized + 'de,
          RB: crate::buffer::ReadBuf + 'de,
          B: crate::buffer::Buffer<Unknown<RB>> + 'de,
        {
          <str as Decode<'de, Str<RB>, LengthDelimited, RB, B, Groto>>::decode(context, src)
        }
      }
    )*
  };
}

impl_!([u8]);
bidi_equivalent!(:<RB: ReadBuf>: impl<str, LengthDelimited> for <Str<RB>, LengthDelimited>);
