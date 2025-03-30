use crate::{
  DecodeError, Deserialize, DeserializeOwned, EncodeError, OutputType, Serialize, Tag, TypeOwned,
  TypeRef, UnknownBuffer, UnknownRefBuffer, Wirable, WireType,
};
use heapless_0_8::Vec;

impl<T, const N: usize> Wirable for Vec<T, N> {}

impl<T, const N: usize> Serialize for Vec<T, N>
where
  T: Serialize,
{
  fn encode(&self, tag: Tag, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let len = self.encoded_len(tag);
    let buf_len = buf.len();
    if buf_len < len {
      return Err(EncodeError::insufficient_buffer(len, buf_len));
    }

    for item in self.iter() {
      match T::WIRE_TYPE {
        WireType::Merged => todo!(),
        WireType::Varint => todo!(),
        WireType::LengthDelimited => todo!(),
        WireType::Byte => todo!(),
        WireType::Fixed16 => todo!(),
        WireType::Fixed32 => todo!(),
        WireType::Fixed64 => todo!(),
        WireType::Fixed128 => todo!(),
      }
    }
    Ok(len)
  }

  fn encoded_len(&self, tag: Tag) -> usize {
    todo!()
  }
}

impl<'de, T, const N: usize> Deserialize<'de> for Vec<T, N>
where
  T: Deserialize<'de>,
{
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownRefBuffer<'de>,
  {
    todo!()
  }
}

impl<T, const N: usize> DeserializeOwned for Vec<T, N>
where
  T: DeserializeOwned,
{
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn decode_from_bytes<U>(src: bytes_1::Bytes, _: &mut U) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    U: UnknownBuffer<bytes_1::Bytes>,
  {
    todo!()
  }
}

// impl<T> OutputType for Vec<u8>
// where
//   T: OutputType,
// {
//   type Serialized<'a>
//     = &'a [T::Serialized<'a>]
//   where
//     Self: Sized + 'a;

//   type Borrowed<'a>
//     = &'a Self
//   where
//     Self: 'a;

//   type SerializedOwned
//     = Self
//   where
//     Self: Sized + 'static;
// }

impl<T: Clone, const N: usize> TypeOwned<Self> for Vec<T, N> {
  fn to_target(&self) -> Result<Self, DecodeError> {
    Ok(self.clone())
  }

  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

impl<T, const N: usize> TypeRef<Self> for Vec<T, N> {
  fn to_target(&self) -> Result<Self, DecodeError> {
    todo!()
  }

  fn into_target(self) -> Result<Self, DecodeError> {
    todo!()
  }
}

impl<'a, T, const N: usize> TypeRef<Vec<T, N>> for &'a [T] {
  fn to_target(&self) -> Result<Vec<T, N>, DecodeError> {
    todo!()
  }

  fn into_target(self) -> Result<Vec<T, N>, DecodeError> {
    todo!()
  }
}
