use super::buffer::BytesBuffer;

pub use varing::{EncodeError as EncodeVarintError, DecodeError as DecodeVarintError};

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

/// The selector used to select fields for a type which implments [`Selectable`].
pub trait Selector<F: Flavor + ?Sized>:
  Clone
  + core::fmt::Debug
  + core::fmt::Display
  + core::hash::Hash
  + Eq
{}

/// A trait for types that can be selected.
pub trait Selectable<F: Flavor + ?Sized> {
  type Selector: Selector<F>;
}

impl<T, F> Selectable<F> for &T
where
  T: Selectable<F> + ?Sized,
  F: Flavor + ?Sized,
{
  type Selector = T::Selector;
}

impl<T, F> Selectable<F> for Option<T>
where
  T: Selectable<F>,
  F: Flavor + ?Sized,
{
  type Selector = T::Selector;
}

/// The wire format used for encoding and decoding.
pub trait WireFormat<F: Flavor + ?Sized>: Copy + Eq + core::hash::Hash + core::fmt::Debug + core::fmt::Display + Into<F::WireType> {
  /// The cooresponding value to the wire type.
  const WIRE_TYPE: F::WireType;
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
  type EncodeError: core::error::Error;
  /// The decode error for this flavor.
  type DecodeError: core::error::Error;

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
