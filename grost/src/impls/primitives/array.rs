use varing::{decode_u32_varint, encode_u32_varint_to};

use crate::{
  DecodeError, Deserialize, DeserializeOwned, EncodeError, IntoTarget, Message, Serialize,
  TypeOwned, TypeRef, Wirable, WireType,
};

impl<const N: usize> Wirable for [u8; N] {
  const WIRE_TYPE: crate::WireType = {
    match N {
      0 => WireType::Merged,
      1 => WireType::Byte,
      2 => WireType::Fixed16,
      4 => WireType::Fixed32,
      8 => WireType::Fixed64,
      16 => WireType::Fixed128,
      _ => WireType::LengthDelimited,
    }
  };
}

impl<const N: usize> Serialize for [u8; N] {
  fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
    if N == 0 {
      return Ok(0);
    }

    if buf.len() < N {
      return Err(EncodeError::insufficient_buffer(N, buf.len()));
    }

    buf[..N].copy_from_slice(self.as_slice());
    Ok(N)
  }

  fn encoded_len(&self) -> usize {
    N
  }

  fn encoded_len_with_prefix(&self) -> usize {
    match N {
      0 | 1 | 2 | 4 | 8 | 16 => N,
      _ => self.as_slice().encoded_len_with_prefix(),
    }
  }

  fn encode_with_prefix(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
    match N {
      0 | 1 | 2 | 4 | 8 | 16 => self.encode(buf),
      _ => {
        let len_size = encode_u32_varint_to(N as u32, buf)
          .map_err(|e| EncodeError::from(e).update(self.encoded_len(), buf.len()))?;
        if len_size + N > buf.len() {
          return Err(EncodeError::insufficient_buffer(len_size + N, buf.len()));
        }

        buf[len_size..len_size + N].copy_from_slice(self.as_slice());

        Ok(len_size + N)
      }
    }
  }
}

impl<'de, const N: usize> Deserialize<'de> for [u8; N] {
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: crate::UnknownRefBuffer<'de>,
  {
    decode(src)
  }

  fn decode_length_prefix<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: crate::UnknownRefBuffer<'de>,
  {
    decode_length_prefix(src)
  }
}

impl<const N: usize> DeserializeOwned for [u8; N] {
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn decode_from_bytes<U>(src: bytes_1::Bytes, _: &mut U) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    U: crate::UnknownBuffer<bytes_1::Bytes>,
  {
    decode_length_prefix(&src)
  }
}

impl<const N: usize> IntoTarget<Self> for [u8; N] {
  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

impl<const N: usize> TypeRef<Self> for [u8; N] {
  fn to(&self) -> Result<Self, DecodeError> {
    Ok(*self)
  }
}

impl<const N: usize> TypeOwned<Self> for [u8; N] {
  fn to(&self) -> Result<Self, DecodeError> {
    Ok(*self)
  }
}

impl<const N: usize> Message for [u8; N] {
  type Serialized<'a>
    = &'a [u8]
  where
    Self: Sized + 'a;

  type Borrowed<'a>
    = &'a [u8; N]
  where
    Self: 'a;

  type SerializedOwned
    = Self
  where
    Self: Sized + 'static;
}

#[inline]
fn decode<const N: usize>(src: &[u8]) -> Result<(usize, [u8; N]), DecodeError> {
  if N == 0 {
    return Ok((0, [0; N]));
  }

  src[..N]
    .try_into()
    .map(|arr| (N, arr))
    .map_err(|_| DecodeError::buffer_underflow())
}

#[inline]
fn decode_length_prefix<const N: usize>(src: &[u8]) -> Result<(usize, [u8; N]), DecodeError> {
  if N == 0 {
    return Ok((0, [0; N]));
  }

  match N {
    1 | 2 | 4 | 8 | 16 => decode(src),
    _ => {
      let (readed, len) = decode_u32_varint(src)?;
      let len = len as usize;
      let total = len + readed;
      if total > src.len() {
        return Err(DecodeError::buffer_underflow());
      }

      src[readed..total]
        .try_into()
        .map(|arr| (total, arr))
        .map_err(|_| DecodeError::buffer_underflow())
    }
  }
}
