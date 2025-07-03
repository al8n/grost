#![allow(warnings)]

use grost::{Object, flavors::groto::*};

#[derive(Object)]
struct NullableBytes {
  #[grost(tag = 1, nullable(bytes), wire_format = "Nullable<LengthDelimited>")]
  #[cfg(any(feature = "std", feature = "alloc"))]
  vec_bytes: Option<std::vec::Vec<u8>>,

  #[grost(tag = 2, nullable(bytes))]
  #[cfg(feature = "bytes_1")]
  bytes: Option<bytes_1::Bytes>,

  #[grost(tag = 3, nullable(bytes))]
  #[cfg(feature = "bytes_1")]
  bytes_mut: Option<bytes_1::BytesMut>,

  #[grost(tag = 4, nullable(bytes), wire_format = "Nullable<LengthDelimited>")]
  #[cfg(feature = "smallvec_1")]
  smallvec_bytes: Option<smallvec_1::SmallVec<[u8; 24]>>,

  #[grost(tag = 5, nullable(bytes), wire_format = "Nullable<LengthDelimited>")]
  #[cfg(all(feature = "tinyvec_1", any(feature = "std", feature = "alloc")))]
  tinyvec_bytes: Option<tinyvec_1::TinyVec<[u8; 24]>>,

  #[grost(tag = 6, nullable(bytes), wire_format = "Nullable<LengthDelimited>")]
  #[cfg(feature = "tinyvec_1")]
  tinyvec_array_bytes: Option<tinyvec_1::ArrayVec<[u8; 24]>>,

  #[grost(tag = 7, nullable(bytes), wire_format = "Nullable<LengthDelimited>")]
  #[cfg(feature = "arrayvec_0_7")]
  arrayvec_bytes: Option<arrayvec_0_7::ArrayVec<u8, 24>>,
}

#[test]
fn compile() {}
