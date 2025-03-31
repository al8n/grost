use crate::{Deserialize, Serialize, Tag, TypeRef, Wirable};

impl Wirable for &[u8] {}

impl Serialize for &[u8] {
  fn encode(&self, buf: &mut [u8]) -> Result<usize, crate::EncodeError> {
    let len = self.len();
    let buf_len = buf.len();
    if buf_len < len {
      return Err(crate::EncodeError::insufficient_buffer(len, buf_len));
    }

    buf[..len].copy_from_slice(self);
    Ok(len)
  }

  fn encoded_len(&self,) -> usize {
    self.len()
  }
}

impl TypeRef<Self> for &[u8] {
  fn to_target(&self) -> Result<Self, crate::DecodeError> {
    Ok(self)
  }

  fn into_target(self) -> Result<Self, crate::DecodeError> {
    Ok(self)
  }
}

impl<'de> Deserialize<'de> for &'de [u8] {
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), crate::DecodeError>
  where
    Self: Sized + 'de,
    B: crate::UnknownRefBuffer<'de>,
  {
    Ok((src.len(), src))
  }
}
