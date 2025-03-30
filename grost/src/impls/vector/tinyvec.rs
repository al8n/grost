use tinyvec::TinyVec;

use crate::{
  DecodeError, Deserialize, DeserializeOwned, EncodeError, OutputType, Serialize, Tag, TypeOwned,
  TypeRef, UnknownBuffer, UnknownRefBuffer, Wirable,
};

/// The bytes type of GraphRPC for `alloc` and `std` environment
pub type TinyBytes<const N: usize> = TinyVec<[u8; N]>;

impl<const N: usize> Wirable for TinyVec<[u8; N]> {}

impl<const N: usize> Serialize for TinyVec<[u8; N]> {
  fn encode(&self, _: Tag, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let len = self.len();
    let buf_len = buf.len();
    if buf_len < len {
      return Err(EncodeError::insufficient_buffer(len, buf_len));
    }

    buf[..len].copy_from_slice(self.as_slice());
    Ok(len)
  }

  fn encoded_len(&self, _: Tag) -> usize {
    self.len()
  }
}

impl<'de, const N: usize> Deserialize<'de> for TinyVec<[u8; N]> {
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownRefBuffer<'de>,
  {
    Ok((src.len(), TinyVec::from(src)))
  }
}

impl<const N: usize> DeserializeOwned for TinyVec<[u8; N]> {
  fn decode_from_bytes<U>(src: bytes_1::Bytes, _: &mut U) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    U: UnknownBuffer<bytes_1::Bytes>,
  {
    Ok((src.len(), TinyVec::from(src.as_ref())))
  }
}

impl<const N: usize> OutputType for TinyVec<[u8; N]> {
  type Serialized<'a>
    = &'a [u8]
  where
    Self: Sized + 'a;

  type Borrowed<'a>
    = &'a Self
  where
    Self: 'a;

  type SerializedOwned
    = bytes_1::Bytes
  where
    Self: Sized + 'static;
}

impl<const N: usize> TypeRef<TinyVec<[u8; N]>> for &[u8] {
  fn to_target(&self) -> Result<TinyVec<[u8; N]>, DecodeError> {
    Ok(TinyVec::from(*self))
  }

  fn into_target(self) -> Result<TinyVec<[u8; N]>, DecodeError> {
    Ok(TinyVec::from(self))
  }
}

impl<const N: usize> TypeRef<Self> for TinyVec<[u8; N]> {
  fn to_target(&self) -> Result<Self, DecodeError> {
    Ok(self.clone())
  }

  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

impl<const N: usize> TypeOwned<TinyVec<[u8; N]>> for TinyVec<[u8; N]> {
  fn to_target(&self) -> Result<TinyVec<[u8; N]>, DecodeError> {
    Ok(self.clone())
  }

  fn into_target(self) -> Result<TinyVec<[u8; N]>, DecodeError> {
    Ok(self)
  }
}

impl<const N: usize> TypeOwned<TinyVec<[u8; N]>> for bytes_1::Bytes {
  fn to_target(&self) -> Result<TinyVec<[u8; N]>, DecodeError> {
    Ok(TinyVec::from(self.as_ref()))
  }

  fn into_target(self) -> Result<TinyVec<[u8; N]>, DecodeError> {
    Ok(TinyVec::from(self.as_ref()))
  }
}
