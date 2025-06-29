#![allow(warnings)]

use grost::Object;

#[derive(Object)]
struct OptionalString {
  #[grost(tag = 1, optional(string))]
  #[cfg(any(feature = "std", feature = "alloc"))]
  string: Option<std::string::String>,

  #[grost(tag = 2, optional(string))]
  #[cfg(feature = "smol_str_0_3")]
  smol_str: Option<smol_str_0_3::SmolStr>,

  #[grost(tag = 3, optional(string))]
  #[cfg(feature = "regex_1")]
  regex: Option<regex_1::Regex>,

  #[grost(tag = 4, optional(string))]
  #[cfg(feature = "regex_1")]
  bytes_regex: Option<regex_1::bytes::Regex>,

  #[grost(tag = 5, optional(string))]
  #[cfg(feature = "tinystr_0_8")]
  tinystr: Option<tinystr_0_8::TinyAsciiStr<16>>,

  #[grost(tag = 6, optional(string))]
  #[cfg(feature = "arrayvec_0_7")]
  array_string: Option<arrayvec_0_7::ArrayString<16>>,
}

#[test]
fn compile() {}
