#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use regex_1::{Regex, bytes::Regex as BytesRegex};
  use smol_str_0_3::SmolStr;

  use crate::error::DecodeError;

  str_bridge!(
    Regex {
      from_str: |s: &str| {
        Regex::new(s).map_err(|_| DecodeError::custom("invalid regex pattern"))
      };
      to_str: Regex::as_str;

      type EncodedOwned = SmolStr {
        from_ref: |s: &SmolStr| {
          Regex::new(s.as_str()).map_err(|_| DecodeError::custom("invalid regex pattern"))
        };
        from: |s: SmolStr| {
          Regex::new(s.as_str()).map_err(|_| DecodeError::custom("invalid regex pattern"))
        };
      }
    },
    BytesRegex {
      from_str: |s: &str| {
        BytesRegex::new(s).map_err(|_| DecodeError::custom("invalid regex pattern"))
      };
      to_str: BytesRegex::as_str;

      type EncodedOwned = SmolStr {
        from_ref: |s: &SmolStr| {
          BytesRegex::new(s.as_str()).map_err(|_| DecodeError::custom("invalid regex pattern"))
        };
        from: |s: SmolStr| {
          BytesRegex::new(s.as_str()).map_err(|_| DecodeError::custom("invalid regex pattern"))
        };
      }
    },
  );
};
