use crate::{Decode, DecodeOwned, Encode};

mod arbitrary_int;
mod bnum;
mod decimal;
mod half;
mod num_complex;
mod num_rational;
mod ruint;

varint!(u16, u32, u64, u128, i16, i32, i64, i128, char);

fixed!(
  32(f32 {
    from_bytes: |src: &[u8]| { Ok(f32::from_le_bytes(src.try_into().unwrap())) },
    to_bytes: |this: &Self, buf: &mut [u8]| {
      buf.copy_from_slice(&this.to_le_bytes());
      Ok(())
    },
  }),
  64(f64 {
    from_bytes: |src: &[u8]| { Ok(f64::from_le_bytes(src.try_into().unwrap())) },
    to_bytes: |this: &Self, buf: &mut [u8]| {
      buf.copy_from_slice(&this.to_le_bytes());
      Ok(())
    },
  }),
);

message!(u8, i8, bool);

wirable!((@byte) <=> (u8));
partial_encode_primitives!(u8);

bridge!(
  u8 {
    i8 {
      from: convert_u8_to_i8,
      to: convert_i8_to_u8,
    },
    bool {
      from: convert_u8_to_bool,
      to: convert_bool_to_u8,
    }
  },
);

impl Encode for u8 {
  fn encode(&self, buf: &mut [u8]) -> Result<usize, crate::EncodeError> {
    if buf.is_empty() {
      return Err(crate::EncodeError::insufficient_buffer(1, 0));
    }

    buf[0] = *self;
    Ok(1)
  }

  fn encoded_len(&self) -> usize {
    1
  }
}

impl<'de> Decode<'de> for u8 {
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), crate::DecodeError>
  where
    Self: Sized + 'de,
    B: crate::UnknownRefBuffer<'de>,
  {
    decode_u8_in(src)
  }
}

impl DecodeOwned for u8 {
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn decode_from_bytes<U>(
    src: bytes_1::Bytes,
    _: &mut U,
  ) -> Result<(usize, Self), crate::DecodeError>
  where
    Self: Sized + 'static,
    U: crate::UnknownBuffer<bytes_1::Bytes>,
  {
    decode_u8_in(&src)
  }
}

fn decode_u8_in(buf: &[u8]) -> Result<(usize, u8), crate::DecodeError> {
  if buf.is_empty() {
    return Err(crate::DecodeError::buffer_underflow());
  }

  Ok((1, buf[0]))
}

#[inline]
const fn convert_bool_to_u8(v: &bool) -> u8 {
  *v as u8
}

#[inline]
const fn convert_u8_to_bool(v: u8) -> bool {
  v != 0
}

#[inline]
const fn convert_i8_to_u8(v: &i8) -> u8 {
  *v as u8
}

#[inline]
const fn convert_u8_to_i8(v: u8) -> i8 {
  v as i8
}
