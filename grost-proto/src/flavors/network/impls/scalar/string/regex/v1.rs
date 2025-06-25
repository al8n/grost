use regex_1::{Regex, bytes::Regex as BytesRegex};

use crate::{
  default_wire_format,
  flavors::{
    Network,
    network::{Error, LengthDelimited},
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
          $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited {
            convert: $to_str;
          },
        },
      );

      $crate::try_decode_bridge!(
        $flavor: &'de str => $crate::__private::decode::Str<B> {
          $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited {
            convert: |s: $crate::__private::decode::Str<B>| $from_str(s.as_ref());
          },
        },
      );

      $crate::selectable!(@scalar Network:
        $ty $([ $(const $g: usize),* ])?
      );

      $crate::partial_ref_state!(&'a Network:
        $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited => $crate::__private::decode::Str<__GROST_READ_BUF__>
      );

      $crate::partial_state!(Network:
        $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited => $ty
      );

      $crate::flatten_state!($ty $([ $(const $g: usize),* ])?);

      $crate::identity_partial_transform!(
        $flavor {
          $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited,
        }
      );

      $crate::identity_transform!(
        $flavor {
          $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited,
        }
      );

      impl $crate::__private::Transform<Network, LengthDelimited, &str> for $ty {
        fn transform(input: &str) -> Result<Self, <Network as crate::flavors::Flavor>::Error>
        where
          Self: Sized,
        {
          <$ty>::new(input).map_err(|_| Error::custom(ERR_MSG))
        }
      }

      impl $crate::__private::PartialTransform<Network, LengthDelimited, &str> for $ty {
        fn partial_transform(input: &str, selector: &bool) -> Result<Option<Self>, <Network as crate::flavors::Flavor>::Error>
        where
          Self: Sized,
        {
          if *selector {
            <Self as $crate::__private::Transform<Network, LengthDelimited, &str>>::transform(input)
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
  Network:
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

default_wire_format!(Network:
  Regex as LengthDelimited;
  BytesRegex as LengthDelimited;
);
