use core::num::NonZeroUsize;

use crate::flavors::Flavor;

/// A message decoding error.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::IsVariant, derive_more::Display)]
pub enum DecodeError<F: Flavor + ?Sized> {
  /// Returned when the buffer does not have enough data to decode the message.
  #[display("buffer underflow")]
  BytesBufferUnderflow,

  /// [`Buffer`](super::buffer::Buffer) overflow, returned when decoding.
  ///
  /// Users can resize the buffer to a larger size (larger than `requried`) and retry decoding.
  #[display("buffer overflow, available: {available}, required: {required}")]
  BufferOverflow {
    /// The available capacity of the buffer.
    available: usize,
    /// The required capacity of the buffer.
    required: NonZeroUsize,
  },

  /// Returned when there is a unknown wire type.
  #[display("unknown identifier{identifier} when decoding {ty} in {flavor} flavor", flavor = F::NAME)]
  UnknownIdentifier {
    /// The type of the message.
    ty: &'static str,
    /// The identifier of the field.
    identifier: F::Identifier,
  },

  /// Returned when the type cannot be decoded in the given wire type format
  #[display("cannot decode {ty} in {wire_type} format in {flavor} flavor", flavor = F::NAME)]
  UnsupportedWireType {
    /// The type of the value.
    ty: &'static str,
    /// The wire type of the value.
    wire_type: F::WireType,
  },

  /// Returned when the type cannot be decoded with the given identifier
  #[display("cannot decode {ty} with identifier({identifier}) format in {flavor} flavor", flavor = F::NAME)]
  UnsupportedIdentifier {
    /// The type of the value.
    ty: &'static str,
    /// The wire type of the value.
    identifier: F::Identifier,
  },

  /// Returned when there is a unknown wire type.
  #[display("identifier mismatch: expect {expect}, actual {actual}")]
  IdentifierMismatch {
    /// The type of the message.
    expect: F::Identifier,
    /// The identifier
    actual: F::Identifier,
  },

  /// Returned when fail to decode the length-delimited
  #[display("length-delimited overflow the maximum value of u32")]
  LengthDelimitedOverflow,

  /// A custom decoding error.
  #[display("{_0}")]
  #[cfg(any(feature = "std", feature = "alloc"))]
  Custom(std::borrow::Cow<'static, str>),

  /// A custom decoding error.
  #[display("{_0}")]
  #[cfg(not(any(feature = "std", feature = "alloc")))]
  Custom(&'static str),
}

impl<F: Flavor + ?Sized> core::error::Error for DecodeError<F> {}

impl<F: Flavor + ?Sized> From<varing::DecodeError> for DecodeError<F> {
  #[inline]
  fn from(e: varing::DecodeError) -> Self {
    Self::from_varint_error(e)
  }
}

impl<F: Flavor + ?Sized> DecodeError<F> {
  /// Creates a new buffer underflow decoding error.
  #[inline]
  pub const fn buffer_underflow() -> Self {
    Self::BytesBufferUnderflow
  }

  /// Creates a new buffer overflow decoding error.
  #[inline]
  pub const fn buffer_overflow(available: usize, required: NonZeroUsize) -> Self {
    Self::BufferOverflow {
      available,
      required,
    }
  }

  /// Creates a new unknown wire type decoding error.
  #[inline]
  pub const fn unknown_identifier(ty: &'static str, identifier: F::Identifier) -> Self {
    Self::UnknownIdentifier { ty, identifier }
  }

  /// Creates a wire type not supported error.
  #[inline]
  pub const fn unsupported_wire_type(ty: &'static str, wire_type: F::WireType) -> Self {
    Self::UnsupportedWireType { ty, wire_type }
  }

  /// Creates an unsupported identifier error
  #[inline]
  pub const fn unsupported_identifier(ty: &'static str, identifier: F::Identifier) -> Self {
    Self::UnsupportedIdentifier { ty, identifier }
  }

  /// Creates a new identifier mismatch decoding error.
  #[inline]
  pub const fn identifier_mismatch(expect: F::Identifier, actual: F::Identifier) -> Self {
    Self::IdentifierMismatch { expect, actual }
  }

  /// Creates a new decoding error from [`varing::DecodeError`].
  #[inline]
  pub const fn from_varint_error(e: varing::DecodeError) -> Self {
    match e {
      varing::DecodeError::Underflow => Self::BytesBufferUnderflow,
      varing::DecodeError::Overflow => Self::LengthDelimitedOverflow,
      #[cfg(any(feature = "std", feature = "alloc"))]
      varing::DecodeError::Custom(e) => Self::Custom(std::borrow::Cow::Borrowed(e)),
      #[cfg(any(feature = "std", feature = "alloc"))]
      _ => Self::Custom(std::borrow::Cow::Borrowed("unknown error")),
      #[cfg(not(any(feature = "std", feature = "alloc")))]
      varing::DecodeError::Custom(e) => Self::Custom(e),
      #[cfg(not(any(feature = "std", feature = "alloc")))]
      _ => Self::Custom("unknown error"),
    }
  }

  /// Creates a custom decoding error.
  #[cfg(any(feature = "std", feature = "alloc"))]
  #[inline]
  pub fn custom<T>(value: T) -> Self
  where
    T: Into<std::borrow::Cow<'static, str>>,
  {
    Self::Custom(value.into())
  }

  /// Creates a custom decoding error.
  #[cfg(not(any(feature = "std", feature = "alloc")))]
  #[inline]
  pub const fn custom(value: &'static str) -> Self {
    Self::Custom(value)
  }
}
