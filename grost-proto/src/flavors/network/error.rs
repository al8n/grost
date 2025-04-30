use super::{Identifier, WireType};
use core::num::NonZeroUsize;

/// A data encoding error
#[derive(Debug, Clone, PartialEq, Eq, derive_more::IsVariant, derive_more::Display)]
pub enum EncodeError {
  /// Returned when the encoded buffer is too small to hold the bytes format of the types.
  #[display("insufficient buffer capacity, required: {required}, remaining: {remaining}")]
  InsufficientBytesBuffer {
    /// The required buffer capacity.
    required: usize,
    /// The remaining buffer capacity.
    remaining: usize,
  },
  /// Returned when the data in encoded format is larger than the maximum allowed size.
  #[display("encoded data size {size} is too large, the maximum allowed size is {maximum} bytes")]
  TooLarge {
    /// The maximum allowed size.
    maximum: usize,
    /// The size of the encoded data.
    size: usize,
  },
  /// Returned when the type cannot be encoded in the given wire type format
  #[display("cannot encode {ty} in {wire_type} format")]
  UnsupportedWireType {
    /// The type of the value.
    ty: &'static str,
    /// The wire type of the value.
    wire_type: WireType,
  },
  /// A custom encoding error.
  #[display("{_0}")]
  #[cfg(any(feature = "std", feature = "alloc"))]
  Custom(std::borrow::Cow<'static, str>),

  /// A custom encoding error.
  #[display("{_0}")]
  #[cfg(not(any(feature = "std", feature = "alloc")))]
  Custom(&'static str),
}

impl core::error::Error for EncodeError {}

impl EncodeError {
  /// Creates an insufficient buffer error.
  #[inline]
  pub const fn insufficient_buffer(required: usize, remaining: usize) -> Self {
    Self::InsufficientBytesBuffer {
      required,
      remaining,
    }
  }

  /// Creates a wire type not supported error.
  #[inline]
  pub const fn unsupported_wire_type(ty: &'static str, wire_type: WireType) -> Self {
    Self::UnsupportedWireType { ty, wire_type }
  }

  /// Creates a too large error.
  #[inline]
  pub const fn too_large(maximum: usize, size: usize) -> Self {
    Self::TooLarge { maximum, size }
  }

  /// Creates a new encoding error from a [`varing::EncodeError`].
  #[inline]
  pub const fn from_varint_error(e: varing::EncodeError) -> Self {
    match e {
      varing::EncodeError::Underflow {
        required,
        remaining,
      } => Self::InsufficientBytesBuffer {
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
      Self::InsufficientBytesBuffer {
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

  /// Returned when there is a unknown wire type.
  #[display("unknown identifier{identifier} when decoding {ty}")]
  UnknownIdentifier {
    /// The type of the message.
    ty: &'static str,
    /// The identifier of the field.
    identifier: Identifier,
  },

  /// Returned when the type cannot be encoded in the given wire type format
  #[display("cannot decode {ty} in {wire_type} format")]
  UnsupportedWireType {
    /// The type of the value.
    ty: &'static str,
    /// The wire type of the value.
    wire_type: WireType,
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
