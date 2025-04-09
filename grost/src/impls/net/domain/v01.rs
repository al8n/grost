use hostaddr_0_1::{Domain, Buffer};

use crate::{Decode, DecodeError, DecodeOwned, Encode, IntoTarget, Message, PartialEncode, TypeOwned, TypeRef, UnknownRefBuffer, Wirable, WireType};

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
  fn partial_encode(&self, _: &Self::Selection, buf: &mut [u8]) -> Result<usize, crate::EncodeError> {
    self.encode(buf)
  }

  #[inline]
  fn partial_encoded_len(&self, _: &Self::Selection,) -> usize {
    self.encoded_len()
  }
}

impl<'de> Decode<'de> for &'de Domain<str> {
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownRefBuffer<'de>
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
    B: UnknownRefBuffer<'de>
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
    B: UnknownRefBuffer<'de>
  {
    <&'de Domain::<str>>::decode(src, ub).map(|(read, v)| (read, v.as_ref()))
  }
}

impl<'de> Decode<'de> for Domain<&'de [u8]> {
  fn decode<B>(src: &'de [u8], ub: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: UnknownRefBuffer<'de>
  {
    <&'de Domain::<[u8]>>::decode(src, ub).map(|(read, v)| (read, v.as_ref()))
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
  (@str $($ty:ty $([const $g:ident: usize])?),+$(,)?) => {
    $(
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

      impl $(<const $g: usize>)? IntoTarget<Domain<$ty>> for crate::smol_str::SmolStr {
        fn into_target(self) -> Result<Domain<$ty>, DecodeError> {
          Domain::<$ty>::try_from(self.as_str())
            .map_err(|e| DecodeError::custom(e.as_str()))
        }
      }
      
      impl $(<const $g: usize>)? TypeOwned<Domain<$ty>> for crate::smol_str::SmolStr {
        fn to(&self) -> Result<Domain<$ty>, DecodeError> {
          Domain::<$ty>::try_from(self.as_str())
            .map_err(|e| DecodeError::custom(e.as_str()))
        }
      }
      
      impl $(<const $g: usize>)? Message for Domain<$ty> {
        type Encoded<'a> = Domain<&'a str>
        where
          Self: Sized + 'a;
      
        type Borrowed<'a> = &'a Self
        where
          Self: 'a;
      
        type EncodedOwned = crate::smol_str::SmolStr
        where
          Self: Sized + 'static;
      }
    )*
  };
  (@bytes $($ty:ty $([const $g:ident: usize])?),+$(,)?) => {
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

      impl $(<const $g: usize>)? IntoTarget<Domain<$ty>> for crate::bytes::Bytes {
        fn into_target(self) -> Result<Domain<$ty>, DecodeError> {
          Domain::<$ty>::try_from(self.as_ref())
            .map_err(|e| DecodeError::custom(e.as_str()))
        }
      }
      
      impl $(<const $g: usize>)? TypeOwned<Domain<$ty>> for crate::bytes::Bytes {
        fn to(&self) -> Result<Domain<$ty>, DecodeError> {
          Domain::<$ty>::try_from(self.as_ref())
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
      
        type EncodedOwned = crate::bytes::Bytes
        where
          Self: Sized + 'static;
      }
    )*
  };
}

impl_decode!(@both Buffer);

impl IntoTarget<Domain<Buffer>> for &[u8] {
  fn into_target(self) -> Result<Domain<Buffer>, DecodeError> {
    Domain::<Buffer>::try_from(self)
      .map_err(|e| DecodeError::custom(e.as_str()))
  }
}

impl IntoTarget<Domain<Buffer>> for &str {
  fn into_target(self) -> Result<Domain<Buffer>, DecodeError> {
    Domain::<Buffer>::try_from(self)
      .map_err(|e| DecodeError::custom(e.as_str()))
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

  impl_message!(@str
    std::string::String,
    std::sync::Arc<str>,
    std::boxed::Box<str>,
    std::rc::Rc<str>,
  );

  impl_message!(@bytes
    std::vec::Vec<u8>,
    std::sync::Arc<[u8]>,
    std::boxed::Box<[u8]>,
    std::rc::Rc<[u8]>,
  );

  impl IntoTarget<Domain<Buffer>> for crate::smol_str::SmolStr {
    fn into_target(self) -> Result<Domain<Buffer>, DecodeError> {
      Domain::<Buffer>::try_from(self.as_str())
        .map_err(|e| DecodeError::custom(e.as_str()))
    }
  }
  
  impl IntoTarget<Domain<Buffer>> for crate::bytes::Bytes {
    fn into_target(self) -> Result<Domain<Buffer>, DecodeError> {
      Domain::<Buffer>::try_from(self.as_ref())
        .map_err(|e| DecodeError::custom(e.as_str()))
    }
  }
};

#[cfg(feature = "smol_str_0_3")]
const _: () = {
  impl_decode!(@both
    smol_str_0_3::SmolStr,
  );

  impl_message!(@str
    smol_str_0_3::SmolStr,
  );
};

#[cfg(feature = "bytes_1")]
const _: () = {
  impl_decode!(@both
    bytes_1::Bytes,
  );

  impl_message!(@bytes
    bytes_1::Bytes,
  );
};

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  impl_decode!(@both
    triomphe_0_1::Arc<[u8]>,
    triomphe_0_1::Arc<str>,
  );

  impl_message!(@bytes
    triomphe_0_1::Arc<[u8]>,
  );

  impl_message!(@str
    triomphe_0_1::Arc<str>,
  );
};

#[cfg(feature = "tinyvec_1")]
const _: () = {
  impl_decode!(@both tinyvec_1::TinyVec<[u8; N]> [const N: usize]);

  impl_message!(@bytes
    tinyvec_1::TinyVec<[u8; N]> [const N: usize],
  );
};

#[cfg(feature = "smallvec_1")]
const _: () = {
  impl_decode!(@both smallvec_1::SmallVec<[u8; N]> [const N: usize]);

  impl_message!(@bytes
    smallvec_1::SmallVec<[u8; N]> [const N: usize],
  );
};
