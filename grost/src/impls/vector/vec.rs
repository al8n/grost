use bytes_1::Bytes;

use crate::{
  DecodeError, Deserialize, DeserializeOwned, EncodeError, OutputType, Serialize, Tag, TypeOwned,
  TypeRef, UnknownBuffer, UnknownRefBuffer, Wirable, WireType,
};
use core::marker::PhantomData;
use std::vec::Vec;

impl<T> Wirable for Vec<T> {}

impl<T> Serialize for Vec<T>
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

impl<'de, T> Deserialize<'de> for Vec<T>
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

impl<T> DeserializeOwned for Vec<T>
where
  T: DeserializeOwned,
{
  fn decode_from_bytes<U>(src: Bytes, _: &mut U) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    U: UnknownBuffer<Bytes>,
  {
    todo!()
  }
}

impl<T> OutputType for Vec<T>
where
  T: OutputType + DeserializeOwned + Clone,
{
  type Serialized<'a>
    = SerializedVec<'a, T::Serialized<'a>>
  where
    Self: Sized + 'a;

  type Borrowed<'a>
    = &'a Self
  where
    Self: 'a;

  type SerializedOwned
    = SerializedOwnedVec<T::SerializedOwned>
  where
    Self: Sized + 'static;
}

impl<T: Clone> TypeOwned<Self> for Vec<T> {
  fn to_target(&self) -> Result<Self, DecodeError> {
    Ok(self.to_vec())
  }

  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

impl<T> TypeRef<Self> for Vec<T> {
  fn to_target(&self) -> Result<Self, DecodeError> {
    todo!()
  }

  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

/// The serialized type for [`Vec<T>`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerializedVec<'a, T> {
  data: &'a [u8],
  _m: PhantomData<T>,
}

impl<'a, T> TypeRef<Vec<T>> for SerializedVec<'a, T::Serialized<'a>>
where
  T: OutputType,
{
  fn to_target(&self) -> Result<Vec<T>, DecodeError> {
    todo!()
  }

  fn into_target(self) -> Result<Vec<T>, DecodeError> {
    todo!()
  }
}

impl<T> Wirable for SerializedVec<'_, T> {}

impl<T> Serialize for SerializedVec<'_, T>
where
  T: Serialize,
{
  fn encode(&self, tag: Tag, buf: &mut [u8]) -> Result<usize, EncodeError> {
    todo!()
  }

  fn encoded_len(&self, tag: Tag) -> usize {
    todo!()
  }
}

impl<'de, T> Deserialize<'de> for SerializedVec<'de, T>
where
  T: Deserialize<'de>,
{
  fn decode<B>(src: &'de [u8], unknown_buffer: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownRefBuffer<'de>,
  {
    todo!()
  }
}

/// The owned serialized type for [`Vec<T>`]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SerializedOwnedVec<T> {
  data: Bytes,
  _m: PhantomData<T>,
}

impl<T> TypeOwned<Vec<T>> for SerializedOwnedVec<T::SerializedOwned>
where
  T: OutputType,
{
  fn to_target(&self) -> Result<Vec<T>, DecodeError> {
    todo!()
  }

  fn into_target(self) -> Result<Vec<T>, DecodeError> {
    todo!()
  }
}

impl<T> Wirable for SerializedOwnedVec<T> {}

impl<T> Serialize for SerializedOwnedVec<T>
where
  T: Serialize,
{
  fn encode(&self, tag: Tag, buf: &mut [u8]) -> Result<usize, EncodeError> {
    todo!()
  }

  fn encoded_len(&self, tag: Tag) -> usize {
    todo!()
  }
}

impl<'de, T> Deserialize<'de> for SerializedOwnedVec<T> {
  fn decode<B>(src: &'de [u8], unknown_buffer: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownRefBuffer<'de>,
  {
    todo!()
  }
}

impl<T> DeserializeOwned for SerializedOwnedVec<T>
where
  T: DeserializeOwned,
{
  fn decode_from_bytes<U>(src: Bytes, unknown_buffer: &mut U) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    U: UnknownBuffer<Bytes>,
  {
    todo!()
  }
}
