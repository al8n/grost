use super::buffer::BytesBuffer;

pub use network::Network;

/// The network flavor
pub mod network;

/// The identifier
pub trait Identifier<F: Flavor + ?Sized>: Copy + core::fmt::Debug + core::fmt::Display {
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
  type EncodeError: core::error::Error + From<super::EncodeError<Self>>;
  /// The decode error for this flavor.
  type DecodeError: core::error::Error + From<super::DecodeError<Self>>;

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
