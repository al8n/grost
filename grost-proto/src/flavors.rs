use super::buffer::BytesBuffer;

pub use varing::{DecodeError as DecodeVarintError, EncodeError as EncodeVarintError};

pub use network::Network;

/// The network flavor
pub mod network;

/// The identifier
pub trait Identifier<F: Flavor + ?Sized>: Copy + core::fmt::Debug + core::fmt::Display {
  /// Encode the identifier into a buffer.
  fn encode(&self, dst: &mut [u8]) -> Result<usize, F::EncodeError>;

  /// Return the length of the encoded identifier.
  fn encoded_len(&self) -> usize;

  /// Decode the identifier from a buffer.
  fn decode<B>(buf: B) -> Result<(usize, Self), F::DecodeError>
  where
    B: BytesBuffer + Sized,
    Self: Sized;
}

/// A trait for type-based field selection within types implementing [`Selectable`].
///
/// `Selector` provides mechanisms to include or exclude specific fields or components
/// within a composite type. This is useful for operations like serialization, validation,
/// or any process where you need to specify which parts of a data structure to process.
pub trait Selector: Clone + core::fmt::Debug + Eq {
  /// Creates a selector that includes all possible fields.
  ///
  /// This constant provides a convenient starting point when you want to select
  /// everything and then potentially exclude specific fields.
  const ALL: Self;

  /// Creates a selector that includes no fields.
  ///
  /// This constant provides a convenient starting point when you want to build
  /// a selector by gradually adding only the specific fields you need.
  const NONE: Self;

  /// Inverts the current selection.
  ///
  /// This method flips the selection state of all fields: previously selected fields
  /// become unselected, and previously unselected fields become selected.
  ///
  /// Returns a mutable reference to self for method chaining.
  fn flip(&mut self) -> &mut Self;

  /// Combines this selector with another, typically using a union operation.
  ///
  /// This method incorporates the fields from `other` into the current selector.
  /// The exact behavior depends on the implementing type, but generally results
  /// in a union of the selected fields from both selectors.
  ///
  /// Returns a mutable reference to self for method chaining.
  fn merge(&mut self, other: Self) -> &mut Self;

  /// Creates a new selector by combining this selector with another.
  ///
  /// This is a convenience method that calls `merge` but returns a new selector
  /// rather than modifying the original. This is useful when you want to preserve
  /// the original selector.
  fn merge_into(mut self, other: Self) -> Self {
      self.merge(other);
      self
  }
}

/// A trait for types that can be selected.
pub trait Selectable {
  type Selector: Selector;
}

impl<T> Selectable for &T
where
  T: Selectable + ?Sized,
{
  type Selector = T::Selector;
}

impl<T> Selectable for Option<T>
where
  T: Selectable,
{
  type Selector = T::Selector;
}

/// The wire format used for encoding and decoding.
pub trait WireFormat<F: Flavor + ?Sized>:
  Copy + Eq + core::hash::Hash + core::fmt::Debug + core::fmt::Display + Into<F::WireType>
{
  /// The cooresponding value to the wire type.
  const WIRE_TYPE: F::WireType;
  /// The name of the wire format.
  const NAME: &'static str;
}

/// The default wire format for a type on flavor `F`.
pub trait DefaultWireFormat<F: Flavor + ?Sized>
where
  Self: super::encode::Encode<F, Self::Format>,
{
  /// The default wire format of the type for this flavor.
  type Format: WireFormat<F>;
}

impl<T, F> DefaultWireFormat<F> for &T
where
  T: DefaultWireFormat<F>,
  F: Flavor + ?Sized,
{
  type Format = T::Format;
}

impl<T, F> DefaultWireFormat<F> for Option<T>
where
  T: DefaultWireFormat<F>,
  F: Flavor + ?Sized,
{
  type Format = T::Format;
}

#[cfg(any(feature = "alloc", feature = "std"))]
const _: () = {
  use std::{boxed::Box, rc::Rc, sync::Arc};

  impl<T, F> DefaultWireFormat<F> for Box<T>
  where
    T: DefaultWireFormat<F>,
    F: Flavor + ?Sized,
    Box<T>: super::encode::Encode<F, T::Format>,
  {
    type Format = T::Format;
  }

  impl<T, F> DefaultWireFormat<F> for Rc<T>
  where
    T: DefaultWireFormat<F>,
    F: Flavor + ?Sized,
    Rc<T>: super::encode::Encode<F, T::Format>,
  {
    type Format = T::Format;
  }

  impl<T, F> DefaultWireFormat<F> for Arc<T>
  where
    T: DefaultWireFormat<F>,
    F: Flavor + ?Sized,
    Arc<T>: super::encode::Encode<F, T::Format>,
  {
    type Format = T::Format;
  }
};

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  use triomphe_0_1::Arc;

  impl<T, F> DefaultWireFormat<F> for Arc<T>
  where
    T: DefaultWireFormat<F>,
    F: Flavor + ?Sized,
    Arc<T>: super::encode::Encode<F, T::Format>,
  {
    type Format = T::Format;
  }
};

/// The flavor of the encoding and decoding.
pub trait Flavor: core::fmt::Debug + 'static {
  /// The identifier used for this flavor.
  type Identifier: Identifier<Self>;
  /// The wire type used for this flavor.
  ///
  /// A wire type is typically a sum type of all possible [`WireFormat`]s supported by this flavor.
  type WireType: Copy + Eq + core::hash::Hash + core::fmt::Debug + core::fmt::Display;

  /// The context used for this flavor.
  #[cfg(not(feature = "quickcheck"))]
  type Context;
  /// The context used for this flavor.
  #[cfg(feature = "quickcheck")]
  type Context: quickcheck::Arbitrary;
  /// The unknown value used for this flavor.
  type Unknown<B>;

  /// The encode error for this flavor.
  type EncodeError: core::error::Error + From<super::encode::EncodeError>;
  /// The decode error for this flavor.
  type DecodeError: core::error::Error + From<super::decode::DecodeError>;

  /// The name of the flavor.
  const NAME: &'static str;

  /// Encodes the unknown value into a buffer.
  fn encode_unknown<B>(
    ctx: &Self::Context,
    value: &Self::Unknown<B>,
    buf: &mut [u8],
  ) -> Result<usize, Self::EncodeError>
  where
    B: BytesBuffer;

  /// Decodes an unknown value from a buffer.
  ///
  /// This function is used as a handler for unknown identifiers when decoding
  /// a message. It is called when the identifier is not recognized by the
  /// flavor.
  fn decode_unknown<B>(
    ctx: &Self::Context,
    identifier: Self::Identifier,
    buf: B,
  ) -> Result<(usize, Self::Unknown<B>), Self::DecodeError>
  where
    B: BytesBuffer;
}
