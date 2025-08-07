use tinystr_0_8::{ParseError, TinyAsciiStr};

use crate::{error::DecodeError, flavors::Groto};

array_str!(impl<const N: usize> TinyAsciiStr<N> {
  fn new = || TinyAsciiStr::try_from_raw([0; N]).expect("empty TinyAsciiStr should always be valid");
  fn from_str = |src: &str| {
    TinyAsciiStr::try_from_utf8(src.as_bytes()).map_err(from_parse_error::<N>)
  };
  fn as_bytes = |s: &Self| s.as_bytes();
});

fn from_parse_error<const N: usize>(value: ParseError) -> DecodeError<Groto> {
  match value {
    ParseError::TooLong { .. } => super::super::larger_than_str_capacity::<N>(),
    ParseError::ContainsNull => {
      DecodeError::other("tinystr types do not support strings with null bytes")
    }
    ParseError::NonAscii => {
      DecodeError::other("attempted to construct TinyAsciiStr from a non-ASCII string")
    }
    _ => DecodeError::other("invalid TinyAsciiStr"),
  }
}
