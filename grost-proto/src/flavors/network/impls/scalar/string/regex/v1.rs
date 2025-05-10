#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use regex_1::{Regex, bytes::Regex as BytesRegex};
  use smol_str_0_3::SmolStr;

  use crate::{
    default_wire_format,
    flavors::{
      Network,
      network::{DecodeError, LengthDelimited},
    },
    into_target, referenceable, type_owned, type_ref,
  };

  const ERR_MSG: &str = "invalid regex pattern";

  macro_rules! try_str_bridge {
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

        $crate::try_decode_bridge!(
          @without_decode_owned $flavor: &'de str {
            $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited {
              convert: $from_str;
            },
          },
        );

        $crate::selectable!(@scalar Network:
          $ty $([ $(const $g: usize),* ])?
        );

        $crate::encoded_state!(&'a Network:
          $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited => &'a str
        );

        str_message!($ty => $owned_ty $([ $(const $g: usize),* ])?);
      )*
    };
  }

  macro_rules! regex_conversion {
    ($($ty:ty),+$(,)?) => {
      $(
        into_target!(Network: SmolStr => $ty {
          |val: SmolStr| <$ty>::new(val.as_str()).map_err(|_| DecodeError::custom(ERR_MSG))
        });
        into_target!(Network: &str => $ty {
          |val: &str| <$ty>::new(val).map_err(|_| DecodeError::custom(ERR_MSG))
        });
        type_ref!(Network: &str => $ty {
          |val: &str| <$ty>::new(val).map_err(|_| DecodeError::custom(ERR_MSG))
        });
        referenceable!(
          Network: $ty as $crate::__private::flavors::network::LengthDelimited => &'a str
        );
        type_owned!( Network: SmolStr => $ty {
          |val: &SmolStr| <$ty>::new(val.as_str()).map_err(|_| DecodeError::custom(ERR_MSG))
        });
      )*
    };
  }

  try_str_bridge!(
    Network:
      Regex {
        from_str: |s: &str| {
          Regex::new(s).map_err(|_| DecodeError::custom(ERR_MSG))
        };
        as_str: Regex::as_str;

        type EncodedOwned = SmolStr;
      },
      BytesRegex {
        from_str: |s: &str| {
          BytesRegex::new(s).map_err(|_| DecodeError::custom(ERR_MSG))
        };
        as_str: BytesRegex::as_str;

        type EncodedOwned = SmolStr;
      },
  );

  regex_conversion!(Regex, BytesRegex,);

  default_wire_format!(Network:
    Regex as LengthDelimited;
    BytesRegex as LengthDelimited;
  );
};
