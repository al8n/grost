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

pub trait WireFormat: Copy + Eq + core::hash::Hash + core::fmt::Debug + core::fmt::Display {}

pub trait Encode2<F: Flavor + ?Sized, W: WireFormat> {
  /// Encode the value into a buffer.
  fn encode<B>(&self, ctx: &F::Context, buf: &mut [u8]) -> Result<usize, F::EncodeError>
  where
    Self: Sized,
    B: BytesBuffer;

  /// Return the length of the encoded value.
  fn encoded_len(&self, ctx: &F::Context) -> Result<usize, F::EncodeError>
  where
    Self: Sized;

  /// Encodes the message into the provided buffer with length-delimited. If the message cannot be encoded as the given wire type,
  /// then it will return `WireTypeNotSupported` err will be returned.
  ///
  /// Returns the number of bytes written to the buffer or an error if the operation fails.
  ///
  /// See also [ trait level documentation ](Encode) for more details about the arguments.
  fn encode_length_delimited<B>(
    &self,
    ctx: &F::Context,
    buf: &mut [u8],
  ) -> Result<usize, F::EncodeError>
  where
    Self: Sized,
    B: BytesBuffer;

  /// Returns the number of bytes needed to encode the message with length-delimited.
  ///
  /// This is used to determine the buffer size required for encoding.
  ///
  /// See also [ trait level documentation ](Encode) for more details about the arguments.
  fn encoded_length_delimited_len(&self, ctx: &F::Context) -> Result<usize, F::EncodeError>
  where
    Self: Sized;
}

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
