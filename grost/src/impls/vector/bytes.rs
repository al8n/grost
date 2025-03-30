use crate::{
  DecodeError, Deserialize, DeserializeOwned, EncodeError, OutputType, Serialize, Tag, TypeOwned,
  TypeRef, UnknownBuffer, UnknownRefBuffer, Wirable,
};
use bytes_1::Bytes;

impl Wirable for Bytes {}

impl Serialize for Bytes {
  fn encode(&self, _: Tag, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let len = self.len();
    let buf_len = buf.len();
    if buf_len < len {
      return Err(EncodeError::insufficient_buffer(len, buf_len));
    }

    buf[..len].copy_from_slice(self);
    Ok(len)
  }

  fn encoded_len(&self, _: Tag) -> usize {
    self.len()
  }
}

impl<'de> Deserialize<'de> for Bytes {
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownRefBuffer<'de>,
  {
    Ok((src.len(), Bytes::copy_from_slice(src)))
  }
}

impl DeserializeOwned for Bytes {
  fn decode_from_bytes<U>(src: Bytes, _: &mut U) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    U: UnknownBuffer<Bytes>,
  {
    Ok((src.len(), src))
  }
}

impl OutputType for Bytes {
  type Serialized<'a>
    = &'a [u8]
  where
    Self: Sized + 'a;

  type Borrowed<'a>
    = &'a Self
  where
    Self: 'a;

  type SerializedOwned
    = Self
  where
    Self: Sized + 'static;
}

impl TypeOwned<Self> for Bytes {
  fn to_target(&self) -> Result<Self, DecodeError> {
    Ok(self.clone())
  }

  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

impl TypeRef<Self> for Bytes {
  fn to_target(&self) -> Result<Self, DecodeError> {
    Ok(self.clone())
  }

  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

impl TypeRef<Bytes> for &[u8] {
  fn to_target(&self) -> Result<Bytes, DecodeError> {
    Ok(Bytes::copy_from_slice(self))
  }

  fn into_target(self) -> Result<Bytes, DecodeError> {
    Ok(Bytes::copy_from_slice(self))
  }
}
