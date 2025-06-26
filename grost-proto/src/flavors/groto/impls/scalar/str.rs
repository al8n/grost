use crate::{
  decode::{BytesSlice, Decode, Str},
  default_wire_format, encode_bridge, flatten_state,
  flavors::groto::{Context, Error, Groto, LengthDelimited, Unknown},
  partial_ref_state, partial_state, selectable,
};

default_wire_format!(
  Groto: str as LengthDelimited
);
selectable!(@scalar Groto:str);
partial_ref_state!(&'a Groto: str as LengthDelimited => Str<__GROST_READ_BUF__>);
// partial_state!(Groto: str => str);
flatten_state!(str);

encode_bridge!(
  Groto: [u8] {
    str as LengthDelimited {
      convert: str::as_bytes;
    },
  },
);

impl<'de, B, UB> Decode<'de, Groto, LengthDelimited, Str<B>, B, UB> for str {
  fn decode(context: &'de Context, src: B) -> Result<(usize, Str<B>), Error>
  where
    Str<B>: Sized + 'de,
    B: crate::buffer::ReadBuf + 'de,
    UB: crate::buffer::Buffer<Unknown<B>> + 'de,
  {
    <[u8] as Decode<'de, Groto, LengthDelimited, BytesSlice<B>, B, UB>>::decode(context, src)
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
      impl<'de, B, UB> Decode<'de, Groto, LengthDelimited, Str<B>, B, UB> for $ty {
        fn decode(
          context: &'de Context,
          src: B,
        ) -> Result<(usize, Str<B>), Error>
        where
          Str<B>: Sized + 'de,
          B: crate::buffer::ReadBuf + 'de,
          UB: crate::buffer::Buffer<Unknown<B>> + 'de,
        {
          <str as Decode<'de, Groto, LengthDelimited, Str<B>, B, UB>>::decode(context, src)
        }
      }
    )*
  };
}

impl_!(&'de str, [u8], &'de [u8]);
