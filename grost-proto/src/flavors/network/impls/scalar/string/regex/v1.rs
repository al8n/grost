const _: () = {
  use regex_1::{Regex, bytes::Regex as BytesRegex};

  use crate::{
    default_wire_format,
    flavors::{
      Network,
      network::{DecodeError, LengthDelimited},
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
          @without_decode_owned $flavor: &'de str {
            $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited {
              convert: $from_str;
            },
          },
        );

        $crate::selectable!(@scalar Network:
          $ty $([ $(const $g: usize),* ])?
        );

        $crate::decoded_state!(&'a Network:
          $ty $([ $(const $g: usize),* ])? as $crate::__private::flavors::network::LengthDelimited => &'a str
        );

        $crate::flatten_state!($ty $([ $(const $g: usize),* ])?);
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
      },
      BytesRegex {
        from_str: |s: &str| {
          BytesRegex::new(s).map_err(|_| DecodeError::custom(ERR_MSG))
        };
        as_str: BytesRegex::as_str;
      },
  );

  default_wire_format!(Network:
    Regex as LengthDelimited;
    BytesRegex as LengthDelimited;
  );
};
