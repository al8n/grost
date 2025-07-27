#![allow(warnings)]

use grost::{Object, flavors::groto::*};

#[derive(Object)]
struct String {
  #[grost(tag = 1, string)]
  #[cfg(any(feature = "std", feature = "alloc"))]
  string: std::string::String,

  #[grost(tag = 2, string)]
  #[cfg(feature = "smol_str_0_3")]
  smol_str: smol_str_0_3::SmolStr,

  #[grost(tag = 3, string)]
  #[cfg(feature = "regex_1")]
  regex: regex_1::Regex,

  #[grost(tag = 4, string)]
  #[cfg(feature = "regex_1")]
  bytes_regex: regex_1::bytes::Regex,

  #[grost(tag = 5, string)]
  #[cfg(feature = "tinystr_0_8")]
  tinystr: tinystr_0_8::TinyAsciiStr<16>,

  #[grost(tag = 6, string)]
  #[cfg(feature = "arrayvec_0_7")]
  array_string: arrayvec_0_7::ArrayString<16>,
}

#[test]
fn compile() {}
