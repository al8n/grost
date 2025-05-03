use crate::flavors::Flavor;

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
  #[display("identifier mismatch: expect {expect}, actual {actual}")]
  IdentifierMismatch {
    /// The expected identifier for encoding.
    expect: F::Identifier,
    /// The actual identifier for encoding.
    actual: F::Identifier,
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

  /// Creates an identifier mismatch error.
  #[inline]
  pub const fn identifier_mismatch(expect: F::Identifier, actual: F::Identifier) -> Self {
    Self::IdentifierMismatch { expect, actual }
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

impl<F: Flavor + ?Sized> From<varing::EncodeError> for EncodeError<F> {
  #[inline]
  fn from(value: varing::EncodeError) -> Self {
    Self::from_varint_error(value)
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<F: Flavor + ?Sized> From<std::borrow::Cow<'static, str>> for EncodeError<F> {
  fn from(value: std::borrow::Cow<'static, str>) -> Self {
    Self::custom(value)
  }
}

impl<F: Flavor + ?Sized> From<&'static str> for EncodeError<F> {
  fn from(value: &'static str) -> Self {
    Self::custom(value)
  }
}
