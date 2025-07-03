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

  #[grost(tag = 3, list(nullable(scalar)))]
  #[cfg(any(feature = "std", feature = "alloc"))]
  vec_nullable: Vec<Option<u16>>,

  #[grost(tag = 4, list(nullable(list(scalar))))]
  #[cfg(any(feature = "std", feature = "alloc"))]
  vec_nullable_vec: Vec<Option<Vec<u16>>>,

  #[grost(tag = 5, list(nullable(list(nullable(scalar)))))]
  #[cfg(any(feature = "std", feature = "alloc"))]
  vec_nullable_vec_nullable: Vec<Option<Vec<Option<u16>>>>,
}

#[derive(Object)]
#[cfg(feature = "smallvec_1")]
struct SmallVecCombinations {
  #[grost(tag = 1, list(scalar))]
  smallvec: smallvec_1::SmallVec<[u16; 16]>,
  #[grost(tag = 2, list(list(scalar)))]
  smallvec_smallvec: smallvec_1::SmallVec<[smallvec_1::SmallVec<[u16; 16]>; 16]>,
  #[grost(tag = 3, list(nullable(scalar)))]
  smallvec_nullable: smallvec_1::SmallVec<[Option<u16>; 16]>,
  #[grost(tag = 4, list(nullable(list(scalar))))]
  smallvec_vec_nullable_smallvec:
    smallvec_1::SmallVec<[Option<smallvec_1::SmallVec<[u16; 16]>>; 16]>,
  #[grost(tag = 5, list(nullable(list(nullable(scalar)))))]
  smallvec_nullable_smallvec_nullable:
    smallvec_1::SmallVec<[Option<smallvec_1::SmallVec<[Option<u16>; 16]>>; 16]>,
}

#[derive(Object)]
#[cfg(feature = "arrayvec_0_7")]
struct ArrayVecCombinations {
  #[grost(tag = 1, list(scalar))]
  arrayvec: arrayvec_0_7::ArrayVec<u16, 16>,
  #[grost(tag = 2, list(list(scalar)))]
  arrayvec_arrayvec: arrayvec_0_7::ArrayVec<arrayvec_0_7::ArrayVec<u16, 16>, 16>,
  #[grost(tag = 3, list(nullable(scalar)))]
  arrayvec_nullable: arrayvec_0_7::ArrayVec<Option<u16>, 16>,
  #[grost(tag = 4, list(nullable(list(scalar))))]
  arrayvec_nullable_vec: arrayvec_0_7::ArrayVec<Option<arrayvec_0_7::ArrayVec<u16, 16>>, 16>,
  #[grost(tag = 5, list(nullable(list(nullable(scalar)))))]
  arrayvec_nullable_vec_nullable:
    arrayvec_0_7::ArrayVec<Option<arrayvec_0_7::ArrayVec<Option<u16>, 16>>, 16>,
}

#[derive(Object)]
#[cfg(feature = "tinyvec_1")]
struct TinyVecCombinations {
  #[grost(tag = 1, list(scalar))]
  tinyvec: tinyvec_1::TinyVec<[u16; 16]>,
  #[grost(tag = 2, list(list(scalar)))]
  tinyvec_tinyvec: tinyvec_1::TinyVec<[tinyvec_1::TinyVec<[u16; 16]>; 16]>,
  #[grost(tag = 3, list(nullable(scalar)))]
  tinyvec_nullable: tinyvec_1::TinyVec<[Option<u16>; 16]>,
  #[grost(tag = 4, list(nullable(list(scalar))))]
  tinyvec_nullable_vec: tinyvec_1::TinyVec<[Option<tinyvec_1::TinyVec<[u16; 16]>>; 16]>,
  #[grost(tag = 5, list(nullable(list(nullable(scalar)))))]
  tinyvec_nullable_vec_nullable:
    tinyvec_1::TinyVec<[Option<tinyvec_1::TinyVec<[Option<u16>; 16]>>; 16]>,
}

#[derive(Object)]
#[cfg(feature = "tinyvec_1")]
struct TinyVecArrayCombinations {
  #[grost(tag = 1, list(scalar))]
  tinyvec_array: tinyvec_1::ArrayVec<[u16; 16]>,
  #[grost(tag = 2, list(list(scalar)))]
  tinyvec_array_vec: tinyvec_1::ArrayVec<[tinyvec_1::ArrayVec<[u16; 16]>; 16]>,
  #[grost(tag = 3, list(nullable(scalar)))]
  tinyvec_array_nullable: tinyvec_1::ArrayVec<[Option<u16>; 16]>,
  #[grost(tag = 4, list(nullable(list(scalar))))]
  tinyvec_array_nullable_vec: tinyvec_1::ArrayVec<[Option<tinyvec_1::ArrayVec<[u16; 16]>>; 16]>,
  #[grost(tag = 5, list(nullable(list(nullable(scalar)))))]
  tinyvec_array_nullable_vec_nullable:
    tinyvec_1::ArrayVec<[Option<tinyvec_1::ArrayVec<[Option<u16>; 16]>>; 16]>,
}

#[test]
fn compile() {}
