use super::buffer::Buffer;

/// The identifier
pub trait Identifier<F: Flavor>: Copy + core::fmt::Debug + core::fmt::Display {
  /// The wire type used for this identifier.
  fn wire_type(&self) -> F::WireType;
  /// The tag used for this identifier.
  fn tag(&self) -> F::Tag;

  /// Encode the identifier into a buffer.
  fn encode(&self, dst: &mut [u8]) -> Result<usize, F::EncodeError>;

  /// Return the length of the encoded identifier.
  fn encoded_len(&self) -> usize;

  /// Decode the identifier from a buffer.
  fn decode<B>(buf: B) -> Result<(usize, Self), F::DecodeError>
  where
    B: Buffer + Sized,
    Self: Sized;
}

/// The flavor of the encoding and decoding.
pub trait Flavor: 'static {
  /// The wire type used for this flavor.
  type WireType: Copy + core::fmt::Debug + core::fmt::Display;
  /// The tag used for this flavor.
  type Tag: Copy + core::fmt::Debug + core::fmt::Display;
  /// The context used for this flavor.
  type Context;
  /// The encode error for this flavor.
  type EncodeError: core::error::Error;
  /// The decode error for this flavor.
  type DecodeError: core::error::Error;
}
