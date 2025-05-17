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
    message,
    network_varint,
    partial_encode_scalar,
    reflection,
    selectable,
    selector::{self, *},
  };
  pub use varing;
}

// /// A trait that defines how to select fields in a struct.
// pub trait Selection<F: flavors::Flavor + ?Sized> {
//   /// Encodes the selections into the provided buffer.
//   fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError>;

//   /// Returns the length of the encoded selections.
//   fn encoded_len(&self) -> usize;

//   /// Decodes the selection set from the provided buffer.
//   #[allow(clippy::type_complexity)]
//   fn decode<'de, UB, UT>(buf: &'de [u8]) -> Result<(usize, SelectionSet<Self, UT, UB>), DecodeError<F>>
//   where
//     UB: buffer::Buffer<unknown::Unknown<F, &'de [u8]>> + 'de,
//     UT: buffer::Buffer<Tag> + 'de,
//     Self: Sized;

//   /// Decodes the selection set from the provided owned buffer.
//   #[allow(clippy::type_complexity)]
//   fn decode_owned<B, UB, UT>(buf: B) -> Result<(usize, SelectionSet<Self, UT, UB>), DecodeError<F>>
//   where
//     B: buffer::BytesBuffer + 'static,
//     UB: buffer::Buffer<unknown::Unknown<F, B>> + 'static,
//     UT: buffer::Buffer<Tag> + 'static,
//     Self: Sized;
// }
