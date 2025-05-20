#[allow(unused_macros)]
macro_rules! impl_selectable_for_bytes {
  ($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl $(<$(const $g: usize),* >)? $crate::__private::selection::Selectable<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty {
      type Selector = bool;
    }
  };
}

#[allow(unused_macros)]
macro_rules! bytes_bridge {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? {
    from_slice: $from_bytes: expr;
    as_slice: $to_bytes: expr;
  }), +$(,)?) => {
    $(
      $crate::encode_bridge!(
        $flavor: [::core::primitive::u8] {
          $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited {
            convert: $to_bytes;
          },
        },
      );

      $crate::decode_bridge!(
        $flavor: &'de [::core::primitive::u8] {
          $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited {
            convert: $from_bytes;
          },
        },
      );

      impl_selectable_for_bytes!($ty $([ $(const $g: usize),* ])?);
    )*
  };
}

#[allow(unused_macros)]
macro_rules! array_bytes {
  (
    impl<const $g:ident: usize> $ty:ty {
      fn new = $new:expr;

      fn from_bytes = $from_bytes:expr;

      fn as_bytes = $as_bytes:expr;
    }
  ) => {
    impl<const $g: ::core::primitive::usize> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty {
      fn encode(
        &self,
        context: &$crate::__private::flavors::network::Context,
        buf: &mut [::core::primitive::u8],
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
        <[::core::primitive::u8] as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::encode(
          self.as_slice(),
          context,
          buf,
        )
      }

      fn encode_length_delimited(
        &self,
        context: &$crate::__private::flavors::network::Context,
        buf: &mut [::core::primitive::u8],
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
        <[::core::primitive::u8] as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::encode_length_delimited(
          self.as_slice(),
          context,
          buf,
        )
      }

      #[inline]
      fn encoded_len(
        &self,
        context: &$crate::__private::flavors::network::Context,
      ) -> usize {
        <[::core::primitive::u8] as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::encoded_len(
          self.as_slice(),
          context,
        )
      }

      #[inline]
      fn encoded_length_delimited_len(
        &self,
        context: &$crate::__private::flavors::network::Context,
      ) -> usize {
        <[::core::primitive::u8] as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::encoded_length_delimited_len(
          self.as_slice(),
          context,
        )
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::selection::Selectable<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty {
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
        <[::core::primitive::u8] as $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, &'de [::core::primitive::u8]>>::decode::<()>(context, src)
          .and_then(|(len, bytes)| {
            $from_bytes(bytes).map(|s| (len, s))
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
        <[::core::primitive::u8] as $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, &'de [::core::primitive::u8]>>::decode_length_delimited::<()>(context, src)
          .and_then(|(len, bytes)| {
            $from_bytes(bytes).map(|s| (len, s))
          })
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::DecodeOwned<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, Self> for $ty {
      $crate::decode_owned_scalar!(@impl $crate::__private::flavors::Network as $crate::__private::flavors::network::LengthDelimited);
    }

    $crate::default_wire_format!(
      $crate::__private::flavors::Network:
        $ty [const N: usize] as $crate::__private::flavors::network::LengthDelimited,
    );
    $crate::decoded_state!(
      &'a $crate::__private::flavors::Network:
        $ty [const N: usize] as $crate::__private::flavors::network::LengthDelimited => &'a [::core::primitive::u8]
    );
    $crate::flatten_state!($ty [const N: usize]);
  };
}

#[allow(unused)]
#[cfg(not(any(feature = "std", feature = "alloc")))]
pub fn larger_than_array_capacity<const N: usize>() -> crate::flavors::network::DecodeError {
  crate::flavors::network::DecodeError::custom(
    "cannot decode array with length greater than the capacity",
  )
}

#[allow(unused)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn larger_than_array_capacity<const N: usize>() -> crate::flavors::network::DecodeError {
  crate::flavors::network::DecodeError::custom(std::format!(
    "cannot decode array with length greater than the capacity {N}"
  ))
}

mod array;
#[cfg(any(feature = "std", feature = "alloc"))]
#[allow(clippy::module_inception)]
mod bytes;
mod heapless;
mod bstr;
mod tinyvec;
