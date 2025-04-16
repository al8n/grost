use crate::{Buffer, Context, Decode, Encode};

// mod arbitrary_int;
// mod bnum;
// mod decimal;
// mod half;
// mod num_complex;
// mod num_rational;
// mod ruint;

// varint!(u16, u32, u64, u128, i16, i32, i64, i128, char);

// fixed!(
//   32(f32 {
//     from_bytes: |src: &[u8]| { Ok(f32::from_le_bytes(src.try_into().unwrap())) },
//     to_bytes: |this: &Self, buf: &mut [u8]| {
//       buf.copy_from_slice(&this.to_le_bytes());
//       Ok(())
//     },
//   }),
//   64(f64 {
//     from_bytes: |src: &[u8]| { Ok(f64::from_le_bytes(src.try_into().unwrap())) },
//     to_bytes: |this: &Self, buf: &mut [u8]| {
//       buf.copy_from_slice(&this.to_le_bytes());
//       Ok(())
//     },
//   }),
// );

// message!(u8, i8, bool);

wirable!((@byte) <=> (u8));
// partial_encode_primitives!(u8);

// bridge!(
//   u8 {
//     i8 {
//       from: convert_u8_to_i8;
//       to: convert_i8_to_u8;
//     },
//     bool {
//       from: convert_u8_to_bool;
//       to: convert_bool_to_u8;
//     }
//   },
// );

impl Encode for u8 {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, crate::EncodeError> {
    if buf.is_empty() {
      return Err(crate::EncodeError::insufficient_buffer(1, 0));
    }

    buf[0] = *self;
    Ok(1)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    1
  }
}

impl<'de> Decode<'de, Self> for u8 {
  type UnknownBuffer<B> = ();

  fn decode<B>(_: &Context, src: B) -> Result<(usize, Self), crate::DecodeError>
  where
    Self: Sized + 'de,
    B: Buffer + 'de,
    Self::UnknownBuffer<B>: crate::UnknownBuffer<B>,
  {
    decode_u8_in(src.as_bytes())
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

#[cfg(test)]
mod tests {
  use crate::bytes::Bytes;

  use super::*;

  quickcheck::quickcheck! {
    fn fuzzy_u8(val: u8) -> bool {
      let ctx = Context::new();
      let encoded_len = val.encoded_len(&ctx);
      let mut buf = std::vec![0; encoded_len];

      let bytes_written = u8::encode(&val, &ctx, &mut buf).unwrap();
      assert_eq!(bytes_written, encoded_len);

      let (bytes_read, decoded) = u8::decode::<_>(&ctx, buf.as_slice()).unwrap();
      assert_eq!(bytes_read, bytes_written);
      assert_eq!(decoded, val);

      let (bytes_read, decoded) = u8::decode::<_>(&ctx, Bytes::from(buf)).unwrap();
      assert_eq!(bytes_read, bytes_written);
      assert_eq!(decoded, val);

      true
    }
  }
}