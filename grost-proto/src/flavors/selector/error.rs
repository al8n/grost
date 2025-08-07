use crate::{
  decode::DecodeError as BaseDecodeError,
  flavors::{Flavor, selector::Select},
};
use core::num::NonZeroUsize;

use super::{SelectorIdentifier, SelectorWireType};

/// The encode error for [`Select`](super::Select)
pub type EncodeError = crate::encode::EncodeError<super::Select>;

/// The error when parsing a [`SelectorTag`](super::SelectorTag).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("invalid selector tag value: {0}")]
pub struct ParseSelectorTagError(pub(super) u8);

impl ParseSelectorTagError {
  /// Returns the invalid selector tag value.
  #[inline]
  pub const fn value(&self) -> u8 {
    self.0
  }
}

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

  /// Unknown selector tag.
  #[display("unknown selector tag value {value} in {flavor} flavor", flavor = Select::NAME)]
  UnknownTagValue {
    /// The unknown selector tag value.
    value: u8,
  },

  /// Unknown selector wire type.
  #[display("unknown selector wire type value {value} in {flavor} flavor", flavor = Select::NAME)]
  UnknownWireTypeValue {
    /// The unknown selector wire type value.
    value: u8,
  },

  /// Unknown selector identifier value.
  #[display("unknown selector identifier type value (wire_type: {wire_type}, tag: {tag}) in {flavor} flavor", flavor = Select::NAME)]
  UnknownIdentifierValue {
    /// The unknown selector identifier tag value.
    tag: u8,
    /// The unknown selector identifier wire type value.
    wire_type: u8,
  },

  /// Returned when there is a unknown wire type.
  #[display("unknown identifier{identifier} when decoding {ty} in {flavor} flavor", flavor = Select::NAME)]
  UnknownIdentifier {
    /// The type of the message.
    ty: &'static str,
    /// The identifier of the field.
    identifier: SelectorIdentifier,
  },

  /// Returned when the type cannot be decoded in the given wire type format
  #[display("cannot decode {ty} in {wire_type} format in {flavor} flavor", flavor = Select::NAME)]
  IncompatibleWireType {
    /// The type of the value.
    ty: &'static str,
    /// The wire type of the value.
    wire_type: SelectorWireType,
  },

  /// Returned when the type cannot be decoded with the given identifier
  #[display("cannot decode {ty} with identifier({identifier}) format in {flavor} flavor", flavor = Select::NAME)]
  UnsupportedIdentifier {
    /// The type of the value.
    ty: &'static str,
    /// The wire type of the value.
    identifier: SelectorIdentifier,
  },

  /// Returned when there is a unknown wire type.
  #[display("identifier mismatch: expect {expect}, actual {actual}")]
  UnexpectedIdentifier {
    /// The type of the message.
    expect: SelectorIdentifier,
    /// The identifier
    actual: SelectorIdentifier,
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

impl From<ParseSelectorTagError> for DecodeError {
  #[inline]
  fn from(e: ParseSelectorTagError) -> Self {
    Self::UnknownTagValue { value: e.0 }
  }
}

impl From<BaseDecodeError<super::Select>> for DecodeError {
  #[inline]
  fn from(e: BaseDecodeError<super::Select>) -> Self {
    match e {
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
      BaseDecodeError::UnexpectedIdentifier { expect, actual } => {
        Self::unexpected_identifier(expect, actual)
      }
      BaseDecodeError::UnknownIdentifier { ty, identifier } => {
        Self::unknown_identifier(ty, identifier)
      }
      BaseDecodeError::IncompatibleWireType { ty, wire_type } => {
        Self::unsupported_wire_type(ty, wire_type)
      }
      BaseDecodeError::LengthDelimitedOverflow => Self::LengthDelimitedOverflow,
      BaseDecodeError::Custom(e) => Self::custom(e),
    }
  }
}

impl From<varing::DecodeError> for DecodeError {
  #[inline]
  fn from(e: varing::DecodeError) -> Self {
    Self::from_varint_error(e)
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

  /// Creates a new unknown selector tag decoding error.
  #[inline]
  pub const fn unknown_tag_value(value: u8) -> Self {
    Self::UnknownTagValue { value }
  }

  /// Creates a new unknown selector wire type decoding error.
  #[inline]
  pub const fn unknown_wire_type_value(value: u8) -> Self {
    Self::UnknownWireTypeValue { value }
  }

  /// Creates a new unknown selector identifier decoding error.
  #[inline]
  pub const fn unknown_identifier_value(wire_type: u8, tag: u8) -> Self {
    Self::UnknownIdentifierValue { tag, wire_type }
  }

  /// Creates a new unknown wire type decoding error.
  #[inline]
  pub const fn unknown_identifier(ty: &'static str, identifier: SelectorIdentifier) -> Self {
    Self::UnknownIdentifier { ty, identifier }
  }

  /// Creates a wire type not supported error.
  #[inline]
  pub const fn unsupported_wire_type(ty: &'static str, wire_type: SelectorWireType) -> Self {
    Self::IncompatibleWireType { ty, wire_type }
  }

  /// Creates an unsupported identifier error
  #[inline]
  pub const fn unsupported_identifier(ty: &'static str, identifier: SelectorIdentifier) -> Self {
    Self::UnsupportedIdentifier { ty, identifier }
  }

  /// Creates a new identifier mismatch decoding error.
  #[inline]
  pub const fn unexpected_identifier(expect: SelectorIdentifier, actual: SelectorIdentifier) -> Self {
    Self::UnexpectedIdentifier { expect, actual }
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
