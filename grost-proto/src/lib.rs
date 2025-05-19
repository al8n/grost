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

pub use convert::*;
// pub use select_set::SelectionSet;

/// The flavors of the encoding/decoding
pub mod flavors;

/// The reflection of the Graph protocol buffer types
pub mod reflection;

/// The buffer trait
pub mod buffer;

/// The decoding trait
pub mod decode;
/// The encoding trait
pub mod encode;

/// The indexing related types and traits
pub mod indexer;

/// The selection related types and traits
pub mod selector;

/// Traits for conversions between types.
pub mod convert;

#[macro_use]
mod macros;
mod map;
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
    buffer::*,
    convert::{self, *},
    debug_assert_read_eq,
    debug_assert_write_eq,
    decode::*,
    decode_owned_scalar,
    decoded_state,
    default_wire_format,
    encode::*,
    flavors::{self, RawTag},
    indexer,
    // map::MapSelector,
    network_varint,
    partial_encode_scalar,
    reflection,
    selectable,
    selector::{self, *},
  };
  pub use varing;
}
