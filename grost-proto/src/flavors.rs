use super::buffer::BytesBuffer;

/// The network flavor
pub mod network;

/// The identifier
pub trait Identifier<F: Flavor>: Copy + core::fmt::Debug + core::fmt::Display {
  /// The wire type used for this identifier.
  fn wire_type(&self) -> F::WireType;

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

/// The flavor of the encoding and decoding.
pub trait Flavor: core::fmt::Debug + 'static {
  /// The wire type used for this flavor.
  type WireType: Copy + core::fmt::Debug + core::fmt::Display;
  /// The context used for this flavor.
  #[cfg(not(feature = "quickcheck"))]
  type Context;
  /// The context used for this flavor.
  #[cfg(feature = "quickcheck")]
  type Context: quickcheck::Arbitrary;

  /// The encode error for this flavor.
  type EncodeError: core::error::Error + From<super::EncodeError>;
  /// The decode error for this flavor.
  type DecodeError: core::error::Error + From<super::DecodeError<Self>>;
}
