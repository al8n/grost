use crate::{DecodeError, Deserialize, EncodeError, Serialize, Wirable};

impl Wirable for &str {}

impl Serialize for &str {
  fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let this_len = self.len();
    let buf_len = buf.len();

    if this_len > buf_len {
      return Err(EncodeError::insufficient_buffer(this_len, buf_len));
    }

    buf[..this_len].copy_from_slice(self.as_bytes());

    Ok(this_len)
  }

  fn encoded_len(&self) -> usize {
    self.len()
  }
}

impl<'de> Deserialize<'de> for &'de str {
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: crate::UnknownRefBuffer<'de>,
  {
    decode_str(src)
  }
}

fn decode_str(src: &[u8]) -> Result<(usize, &str), DecodeError> {
  crate::utils::from_utf8(src)
    .map(|s| (src.len(), s))
    .map_err(|_| DecodeError::custom("invalid UTF-8"))
}
