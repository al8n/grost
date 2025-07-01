use regex_1::{Regex, bytes::Regex as BytesRegex};

use crate::{
  buffer::ReadBuf,
  decode::Str,
  default_wire_format,
  flavors::{
    Groto,
    groto::{Error, LengthDelimited},
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

      $crate::try_decode_bridge!(
        $flavor: &'de str => $crate::__private::decode::Str<RB> {
          $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::groto::LengthDelimited {
            convert: |s: $crate::__private::decode::Str<RB>| $from_str(s.as_ref());
          },
        },
      );

      impl<'a, RB, B> $crate::__private::decode::Decode<'a, $crate::__private::decode::Str<RB>, $crate::__private::flavors::groto::LengthDelimited, RB, B, $crate::__private::flavors::Groto> for $ty {
        fn decode(
          context: &'a <$crate::__private::flavors::Groto as $crate::flavors::Flavor>::Context,
          src: RB,
        ) -> ::core::result::Result<(usize, $crate::__private::decode::Str<RB>), <$crate::__private::flavors::Groto as $crate::flavors::Flavor>::Error>
        where
          $crate::__private::decode::Str<B>: Sized + 'a,
          RB: $crate::buffer::ReadBuf + 'a,
          B: $crate::buffer::Buffer<<$crate::__private::flavors::Groto as $crate::flavors::Flavor>::Unknown<RB>> + 'a,
        {
          <&str as $crate::__private::decode::Decode<'a, $crate::__private::decode::Str<RB>,$crate::__private::flavors::groto::LengthDelimited, RB, B, $crate::__private::flavors::Groto>>::decode(context, src)
        }
      }

      $crate::selectable!(@scalar Groto:
        $ty $([ $(const $g: usize),* ])?
      );

      $crate::partial_ref_state!(&'a Groto:
        $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::groto::LengthDelimited => $crate::__private::decode::Str<__GROST_READ_BUF__>,
      );

      $crate::partial_state!(Groto:
        $ty $([ $(const $g: usize),* ])? => $ty
      );

      $crate::flatten_state!($ty $([ $(const $g: usize),* ])?);

      identity_partial_transform!(
        $flavor {
          $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::groto::LengthDelimited,
        }
      );

      $crate::groto_identity_transform!(
        $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::groto::LengthDelimited,
      );

      impl $crate::__private::convert::Transform<&str, Self, LengthDelimited, Groto> for $ty {
        fn transform(input: &str) -> Result<Self, <Groto as crate::flavors::Flavor>::Error>
        where
          Self: Sized,
        {
          <$ty>::new(input).map_err(|_| Error::custom(ERR_MSG))
        }
      }

      impl $crate::__private::convert::PartialTransform<&str, Option<$ty>, LengthDelimited, Groto> for $ty {
        fn partial_transform(input: &str, selector: &bool) -> Result<Option<Self>, <Groto as crate::flavors::Flavor>::Error>
        where
          Self: Sized,
        {
          if *selector {
            <Self as $crate::__private::convert::Transform<&str, Self, LengthDelimited, Groto>>::transform(input)
              .map(Some)
          } else {
            Ok(None)
          }
        }
      }

      impl<B: ReadBuf> $crate::__private::convert::Transform<Str<B>, Self, LengthDelimited, Groto> for $ty {
        fn transform(input: Str<B>) -> Result<Self, <Groto as crate::flavors::Flavor>::Error>
        where
          Self: Sized,
        {
          <$ty>::new(input.as_ref()).map_err(|_| Error::custom(ERR_MSG))
        }
      }

      impl<B: ReadBuf> $crate::__private::convert::PartialTransform<Str<B>, Option<$ty>, LengthDelimited, Groto> for $ty {
        fn partial_transform(input: Str<B>, selector: &bool) -> Result<Option<Self>, <Groto as crate::flavors::Flavor>::Error>
        where
          Self: Sized,
        {
          if *selector {
            <Self as $crate::__private::convert::Transform<Str<B>, Self, LengthDelimited, Groto>>::transform(input)
              .map(Some)
          } else {
            Ok(None)
          }
        }
      }
    )*
  };
}

try_str_bridge!(
  Groto:
    Regex {
      from_str: |s: &str| {
        Regex::new(s).map_err(|_| Error::custom(ERR_MSG))
      };
      as_str: Regex::as_str;
    },
    BytesRegex {
      from_str: |s: &str| {
        BytesRegex::new(s).map_err(|_| Error::custom(ERR_MSG))
      };
      as_str: BytesRegex::as_str;
    },
);

default_wire_format!(Groto:
  Regex as LengthDelimited;
  BytesRegex as LengthDelimited;
);
