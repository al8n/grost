#[allow(unused_macros)]
macro_rules! str_bridge {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? {
    from_str: $from_str: expr;
    as_str: $to_str: expr;
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
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::Error> {
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
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::Error> {
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

    impl<W: ?Sized, const $g: ::core::primitive::usize> $crate::__private::selection::Selectable<$crate::__private::flavors::Network, W> for $ty {
      type Selector = ::core::primitive::bool;
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty {
      $crate::partial_encode_scalar!(@impl $crate::__private::flavors::Network as $crate::__private::flavors::network::LengthDelimited);
    }

    impl<'de, UB, const $g: ::core::primitive::usize> $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, Self, UB> for $ty {
      fn decode<B>(
        context: &$crate::__private::flavors::network::Context,
        src: B,
      ) -> Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::Error>
      where
        Self: ::core::marker::Sized + 'de,
        B: $crate::__private::Buf<'de>,
        UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'de,
      {
        <::core::primitive::str as $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, &'de str, UB>>::decode(context, src)
          .and_then(|(len, bytes)| {
            $from_str(bytes).map(|s| (len, s))
          })
      }
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
  };
}

#[allow(unused)]
#[cfg(not(any(feature = "std", feature = "alloc")))]
pub fn larger_than_str_capacity<const N: usize>() -> crate::flavors::network::Error {
  crate::flavors::network::Error::custom(
    "cannot decode string with length greater than the capacity",
  )
}

#[allow(unused)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn larger_than_str_capacity<const N: usize>() -> crate::flavors::network::Error {
  crate::flavors::network::Error::custom(std::format!(
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
