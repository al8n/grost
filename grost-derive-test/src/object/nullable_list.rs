#![allow(warnings)]

use grost::{Object, flavors::groto::*};

#[cfg(any(feature = "std", feature = "alloc"))]
use std::vec::Vec;

#[derive(Object)]
#[cfg(any(feature = "std", feature = "alloc"))]
struct OptionalVecCombinations {
  #[grost(tag = 1, nullable(list(scalar)))]
  nullable_vec_list: Option<Vec<u16>>,

  #[grost(tag = 2, nullable(list(list(scalar))))]
  nullable_vec_vec: Option<Vec<Vec<u16>>>,

  #[grost(tag = 3, nullable(list(nullable(scalar))))]
  nullable_vec_nullable: Option<Vec<Option<u16>>>,

  #[grost(tag = 4, nullable(list(nullable(list(scalar)))))]
  nullable_vec_nullable_vec: Option<Vec<Option<Vec<u16>>>>,

  #[grost(tag = 5, nullable(list(nullable(list(nullable(scalar))))))]
  nullable_vec_nullable_vec_nullable: Option<Vec<Option<Vec<Option<u16>>>>>,
}

#[derive(Object)]
#[cfg(feature = "smallvec_1")]
struct OptionalSmallVecCombinations {
  #[grost(tag = 1, nullable(list(scalar)))]
  nullable_smallvec_list: Option<smallvec_1::SmallVec<[u16; 16]>>,
  #[grost(tag = 2, nullable(list(list(scalar))))]
  nullable_smallvec_vec: Option<smallvec_1::SmallVec<[smallvec_1::SmallVec<[u16; 16]>; 16]>>,
  #[grost(tag = 3, nullable(list(nullable(scalar))))]
  nullable_smallvec_nullable: Option<smallvec_1::SmallVec<[Option<u16>; 16]>>,
  #[grost(tag = 4, nullable(list(nullable(list(scalar)))))]
  nullable_smallvec_nullable_vec:
    Option<smallvec_1::SmallVec<[Option<smallvec_1::SmallVec<[u16; 16]>>; 16]>>,
  #[grost(tag = 5, nullable(list(nullable(list(nullable(scalar))))))]
  nullable_smallvec_nullable_vec_nullable:
    Option<smallvec_1::SmallVec<[Option<smallvec_1::SmallVec<[Option<u16>; 16]>>; 16]>>,
}

#[derive(Object)]
#[cfg(feature = "arrayvec_0_7")]
struct OptionalArrayVecCombinations {
  #[grost(tag = 1, nullable(list(scalar)))]
  nullable_arrayvec_list: Option<arrayvec_0_7::ArrayVec<u16, 16>>,
  #[grost(tag = 2, nullable(list(list(scalar))))]
  nullable_arrayvec_vec: Option<arrayvec_0_7::ArrayVec<arrayvec_0_7::ArrayVec<u16, 16>, 16>>,
  #[grost(tag = 3, nullable(list(nullable(scalar))))]
  nullable_arrayvec_nullable: Option<arrayvec_0_7::ArrayVec<Option<u16>, 16>>,
  #[grost(tag = 4, nullable(list(nullable(list(scalar)))))]
  nullable_arrayvec_nullable_vec:
    Option<arrayvec_0_7::ArrayVec<Option<arrayvec_0_7::ArrayVec<u16, 16>>, 16>>,
  #[grost(tag = 5, nullable(list(nullable(list(nullable(scalar))))))]
  nullable_arrayvec_nullable_vec_nullable:
    Option<arrayvec_0_7::ArrayVec<Option<arrayvec_0_7::ArrayVec<Option<u16>, 16>>, 16>>,
}

#[derive(Object)]
#[cfg(feature = "tinyvec_1")]
struct OptionalTinyVecCombinations {
  #[grost(tag = 1, nullable(list(scalar)))]
  nullable_tinyvec_list: Option<tinyvec_1::TinyVec<[u16; 16]>>,
  #[grost(tag = 2, nullable(list(list(scalar))))]
  nullable_tinyvec_vec: Option<tinyvec_1::TinyVec<[tinyvec_1::TinyVec<[u16; 16]>; 16]>>,
  #[grost(tag = 3, nullable(list(nullable(scalar))))]
  nullable_tinyvec_nullable: Option<tinyvec_1::TinyVec<[Option<u16>; 16]>>,
  #[grost(tag = 4, nullable(list(nullable(list(scalar)))))]
  nullable_tinyvec_nullable_vec:
    Option<tinyvec_1::TinyVec<[Option<tinyvec_1::TinyVec<[u16; 16]>>; 16]>>,
  #[grost(tag = 5, nullable(list(nullable(list(nullable(scalar))))))]
  nullable_tinyvec_nullable_vec_nullable:
    Option<tinyvec_1::TinyVec<[Option<tinyvec_1::TinyVec<[Option<u16>; 16]>>; 16]>>,
}

#[derive(Object)]
#[cfg(feature = "tinyvec_1")]
struct OptionalTinyVecArrayCombinations {
  #[grost(tag = 1, nullable(list(scalar)))]
  nullable_tinyvec_array_list: Option<tinyvec_1::ArrayVec<[u16; 16]>>,
  #[grost(tag = 2, nullable(list(list(scalar))))]
  nullable_tinyvec_array_vec: Option<tinyvec_1::ArrayVec<[tinyvec_1::ArrayVec<[u16; 16]>; 16]>>,
  #[grost(tag = 3, nullable(list(nullable(scalar))))]
  nullable_tinyvec_array_nullable: Option<tinyvec_1::ArrayVec<[Option<u16>; 16]>>,
  #[grost(tag = 4, nullable(list(nullable(list(scalar)))))]
  nullable_tinyvec_array_nullable_vec:
    Option<tinyvec_1::ArrayVec<[Option<tinyvec_1::ArrayVec<[u16; 16]>>; 16]>>,
  #[grost(tag = 5, nullable(list(nullable(list(nullable(scalar))))))]
  nullable_tinyvec_array_nullable_vec_nullable:
    Option<tinyvec_1::ArrayVec<[Option<tinyvec_1::ArrayVec<[Option<u16>; 16]>>; 16]>>,
}

#[test]
fn compile() {}
