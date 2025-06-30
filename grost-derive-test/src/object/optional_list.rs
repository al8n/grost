use grost::{Object, flavors::groto::*};

#[cfg(any(feature = "std", feature = "alloc"))]
use std::vec::Vec;

#[derive(Object)]
#[cfg(any(feature = "std", feature = "alloc"))]
struct OptionalVec {
  #[grost(tag = 1, optional(list(scalar)))]
  #[cfg(any(feature = "std", feature = "alloc"))]
  optional_vec_list: Option<Vec<u16>>,
  
  #[grost(tag = 2, optional(list(list(scalar))))]
  #[cfg(any(feature = "std", feature = "alloc"))]
  optional_vec_vec: Option<Vec<Vec<u16>>>,

  #[grost(tag = 3, optional(list(optional(scalar))))]
  #[cfg(any(feature = "std", feature = "alloc"))]
  optional_vec_optional: Option<Vec<Option<u16>>>,

  // #[grost(tag = 4, optional(list(optional(list(scalar)))))]
  // #[cfg(any(feature = "std", feature = "alloc"))]
  // optional_vec_optional_vec: Option<Vec<Option<Vec<u16>>>>,

  // #[grost(tag = 5, optional(list(optional(list(optional(scalar))))))]
  // #[cfg(any(feature = "std", feature = "alloc"))]
  // optional_vec_optional_vec_optional: Option<Vec<Option<Vec<Option<u16>>>>>,
}

