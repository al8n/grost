

use heapless_0_8::String;

use crate::{Deserialize, EncodeError, Serialize, Wirable, WireType};

impl<const N: usize> Wirable for String<N> {
  const WIRE_TYPE: WireType = {
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

impl<const N: usize> Serialize for String<N> {
  fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
    if N == 0 {
      return Ok(0);
    }

    if buf.len() < N {
      return Err(EncodeError::insufficient_buffer(N, buf.len()));
    }

    buf[..N].copy_from_slice(self.as_bytes());
    Ok(N)
  }

  #[inline]
  fn encoded_len(&self) -> usize {
    N
  }

  fn encoded_len_with_prefix(&self) -> usize {
    match N {
      0 | 1 | 2 | 4 | 8 | 16 => N,
      _ => self.as_bytes().encoded_len_with_prefix(),
    }
  }

  fn encode_with_prefix(&self, buf: &mut [u8]) -> Result<usize, crate::EncodeError> {
    match N {
      0 | 1 | 2 | 4 | 8 | 16 => self.encode(buf),
      _ => self.as_bytes().encode_with_prefix(buf),
    }
  }
}

impl<'de, const N: usize> Deserialize<'de> for String<N> {
  fn decode<B>(src: &'de [u8], unknown_buffer: &mut B) -> Result<(usize, Self), crate::DecodeError>
  where
    Self: Sized + 'de,
    B: crate::UnknownRefBuffer<'de>
  {
    todo!()
  }
}
