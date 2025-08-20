#[allow(unused_macros)]
macro_rules! str_bridge {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? {
    from_str: $from_str: expr;
    as_str: $to_str: expr;
  }), +$(,)?) => {
    $(
      $crate::encode_bridge!(
        $flavor: str {
          $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::groto::LengthDelimited {
            convert: $to_str;
          },
        },
      );

      $crate::default_string_wire_format!(Groto: $ty as $crate::__private::flavors::groto::LengthDelimited);
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
    impl<const $g: ::core::primitive::usize> $crate::__private::Encode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto> for $ty {
      fn encode_raw<WB>(
        &self,
        context: &$crate::__private::flavors::groto::Context,
        buf: impl Into<$crate::__private::buffer::ChunkWriter<WB>>,
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::groto::EncodeError>
      where
        WB: $crate::__private::buffer::ChunkMut,
      {
        <::core::primitive::str as $crate::__private::Encode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto>>::encode_raw(
          self.as_str(),
          context,
          buf,
        )
      }

      fn encoded_raw_len(&self, context: &$crate::__private::flavors::groto::Context) -> ::core::primitive::usize {
        <::core::primitive::str as $crate::__private::Encode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto>>::encoded_raw_len(
          self.as_str(),
          context,
        )
      }

      fn encode<WB>(
        &self,
        context: &$crate::__private::flavors::groto::Context,
        buf: impl Into<$crate::__private::buffer::ChunkWriter<WB>>,
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::groto::EncodeError>
      where
        WB: $crate::__private::buffer::ChunkMut,
      {
        <::core::primitive::str as $crate::__private::Encode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto>>::encode(
          self.as_str(),
          context,
          buf,
        )
      }

      fn encode_length_delimited<WB>(
        &self,
        context: &$crate::__private::flavors::groto::Context,
        buf: impl Into<$crate::__private::buffer::ChunkWriter<WB>>,
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::groto::EncodeError>
      where
        WB: $crate::__private::buffer::ChunkMut,
      {
        <::core::primitive::str as $crate::__private::Encode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto>>::encode_length_delimited(
          self.as_str(),
          context,
          buf,
        )
      }

      #[inline]
      fn encoded_len(
        &self,
        context: &$crate::__private::flavors::groto::Context,
      ) -> ::core::primitive::usize {
        <::core::primitive::str as $crate::__private::Encode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto>>::encoded_len(
          self.as_str(),
          context,
        )
      }

      #[inline]
      fn encoded_length_delimited_len(
        &self,
        context: &$crate::__private::flavors::groto::Context,
      ) -> ::core::primitive::usize {
        <::core::primitive::str as $crate::__private::Encode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto>>::encoded_length_delimited_len(
          self.as_str(),
          context,
        )
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::selection::Selectable<$crate::__private::flavors::Groto> for $ty {
      type Selector = ::core::primitive::bool;
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::convert::PartialIdentity<$crate::__private::flavors::Groto> for $ty {
      fn partial_identity<'a>(
        input: &'a mut Self::Output,
        _: &'a Self::Selector,
      ) -> &'a mut Self {
        input
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialEncode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto> for $ty {
      $crate::partial_encode_scalar!(@impl $crate::__private::flavors::Groto as $crate::__private::flavors::groto::LengthDelimited);
    }

    impl<'de, RB, B, const $g: ::core::primitive::usize> $crate::__private::decode::Decode<'de, $crate::__private::flavors::groto::LengthDelimited, RB, B, $crate::__private::flavors::Groto,> for $ty {
      fn decode(
        _: &'de $crate::__private::flavors::groto::Context,
        mut src: RB,
      ) -> Result<(::core::primitive::usize, Self), $crate::__private::flavors::groto::DecodeError>
      where
        Self: ::core::marker::Sized + 'de,
        RB: $crate::__private::Chunk + 'de,
        B: $crate::__private::UnknownBuffer<RB, $crate::__private::flavors::Groto> + 'de,
      {
        let res = $crate::__private::flavors::groto::impls::decode_str(&mut src)
          .and_then(|(len, s)| {
            $from_str(s).map(|s| (len, s))
          })?;
        src.advance(res.0);
        Ok(res)
      }
    }

    $crate::default_string_wire_format!(
      $crate::__private::flavors::Groto:
        $ty [const N: usize] as $crate::__private::flavors::groto::LengthDelimited;
    );

    $crate::ref_state!(
      @scalar &'a $crate::__private::flavors::Groto:
        $ty [const N: usize] as $crate::__private::flavors::groto::LengthDelimited
    );

    $crate::partial_ref_state!(
      @scalar &'a $crate::__private::flavors::Groto:
        $ty [const N: usize] as $crate::__private::flavors::groto::LengthDelimited
    );

    $crate::partial_state!(
      $crate::__private::flavors::Groto:
        $ty [const N: usize] => $ty
    );

    $crate::flatten_state!($ty [const N: usize]);

    bidi_equivalent!([const N: usize] impl <str, $crate::__private::flavors::groto::LengthDelimited> for <$ty, $crate::__private::flavors::groto::LengthDelimited>);
    bidi_equivalent!(:<RB: $crate::__private::buffer::Chunk>: [const N: usize] impl <$crate::__private::decode::Str<RB>, $crate::__private::flavors::groto::LengthDelimited> for <$ty, $crate::__private::flavors::groto::LengthDelimited>);
  };
}

#[allow(unused)]
#[cfg(not(any(feature = "std", feature = "alloc")))]
pub fn larger_than_str_capacity<const N: usize>() -> crate::flavors::groto::DecodeError {
  crate::flavors::groto::DecodeError::other(
    "cannot decode string with length greater than the capacity",
  )
}

#[allow(unused)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn larger_than_str_capacity<const N: usize>() -> crate::flavors::groto::DecodeError {
  crate::flavors::groto::DecodeError::other(std::format!(
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
