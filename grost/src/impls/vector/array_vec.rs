use tinyvec::ArrayVec;

use crate::{
  DecodeError, Deserialize, DeserializeOwned, EncodeError, Message, Serialize, Tag, TypeOwned,
  TypeRef, UnknownBuffer, UnknownRefBuffer, Wirable,
};

/// The bytes type for `no-alloc` environment
pub type ArrayBytes<const N: usize> = ArrayVec<[u8; N]>;

impl<const N: usize> Wirable for ArrayVec<[u8; N]> {}

impl<const N: usize> Serialize for ArrayVec<[u8; N]> {
  fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let len = self.len();
    let buf_len = buf.len();
    if buf_len < len {
      return Err(EncodeError::insufficient_buffer(len, buf_len));
    }

    buf[..len].copy_from_slice(self.as_slice());
    Ok(len)
  }

  fn encoded_len(&self,) -> usize {
    self.len()
  }
}

impl<'de, const N: usize> Deserialize<'de> for ArrayVec<[u8; N]> {
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownRefBuffer<'de>,
  {
    if src.len() > N {
      return Err(DecodeError::custom(
        "ArrayVec does not have enough capacity",
      ));
    }

    let mut vec = ArrayVec::new();
    for b in src {
      vec.push(*b);
    }

    Ok((src.len(), vec))
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<const N: usize> DeserializeOwned for ArrayVec<[u8; N]> {
  fn decode_from_bytes<U>(src: bytes::Bytes, _: &mut U) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    U: UnknownBuffer<bytes::Bytes>,
  {
    if src.len() > N {
      return Err(DecodeError::custom(
        "ArrayVec does not have enough capacity",
      ));
    }

    let mut vec = ArrayVec::new();
    for b in &src {
      vec.push(*b);
    }

    Ok((src.len(), vec))
  }
}

impl<const N: usize> Message for ArrayVec<[u8; N]> {
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

impl<const N: usize> TypeRef<ArrayVec<[u8; N]>> for &[u8] {
  fn to_target(&self) -> Result<ArrayVec<[u8; N]>, DecodeError> {
    if self.len() > N {
      return Err(DecodeError::custom(
        "ArrayVec does not have enough capacity",
      ));
    }

    let mut vec = ArrayVec::new();
    for b in self.iter() {
      vec.push(*b);
    }
    Ok(vec)
  }

  fn into_target(self) -> Result<ArrayVec<[u8; N]>, DecodeError> {
    self.to_target()
  }
}

impl<const N: usize> TypeRef<Self> for ArrayVec<[u8; N]> {
  fn to_target(&self) -> Result<Self, DecodeError> {
    Ok(*self)
  }

  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

impl<const N: usize> TypeOwned<Self> for ArrayVec<[u8; N]> {
  fn to_target(&self) -> Result<Self, DecodeError> {
    Ok(*self)
  }

  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}
