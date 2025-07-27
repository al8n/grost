#![allow(warnings)]

use grost::Object;

#[derive(Object)]
struct NullableString {
  #[grost(tag = 1, nullable(string))]
  #[cfg(any(feature = "std", feature = "alloc"))]
  string: Option<std::string::String>,

  #[grost(tag = 2, nullable(string))]
  #[cfg(feature = "smol_str_0_3")]
  smol_str: Option<smol_str_0_3::SmolStr>,

  #[grost(tag = 3, nullable(string))]
  #[cfg(feature = "regex_1")]
  regex: Option<regex_1::Regex>,

  #[grost(tag = 4, nullable(string))]
  #[cfg(feature = "regex_1")]
  bytes_regex: Option<regex_1::bytes::Regex>,

  #[grost(tag = 5, nullable(string))]
  #[cfg(feature = "tinystr_0_8")]
  tinystr: Option<tinystr_0_8::TinyAsciiStr<16>>,

  #[grost(tag = 6, nullable(string))]
  #[cfg(feature = "arrayvec_0_7")]
  array_string: Option<arrayvec_0_7::ArrayString<16>>,
}

#[test]
fn compile() {}
