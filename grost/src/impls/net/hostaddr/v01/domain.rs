use hostaddr_0_1::{Buffer, Domain};

use crate::{
  Decode, DecodeError, DecodeOwned, Encode, IntoTarget, Message, PartialEncode, TypeOwned, TypeRef,
  UnknownRefBuffer, Wirable, WireType,
};

impl Wirable for Buffer {}

impl Encode for Buffer {
  #[inline]
  fn encode(&self, buf: &mut [u8]) -> Result<usize, crate::EncodeError> {
    self.as_bytes().encode(buf)
  }

  #[inline]
  fn encoded_len(&self) -> usize {
    self.as_bytes().encoded_len()
  }
}

impl<S> Wirable for Domain<S>
where
  S: Wirable + ?Sized,
{
  const WIRE_TYPE: WireType = S::WIRE_TYPE;
}

impl<S> Encode for Domain<S>
where
  S: Encode,
{
  #[inline]
  fn encode(&self, buf: &mut [u8]) -> Result<usize, crate::EncodeError> {
    self.as_ref().into_inner().encode(buf)
  }

  #[inline]
  fn encoded_len(&self) -> usize {
    self.as_ref().into_inner().encoded_len()
  }
}

impl<S> PartialEncode for Domain<S>
where
  S: Encode,
{
  type Selection = ();

  #[inline]
  fn partial_encode(
    &self,
    buf: &mut [u8],
    _: &Self::Selection,
  ) -> Result<usize, crate::EncodeError> {
    self.encode(buf)
  }

  #[inline]
  fn partial_encoded_len(&self, _: &Self::Selection) -> usize {
    self.encoded_len()
  }
}

impl<'de> Decode<'de> for &'de Domain<str> {
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownRefBuffer<'de>,
  {
    Domain::<[u8]>::try_from_ascii_bytes(src)
      .map(|domain| (src.len(), domain.as_str()))
      .map_err(|e| DecodeError::custom(e.as_str()))
  }
}

impl<'de> Decode<'de> for &'de Domain<[u8]> {
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownRefBuffer<'de>,
  {
    Domain::<[u8]>::try_from_ascii_bytes(src)
      .map(|domain| (src.len(), domain))
      .map_err(|e| DecodeError::custom(e.as_str()))
  }
}

impl<'de> Decode<'de> for Domain<&'de str> {
  fn decode<B>(src: &'de [u8], ub: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownRefBuffer<'de>,
  {
    <&'de Domain<str>>::decode(src, ub).map(|(read, v)| (read, v.as_ref()))
  }
}

impl<'de> Decode<'de> for Domain<&'de [u8]> {
  fn decode<B>(src: &'de [u8], ub: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownRefBuffer<'de>,
  {
    <&'de Domain<[u8]>>::decode(src, ub).map(|(read, v)| (read, v.as_ref()))
  }
}

macro_rules! impl_decode {
  (@both $($ty:ty $([const $g:ident: usize])?),+$(,)?) => {
    impl_decode!($($ty $([const $g: usize])?),+);
    impl_decode!(@owned $($ty $([const $g: usize])?),+);
  };
  ($($ty:ty $([const $g:ident: usize])?),+$(,)?) => {
    $(
      impl<'de, $(const $g: usize)?> Decode<'de> for Domain<$ty> {
        fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
        where
          Self: Sized + 'de,
          B: UnknownRefBuffer<'de>
        {
          Self::try_from(src)
            .map(|domain| (src.len(), domain))
            .map_err(|e| DecodeError::custom(e.as_str()))
        }
      }
    )*
  };
  (@owned $($ty:ty $([const $g:ident: usize])?),+$(,)?) => {
    $(
      impl $(<const $g: usize>)? DecodeOwned for Domain<$ty> {
        #[cfg(any(feature = "std", feature = "alloc"))]
        fn decode_from_bytes<U>(
          src: crate::bytes::Bytes,
          _: &mut U,
        ) -> Result<(usize, Self), DecodeError>
        where
          Self: Sized + 'static,
          U: crate::UnknownBuffer<crate::bytes::Bytes>
        {
          Self::try_from(src.as_ref())
            .map(|domain| (src.len(), domain))
            .map_err(|e| DecodeError::custom(e.as_str()))
        }
      }
    )*
  };
}

macro_rules! impl_message {
  ($($ty:ty $([const $g:ident: usize])?),+$(,)?) => {
    $(
      impl $(<const $g: usize>)? IntoTarget<Domain<$ty>> for Domain<&[u8]> {
        fn into_target(self) -> Result<Domain<$ty>, DecodeError> {
          Domain::<$ty>::try_from(self.into_inner())
            .map_err(|e| DecodeError::custom(e.as_str()))
        }
      }

      impl $(<const $g: usize>)? TypeRef<Domain<$ty>> for Domain<&[u8]> {
        fn to(&self) -> Result<Domain<$ty>, DecodeError> {
          Domain::<$ty>::try_from(self.into_inner())
            .map_err(|e| DecodeError::custom(e.as_str()))
        }
      }

      impl $(<const $g: usize>)? IntoTarget<Domain<$ty>> for Domain<&str> {
        fn into_target(self) -> Result<Domain<$ty>, DecodeError> {
          Domain::<$ty>::try_from(self.into_inner())
            .map_err(|e| DecodeError::custom(e.as_str()))
        }
      }

      impl $(<const $g: usize>)? TypeRef<Domain<$ty>> for Domain<&str> {
        fn to(&self) -> Result<Domain<$ty>, DecodeError> {
          Domain::<$ty>::try_from(self.into_inner())
            .map_err(|e| DecodeError::custom(e.as_str()))
        }
      }

      impl $(<const $g: usize>)? IntoTarget<Domain<$ty>> for Domain<crate::bytes::Bytes> {
        fn into_target(self) -> Result<Domain<$ty>, DecodeError> {
          Domain::<$ty>::try_from(self.into_inner().as_ref())
            .map_err(|e| DecodeError::custom(e.as_str()))
        }
      }

      impl $(<const $g: usize>)? TypeOwned<Domain<$ty>> for Domain<crate::bytes::Bytes> {
        fn to(&self) -> Result<Domain<$ty>, DecodeError> {
          Domain::<$ty>::try_from(self.as_inner().as_ref())
            .map_err(|e| DecodeError::custom(e.as_str()))
        }
      }

      impl $(<const $g: usize>)? Message for Domain<$ty> {
        type Encoded<'a> = Domain<&'a [u8]>
        where
          Self: Sized + 'a;

        type Borrowed<'a> = &'a Self
        where
          Self: 'a;

        type EncodedOwned = Domain<crate::bytes::Bytes>
        where
          Self: Sized + 'static;
      }
    )*
  };
}

impl_decode!(@both Buffer);

impl IntoTarget<Domain<Buffer>> for &Domain<[u8]> {
  fn into_target(self) -> Result<Domain<Buffer>, DecodeError> {
    Ok(self.into())
  }
}

impl IntoTarget<Domain<Buffer>> for &Domain<str> {
  fn into_target(self) -> Result<Domain<Buffer>, DecodeError> {
    Ok(self.into())
  }
}

conversion!(Domain<Buffer>);

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  impl_decode!(@both
    std::string::String,
    std::vec::Vec<u8>,
    std::sync::Arc<[u8]>,
    std::sync::Arc<str>,
    std::boxed::Box<[u8]>,
    std::boxed::Box<str>,
    std::rc::Rc<[u8]>,
    std::rc::Rc<str>,
  );

  impl_message!(
    std::string::String,
    std::sync::Arc<str>,
    std::boxed::Box<str>,
    std::rc::Rc<str>,
    std::vec::Vec<u8>,
    std::sync::Arc<[u8]>,
    std::boxed::Box<[u8]>,
    std::rc::Rc<[u8]>,
  );
};

#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use smol_str_0_3::SmolStr;

  impl_decode!(@both
    SmolStr,
  );

  impl_message!(SmolStr);
};

#[cfg(feature = "bytes_1")]
const _: () = {
  use bytes_1::Bytes;

  impl DecodeOwned for Domain<Bytes> {
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn decode_from_bytes<U>(src: Bytes, _: &mut U) -> Result<(usize, Self), DecodeError>
    where
      Self: Sized + 'static,
      U: crate::UnknownBuffer<Bytes>,
    {
      let len = src.len();
      Self::try_from(src)
        .map(|domain| (len, domain))
        .map_err(|e| DecodeError::custom(e.as_str()))
    }
  }

  impl_decode!(Bytes,);

  conversion!(@clone Domain<Bytes>);
};

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  impl_decode!(@both
    triomphe_0_1::Arc<[u8]>,
    triomphe_0_1::Arc<str>,
  );

  impl_message!(triomphe_0_1::Arc<[u8]>, triomphe_0_1::Arc<str>,);
};

#[cfg(feature = "tinyvec_1")]
const _: () = {
  impl_decode!(@both tinyvec_1::TinyVec<[u8; N]> [const N: usize]);

  impl_message!(
    tinyvec_1::TinyVec<[u8; N]> [const N: usize],
  );
};

#[cfg(feature = "smallvec_1")]
const _: () = {
  impl_decode!(@both smallvec_1::SmallVec<[u8; N]> [const N: usize]);

  impl_message!(
    smallvec_1::SmallVec<[u8; N]> [const N: usize],
  );
};
