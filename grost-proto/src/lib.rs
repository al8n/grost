#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "bytes_1")]
pub use bytes_1 as bytes;

#[cfg(feature = "smol_str_0_3")]
pub use smol_str_0_3 as smol_str;

#[cfg(feature = "quickcheck")]
pub use quickcheck;

// pub use select_set::SelectionSet;

/// The flavors of the encoding/decoding
pub mod flavors;

/// The identifier related types and traits
pub mod identifier;

/// The reflection of the Graph protocol buffer types
pub mod reflection;

/// The buffer trait
pub mod buffer;

/// The decoding trait
pub mod decode;
/// The encoding trait
pub mod encode;
/// The error types
pub mod error;

/// The indexing related types and traits
pub mod indexer;

/// The selection related types and traits
pub mod selection;

/// Traits for conversions between types.
pub mod convert;

/// Marker types
pub mod marker;

/// Common types and traits for list-like structures
pub mod list;

/// Common types for map-like structures
pub mod map;

/// Common types for set structures
pub mod set;

/// Common types for objects
pub mod object;

/// States
pub mod state;

/// The unknown data
pub mod unknown;

#[macro_use]
mod macros;
mod select_set;
mod utils;

#[doc(hidden)]
#[cfg(debug_assertions)]
#[inline]
pub fn debug_assert_write_eq<T: ?Sized>(actual: usize, expected: usize) {
  debug_assert_eq!(
    actual,
    expected,
    "{}: expect writting {expected} bytes, but actual write {actual} bytes",
    core::any::type_name::<T>()
  );
}

#[doc(hidden)]
#[cfg(debug_assertions)]
#[inline]
pub fn debug_assert_read_eq<T: ?Sized>(actual: usize, expected: usize) {
  debug_assert_eq!(
    actual,
    expected,
    "{}: expect reading {expected} bytes, but actual read {actual} bytes",
    core::any::type_name::<T>()
  );
}

#[doc(hidden)]
pub mod __private {
  pub use super::{
    buffer::{self, *},
    convert::{self, *},
    debug_assert_read_eq, debug_assert_write_eq,
    decode::{self, *},
    encode::{self, *},
    error::{self, *},
    identifier,
    flavors, groto_varint, indexer, marker, partial_encode_scalar, partial_ref_state, reflection,
    selectable,
    selection::{self, *},
    state,
  };
  pub use either;
  pub use paste;
  pub use varing;

  #[cfg(not(any(feature = "std", feature = "alloc")))]
  pub fn larger_than_array_capacity<F, const N: usize>() -> Error<F>
  where
    F: flavors::Flavor + ?Sized,
  {
    Error::custom("cannot decode array with length greater than the capacity")
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  pub fn larger_than_array_capacity<F, const N: usize>() -> Error<F>
  where
    F: flavors::Flavor + ?Sized,
  {
    Error::custom(std::format!(
      "cannot decode array with length greater than the capacity {N}"
    ))
  }
}
