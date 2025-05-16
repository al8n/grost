use crate::{
  decode::Decode,
  default_wire_format, encode_bridge,
  flavors::network::{Context, DecodeError, LengthDelimited, Network, Unknown},
  selectable, state, try_decode_bridge,
};

default_wire_format!(
  Network: str as LengthDelimited
);
selectable!(@scalar Network:str);
state!(&'a Network: str as LengthDelimited => &'a str);

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
      impl<'de> Decode<'de, Network, LengthDelimited, &'de str> for $ty {
        fn decode<UB>(
          context: &Context,
          src: &'de [u8],
        ) -> Result<(usize, &'de str), DecodeError>
        where
          &'de str: Sized + 'de,
          UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
        {
          <&'de str as Decode<'de, Network, LengthDelimited, &'de str>>::decode::<UB>(context, src)
        }

        fn decode_length_delimited<UB>(
          context: &Context,
          src: &'de [u8],
        ) -> Result<(usize, &'de str), DecodeError>
        where
          &'de str: Sized + 'de,
          UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
        {
          <&'de str as Decode<'de, Network, LengthDelimited, &'de str>>::decode_length_delimited::<UB>(context, src)
        }
      }
    )*
  };
}

impl_!(str, [u8], &'de [u8]);

fn decode_str(src: &[u8]) -> Result<&str, DecodeError> {
  crate::utils::from_utf8(src).map_err(|_| DecodeError::custom("invalid UTF-8"))
}
