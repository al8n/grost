use super::{Identifier, Network, WireType};
use crate::{decode::DecodeError as BaseDecodeError, flavors::Flavor};
use core::num::NonZeroUsize;

pub use super::tag::ParseTagError;

/// The encode error for [`Network`] flavor.
pub type EncodeError = crate::encode::EncodeError<Network>;

/// A message decoding error.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::IsVariant, derive_more::Display)]
pub enum DecodeError {
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

  /// Returned when the buffer does not contain the required field.
  #[display("{field}{identifier} in {ty} is not found in buffer")]
  FieldNotFound {
    /// The type of the message.
    ty: &'static str,
    /// The name of the field.
    field: &'static str,
    /// The identifier of the field.
    identifier: Identifier,
  },

  /// Returned when the buffer contains duplicate fields for the same tag in a message.
  #[display("duplicate field {field} with identifier{identifier} in {ty}")]
  DuplicateField {
    /// The type of the message.
    ty: &'static str,
    /// The name of the field.
    field: &'static str,
    /// The identifier of the field.
    identifier: Identifier,
  },

  /// Returned when parsing a tag fails.
  #[display("failed to parse tag {_0}")]
  ParseTagError(ParseTagError),

  /// Returned when there is a unknown identifier.
  #[display("unknown identifier{identifier} when decoding {ty} in {flavor} flavor", flavor = Network::NAME)]
  UnknownIdentifier {
    /// The type of the message.
    ty: &'static str,
    /// The identifier of the field.
    identifier: Identifier,
  },

  /// Returned when the type cannot be decoded in the given wire type format
  #[display("cannot decode {ty} in {wire_type} format in {flavor} flavor", flavor = Network::NAME)]
  UnsupportedWireType {
    /// The type of the value.
    ty: &'static str,
    /// The wire type of the value.
    wire_type: WireType,
  },

  /// Returned when the type cannot be decoded with the given identifier
  #[display("cannot decode {ty} with identifier({identifier}) format in {flavor} flavor", flavor = Network::NAME)]
  UnsupportedIdentifier {
    /// The type of the value.
    ty: &'static str,
    /// The wire type of the value.
    identifier: Identifier,
  },

  /// Returned when there is a unknown wire type.
  #[display("identifier mismatch: expect {expect}, actual {actual}")]
  IdentifierMismatch {
    /// The type of the message.
    expect: Identifier,
    /// The identifier
    actual: Identifier,
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

impl core::error::Error for DecodeError {}

impl From<BaseDecodeError<Network>> for DecodeError {
  fn from(value: BaseDecodeError<Network>) -> Self {
    match value {
      BaseDecodeError::BytesBufferUnderflow => Self::BytesBufferUnderflow,
      BaseDecodeError::BufferOverflow {
        available,
        required,
      } => Self::BufferOverflow {
        available,
        required,
      },
      BaseDecodeError::UnsupportedIdentifier { ty, identifier } => {
        Self::unsupported_identifier(ty, identifier)
      }
      BaseDecodeError::IdentifierMismatch { expect, actual } => {
        Self::identifier_mismatch(expect, actual)
      }
      BaseDecodeError::UnknownIdentifier { ty, identifier } => {
        Self::unknown_identifier(ty, identifier)
      }
      BaseDecodeError::UnsupportedWireType { ty, wire_type } => {
        Self::unsupported_wire_type(ty, wire_type)
      }
      BaseDecodeError::LengthDelimitedOverflow => Self::LengthDelimitedOverflow,
      BaseDecodeError::Custom(e) => Self::Custom(e),
    }
  }
}

impl From<varing::DecodeError> for DecodeError {
  #[inline]
  fn from(e: varing::DecodeError) -> Self {
    Self::from_varint_error(e)
  }
}

impl From<ParseTagError> for DecodeError {
  #[inline]
  fn from(e: ParseTagError) -> Self {
    Self::ParseTagError(e)
  }
}

impl DecodeError {
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

  /// Creates a new missing field decoding error.
  #[inline]
  pub const fn field_not_found(
    ty: &'static str,
    field: &'static str,
    identifier: Identifier,
  ) -> Self {
    Self::FieldNotFound {
      ty,
      field,
      identifier,
    }
  }

  /// Creates a new duplicate field decoding error.
  #[inline]
  pub const fn duplicate_field(
    ty: &'static str,
    field: &'static str,
    identifier: Identifier,
  ) -> Self {
    Self::DuplicateField {
      ty,
      field,
      identifier,
    }
  }

  /// Creates a new unknown wire type decoding error.
  #[inline]
  pub const fn unknown_identifier(ty: &'static str, identifier: Identifier) -> Self {
    Self::UnknownIdentifier { ty, identifier }
  }

  /// Creates a wire type not supported error.
  #[inline]
  pub const fn unsupported_wire_type(ty: &'static str, wire_type: WireType) -> Self {
    Self::UnsupportedWireType { ty, wire_type }
  }

  /// Creates a new identifier mismatch decoding error.
  #[inline]
  pub const fn identifier_mismatch(expect: Identifier, actual: Identifier) -> Self {
    Self::IdentifierMismatch { expect, actual }
  }

  /// Creates an unsupported identifier error
  #[inline]
  pub const fn unsupported_identifier(ty: &'static str, identifier: Identifier) -> Self {
    Self::UnsupportedIdentifier { ty, identifier }
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

  /// Creates a parse tag error.
  #[inline]
  pub const fn parse_tag_error(e: ParseTagError) -> Self {
    Self::ParseTagError(e)
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
