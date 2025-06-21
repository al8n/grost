use crate::{
  decode::{BytesSlice, Decode, Str},
  decoded_state, default_wire_format, encode_bridge, flatten_state,
  flavors::network::{Context, Error, LengthDelimited, Network, Unknown},
  selectable,
};

default_wire_format!(
  Network: str as LengthDelimited
);
selectable!(@scalar Network:str);
decoded_state!(&'a Network: str as LengthDelimited => Str<__GROST_READ_BUF__>);
flatten_state!(str);

encode_bridge!(
  Network: [u8] {
    str as LengthDelimited {
      convert: str::as_bytes;
    },
  },
);

impl<'de, B, UB> Decode<'de, Network, LengthDelimited, Str<B>, B, UB> for str {
  fn decode(context: &'de Context, src: B) -> Result<(usize, Str<B>), Error>
  where
    Str<B>: Sized + 'de,
    B: crate::buffer::ReadBuf + 'de,
    UB: crate::buffer::Buffer<Unknown<B>> + 'de,
  {
    <[u8] as Decode<'de, Network, LengthDelimited, BytesSlice<B>, B, UB>>::decode(context, src)
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
      impl<'de, B, UB> Decode<'de, Network, LengthDelimited, Str<B>, B, UB> for $ty {
        fn decode(
          context: &'de Context,
          src: B,
        ) -> Result<(usize, Str<B>), Error>
        where
          Str<B>: Sized + 'de,
          B: crate::buffer::ReadBuf + 'de,
          UB: crate::buffer::Buffer<Unknown<B>> + 'de,
        {
          <str as Decode<'de, Network, LengthDelimited, Str<B>, B, UB>>::decode(context, src)
        }
      }
    )*
  };
}

impl_!(&'de str, [u8], &'de [u8]);
