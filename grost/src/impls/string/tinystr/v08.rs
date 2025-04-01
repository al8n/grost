use tinystr_0_8::{ParseError, TinyAsciiStr};

use crate::DecodeError;

array_str!(impl<const N: usize> TinyAsciiStr<N> {
  fn new = || TinyAsciiStr::try_from_raw([0; N]).expect("empty TinyAsciiStr should always be valid");
  fn from_str = |src: &str| {
    TinyAsciiStr::try_from_utf8(src.as_bytes()).map_err(from_parse_error::<N>)
  };
  fn decode = |src: &[u8]| {
    TinyAsciiStr::try_from_utf8(src).map(|s| (src.len(), s)).map_err(from_parse_error::<N>)
  };
  fn as_bytes = |s: &Self| s.as_bytes();
});

fn from_parse_error<const N: usize>(value: ParseError) -> DecodeError {
  match value {
    ParseError::TooLong { .. } => crate::__private::larger_than_str_capacity::<N>(),
    ParseError::ContainsNull => {
      DecodeError::custom("tinystr types do not support strings with null bytes")
    }
    ParseError::NonAscii => {
      DecodeError::custom("attempted to construct TinyAsciiStr from a non-ASCII string")
    }
    _ => DecodeError::custom("invalid TinyAsciiStr"),
  }
}
