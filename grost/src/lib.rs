#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

// pub use decode::*;
// pub use encode::*;
pub use flavors::Flavor;
pub use grost_proto::{
  IntoTarget, Message, PartialMessage, Tag, TypeBorrowed, TypeOwned,
  TypeRef, buffer,
  decode::{Decode, DecodeOwned},
  encode::{Encode, PartialEncode},
  reflection,
};
// pub use impls::*;
// pub use selection_set::SelectionSet;

#[cfg(feature = "bytes_1")]
pub use bytes_1 as bytes;

#[cfg(feature = "smol_str_0_3")]
pub use smol_str_0_3 as smol_str;

#[cfg(feature = "tinyvec_1")]
pub use tinyvec_1 as tinyvec;

#[macro_use]
mod macros;

/// The flavors of the encoding/decoding
pub mod flavors;
mod selection_set;

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
  pub use bitflags;
  pub use either;
  pub use grost_proto::__private::*;
  pub use varing;

  #[cfg(feature = "bnum_0_13")]
  pub use bnum_0_13 as bnum;
  #[cfg(feature = "bytes")]
  pub use bytes_1 as bytes;
  #[cfg(feature = "smol_str")]
  pub use smol_str_0_3 as smol_str;

  #[cfg(not(feature = "simdutf8"))]
  pub use ::core::str::from_utf8;
  #[cfg(feature = "simdutf8")]
  pub use simdutf8::basic::from_utf8;

  #[cfg(feature = "arbitrary")]
  pub use arbitrary;
  #[cfg(feature = "quickcheck")]
  pub use quickcheck;

  pub use memchr;
  pub use thiserror;
}
