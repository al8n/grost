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
        $flavor: &'de str => $crate::__private::decode::Str<B> {
          $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited {
            convert: |s: $crate::__private::decode::Str<B>| $from_str(s.as_ref());
          },
        },
      );
      $crate::default_wire_format!(Network: $ty as $crate::__private::flavors::network::LengthDelimited);

      impl<'a, B, UB, $( $(const $g: usize),* )?> $crate::decode::Decode<'a, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, $crate::__private::decode::Str<B>, B, UB> for $ty {
        fn decode(context: &'a $crate::__private::flavors::network::Context, src: B) -> Result<(usize, $crate::__private::decode::Str<B>), $crate::__private::flavors::network::Error>
        where
          $crate::__private::decode::Str<B>: Sized + 'a,
          B: $crate::buffer::ReadBuf,
          UB: $crate::buffer::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'a
        {
          <str as $crate::decode::Decode<'a, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, $crate::__private::decode::Str<B>, B, UB>>::decode(context, src)
        }
      }
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

    impl<const $g: ::core::primitive::usize> $crate::__private::selection::Selectable<$crate::__private::flavors::Network> for $ty {
      type Selector = ::core::primitive::bool;
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty {
      $crate::partial_encode_scalar!(@impl $crate::__private::flavors::Network as $crate::__private::flavors::network::LengthDelimited);
    }

    impl<'de, B, UB, const $g: ::core::primitive::usize> $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, Self, B, UB> for $ty {
      fn decode(
        context: &'de $crate::__private::flavors::network::Context,
        src: B,
      ) -> Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::Error>
      where
        Self: ::core::marker::Sized + 'de,
        B: $crate::__private::ReadBuf + 'de,
        UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'de,
      {
        <::core::primitive::str as $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, $crate::__private::decode::Str<B>, B, UB>>::decode(context, src)
          .and_then(|(len, bytes)| {
            $from_str(bytes.as_ref()).map(|s| (len, s))
          })
      }
    }

    $crate::default_wire_format!(
      $crate::__private::flavors::Network:
        $ty [const N: usize] as $crate::__private::flavors::network::LengthDelimited;
    );

    $crate::partial_ref_state!(
      &'a $crate::__private::flavors::Network:
        $ty [const N: usize] as $crate::__private::flavors::network::LengthDelimited => $crate::__private::decode::Str<__GROST_READ_BUF__>
    );

    $crate::partial_state!(
      $crate::__private::flavors::Network:
        $ty [const N: usize] as $crate::__private::flavors::network::LengthDelimited => $ty
    );

    $crate::flatten_state!($ty [const N: usize]);

    $crate::identity_transform!(
      $crate::__private::flavors::Network {
        $ty [const N: usize] as $crate::__private::flavors::network::LengthDelimited
      }
    );

    $crate::identity_partial_transform!(
      $crate::__private::flavors::Network {
        $ty [const N: usize] as $crate::__private::flavors::network::LengthDelimited
      }
    );

    impl<const $g: ::core::primitive::usize> $crate::__private::Transform<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, &str> for $ty {
      fn transform(input: &str) -> ::core::result::Result<Self, <$crate::__private::flavors::Network as crate::flavors::Flavor>::Error>
      where
        Self: Sized,
      {
        $from_str(input)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialTransform<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, &str> for $ty {
      fn partial_transform(input: &str, selector: &bool) -> ::core::result::Result<::core::option::Option<Self>, <$crate::__private::flavors::Network as crate::flavors::Flavor>::Error>
      where
        Self: Sized,
      {
        if *selector {
          <Self as $crate::__private::Transform<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, &str>>::transform(input)
            .map(Some)
        } else {
          Ok(None)
        }
      }
    }
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
mod smol_str;

mod arrayvec;
mod heapless;
#[cfg(any(feature = "std", feature = "alloc"))]
mod regex;
mod tinystr;
