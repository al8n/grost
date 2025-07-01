#![allow(warnings)]

use grost::{Object, flavors::groto::*};

#[cfg(any(feature = "std", feature = "alloc"))]
use std::vec::Vec;

#[derive(Object)]
#[cfg(any(feature = "std", feature = "alloc"))]
struct OptionalVecCombinations {
  #[grost(tag = 1, optional(list(scalar)))]
  optional_vec_list: Option<Vec<u16>>,

  #[grost(tag = 2, optional(list(list(scalar))))]
  optional_vec_vec: Option<Vec<Vec<u16>>>,

  #[grost(tag = 3, optional(list(optional(scalar))))]
  optional_vec_optional: Option<Vec<Option<u16>>>,

  #[grost(tag = 4, optional(list(optional(list(scalar)))))]
  optional_vec_optional_vec: Option<Vec<Option<Vec<u16>>>>,

  #[grost(tag = 5, optional(list(optional(list(optional(scalar))))))]
  optional_vec_optional_vec_optional: Option<Vec<Option<Vec<Option<u16>>>>>,
}

#[derive(Object)]
#[cfg(feature = "smallvec_1")]
struct OptionalSmallVecCombinations {
  #[grost(tag = 1, optional(list(scalar)))]
  optional_smallvec_list: Option<smallvec_1::SmallVec<[u16; 16]>>,
  #[grost(tag = 2, optional(list(list(scalar))))]
  optional_smallvec_vec: Option<smallvec_1::SmallVec<[smallvec_1::SmallVec<[u16; 16]>; 16]>>,
  #[grost(tag = 3, optional(list(optional(scalar))))]
  optional_smallvec_optional: Option<smallvec_1::SmallVec<[Option<u16>; 16]>>,
  #[grost(tag = 4, optional(list(optional(list(scalar)))))]
  optional_smallvec_optional_vec:
    Option<smallvec_1::SmallVec<[Option<smallvec_1::SmallVec<[u16; 16]>>; 16]>>,
  #[grost(tag = 5, optional(list(optional(list(optional(scalar))))))]
  optional_smallvec_optional_vec_optional:
    Option<smallvec_1::SmallVec<[Option<smallvec_1::SmallVec<[Option<u16>; 16]>>; 16]>>,
}

#[derive(Object)]
#[cfg(feature = "arrayvec_0_7")]
struct OptionalArrayVecCombinations {
  #[grost(tag = 1, optional(list(scalar)))]
  optional_arrayvec_list: Option<arrayvec_0_7::ArrayVec<u16, 16>>,
  #[grost(tag = 2, optional(list(list(scalar))))]
  optional_arrayvec_vec: Option<arrayvec_0_7::ArrayVec<arrayvec_0_7::ArrayVec<u16, 16>, 16>>,
  #[grost(tag = 3, optional(list(optional(scalar))))]
  optional_arrayvec_optional: Option<arrayvec_0_7::ArrayVec<Option<u16>, 16>>,
  #[grost(tag = 4, optional(list(optional(list(scalar)))))]
  optional_arrayvec_optional_vec:
    Option<arrayvec_0_7::ArrayVec<Option<arrayvec_0_7::ArrayVec<u16, 16>>, 16>>,
  #[grost(tag = 5, optional(list(optional(list(optional(scalar))))))]
  optional_arrayvec_optional_vec_optional:
    Option<arrayvec_0_7::ArrayVec<Option<arrayvec_0_7::ArrayVec<Option<u16>, 16>>, 16>>,
}

#[derive(Object)]
#[cfg(feature = "tinyvec_1")]
struct OptionalTinyVecCombinations {
  #[grost(tag = 1, optional(list(scalar)))]
  optional_tinyvec_list: Option<tinyvec_1::TinyVec<[u16; 16]>>,
  #[grost(tag = 2, optional(list(list(scalar))))]
  optional_tinyvec_vec: Option<tinyvec_1::TinyVec<[tinyvec_1::TinyVec<[u16; 16]>; 16]>>,
  #[grost(tag = 3, optional(list(optional(scalar))))]
  optional_tinyvec_optional: Option<tinyvec_1::TinyVec<[Option<u16>; 16]>>,
  #[grost(tag = 4, optional(list(optional(list(scalar)))))]
  optional_tinyvec_optional_vec:
    Option<tinyvec_1::TinyVec<[Option<tinyvec_1::TinyVec<[u16; 16]>>; 16]>>,
  #[grost(tag = 5, optional(list(optional(list(optional(scalar))))))]
  optional_tinyvec_optional_vec_optional:
    Option<tinyvec_1::TinyVec<[Option<tinyvec_1::TinyVec<[Option<u16>; 16]>>; 16]>>,
}

#[derive(Object)]
#[cfg(feature = "tinyvec_1")]
struct OptionalTinyVecArrayCombinations {
  #[grost(tag = 1, optional(list(scalar)))]
  optional_tinyvec_array_list: Option<tinyvec_1::ArrayVec<[u16; 16]>>,
  #[grost(tag = 2, optional(list(list(scalar))))]
  optional_tinyvec_array_vec: Option<tinyvec_1::ArrayVec<[tinyvec_1::ArrayVec<[u16; 16]>; 16]>>,
  #[grost(tag = 3, optional(list(optional(scalar))))]
  optional_tinyvec_array_optional: Option<tinyvec_1::ArrayVec<[Option<u16>; 16]>>,
  #[grost(tag = 4, optional(list(optional(list(scalar)))))]
  optional_tinyvec_array_optional_vec:
    Option<tinyvec_1::ArrayVec<[Option<tinyvec_1::ArrayVec<[u16; 16]>>; 16]>>,
  #[grost(tag = 5, optional(list(optional(list(optional(scalar))))))]
  optional_tinyvec_array_optional_vec_optional:
    Option<tinyvec_1::ArrayVec<[Option<tinyvec_1::ArrayVec<[Option<u16>; 16]>>; 16]>>,
}

#[test]
fn compile() {}
