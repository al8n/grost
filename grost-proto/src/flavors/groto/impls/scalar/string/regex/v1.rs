use regex_1::{Regex, bytes::Regex as BytesRegex};

use crate::{
  buffer::Buf,
  default_string_wire_format,
  flavors::{
    Groto,
    groto::{DecodeError, LengthDelimited},
  },
};

const ERR_MSG: &str = "invalid regex pattern";

macro_rules! try_str_bridge {
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

      $crate::selectable!(@scalar Groto:
        $ty $([ $(const $g: usize),* ])?
      );

      $crate::ref_state!(&'a Groto:
        $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::groto::LengthDelimited => $crate::__private::decode::Str<__GROST_READ_BUF__>,
      );

      $crate::partial_ref_state!(&'a Groto:
        $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::groto::LengthDelimited => $crate::__private::decode::Str<__GROST_READ_BUF__>,
      );

      $crate::partial_state!(Groto:
        $ty $([ $(const $g: usize),* ])? => $ty
      );

      $crate::flatten_state!($ty $([ $(const $g: usize),* ])?);

      bidi_equivalent!(impl <str, $crate::__private::flavors::groto::LengthDelimited> for <$ty, $crate::__private::flavors::groto::LengthDelimited>);
      bidi_equivalent!(:<RB: $crate::__private::buffer::Buf>: impl <$ty, $crate::__private::flavors::groto::LengthDelimited> for <$crate::__private::decode::Str<RB>, $crate::__private::flavors::groto::LengthDelimited>);

      impl<'de, RB, B> $crate::__private::convert::TryFromPartialRef<'de, LengthDelimited, RB, B, Groto> for $ty
      {
        fn try_from_partial_ref(
          _: &'de $crate::__private::flavors::groto::Context,
          input: <Self as $crate::__private::state::State<$crate::__private::state::PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output,
        ) -> Result<Self, DecodeError>
        where
          Self: Sized,
          <Self as $crate::__private::state::State<$crate::__private::state::PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output: Sized,
          RB: $crate::__private::buffer::Buf,
          B: $crate::__private::buffer::UnknownBuffer<RB, $crate::__private::flavors::Groto>,
        {
          <$ty>::new(input.as_ref()).map_err(|_| DecodeError::other(ERR_MSG))
        }
      }

      impl<'de, RB, B> $crate::__private::convert::TryFromRef<'de, LengthDelimited, RB, B, Groto> for $ty
      where
        RB: Buf,
        B: $crate::__private::buffer::UnknownBuffer<RB, $crate::__private::flavors::Groto> + 'de,
      {
        fn try_from_ref(
          _: &$crate::__private::flavors::groto::Context,
          input: <Self as $crate::__private::state::State<$crate::__private::state::Ref<'de, LengthDelimited, RB, B, Groto>>>::Output,
        ) -> Result<Self, DecodeError>
        where
          Self: Sized,
        {
          <$ty>::new(input.as_ref()).map_err(|_| DecodeError::other(ERR_MSG))
        }
      }

      impl<'de, RB, B> $crate::__private::convert::PartialTryFromRef<'de, LengthDelimited, RB, B, Groto> for $ty
      where
        RB: Buf,
        B: $crate::__private::buffer::UnknownBuffer<RB, $crate::__private::flavors::Groto> + 'de,
      {
        fn partial_try_from_ref(
          _: &'de $crate::__private::flavors::groto::Context,
          input: <Self as $crate::__private::state::State<$crate::__private::state::PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output,
          _: &bool,
        ) -> Result<Self, DecodeError>
        where
          Self: Sized,
        {
          <$ty>::new(input.as_ref()).map_err(|_| DecodeError::other(ERR_MSG))
        }
      }

      impl $crate::__private::convert::PartialIdentity<Groto> for $ty {
        fn partial_identity<'a>(input: &'a mut <Self as $crate::__private::state::State<$crate::__private::state::Partial<Groto>>>::Output, _: &'a bool) -> &'a mut Self
        where
          Self: Sized,
        {
          input
        }
      }
    )*
  };
}

try_str_bridge!(
  Groto:
    Regex {
      from_str: |s: &str| {
        Regex::new(s).map_err(|_| DecodeError::other(ERR_MSG))
      };
      as_str: Regex::as_str;
    },
    BytesRegex {
      from_str: |s: &str| {
        BytesRegex::new(s).map_err(|_| DecodeError::other(ERR_MSG))
      };
      as_str: BytesRegex::as_str;
    },
);

default_string_wire_format!(Groto:
  Regex as LengthDelimited;
  BytesRegex as LengthDelimited;
);
