use super::buffer::BytesBuffer;

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

/// The wire format used for encoding and decoding.
pub trait WireFormat: Copy + Eq + core::hash::Hash + core::fmt::Debug + core::fmt::Display {}

/// The default wire format for a type on flavor `F`.
pub trait DefaultWireFormat<F: Flavor + ?Sized>
where
  Self: super::encode::Encode<F, Self::Format>,
{
  /// The default wire format of the type for this flavor.
  type Format: WireFormat;

  /// The value of the default wire format.
  const VALUE: Self::Format;
}

impl<T, F> DefaultWireFormat<F> for &T
where
  T: DefaultWireFormat<F>,
  F: Flavor + ?Sized,
{
  type Format = T::Format;
  const VALUE: Self::Format = T::VALUE;
}

impl<T, F> DefaultWireFormat<F> for Option<T>
where
  T: DefaultWireFormat<F>,
  F: Flavor + ?Sized,
{
  type Format = T::Format;
  const VALUE: Self::Format = T::VALUE;
}

#[cfg(any(feature = "alloc", feature = "std"))]
const _: () = {
  use std::{sync::Arc, boxed::Box, rc::Rc,};

  impl<T, F> DefaultWireFormat<F> for Box<T>
  where
    T: DefaultWireFormat<F>,
    F: Flavor + ?Sized,
    Box<T>: super::encode::Encode<F, T::Format>,
  {
    type Format = T::Format;
    const VALUE: Self::Format = T::VALUE;
  }

  impl<T, F> DefaultWireFormat<F> for Rc<T>
  where
    T: DefaultWireFormat<F>,
    F: Flavor + ?Sized,
    Rc<T>: super::encode::Encode<F, T::Format>,
  {
    type Format = T::Format;
    const VALUE: Self::Format = T::VALUE;
  }

  impl<T, F> DefaultWireFormat<F> for Arc<T>
  where
    T: DefaultWireFormat<F>,
    F: Flavor + ?Sized,
    Arc<T>: super::encode::Encode<F, T::Format>,
  {
    type Format = T::Format;
    const VALUE: Self::Format = T::VALUE;
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
    const VALUE: Self::Format = T::VALUE;
  }
};

/// The flavor of the encoding and decoding.
pub trait Flavor: core::fmt::Debug + 'static {
  /// The identifier used for this flavor.
  type Identifier: Identifier<Self>;

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
