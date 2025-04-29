use crate::{
  decode::Decode,
  encode_bridge,
  flavors::network::{Context, DecodeError, Network, Unknown, WireType},
  try_decode_bridge,
};

encode_bridge!(
  Network: [u8] {
    str {
      convert: str::as_bytes;
    },
  },
);

try_decode_bridge!(
  Network: &'de [u8] {
    &'de str {
      convert: decode_str;
    },
  },
);

macro_rules! impl_ {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<'de> Decode<'de, Network, &'de str> for $ty {
        fn decode<UB>(
          context: &Context,
          wire_type: WireType,
          src: &'de [u8],
        ) -> Result<(usize, &'de str), DecodeError>
        where
          &'de str: Sized + 'de,
          UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
        {
          <&'de str as Decode<'de, Network, &'de str>>::decode::<UB>(context, wire_type, src)
        }

        fn decode_length_delimited<UB>(
          context: &Context,
          wire_type: WireType,
          src: &'de [u8],
        ) -> Result<(usize, &'de str), DecodeError>
        where
          &'de str: Sized + 'de,
          UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
        {
          <&'de str as Decode<'de, Network, &'de str>>::decode_length_delimited::<UB>(context, wire_type, src)
        }
      }
    )*
  };
}

impl_!(str, [u8], &'de [u8]);

fn decode_str(src: &[u8]) -> Result<&str, DecodeError> {
  crate::utils::from_utf8(src).map_err(|_| DecodeError::custom("invalid UTF-8"))
}
