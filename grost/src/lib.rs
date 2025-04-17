#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

pub use buffer::Buffer;
pub use context::Context;
pub use decode::*;
pub use encode::*;
pub use grost_types::{FieldInfo, EnumInfo, EnumVariantInfo, Identifier, StructInfo, Tag, WireType, skip};
pub use impls::*;
// pub use selection_set::SelectionSet;
pub use unknown::*;

use error::{DecodeError, EncodeError};

#[cfg(feature = "bytes_1")]
pub use bytes_1 as bytes;

#[cfg(feature = "smol_str_0_3")]
pub use smol_str_0_3 as smol_str;

#[cfg(feature = "tinyvec_1")]
pub use tinyvec_1 as tinyvec;

mod buffer;
mod context;
mod decode;
mod encode;

/// The error module contains all the error types used in the `Grost`.
pub mod error;
#[macro_use]
mod macros;
/// Traits implemented for primitive types and common types.
mod impls;
mod selection_set;
mod unknown;
mod utils;

pub trait PartialMessage {
  type UnknownBuffer<B: ?Sized>: UnknownBuffer<B>;

  /// A encoded representation of this type with lifetime 'a.
  ///
  /// This type can be converted back to the original type and decoded from raw bytes.
  type Encoded<'a>: Copy + TypeRef<Self> + Encode + Decode<'a, Self::Encoded<'a>>
  where
    Self: Sized + 'a;

  /// A borrowed view of this type with lifetime 'a.
  ///
  /// This type provides a non-owned view that can be created from a reference
  /// and encoded when needed.
  type Borrowed<'a>: Copy + TypeBorrowed<'a, Self> + Encode
  where
    Self: 'a;

  /// An owned encoded representation of this type.
  type EncodedOwned: Clone + TypeOwned<Self> + Encode + Decode<'static, Self::EncodedOwned>
  where
    Self: Sized + 'static;
}

/// A message type that can be encoded and decoded.
///
/// This trait defines how output types can be encoded, decoded,
/// borrowed, and converted between different representations.
///
/// * `Encoded<'a>` - A encoded representation with lifetime 'a
/// * `Borrowed<'a>` - A borrowed view with lifetime 'a
/// * `EncodedOwned` - An owned encoded representation
pub trait Message: Encode {
  /// The partial type of this message.
  type Partial: PartialMessage;

  /// A encoded representation of this type with lifetime 'a.
  ///
  /// This type can be converted back to the original type and decoded from raw bytes.
  type Encoded<'a>: Copy + TypeRef<Self> + Encode + Decode<'a, Self::Encoded<'a>>
  where
    Self: Sized + 'a;

  /// A borrowed view of this type with lifetime 'a.
  ///
  /// This type provides a non-owned view that can be created from a reference
  /// and encoded when needed.
  type Borrowed<'a>: Copy + TypeBorrowed<'a, Self> + Encode
  where
    Self: 'a;

  /// An owned encoded representation of this type.
  type EncodedOwned: Clone + TypeOwned<Self> + Encode + Decode<'static, Self::EncodedOwned>
  where
    Self: Sized + 'static;
}

/// A trait for consuming `Self` and converting it to `T`.
pub trait IntoTarget<T> {
  /// Consumes this type and converts it to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn into_target(self) -> Result<T, DecodeError>;
}

/// A trait for types that can be converted to another type.
///
/// This trait enables bidirectional conversion between encoded
/// representations and their corresponding decoded types.
///
/// * `T` - The target type to convert to
pub trait TypeRef<T>: IntoTarget<T> {
  /// Converts a reference of this type to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn to(&self) -> Result<T, DecodeError>;
}

/// A trait for types that can be converted to another type.
///
/// This trait enables bidirectional conversion between encoded
/// representations and their corresponding decoded types.
///
/// * `T` - The target type to convert to
pub trait TypeBorrowed<'a, T: ?Sized> {
  /// Converts a reference of this type to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn from_borrow(val: &'a T) -> Self;
}

impl<'a, T: ?Sized> TypeBorrowed<'a, T> for &'a T {
  fn from_borrow(val: &'a T) -> Self {
    val
  }
}

/// A trait for types that can be converted to another type.
///
/// This trait enables bidirectional conversion between encoded
/// representations and their corresponding decoded types.
///
/// * `T` - The target type to convert to
pub trait TypeOwned<T>: IntoTarget<T> {
  /// Converts a reference of this type to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn to(&self) -> Result<T, DecodeError>;
}

/// A trait for types that can be encoded with a specific wire format.
///
/// This trait defines the wire encoding format to be used when serializing a type.
/// It serves as a foundation for serialization traits by specifying how data should
/// be represented on the wire.
pub trait Wirable {
  /// The wire type of the data, which determines how the data is encoded.
  ///
  /// Defaults to [`WireType::LengthDelimited`].
  const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

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
  pub use super::*;
  pub use either;
  pub use error::*;
  pub use grost_types::*;
  pub use varing;

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

  pub use utils::network;

  #[cfg(not(any(feature = "std", feature = "alloc")))]
  pub fn larger_than_str_capacity<const N: usize>() -> crate::DecodeError {
    crate::DecodeError::custom("cannot decode string with length greater than the capacity")
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  pub fn larger_than_str_capacity<const N: usize>() -> crate::DecodeError {
    crate::DecodeError::custom(std::format!(
      "cannot decode string with length greater than the capacity {N}"
    ))
  }

  #[cfg(not(any(feature = "std", feature = "alloc")))]
  pub fn larger_than_array_capacity<const N: usize>() -> crate::DecodeError {
    crate::DecodeError::custom("cannot decode array with length greater than the capacity")
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  pub fn larger_than_array_capacity<const N: usize>() -> crate::DecodeError {
    crate::DecodeError::custom(std::format!(
      "cannot decode array with length greater than the capacity {N}"
    ))
  }
}
