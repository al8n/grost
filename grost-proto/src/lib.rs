#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "bytes_1")]
pub use bytes_1 as bytes;

pub use convert::*;
pub use tag::Tag;

/// The flavors of the encoding/decoding
pub mod flavors;

/// The reflection of the Graph protocol buffer types
pub mod reflection;

/// The unknown data types
pub mod unknown;

/// The buffer trait
pub mod buffer;

/// The encoding trait
pub mod encode;

/// The decoding trait
pub mod decode;

mod convert;
mod tag;

/// A trait for types that can be encoded with a specific wire format.
///
/// This trait defines the wire encoding format to be used when serializing a type.
/// It serves as a foundation for serialization traits by specifying how data should
/// be represented on the wire.
pub trait Wirable<F: flavors::Flavor> {
  /// The wire type of the data, which determines how the data is encoded.
  const WIRE_TYPE: F::WireType;
}
