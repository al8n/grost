#[allow(unused_macros)]
macro_rules! bytes_bridge {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?), +$(,)?) => {
    $(
      impl<'a, RB, B, $( $(const $g: usize),* )?> $crate::decode::Decode<'a, $crate::__private::decode::BytesSlice<RB>, $crate::__private::flavors::groto::LengthDelimited, RB, B, $crate::__private::flavors::Groto> for $ty {
        fn decode(context: &'a $crate::__private::flavors::groto::Context, src: RB) -> Result<(usize, $crate::__private::decode::BytesSlice<RB>), $crate::__private::flavors::groto::Error>
        where
          $crate::__private::decode::BytesSlice<RB>: Sized + 'a,
          RB: $crate::buffer::ReadBuf + 'a,
          B: $crate::buffer::Buffer<$crate::__private::flavors::groto::Unknown<RB>> + 'a
        {
          <[u8] as $crate::decode::Decode<'a, $crate::__private::decode::BytesSlice<RB>, $crate::__private::flavors::groto::LengthDelimited, RB, B, $crate::__private::flavors::Groto>>::decode(context, src)
        }
      }
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
    impl<const $g: ::core::primitive::usize> $crate::__private::Encode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto> for $ty {
      fn encode(
        &self,
        context: &$crate::__private::flavors::groto::Context,
        buf: &mut [::core::primitive::u8],
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::groto::Error> {
        <[::core::primitive::u8] as $crate::__private::Encode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto>>::encode(
          self.as_slice(),
          context,
          buf,
        )
      }

      fn encode_length_delimited(
        &self,
        context: &$crate::__private::flavors::groto::Context,
        buf: &mut [::core::primitive::u8],
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::groto::Error> {
        <[::core::primitive::u8] as $crate::__private::Encode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto>>::encode_length_delimited(
          self.as_slice(),
          context,
          buf,
        )
      }

      #[inline]
      fn encoded_len(
        &self,
        context: &$crate::__private::flavors::groto::Context,
      ) -> usize {
        <[::core::primitive::u8] as $crate::__private::Encode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto>>::encoded_len(
          self.as_slice(),
          context,
        )
      }

      #[inline]
      fn encoded_length_delimited_len(
        &self,
        context: &$crate::__private::flavors::groto::Context,
      ) -> usize {
        <[::core::primitive::u8] as $crate::__private::Encode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto>>::encoded_length_delimited_len(
          self.as_slice(),
          context,
        )
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::selection::Selectable<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto> for $ty {
      type Selector = ::core::primitive::bool;
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialEncode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto> for $ty {
      $crate::partial_encode_scalar!(@impl $crate::__private::flavors::Groto as $crate::__private::flavors::groto::LengthDelimited);
    }

    impl<'de, const $g: ::core::primitive::usize> $crate::__private::decode::Decode<'a, $crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto

,
, Self> for $ty {
      fn decode<UB>(
        context: &$crate::__private::flavors::groto::Context,
        src: &'de [::core::primitive::u8],
      ) -> Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Groto as $crate::__private::flavors::Flavor>::Error>
      where
        Self: ::core::marker::Sized + 'de,
        UB: $crate::__private::Buffer<$crate::__private::flavors::groto::Unknown<&'de [::core::primitive::u8]>>,
      {
        <[::core::primitive::u8] as $crate::__private::decode::Decode<'a, $crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto

,
, &'de [::core::primitive::u8]>>::decode::<()>(context, src)
          .and_then(|(len, bytes)| {
            $from_bytes(bytes).map(|s| (len, s))
          })
      }

      fn decode_length_delimited<UB>(
        context: &$crate::__private::flavors::groto::Context,
        src: &'de [::core::primitive::u8],
      ) -> Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Groto as $crate::__private::flavors::Flavor>::Error>
      where
        Self: ::core::marker::Sized + 'de,
        UB: $crate::__private::Buffer<$crate::__private::flavors::groto::Unknown<&'de [::core::primitive::u8]>>,
      {
        <[::core::primitive::u8] as $crate::__private::decode::Decode<'a, $crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto

,
, &'de [::core::primitive::u8]>>::decode_length_delimited::<()>(context, src)
          .and_then(|(len, bytes)| {
            $from_bytes(bytes).map(|s| (len, s))
          })
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::decode::DecodeOwned<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto, Self> for $ty {
      $crate::decode_owned_scalar!(@impl $crate::__private::flavors::Groto as $crate::__private::flavors::groto::LengthDelimited);
    }

    $crate::default_wire_format!(
      $crate::__private::flavors::Groto:
        $ty [const N: usize] as $crate::__private::flavors::groto::LengthDelimited,
    );
    $crate::partial_ref_state!(
      &'a $crate::__private::flavors::Groto:
        $ty [const N: usize] as $crate::__private::flavors::groto::LengthDelimited => &'a [::core::primitive::u8]
    );
    $crate::partial_state!(
      $crate::__private::flavors::Groto:
        $ty [const N: usize] => $ty
    );

    impl<'a, UB, $( $(const $g: usize),* )?> $crate::decode::Decode<'a, $crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto
,
, &'a [u8], UB> for $ty {
      fn decode(context: &$crate::__private::flavors::groto::Context, src: B) -> Result<(usize, &'a [u8]), $crate::__private::flavors::groto::Error>
      where
        &'a [u8]: Sized + 'a,
        B: $crate::buffer::ReadBuf,
        UB: $crate::buffer::Buffer<$crate::__private::flavors::groto::Unknown<B>> + 'a
      {
        <[u8] as $crate::decode::Decode<'a, $crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto
,
, &'a [u8], UB>>::decode(context, src)
      }
    }
    $crate::flatten_state!($ty [const N: usize]);
  };
}

#[allow(unused)]
#[cfg(not(any(feature = "std", feature = "alloc")))]
pub fn larger_than_array_capacity<const N: usize>() -> crate::flavors::groto::Error {
  crate::flavors::groto::Error::custom("cannot decode array with length greater than the capacity")
}

#[allow(unused)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn larger_than_array_capacity<const N: usize>() -> crate::flavors::groto::Error {
  crate::flavors::groto::Error::custom(std::format!(
    "cannot decode array with length greater than the capacity {N}"
  ))
}

mod array;
#[cfg(any(feature = "std", feature = "alloc"))]
#[allow(clippy::module_inception)]
mod bytes;
mod heapless;
