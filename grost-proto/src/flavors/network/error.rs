use super::{Identifier, Network, Tag, WireType};
use crate::{
  error::Error as BaseError,
  flavors::{Flavor, FlavorError},
};
use core::num::NonZeroUsize;

pub use super::tag::ParseTagError;

/// An error when encoding or decoding a message.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::IsVariant, thiserror::Error)]
pub enum Error {
  /// Returned when the encoded buffer is too small to hold the bytes format of the types.
  #[error("insufficient buffer capacity, required: {required}, remaining: {remaining}")]
  InsufficientBytesBuffer {
    /// The required buffer capacity.
    required: usize,
    /// The remaining buffer capacity.
    remaining: usize,
  },
  /// Returned when the data in encoded format is larger than the maximum allowed size.
  #[error("encoded data size {size} is too large, the maximum allowed size is {maximum} bytes")]
  TooLarge {
    /// The maximum allowed size.
    maximum: usize,
    /// The size of the encoded data.
    size: usize,
  },
  /// Returned when the tag value is not in range `1..=536870911`.
  #[error("tag value {0} is not in range 1..={max}", max = (1u32 << 29) - 1)]
  UnsupportedTagValue(u32),
  /// Returned when the type cannot be encoded in the given wire type format
  #[error("cannot encode {ty} in {wire_type} format in network flavor")]
  UnsupportedWireType {
    /// The type of the value.
    ty: &'static str,
    /// The wire type of the value.
    wire_type: WireType,
  },
  /// Returned when the type cannot be encoded in the given wire type format
  #[error("cannot encode {ty} with tag({tag}) in network flavor")]
  UnsupportedTag {
    /// The type of the value.
    ty: &'static str,
    /// The tag of the value.
    tag: Tag,
  },
  /// Returned when the type cannot be encoded with the given identifier
  #[error("cannot encode {ty} with identifier({identifier}) format in network flavor")]
  UnsupportedIdentifier {
    /// The type of the value.
    ty: &'static str,
    /// The wire type of the value.
    identifier: Identifier,
  },
  /// Returned when the expect identifier for encoding is mismatch the actual identifier for encoding.
  #[error("identifier mismatch: expect {expect}, actual {actual}")]
  IdentifierMismatch {
    /// The expected identifier for encoding.
    expect: Identifier,
    /// The actual identifier for encoding.
    actual: Identifier,
  },
  /// Returned when the buffer does not have enough data to decode the message.
  #[error("buffer underflow")]
  BytesBufferUnderflow,
  /// [`Buffer`](super::buffer::Buffer) overflow, returned when decoding.
  ///
  /// Users can resize the buffer to a larger size (larger than `requried`) and retry decoding.
  #[error("buffer overflow, available: {available}, required: {required}")]
  BufferOverflow {
    /// The available capacity of the buffer.
    available: usize,
    /// The required capacity of the buffer.
    required: NonZeroUsize,
  },
  /// Returned when the buffer does not contain the required field.
  #[error("{field}{identifier} in {ty} is not found in buffer")]
  FieldNotFound {
    /// The type of the message.
    ty: &'static str,
    /// The name of the field.
    field: &'static str,
    /// The identifier of the field.
    identifier: Identifier,
  },
  /// Returned when the buffer contains duplicate fields for the same tag in a message.
  #[error("duplicate field {field} with identifier{identifier} in {ty}")]
  DuplicateField {
    /// The type of the message.
    ty: &'static str,
    /// The name of the field.
    field: &'static str,
    /// The identifier of the field.
    identifier: Identifier,
  },
  /// Returned when parsing a tag fails.
  #[error("failed to parse tag {_0}")]
  ParseTagError(ParseTagError),
  /// Returned when there is a unknown identifier.
  #[error("unknown identifier{identifier} when decoding {ty} in {flavor} flavor", flavor = Network::NAME)]
  UnknownIdentifier {
    /// The type of the message.
    ty: &'static str,
    /// The identifier of the field.
    identifier: Identifier,
  },
  /// Returned when fail to decode the length-delimited
  #[error("length-delimited overflow the maximum value of u32")]
  LengthDelimitedOverflow,

  /// A custom encoding error.
  #[error("{_0}")]
  #[cfg(any(feature = "std", feature = "alloc"))]
  Custom(std::borrow::Cow<'static, str>),

  /// A custom encoding error.
  #[error("{_0}")]
  #[cfg(not(any(feature = "std", feature = "alloc")))]
  Custom(&'static str),
}

impl FlavorError<Network> for Error {
  fn update_insufficient_buffer(&mut self, required: usize, remaining: usize) {
    if let Self::InsufficientBytesBuffer {
      required: r,
      remaining: rem,
    } = self
    {
      *r = required;
      *rem = remaining;
    }
  }
}

impl From<ParseTagError> for Error {
  #[inline]
  fn from(e: ParseTagError) -> Self {
    Self::unsupported_tag_value(e.value())
  }
}

impl From<varing::EncodeError> for Error {
  #[inline]
  fn from(value: varing::EncodeError) -> Self {
    Self::from_varint_encode_error(value)
  }
}

impl From<varing::DecodeError> for Error {
  #[inline]
  fn from(e: varing::DecodeError) -> Self {
    Self::from_varint_decode_error(e)
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl From<std::borrow::Cow<'static, str>> for Error {
  fn from(value: std::borrow::Cow<'static, str>) -> Self {
    Self::custom(value)
  }
}

impl From<&'static str> for Error {
  fn from(value: &'static str) -> Self {
    Self::custom(value)
  }
}

impl Error {
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

  /// Creates a new unsupported tag value error.
  #[inline]
  pub const fn unsupported_tag_value(tag: u32) -> Self {
    Self::UnsupportedTagValue(tag)
  }

  /// Creates an unsupported wire type error.
  #[inline]
  pub const fn unsupported_wire_type(ty: &'static str, wire_type: WireType) -> Self {
    Self::UnsupportedWireType { ty, wire_type }
  }

  /// Creates an unsupported tag error.
  #[inline]
  pub const fn unsupported_tag(ty: &'static str, tag: Tag) -> Self {
    Self::UnsupportedTag { ty, tag }
  }

  /// Creates an unsupported identifier error
  #[inline]
  pub const fn unsupported_identifier(ty: &'static str, identifier: Identifier) -> Self {
    Self::UnsupportedIdentifier { ty, identifier }
  }

  /// Creates an identifier mismatch error.
  #[inline]
  pub const fn identifier_mismatch(expect: Identifier, actual: Identifier) -> Self {
    Self::IdentifierMismatch { expect, actual }
  }

  /// Creates a new encoding error from a [`varing::EncodeError`].
  #[inline]
  pub const fn from_varint_encode_error(e: varing::EncodeError) -> Self {
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
  /// Creates a new decoding error from [`varing::DecodeError`].
  #[inline]
  pub const fn from_varint_decode_error(e: varing::DecodeError) -> Self {
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

  /// Creates a new unknown wire type decoding error.
  #[inline]
  pub const fn unknown_identifier(ty: &'static str, identifier: Identifier) -> Self {
    Self::UnknownIdentifier { ty, identifier }
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
}

impl From<BaseError<Network>> for Error {
  fn from(value: BaseError<Network>) -> Self {
    match value {
      BaseError::InsufficientBytesBuffer {
        required,
        remaining,
      } => Self::insufficient_buffer(required, remaining),
      BaseError::TooLarge { maximum, size } => Self::too_large(maximum, size),
      BaseError::UnsupportedWireType { ty, wire_type } => {
        Self::unsupported_wire_type(ty, wire_type)
      }
      BaseError::UnsupportedTag { ty, tag } => Self::unsupported_tag(ty, tag),
      BaseError::UnsupportedIdentifier { ty, identifier } => {
        Self::unsupported_identifier(ty, identifier)
      }
      BaseError::IdentifierMismatch { expect, actual } => Self::identifier_mismatch(expect, actual),
      BaseError::BytesBufferUnderflow => Self::BytesBufferUnderflow,
      BaseError::BufferOverflow {
        available,
        required,
      } => Self::BufferOverflow {
        available,
        required,
      },
      BaseError::UnknownIdentifier { ty, identifier } => Self::unknown_identifier(ty, identifier),
      BaseError::LengthDelimitedOverflow => Self::LengthDelimitedOverflow,
      BaseError::Custom(cow) => Self::Custom(cow),
    }
  }
}
