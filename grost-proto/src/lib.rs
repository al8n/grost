#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "bytes_1")]
pub use bytes_1 as bytes;

pub use convert::*;
pub use select_set::SelectionSet;
pub use error::*;
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
mod error;
mod select_set;
mod tag;


/// A trait for types that can be encoded with a specific wire format.
///
/// This trait defines the wire encoding format to be used when serializing a type.
/// It serves as a foundation for serialization traits by specifying how data should
/// be represented on the wire.
pub trait Wirable<F: flavors::Flavor + ?Sized> {
  /// The wire type of the data, which determines how the data is encoded.
  const WIRE_TYPE: F::WireType;
}

/// A trait that defines how to select fields in a struct.
pub trait Selection<F: flavors::Flavor + ?Sized> {
  /// Encodes the selections into the provided buffer.
  fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError>;

  /// Returns the length of the encoded selections.
  fn encoded_len(&self) -> usize;

  /// Decodes the selection set from the provided buffer.
  #[allow(clippy::type_complexity)]
  fn decode<'de, UB, UT>(buf: &'de [u8]) -> Result<(usize, SelectionSet<Self, UT, UB>), DecodeError<F>>
  where
    UB: buffer::Buffer<unknown::Unknown<F, &'de [u8]>> + 'de,
    UT: buffer::Buffer<Tag> + 'de,
    Self: Sized;

  /// Decodes the selection set from the provided owned buffer.
  #[allow(clippy::type_complexity)]
  fn decode_owned<B, UB, UT>(buf: B) -> Result<(usize, SelectionSet<Self, UT, UB>), DecodeError<F>>
  where
    B: buffer::BytesBuffer + 'static,
    UB: buffer::Buffer<unknown::Unknown<F, B>> + 'static,
    UT: buffer::Buffer<Tag> + 'static,
    Self: Sized;
}
