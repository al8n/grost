use core::num::NonZeroUsize;

use super::{Tag, flavors::Flavor};

/// A data encoding error
#[derive(Debug, Clone, PartialEq, Eq, derive_more::IsVariant, derive_more::Display)]
pub enum EncodeError<F: Flavor + ?Sized> {
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
    wire_type: F::WireType,
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

impl<F: Flavor + ?Sized> core::error::Error for EncodeError<F> {}

impl<F: Flavor + ?Sized> EncodeError<F> {
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
  pub const fn unsupported_wire_type(ty: &'static str, wire_type: F::WireType) -> Self {
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

impl<F: Flavor + ?Sized> From<varing::EncodeError> for EncodeError<F> {
  #[inline]
  fn from(value: varing::EncodeError) -> Self {
    Self::from_varint_error(value)
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<F: Flavor + ?Sized> From<std::borrow::Cow<'static, str>> for EncodeError<F> {
  fn from(value: std::borrow::Cow<'static, str>) -> Self {
    Self::Custom(value)
  }
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
impl<F: Flavor + ?Sized> From<&'static str> for EncodeError<F> {
  fn from(value: &'static str) -> Self {
    Self::Custom(value)
  }
}

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

  /// Returned when the buffer does not contain the required field.
  #[display("{field} in {ty} is not found in buffer")]
  FieldNotFound {
    /// The type of the message.
    ty: &'static str,
    /// The name of the field.
    field: &'static str,
    /// The wire type of the field.
    wire_type: F::WireType,
    /// The tag of the field.
    tag: Tag,
  },

  /// Returned when the buffer contains duplicate fields for the same tag in a message.
  #[display("duplicate field {field} with identifier(wire_type: {wire_type}, tag: {tag}) in {ty}")]
  DuplicateField {
    /// The type of the message.
    ty: &'static str,
    /// The name of the field.
    field: &'static str,
    /// The tag of the field.
    tag: Tag,
    /// The wire type of the field.
    wire_type: F::WireType,
  },

  /// Returned when there is a unknown wire type.
  #[display("unknown identifier(wire_type: {wire_type}, tag: {tag}) when decoding {ty}")]
  UnknownIdentifier {
    /// The type of the message.
    ty: &'static str,
    /// The identifier
    wire_type: F::WireType,
    /// The tag
    tag: Tag,
  },

  /// Returned when the type cannot be encoded in the given wire type format
  #[display("cannot encode {ty} in {wire_type} format")]
  UnsupportedWireType {
    /// The type of the value.
    ty: &'static str,
    /// The wire type of the value.
    wire_type: F::WireType,
  },

  /// Returned when there is a unknown wire type.
  #[display("identifier mismatch: expect (wire_type: {}, tag: {}), actual (wire_type: {}, tag: {})", expect.0, expect.1, actual.0, actual.1)]
  IdentifierMismatch {
    /// The type of the message.
    expect: (F::WireType, Tag),
    /// The identifier
    actual: (F::WireType, Tag),
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

  /// Creates a new missing field decoding error.
  #[inline]
  pub const fn field_not_found(
    ty: &'static str,
    field: &'static str,
    wire_type: F::WireType,
    tag: Tag,
  ) -> Self {
    Self::FieldNotFound {
      ty,
      field,
      wire_type,
      tag,
    }
  }

  /// Creates a new duplicate field decoding error.
  #[inline]
  pub const fn duplicate_field(
    ty: &'static str,
    field: &'static str,
    wire_type: F::WireType,
    tag: Tag,
  ) -> Self {
    Self::DuplicateField {
      ty,
      field,
      wire_type,
      tag,
    }
  }

  /// Creates a new unknown wire type decoding error.
  #[inline]
  pub const fn unknown_identifier(ty: &'static str, wire_type: F::WireType, tag: Tag) -> Self {
    Self::UnknownIdentifier { ty, wire_type, tag }
  }

  /// Creates a wire type not supported error.
  #[inline]
  pub const fn unsupported_wire_type(ty: &'static str, wire_type: F::WireType) -> Self {
    Self::UnsupportedWireType { ty, wire_type }
  }

  /// Creates a new identifier mismatch decoding error.
  #[inline]
  pub const fn identifier_mismatch(expect: (F::WireType, Tag), actual: (F::WireType, Tag)) -> Self {
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
