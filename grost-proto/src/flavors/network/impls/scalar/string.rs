#[allow(unused_macros)]
macro_rules! str_message {
  ($ty:ty => $owned_ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialMessage<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty {
      type UnknownBuffer<B> = ();

      type Decoded<'a> = &'a str
        where
          Self: Sized + 'a;

      type Borrowed<'a> = &'a Self
        where
          Self: 'a;

      type EncodedOwned = $owned_ty
        where
          Self: Sized + 'static;
    }

    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Message<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty {
      type Partial = Self;

      type Decoded<'a> = &'a str
        where
          Self: Sized + 'a;

      type Borrowed<'a> = &'a Self
        where
          Self: 'a;

      type EncodedOwned = $owned_ty
        where
          Self: Sized + 'static;
    }

    $crate::type_reflection! {
      $crate::__private::flavors::Network:
        $ty => $crate::__private::reflection::Type::string(),
    }
  };
  (@without_type_reflection $ty:ty => $owned_ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialMessage<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty {
      type UnknownBuffer<B> = ();

      type Decoded<'a> = &'a str
        where
          Self: Sized + 'a;

      type Borrowed<'a> = &'a Self
        where
          Self: 'a;

      type EncodedOwned = $owned_ty
        where
          Self: Sized + 'static;
    }

    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Message<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty {
      type Partial = Self;

      type Decoded<'a> = &'a str
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
          $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited {
            convert: $to_str;
          },
        },
      );

      $crate::decode_bridge!(
        $flavor: &'de str {
          $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited {
            convert: $from_str;
          },
        },
      );

      str_message!($ty => $owned_ty $([ $(const $g: usize),* ])?);

      $crate::default_wire_format!(Network: $ty as $crate::__private::flavors::network::LengthDelimited);
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
    impl<const $g: ::core::primitive::usize> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty {
      fn encode(
        &self,
        context: &$crate::__private::flavors::network::Context,
        buf: &mut [::core::primitive::u8],
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
        <::core::primitive::str as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::encode(
          self.as_str(),
          context,
          buf,
        )
      }

      fn encode_length_delimited(
        &self,
        context: &$crate::__private::flavors::network::Context,
        buf: &mut [::core::primitive::u8],
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
        <::core::primitive::str as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::encode_length_delimited(
          self.as_str(),
          context,
          buf,
        )
      }

      #[inline]
      fn encoded_len(
        &self,
        context: &$crate::__private::flavors::network::Context,
      ) -> ::core::primitive::usize {
        <::core::primitive::str as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::encoded_len(
          self.as_str(),
          context,
        )
      }

      #[inline]
      fn encoded_length_delimited_len(
        &self,
        context: &$crate::__private::flavors::network::Context,
      ) -> ::core::primitive::usize {
        <::core::primitive::str as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::encoded_length_delimited_len(
          self.as_str(),
          context,
        )
      }
    }

    impl<W: ?Sized, const $g: ::core::primitive::usize> $crate::__private::Selectable<$crate::__private::flavors::Network, W> for $ty {
      type Selector = ::core::primitive::bool;
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty {
      $crate::partial_encode_scalar!(@impl $crate::__private::flavors::Network as $crate::__private::flavors::network::LengthDelimited);
    }

    impl<'de, const $g: ::core::primitive::usize> $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, Self> for $ty {
      fn decode<UB>(
        context: &$crate::__private::flavors::network::Context,
        src: &'de [::core::primitive::u8],
      ) -> Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
      where
        Self: ::core::marker::Sized + 'de,
        UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<&'de [::core::primitive::u8]>>,
      {
        <::core::primitive::str as $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, &'de str>>::decode::<()>(context, src)
          .and_then(|(len, bytes)| {
            $from_str(bytes).map(|s| (len, s))
          })
      }

      fn decode_length_delimited<UB>(
        context: &$crate::__private::flavors::network::Context,
        src: &'de [::core::primitive::u8],
      ) -> Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
      where
        Self: ::core::marker::Sized + 'de,
        UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<&'de [::core::primitive::u8]>>,
      {
        <::core::primitive::str as $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, &'de str>>::decode_length_delimited::<()>(context, src)
          .and_then(|(len, bytes)| {
            $from_str(bytes).map(|s| (len, s))
          })
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::DecodeOwned<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, Self> for $ty {
      fn decode_owned<B, UB>(
        ctx: &$crate::__private::flavors::network::Context,
        src: B,
      ) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
      where
        Self: ::core::marker::Sized + 'static,
        B: $crate::__private::BytesBuffer + 'static,
        UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'static,
      {
        <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, Self>>::decode::<()>(ctx, $crate::__private::BytesBuffer::as_bytes(&src))
      }

      fn decode_length_delimited_owned<B, UB>(
        ctx: &$crate::__private::flavors::network::Context,
        src: B,
      ) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
      where
        Self: ::core::marker::Sized + 'static,
        B: $crate::__private::BytesBuffer + 'static,
        UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'static,
      {
        <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, Self>>::decode_length_delimited::<()>(ctx, $crate::__private::BytesBuffer::as_bytes(&src))
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

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialMessage<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty {
      type UnknownBuffer<B> = ();

      type Decoded<'a>
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

    impl<const $g: ::core::primitive::usize> $crate::__private::Message<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty {
      type Partial = Self;

      type Decoded<'a>
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

    $crate::default_wire_format!(
      $crate::__private::flavors::Network:
        $ty [const N: usize] as $crate::__private::flavors::network::LengthDelimited;
    );

    $crate::decoded_state!(
      &'a $crate::__private::flavors::Network:
        $ty [const N: usize] as $crate::__private::flavors::network::LengthDelimited => &'a str
    );

    $crate::flatten_state!($ty [const N: usize]);

    $crate::type_reflection! {
      $crate::__private::flavors::Network:
        $ty [const N: usize] => $crate::__private::reflection::Type::string(),
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
#[cfg(any(feature = "std", feature = "alloc"))]
mod regex;
mod tinystr;
#[cfg(any(feature = "std", feature = "alloc"))]
mod triomphe;
