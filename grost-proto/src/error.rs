use core::num::NonZeroUsize;

use crate::{
  buffer::{TryAdvanceError, TryPeekError, TryReadError, TrySegmentError},
  flavors::Flavor,
};

pub use varing::{DecodeError as DecodeVarintError, EncodeError as EncodeVarintError};

/// Invalid tag error
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("tag value {0} is not in range 1..={max}", max = (1u32 << 29) - 1)]
pub struct ParseTagError(pub(crate) u32);

impl ParseTagError {
  /// Returns the invalid tag value.
  #[inline]
  pub const fn value(&self) -> u32 {
    self.0
  }
}

/// A data encoding error
#[derive(Debug, Clone, PartialEq, Eq, derive_more::IsVariant, derive_more::Display)]
pub enum Error<F: Flavor + ?Sized> {
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
  #[display("cannot encode {ty} in {wire_type} format in {flavor} flavor", flavor = F::NAME)]
  UnsupportedWireType {
    /// The type of the value.
    ty: &'static str,
    /// The wire type of the value.
    wire_type: F::WireType,
  },

  /// Returned when the type cannot be encoded in the given wire type format
  #[display("cannot encode {ty} with tag({tag}) in {flavor} flavor", flavor = F::NAME)]
  UnsupportedTag {
    /// The type of the value.
    ty: &'static str,
    /// The tag of the value.
    tag: F::Tag,
  },

  /// Returned when the type cannot be encoded with the given identifier
  #[display("cannot encode {ty} with identifier({identifier}) format in {flavor} flavor", flavor = F::NAME)]
  UnsupportedIdentifier {
    /// The type of the value.
    ty: &'static str,
    /// The wire type of the value.
    identifier: F::Identifier,
  },

  /// Returned when the expect identifier for encoding is mismatch the actual identifier for encoding.
  #[display("unexpected identifier {actual}, expected {expected}")]
  UnexpectedIdentifier {
    /// The expected identifier for encoding.
    expected: F::Identifier,
    /// The actual identifier for encoding.
    actual: F::Identifier,
  },
  /// Returned when the wire type is unexpected for the given type.
  #[display("unexpected wire type {actual}, expected {expected}")]
  UnexpectedWireType {
    /// The expected wire type.
    expected: F::WireType,
    /// The actual wire type.
    actual: F::WireType,
  },

  /// Returned when the type cannot be merged in the given wire type format
  #[display("cannot merge {ty} in {wire_type} format in {flavor} flavor", flavor = F::NAME)]
  Unmergeable {
    /// The type of the value.
    ty: &'static str,
    /// The wire type.
    wire_type: F::WireType,
  },

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

  /// Returned when the field is duplicated in the message.
  #[display("duplicated field {name} with identifier{identifier} when decoding {ty} in {flavor} flavor", flavor = F::NAME)]
  DuplicatedField {
    /// The field name.
    name: &'static str,
    /// The type of the message.
    ty: &'static str,
    /// The identifier of the field.
    identifier: F::Identifier,
  },

  /// Returned when the field is not found but is required in the message.
  #[display("field {field_name} not found when constructing {struct_name}")]
  FieldNotFound {
    /// The structure name.
    struct_name: &'static str,
    /// The field name.
    field_name: &'static str,
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

impl<F: Flavor + ?Sized> core::error::Error for Error<F> {}

impl<F: Flavor + ?Sized> Error<F> {
  /// Creates an insufficient buffer error.
  #[inline]
  pub const fn insufficient_buffer(required: usize, remaining: usize) -> Self {
    Self::InsufficientBytesBuffer {
      required,
      remaining,
    }
  }

  /// Creates a too large error.
  #[inline]
  pub const fn too_large(maximum: usize, size: usize) -> Self {
    Self::TooLarge { maximum, size }
  }

  /// Creates an unsupported wire type error.
  #[inline]
  pub const fn unsupported_wire_type(ty: &'static str, wire_type: F::WireType) -> Self {
    Self::UnsupportedWireType { ty, wire_type }
  }

  /// Creates an unsupported tag error.
  #[inline]
  pub const fn unsupported_tag(ty: &'static str, tag: F::Tag) -> Self {
    Self::UnsupportedTag { ty, tag }
  }

  /// Creates an unsupported identifier error
  #[inline]
  pub const fn unsupported_identifier(ty: &'static str, identifier: F::Identifier) -> Self {
    Self::UnsupportedIdentifier { ty, identifier }
  }

  /// Creates an unexpected identifier error.
  #[inline]
  pub const fn unexpected_identifier(expected: F::Identifier, actual: F::Identifier) -> Self {
    Self::UnexpectedIdentifier { expected, actual }
  }

  /// Creates an unexpected wire type error.
  #[inline]
  pub const fn unexpected_wire_type(expected: F::WireType, actual: F::WireType) -> Self {
    Self::UnexpectedWireType { expected, actual }
  }

  /// Creates a new duplicated field error.
  #[inline]
  pub const fn duplicated_field(
    name: &'static str,
    ty: &'static str,
    identifier: F::Identifier,
  ) -> Self {
    Self::DuplicatedField {
      name,
      ty,
      identifier,
    }
  }

  /// Creates a field not found error.
  #[inline]
  pub const fn field_not_found(struct_name: &'static str, field_name: &'static str) -> Self {
    Self::FieldNotFound {
      struct_name,
      field_name,
    }
  }

  /// Creates a new encoding error from a [`EncodeVarintError`].
  #[inline]
  pub const fn from_varint_encode_error(e: EncodeVarintError) -> Self {
    match e {
      EncodeVarintError::Underflow {
        required,
        remaining,
      } => Self::InsufficientBytesBuffer {
        required,
        remaining,
      },
      #[cfg(any(feature = "std", feature = "alloc"))]
      EncodeVarintError::Custom(e) => Self::Custom(std::borrow::Cow::Borrowed(e)),
      #[cfg(any(feature = "std", feature = "alloc"))]
      _ => Self::Custom(std::borrow::Cow::Borrowed("unknown error")),
      #[cfg(not(any(feature = "std", feature = "alloc")))]
      EncodeVarintError::Custom(e) => Self::Custom(e),
      #[cfg(not(any(feature = "std", feature = "alloc")))]
      _ => Self::Custom("unknown error"),
    }
  }

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

  /// Creates a new unmergeable decoding error.
  #[inline]
  pub const fn unmergeable(ty: &'static str, wire_type: F::WireType) -> Self {
    Self::Unmergeable { ty, wire_type }
  }

  /// Creates a new decoding error from [`DecodeVarintError`].
  #[inline]
  pub const fn from_varint_decode_error(e: DecodeVarintError) -> Self {
    match e {
      DecodeVarintError::Underflow => Self::BytesBufferUnderflow,
      DecodeVarintError::Overflow => Self::LengthDelimitedOverflow,
      #[cfg(any(feature = "std", feature = "alloc"))]
      DecodeVarintError::Custom(e) => Self::Custom(std::borrow::Cow::Borrowed(e)),
      #[cfg(any(feature = "std", feature = "alloc"))]
      _ => Self::Custom(std::borrow::Cow::Borrowed("unknown error")),
      #[cfg(not(any(feature = "std", feature = "alloc")))]
      DecodeVarintError::Custom(e) => Self::Custom(e),
      #[cfg(not(any(feature = "std", feature = "alloc")))]
      _ => Self::Custom("unknown error"),
    }
  }

  /// Creates a new error from [`TryAdvanceError`].
  #[inline]
  pub const fn from_try_advance_error(e: TryAdvanceError) -> Self {
    Self::insufficient_buffer(e.requested(), e.available())
  }

  /// Creates a new error from [`TryPeekError`].
  #[inline]
  pub const fn from_try_peek_error(e: TryPeekError) -> Self {
    Self::insufficient_buffer(e.requested(), e.available())
  }

  /// Creates a new error from [`TryReadError`].
  #[inline]
  pub const fn from_try_read_error(e: TryReadError) -> Self {
    Self::insufficient_buffer(e.requested(), e.available())
  }

  /// Creates a new error from [`TrySegmentError`].
  #[inline]
  pub const fn from_try_segment_error(e: TrySegmentError) -> Self {
    Self::insufficient_buffer(e.end(), e.available())
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
  pub const fn update(mut self, required: usize, remaining: usize) -> Self {
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

impl<F: Flavor + ?Sized> From<EncodeVarintError> for Error<F> {
  #[inline]
  fn from(value: EncodeVarintError) -> Self {
    Self::from_varint_encode_error(value)
  }
}

impl<F: Flavor + ?Sized> From<DecodeVarintError> for Error<F> {
  #[inline]
  fn from(e: DecodeVarintError) -> Self {
    Self::from_varint_decode_error(e)
  }
}

impl<F: Flavor + ?Sized> From<TryAdvanceError> for Error<F> {
  #[inline]
  fn from(e: TryAdvanceError) -> Self {
    Self::from_try_advance_error(e)
  }
}

impl<F: Flavor + ?Sized> From<TryPeekError> for Error<F> {
  #[inline]
  fn from(e: TryPeekError) -> Self {
    Self::from_try_peek_error(e)
  }
}

impl<F: Flavor + ?Sized> From<TryReadError> for Error<F> {
  #[inline]
  fn from(e: TryReadError) -> Self {
    Self::from_try_read_error(e)
  }
}

impl<F: Flavor + ?Sized> From<TrySegmentError> for Error<F> {
  #[inline]
  fn from(e: TrySegmentError) -> Self {
    Self::from_try_segment_error(e)
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<F: Flavor + ?Sized> From<std::borrow::Cow<'static, str>> for Error<F> {
  fn from(value: std::borrow::Cow<'static, str>) -> Self {
    Self::custom(value)
  }
}

impl<F: Flavor + ?Sized> From<&'static str> for Error<F> {
  fn from(value: &'static str) -> Self {
    Self::custom(value)
  }
}
