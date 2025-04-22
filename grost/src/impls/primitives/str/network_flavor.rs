use grost_proto::{buffer::Buffer, unknown::Unknown};

use crate::{
  Decode, Encode, Wirable,
  flavors::network::{Context, DecodeError, EncodeError, Network, WireType},
};

impl Wirable<Network> for str {
  const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

impl Encode<Network> for str {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let this_len = self.len();
    let buf_len = buf.len();

    if this_len > buf_len {
      return Err(EncodeError::insufficient_buffer(this_len, buf_len));
    }

    buf[..this_len].copy_from_slice(self.as_bytes());

    Ok(this_len)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    self.len()
  }
}

partial_encode_primitives!(Network: str);

impl<'de> Decode<'de, Network, Self> for &'de str {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<Network, &'de [u8]>> + 'de,
  {
    decode_str(src)
  }
}

fn decode_str(src: &[u8]) -> Result<(usize, &str), DecodeError> {
  crate::utils::from_utf8(src)
    .map(|s| (src.len(), s))
    .map_err(|_| DecodeError::custom("invalid UTF-8"))
}
