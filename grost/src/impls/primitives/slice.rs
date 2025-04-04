use crate::{Decode, DecodeError, Encode, EncodeError, IntoTarget, TypeRef, Wirable};

impl Wirable for [u8] {}

impl Encode for [u8] {
  fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let this_len = self.len();
    let buf_len = buf.len();

    if this_len > buf_len {
      return Err(EncodeError::insufficient_buffer(this_len, buf_len));
    }

    Ok(this_len)
  }

  #[inline]
  fn encoded_len(&self) -> usize {
    self.len()
  }
}

partial_encode_primitives!([u8]);

impl<'de> Decode<'de> for &'de [u8] {
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: crate::UnknownRefBuffer<'de>,
  {
    Ok((src.len(), src))
  }
}

impl<const N: usize> IntoTarget<[u8; N]> for &[u8] {
  fn into_target(self) -> Result<[u8; N], DecodeError> {
    self.try_into().map_err(|_| DecodeError::buffer_underflow())
  }
}

impl<const N: usize> TypeRef<[u8; N]> for &[u8] {
  fn to(&self) -> Result<[u8; N], DecodeError> {
    self.into_target()
  }
}
