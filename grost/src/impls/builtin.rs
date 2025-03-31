use crate::{Deserialize, DeserializeOwned, Serialize,};

varing!(u16, u32, u64, u128, i16, i32, i64, i128, char);
zst!((), ::core::marker::PhantomPinned);
message!(u8, i8, bool);
wirable!((@byte) <=> (u8));
partial_serialize_primitives!(u8);
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

macro_rules! impl_for_phantom {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<T: ?::core::marker::Sized> $crate::Wirable for $ty {
        const WIRE_TYPE: $crate::WireType = $crate::WireType::Merged;
      }
      
      impl<T: ?::core::marker::Sized> $crate::Serialize for $ty {
        #[inline]
        fn encode(&self, _: $crate::Tag, _: &mut [u8]) -> ::core::result::Result<::core::primitive::usize, $crate::EncodeError> {
          ::core::result::Result::Ok(0)
        }
    
        #[inline]
        fn encoded_len(&self, _: $crate::Tag) -> ::core::primitive::usize {
          0
        }
      }
      
      impl<'de, T: ?::core::marker::Sized> $crate::Deserialize<'de> for $ty {
        fn decode<B>(_: &'de [u8], _: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::DecodeError>
        where
          Self: ::core::marker::Sized + 'de,
          B: $crate::UnknownRefBuffer<'de>,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }
      }
      
      impl<T: ?::core::marker::Sized> $crate::DeserializeOwned for $ty
      where
        Self: 'static,
      {
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn decode_from_bytes<U>(
          _: $crate::bytes::Bytes,
          _: &mut U,
        ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::DecodeError>
        where
          Self: ::core::marker::Sized + 'static,
          U: $crate::UnknownBuffer<$crate::bytes::Bytes>,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }
      }

      impl<T: ?::core::marker::Sized> $crate::PartialSerialize for $ty {
        type Selection = ();

        #[inline]
        fn partial_encode(&self, _: $crate::Tag, _: &Self::Selection, _: &mut [u8]) -> ::core::result::Result<::core::primitive::usize, $crate::EncodeError> {
          ::core::result::Result::Ok(0)
        }
      
        #[inline]
        fn partial_encoded_len(&self, _: $crate::Tag, _: &Self::Selection,) -> ::core::primitive::usize {
          0
        }
      }
    )*
  };
}

impl_for_phantom!(
  ::core::marker::PhantomData<T>,
);
