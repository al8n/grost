use crate::{
  decode::Decode,
  decoded_state, default_wire_format, encode_bridge, flatten_state,
  flavors::network::{Context, Error, LengthDelimited, Network, Unknown},
  selectable, try_decode_bridge,
};

default_wire_format!(
  Network: str as LengthDelimited
);
selectable!(@scalar Network:str);
decoded_state!(&'a Network: str as LengthDelimited => &'a str);
flatten_state!(str);

encode_bridge!(
  Network: [u8] {
    str as LengthDelimited {
      convert: str::as_bytes;
    },
  },
);

try_decode_bridge!(
  @without_decode_owned Network: &'de [u8] {
    &'de str as LengthDelimited {
      convert: decode_str;
    },
  },
);

macro_rules! impl_ {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<'de, B> Decode<'de, Network, LengthDelimited, &'de str, B> for $ty {
        fn decode(
          context: &Context,
          src: &'de [u8],
        ) -> Result<(usize, &'de str), Error>
        where
          &'de str: Sized + 'de,
          B: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
        {
          <&'de str as Decode<'de, Network, LengthDelimited, &'de str, B>>::decode(context, src)
        }
      }
    )*
  };
}

impl_!(str, [u8], &'de [u8]);

fn decode_str(src: &[u8]) -> Result<&str, Error> {
  crate::utils::from_utf8(src).map_err(|_| Error::custom("invalid UTF-8"))
}
