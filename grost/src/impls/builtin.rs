use crate::{Deserialize, DeserializeOwned, OutputType, Serialize, TypeOwned, TypeRef};

macro_rules! impl_for_builtin_integers {
  ($($ty:ident),+$(,)?) => {
    wirable!(@outer (varint) <=> ($($ty,)*));

    $(
      impl Serialize for $ty {
        impl_varing_types!(@serialize);
      }

      impl<'de> Deserialize<'de> for $ty {
        impl_varing_types!(@deserialize);
      }

      impl DeserializeOwned for $ty {
        impl_varing_types!(@deserialize_owned);
      }

      impl OutputType for $ty {
        impl_output_type_for_self!();
      }

      impl TypeRef<Self> for $ty {
        impl_varing_types!(@type_ref);
      }

      impl TypeOwned<Self> for $ty {
        impl_varing_types!(@type_owned);
      }
    )*
  };
}

impl_for_builtin_integers!(u16, u32, u64, u128, i16, i32, i64, i128);

impl Serialize for u8 {
  fn encode(&self, _: crate::Tag, buf: &mut [u8]) -> Result<usize, crate::EncodeError> {
    if buf.is_empty() {
      return Err(crate::EncodeError::insufficient_buffer(1, 0));
    }

    buf[0] = *self;
    Ok(1)
  }

  fn encoded_len(&self, _: crate::Tag) -> usize {
    1
  }
}

impl<'de> Deserialize<'de> for u8 {
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), crate::DecodeError>
  where
    Self: Sized + 'de,
    B: crate::UnknownRefBuffer<'de>,
  {
    decode_u8_in(src)
  }
}

impl DeserializeOwned for u8 {
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

impl_type_conversion_for_self!(@copy u8, i8, bool);

impl_output_type_for_self!(@outer (u8, i8, bool));

wirable!(@outer (byte) <=> (u8, i8, bool));

macro_rules! impl_for_type_to_u8 {
  ($($ty:ident::$into_u8:ident), +$(,)?) => {
    $(
      impl Serialize for $ty {
        fn encode(&self, tag: crate::Tag, buf: &mut [u8]) -> Result<usize, crate::EncodeError> {
          <u8 as Serialize>::encode(&(*self as u8), tag, buf)
        }

        fn encoded_len(&self, tag: crate::Tag) -> usize {
          <u8 as Serialize>::encoded_len(&(*self as u8), tag)
        }
      }

      impl<'de> Deserialize<'de> for $ty {
        fn decode<B>(src: &'de [u8], ub: &mut B) -> Result<(usize, Self), crate::DecodeError>
          where
            Self: Sized + 'de,
            B: crate::UnknownRefBuffer<'de> {
          <u8 as Deserialize>::decode(src, ub).map(|(n, v)| (n, $into_u8(v)))
        }
      }

      impl DeserializeOwned for $ty {
        #[cfg(any(feature = "std", feature = "alloc"))]
        fn decode_from_bytes<U>(
            src: bytes_1::Bytes,
            ub: &mut U,
          ) -> Result<(usize, Self), crate::DecodeError>
          where
            Self: Sized + 'static,
            U: crate::UnknownBuffer<bytes_1::Bytes> {
          <u8 as DeserializeOwned>::decode_from_bytes(src, ub).map(|(n, v)| (n, $into_u8(v)))
        }
      }
    )*
  };
}

impl_for_type_to_u8!(i8::convert_u8_to_i8, bool::convert_u8_to_bool);

#[inline]
const fn convert_u8_to_bool(v: u8) -> bool {
  v != 0
}

#[inline]
const fn convert_u8_to_i8(v: u8) -> i8 {
  v as i8
}
