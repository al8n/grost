#[allow(unused_macros)]
macro_rules! str_message {
  ($ty:ty => $owned_ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialMessage<$crate::__private::flavors::Network> for $ty {
      type UnknownBuffer<B> = ();

      type Encoded<'a> = &'a str
        where
          Self: Sized + 'a;

      type Borrowed<'a> = &'a Self
        where
          Self: 'a;

      type EncodedOwned = $owned_ty
        where
          Self: Sized + 'static;
    }

    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Message<$crate::__private::flavors::Network> for $ty {
      type Partial = Self;

      type Encoded<'a> = &'a str
        where
          Self: Sized + 'a;

      type Borrowed<'a> = &'a Self
        where
          Self: 'a;

      type EncodedOwned = $owned_ty
        where
          Self: Sized + 'static;
    }
  };
}

#[allow(unused_macros)]
macro_rules! str_bridge {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? {
    from_str: $from_str: expr;
    as_str: $to_str: expr;

    type EncodedOwned = $owned_ty:ty;
  }), +$(,)?) => {
    $(
      $crate::encode_bridge!(
        $flavor: str {
          $ty $([ $(const $g: usize),* ])? {
            convert: $to_str;
          },
        },
      );

      $crate::decode_bridge!(
        $flavor: &'de str {
          $ty $([ $(const $g: usize),* ])? {
            convert: $from_str;
          },
        },
      );

      str_message!($ty => $owned_ty $([ $(const $g: usize),* ])?);
    )*
  };
}

#[allow(unused_macros)]
macro_rules! array_str {
  (
    impl<const $g:ident: usize> $ty:ty {
      fn new = $new:expr;

      fn from_str = $from_str:expr;

      fn as_bytes = $as_bytes:expr;
    }
  ) => {
    impl<const $g: ::core::primitive::usize> $crate::__private::Encode<$crate::__private::flavors::Network> for $ty {
      fn encode(
        &self,
        context: &$crate::__private::flavors::network::Context,
        wire_type: $crate::__private::flavors::network::WireType,
        buf: &mut [::core::primitive::u8],
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
        match wire_type {
          $crate::__private::flavors::network::WireType::Zst if N == 0 => ::core::result::Result::Ok(0),
          $crate::__private::flavors::network::WireType::LengthDelimited => {
            <::core::primitive::str as $crate::__private::Encode<$crate::__private::flavors::Network>>::encode(
              self.as_str(),
              context,
              wire_type,
              buf,
            )
          }
          _ => ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::unsupported_wire_type(
            ::core::any::type_name::<Self>(),
            wire_type,
          )),
        }
      }

      fn encode_length_delimited(
        &self,
        context: &$crate::__private::flavors::network::Context,
        wire_type: $crate::__private::flavors::network::WireType,
        buf: &mut [::core::primitive::u8],
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
        match wire_type {
          $crate::__private::flavors::network::WireType::Zst if N == 0 => ::core::result::Result::Ok(0),
          $crate::__private::flavors::network::WireType::LengthDelimited => {
            <::core::primitive::str as $crate::__private::Encode<$crate::__private::flavors::Network>>::encode_length_delimited(
              self.as_str(),
              context,
              wire_type,
              buf,
            )
          }
          _ => ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::unsupported_wire_type(
            ::core::any::type_name::<Self>(),
            wire_type,
          )),
        }
      }

      #[inline]
      fn encoded_len(
        &self,
        context: &$crate::__private::flavors::network::Context,
        wire_type: $crate::__private::flavors::network::WireType,
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
        match wire_type {
          $crate::__private::flavors::network::WireType::Zst if N == 0 => ::core::result::Result::Ok(0),
          $crate::__private::flavors::network::WireType::LengthDelimited => {
            <::core::primitive::str as $crate::__private::Encode<$crate::__private::flavors::Network>>::encoded_len(
              self.as_str(),
              context,
              wire_type,
            )
          }
          _ => ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::unsupported_wire_type(
            ::core::any::type_name::<Self>(),
            wire_type,
          )),
        }
      }

      #[inline]
      fn encoded_length_delimited_len(
        &self,
        context: &$crate::__private::flavors::network::Context,
        wire_type: $crate::__private::flavors::network::WireType,
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
        match wire_type {
          $crate::__private::flavors::network::WireType::Zst if N == 0 => ::core::result::Result::Ok(0),
          $crate::__private::flavors::network::WireType::LengthDelimited => {
            <::core::primitive::str as $crate::__private::Encode<$crate::__private::flavors::Network>>::encoded_length_delimited_len(
              self.as_str(),
              context,
              wire_type,
            )
          }
          _ => ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::unsupported_wire_type(
            ::core::any::type_name::<Self>(),
            wire_type,
          )),
        }
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialEncode<$crate::__private::flavors::Network> for $ty {
      $crate::partial_encode_primitives!(@impl $crate::__private::flavors::Network);
    }

    impl<'de, const $g: ::core::primitive::usize> $crate::__private::Decode<'de, $crate::__private::flavors::Network, Self> for $ty {
      fn decode<UB>(
        context: &$crate::__private::flavors::network::Context,
        wire_type: $crate::__private::flavors::network::WireType,
        src: &'de [::core::primitive::u8],
      ) -> Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
      where
        Self: ::core::marker::Sized + 'de,
        UB: $crate::__private::Buffer<$crate::__private::network::Unknown<&'de [::core::primitive::u8]>>,
      {
        match wire_type {
          $crate::__private::flavors::network::WireType::Zst if N == 0 => ::core::result::Result::Ok((0, $new())),
          $crate::__private::flavors::network::WireType::LengthDelimited => {
            <::core::primitive::str as $crate::__private::Decode<'de, $crate::__private::flavors::Network, &'de str>>::decode::<()>(context, wire_type, src)
              .and_then(|(len, bytes)| {
                $from_str(bytes).map(|s| (len, s))
              })
          }
          _ => ::core::result::Result::Err($crate::__private::flavors::network::DecodeError::unsupported_wire_type(
            ::core::any::type_name::<Self>(),
            wire_type,
          )),
        }
      }

      fn decode_length_delimited<UB>(
        context: &$crate::__private::flavors::network::Context,
        wire_type: $crate::__private::flavors::network::WireType,
        src: &'de [::core::primitive::u8],
      ) -> Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
      where
        Self: ::core::marker::Sized + 'de,
        UB: $crate::__private::Buffer<$crate::__private::network::Unknown<&'de [::core::primitive::u8]>>,
      {
        match wire_type {
          $crate::__private::flavors::network::WireType::Zst if N == 0 => ::core::result::Result::Ok((0, $new())),
          $crate::__private::flavors::network::WireType::LengthDelimited => {
            <::core::primitive::str as $crate::__private::Decode<'de, $crate::__private::flavors::Network, &'de str>>::decode_length_delimited::<()>(context, wire_type, src)
              .and_then(|(len, bytes)| {
                $from_str(bytes).map(|s| (len, s))
              })
          }
          _ => ::core::result::Result::Err($crate::__private::flavors::network::DecodeError::unsupported_wire_type(
            ::core::any::type_name::<Self>(),
            wire_type,
          )),
        }
      }

    }

    impl<const $g: ::core::primitive::usize> $crate::__private::DecodeOwned<$crate::__private::flavors::Network, Self> for $ty {
      fn decode_owned<B, UB>(
        ctx: &$crate::__private::flavors::network::Context,
        wire_type: $crate::__private::flavors::network::WireType,
        src: B,
      ) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
      where
        Self: ::core::marker::Sized + 'static,
        B: $crate::__private::BytesBuffer + 'static,
        UB: $crate::__private::Buffer<$crate::__private::network::Unknown<B>> + 'static,
      {
        <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, Self>>::decode::<()>(ctx, wire_type, $crate::__private::BytesBuffer::as_bytes(&src))
      }

      fn decode_length_delimited_owned<B, UB>(
        ctx: &$crate::__private::flavors::network::Context,
        wire_type: $crate::__private::flavors::network::WireType,
        src: B,
      ) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
      where
        Self: ::core::marker::Sized + 'static,
        B: $crate::__private::BytesBuffer + 'static,
        UB: $crate::__private::Buffer<$crate::__private::network::Unknown<B>> + 'static,
      {
        <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, Self>>::decode_length_delimited::<()>(ctx, wire_type, $crate::__private::BytesBuffer::as_bytes(&src))
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::IntoTarget<$crate::__private::flavors::Network, Self> for $ty {
      fn into_target(self) -> ::core::result::Result<Self, <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError> {
        ::core::result::Result::Ok(self)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::TypeRef<$crate::__private::flavors::Network, Self> for $ty {
      fn to(&self) -> ::core::result::Result<Self, <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError> {
        ::core::result::Result::Ok(::core::clone::Clone::clone(self))
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::TypeOwned<$crate::__private::flavors::Network, Self> for $ty {
      fn to(&self) -> ::core::result::Result<Self, <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError> {
        ::core::result::Result::Ok(::core::clone::Clone::clone(self))
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::IntoTarget<$crate::__private::flavors::Network, $ty> for &str {
      fn into_target(self) -> ::core::result::Result<$ty, <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError> {
        $from_str(self)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::TypeRef<$crate::__private::flavors::Network, $ty> for &str {
      fn to(&self) -> ::core::result::Result<$ty, <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError> {
        $from_str(*self)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialMessage<$crate::__private::flavors::Network> for $ty {
      type UnknownBuffer<B> = ();

      type Encoded<'a>
        = &'a ::core::primitive::str
      where
        Self: ::core::marker::Sized + 'a;

      type Borrowed<'a>
        = &'a Self
      where
        Self: 'a;

      type EncodedOwned
        = Self
      where
        Self: ::core::marker::Sized + 'static;
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::Message<$crate::__private::flavors::Network> for $ty {
      type Partial = Self;

      type Encoded<'a>
        = &'a ::core::primitive::str
      where
        Self: ::core::marker::Sized + 'a;

      type Borrowed<'a>
        = &'a Self
      where
        Self: 'a;

      type EncodedOwned
        = Self
      where
        Self: ::core::marker::Sized + 'static;
    }
  };
}

#[allow(unused)]
#[cfg(not(any(feature = "std", feature = "alloc")))]
pub fn larger_than_str_capacity<const N: usize>() -> crate::flavors::network::DecodeError {
  crate::flavors::network::DecodeError::custom(
    "cannot decode string with length greater than the capacity",
  )
}

#[allow(unused)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn larger_than_str_capacity<const N: usize>() -> crate::flavors::network::DecodeError {
  crate::flavors::network::DecodeError::custom(std::format!(
    "cannot decode string with length greater than the capacity {N}"
  ))
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[allow(clippy::module_inception)]
mod string;

#[cfg(any(feature = "std", feature = "alloc"))]
mod arc;

#[cfg(any(feature = "std", feature = "alloc"))]
mod rc;

#[cfg(any(feature = "std", feature = "alloc"))]
mod boxed;

#[cfg(any(feature = "std", feature = "alloc"))]
mod smol_str;

mod arrayvec;
mod heapless;
mod tinystr;

#[cfg(any(feature = "std", feature = "alloc"))]
mod triomphe;
