use crate::{
  encode_bridge,
  flavors::network::{DecodeError, Network},
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

fn decode_str(src: &[u8]) -> Result<&str, DecodeError> {
  crate::utils::from_utf8(src).map_err(|_| DecodeError::custom("invalid UTF-8"))
}
