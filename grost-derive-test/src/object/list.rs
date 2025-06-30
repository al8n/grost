#![allow(warnings)]

use grost::Object;

#[cfg(any(feature = "std", feature = "alloc"))]
use std::vec::Vec;

#[derive(Object)]
#[cfg(any(feature = "std", feature = "alloc"))]
struct VecCombinations {
  #[grost(tag = 1, list(scalar))]
  #[cfg(any(feature = "std", feature = "alloc"))]
  vec: Vec<u16>,

  #[grost(tag = 2, list(list(scalar)))]
  #[cfg(any(feature = "std", feature = "alloc"))]
  vec_vec: Vec<Vec<u16>>,

  #[grost(tag = 3, list(optional(scalar)))]
  #[cfg(any(feature = "std", feature = "alloc"))]
  vec_optional: Vec<Option<u16>>,

  #[grost(tag = 4, list(optional(list(scalar))))]
  #[cfg(any(feature = "std", feature = "alloc"))]
  vec_optional_vec: Vec<Option<Vec<u16>>>,

  #[grost(tag = 5, list(optional(list(optional(scalar)))))]
  #[cfg(any(feature = "std", feature = "alloc"))]
  vec_optional_vec_optional: Vec<Option<Vec<Option<u16>>>>,
}

#[derive(Object)]
#[cfg(feature = "smallvec_1")]
struct SmallVecCombinations {
  #[grost(tag = 1, list(scalar))]
  smallvec: smallvec_1::SmallVec<[u16; 16]>,
  #[grost(tag = 2, list(list(scalar)))]
  smallvec_smallvec: smallvec_1::SmallVec<[smallvec_1::SmallVec<[u16; 16]>; 16]>,
  #[grost(tag = 3, list(optional(scalar)))]
  smallvec_optional: smallvec_1::SmallVec<[Option<u16>; 16]>,
  #[grost(tag = 4, list(optional(list(scalar))))]
  smallvec_vec_optional_smallvec: smallvec_1::SmallVec<
    [Option<smallvec_1::SmallVec<[u16; 16]>>; 16],
  >,
  #[grost(tag = 5, list(optional(list(optional(scalar)))))]
  smallvec_optional_smallvec_optional: smallvec_1::SmallVec<
    [Option<smallvec_1::SmallVec<[Option<u16>; 16]>>; 16],
  >,
}

#[derive(Object)]
#[cfg(feature = "arrayvec_0_7")]
struct ArrayVecCombinations {
  #[grost(tag = 1, list(scalar))]
  arrayvec: arrayvec_0_7::ArrayVec<u16, 16>,
  #[grost(tag = 2, list(list(scalar)))]
  arrayvec_arrayvec: arrayvec_0_7::ArrayVec<arrayvec_0_7::ArrayVec<u16, 16>, 16>,
  #[grost(tag = 3, list(optional(scalar)))]
  arrayvec_optional: arrayvec_0_7::ArrayVec<Option<u16>, 16>,
  #[grost(tag = 4, list(optional(list(scalar))))]
  arrayvec_optional_vec: arrayvec_0_7::ArrayVec<
    Option<arrayvec_0_7::ArrayVec<u16, 16>>,
    16,
  >,
  #[grost(tag = 5, list(optional(list(optional(scalar)))))]
  arrayvec_optional_vec_optional: arrayvec_0_7::ArrayVec<
    Option<arrayvec_0_7::ArrayVec<Option<u16>, 16>>,
    16,
  >,
}

#[derive(Object)]
#[cfg(feature = "tinyvec_1")]
struct TinyVecCombinations {
  #[grost(tag = 1, list(scalar))]
  tinyvec: tinyvec_1::TinyVec<[u16; 16]>,
  #[grost(tag = 2, list(list(scalar)))]
  tinyvec_tinyvec: tinyvec_1::TinyVec<[tinyvec_1::TinyVec<[u16; 16]>; 16]>,
  #[grost(tag = 3, list(optional(scalar)))]
  tinyvec_optional: tinyvec_1::TinyVec<[Option<u16>; 16]>,
  #[grost(tag = 4, list(optional(list(scalar))))]
  tinyvec_optional_vec: tinyvec_1::TinyVec<[Option<tinyvec_1::TinyVec<[u16; 16]>>; 16]>,
  #[grost(tag = 5, list(optional(list(optional(scalar)))))]
  tinyvec_optional_vec_optional: tinyvec_1::TinyVec<[Option<tinyvec_1::TinyVec<[Option<u16>; 16]>>; 16]>,
}

#[derive(Object)]
#[cfg(feature = "tinyvec_1")]
struct TinyVecArrayCombinations {
  #[grost(tag = 1, list(scalar))]
  tinyvec_array: tinyvec_1::ArrayVec<[u16; 16]>,
  #[grost(tag = 2, list(list(scalar)))]
  tinyvec_array_vec: tinyvec_1::ArrayVec<[tinyvec_1::ArrayVec<[u16; 16]>; 16]>,
  #[grost(tag = 3, list(optional(scalar)))]
  tinyvec_array_optional: tinyvec_1::ArrayVec<[Option<u16>; 16]>,
  #[grost(tag = 4, list(optional(list(scalar))))]
  tinyvec_array_optional_vec: tinyvec_1::ArrayVec<[Option<tinyvec_1::ArrayVec<[u16; 16]>>; 16]>,
  #[grost(tag = 5, list(optional(list(optional(scalar)))))]
  tinyvec_array_optional_vec_optional: tinyvec_1::ArrayVec<[Option<tinyvec_1::ArrayVec<[Option<u16>; 16]>>; 16]>,
}

#[test]
fn compile() {}
