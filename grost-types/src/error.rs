use core::num::NonZeroUsize;

use crate::Identifier;

use super::Tag;

/// A data encoding error
#[derive(Debug, thiserror::Error)]
pub enum EncodeError {
  /// Returned when the encoded buffer is too small to hold the bytes format of the types.
  #[error("insufficient buffer capacity, required: {required}, remaining: {remaining}")]
  InsufficientBuffer {
    /// The required buffer capacity.
    required: usize,
    /// The remaining buffer capacity.
    remaining: usize,
  },
  /// Returned when the data in encoded format is larger than the maximum allowed size.
  #[error("encoded data is too large, the maximum allowed size is {MAX} bytes", MAX = u32::MAX)]
  TooLarge,
  /// A custom encoding error.
  #[error("{0}")]
  #[cfg(any(feature = "std", feature = "alloc"))]
  Custom(std::borrow::Cow<'static, str>),

  /// A custom encoding error.
  #[error("{0}")]
  #[cfg(not(any(feature = "std", feature = "alloc")))]
  Custom(&'static str),
}

impl EncodeError {
  /// Creates an insufficient buffer error.
  #[inline]
  pub const fn insufficient_buffer(required: usize, remaining: usize) -> Self {
    Self::InsufficientBuffer {
      required,
      remaining,
    }
  }

  /// Creates a new encoding error from a [`varing::EncodeError`].
  #[inline]
  pub const fn from_varint_error(e: varing::EncodeError) -> Self {
    match e {
      varing::EncodeError::Underflow {
        required,
        remaining,
      } => Self::InsufficientBuffer {
        required,
        remaining,
      },
      #[cfg(any(feature = "std", feature = "alloc"))]
      varing::EncodeError::Custom(e) => Self::Custom(std::borrow::Cow::Borrowed(e)),
      #[cfg(any(feature = "std", feature = "alloc"))]
      _ => Self::Custom(std::borrow::Cow::Borrowed("unknown error")),
      #[cfg(not(any(feature = "std", feature = "alloc")))]
      varing::EncodeError::Custom(e) => Self::Custom(e),
      #[cfg(not(any(feature = "std", feature = "alloc")))]
      _ => Self::Custom("unknown error"),
    }
  }

  /// Creates a custom encoding error.
  #[cfg(any(feature = "std", feature = "alloc"))]
  #[inline]
  pub fn custom<T>(value: T) -> Self
  where
    T: Into<std::borrow::Cow<'static, str>>,
  {
    Self::Custom(value.into())
  }

  /// Creates a custom encoding error.
  #[cfg(not(any(feature = "std", feature = "alloc")))]
  #[inline]
  pub const fn custom(value: &'static str) -> Self {
    Self::Custom(value)
  }

  /// Update the error with the required and remaining buffer capacity.
  pub fn update(mut self, required: usize, remaining: usize) -> Self {
    match self {
      Self::InsufficientBuffer {
        required: ref mut r,
        remaining: ref mut rem,
      } => {
        *r = required;
        *rem = remaining;
        self
      }
      _ => self,
    }
  }
}

impl From<varing::EncodeError> for EncodeError {
  #[inline]
  fn from(value: varing::EncodeError) -> Self {
    Self::from_varint_error(value)
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl From<std::borrow::Cow<'static, str>> for EncodeError {
  fn from(value: std::borrow::Cow<'static, str>) -> Self {
    Self::Custom(value)
  }
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
impl From<&'static str> for EncodeError {
  fn from(value: &'static str) -> Self {
    Self::Custom(value)
  }
}

/// A message decoding error.
///
/// `DecodeError` indicates that the input buffer does not contain a valid
/// message. The error details should be considered 'best effort': in
/// general it is not possible to exactly pinpoint why data is malformed.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, derive_more::IsVariant)]
pub enum DecodeError {
  /// Returned when the buffer does not have enough data to decode the message.
  #[error("buffer underflow")]
  BufferUnderflow,

  /// Unknown buffer overflow, returned when decoding and meet the unknown tag,
  /// This typically happens when a older version tries to decode a newer version message.
  ///
  /// Users can resize the unknown buffer to a larger size and retry decoding.
  #[error("unknown buffer overflow")]
  UnknownBufferOverflow {
    /// The available capacity of the unknown buffer.
    available: usize,
    /// The required capacity of the unknown buffer.
    required: NonZeroUsize,
  },

  /// Returned when the buffer does not contain the required field.
  #[error("missing {field} in {ty}")]
  MissingField {
    /// The type of the message.
    ty: &'static str,
    /// The name of the field.
    field: &'static str,
  },

  /// Returned when the buffer contains duplicate fields for the same tag in a message.
  #[error("duplicate field {field} with tag {tag} in {ty}")]
  DuplicateField {
    /// The type of the message.
    ty: &'static str,
    /// The name of the field.
    field: &'static str,
    /// The tag of the field.
    tag: Tag,
  },

  /// Returned when there is a unknown wire type.
  #[error("unknown identifier({identifier}) when decoding {ty}")]
  UnknownIdentifier {
    /// The type of the message.
    ty: &'static str,
    /// The identifier
    identifier: Identifier,
  },

  /// Returned when there is a unknown wire type.
  #[error("identifier mismatch: expect {expect}, actual {actual}")]
  IdentifierMismatch {
    /// The type of the message.
    expect: Identifier,
    /// The identifier
    actual: Identifier,
  },

  /// Returned when fail to decode the length-delimited
  #[error("length-delimited overflow the maximum value of u32")]
  LengthDelimitedOverflow,

  /// A custom decoding error.
  #[error("{0}")]
  #[cfg(any(feature = "std", feature = "alloc"))]
  Custom(std::borrow::Cow<'static, str>),

  /// A custom decoding error.
  #[error("{0}")]
  #[cfg(not(any(feature = "std", feature = "alloc")))]
  Custom(&'static str),
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
    Self::BufferUnderflow
  }

  /// Creates a new unknown buffer overflow decoding error.
  #[inline]
  pub const fn unknown_buffer_overflow(available: usize, required: NonZeroUsize) -> Self {
    Self::UnknownBufferOverflow {
      available,
      required,
    }
  }

  /// Creates a new missing field decoding error.
  #[inline]
  pub const fn missing_field(ty: &'static str, field: &'static str) -> Self {
    Self::MissingField { ty, field }
  }

  /// Creates a new duplicate field decoding error.
  #[inline]
  pub const fn duplicate_field(ty: &'static str, field: &'static str, tag: Tag) -> Self {
    Self::DuplicateField { ty, field, tag }
  }

  /// Creates a new unknown wire type decoding error.
  #[inline]
  pub const fn unknown_identifier(ty: &'static str, identifier: Identifier) -> Self {
    Self::UnknownIdentifier { ty, identifier }
  }

  /// Creates a new identifier mismatch decoding error.
  #[inline]
  pub const fn identifier_mismatch(expect: Identifier, actual: Identifier) -> Self {
    Self::IdentifierMismatch { expect, actual }
  }

  /// Creates a new decoding error from [`varing::DecodeError`].
  #[inline]
  pub const fn from_varint_error(e: varing::DecodeError) -> Self {
    match e {
      varing::DecodeError::Underflow => Self::BufferUnderflow,
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
