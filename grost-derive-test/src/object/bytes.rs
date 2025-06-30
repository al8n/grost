#![allow(warnings)]

use grost::{Object, flavors::groto::*};

#[derive(Object)]
struct Bytes {
  #[grost(tag = 1, bytes, wire_format = "LengthDelimited")]
  #[cfg(any(feature = "std", feature = "alloc"))]
  vec_bytes: std::vec::Vec<u8>,

  #[grost(tag = 2, bytes)]
  #[cfg(feature = "bytes_1")]
  bytes: bytes_1::Bytes,

  #[grost(tag = 3, bytes)]
  #[cfg(feature = "bytes_1")]
  bytes_mut: bytes_1::BytesMut,

  #[grost(tag = 4, bytes, wire_format = "LengthDelimited")]
  #[cfg(feature = "smallvec_1")]
  smallvec_bytes: smallvec_1::SmallVec<[u8; 24]>,

  #[grost(tag = 5, bytes, wire_format = "LengthDelimited")]
  #[cfg(all(feature = "tinyvec_1", any(feature = "std", feature = "alloc")))]
  tinyvec_bytes: tinyvec_1::TinyVec<[u8; 24]>,

  #[grost(tag = 6, bytes, wire_format = "LengthDelimited")]
  #[cfg(feature = "tinyvec_1")]
  tinyvec_array_bytes: tinyvec_1::ArrayVec<[u8; 24]>,

  #[grost(tag = 7, bytes, wire_format = "LengthDelimited")]
  #[cfg(feature = "arrayvec_0_7")]
  arrayvec_bytes: arrayvec_0_7::ArrayVec<u8, 24>,
}

#[test]
fn compile() {}
