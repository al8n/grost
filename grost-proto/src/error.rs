use core::num::NonZeroUsize;

use crate::flavors::Flavor;

pub use bufkit::error::{
  OutOfBounds, ReadVarintError, TryAdvanceError, TryPeekError, TryReadError, TrySegmentError,
  TryWriteAtError, TryWriteError, WriteVarintAtError, WriteVarintError,
};

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

/// Error type for decoding tags.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DecodeTagError {
  /// Constructed from a `ReadVarintError`.
  #[error(transparent)]
  Read(#[from] ReadVarintError),
  /// Constructed from a `ParseTagError`.
  #[error(transparent)]
  Parse(#[from] ParseTagError),
}

/// An error that occurs when the output buffer has insufficient capacity to hold the encoded data.
///
/// This error indicates a recoverable condition where encoding could succeed if provided with
/// a larger buffer. It typically occurs during encoding operations when the caller-provided
/// buffer is too small.
///
/// - Distinction from [`PayloadTooLarge`]
///   - `BufferTooSmall`: The encoding operation is valid but needs more buffer space
///   - [`PayloadTooLarge`]: The encoded size would exceed protocol/system limits regardless of buffer size
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("buffer too small for encoding: need {requested} bytes, have {available} bytes")]
pub struct BufferTooSmall {
  requested: NonZeroUsize,
  available: usize,
}

impl BufferTooSmall {
  /// Creates a new `BufferTooSmall` error.
  ///
  /// # Panics
  ///
  /// In debug builds, panics if `requested <= available` (this would not be an error condition).
  pub const fn new(requested: usize, available: usize) -> Self {
    debug_assert!(
      requested > available,
      "BufferTooSmall: requested must be greater than available"
    );
    Self {
      requested: NonZeroUsize::new(requested).expect("BufferTooSmall: requested must be non-zero"),
      available,
    }
  }

  /// Returns the number of bytes needed to complete the encoding operation.
  ///
  /// This is the minimum buffer size that would allow the encoding to succeed.
  #[inline]
  pub const fn requested(&self) -> usize {
    self.requested.get()
  }

  /// Returns the number of bytes available in the provided buffer.
  ///
  /// This is the actual size of the buffer that was insufficient.
  #[inline]
  pub const fn available(&self) -> usize {
    self.available
  }

  /// Returns the additional bytes needed beyond what's available.
  ///
  /// This is equivalent to `requested() - available()`.
  #[inline]
  pub const fn shortage(&self) -> usize {
    self.requested() - self.available()
  }

  const fn update(mut self, requested: usize, available: usize) -> Self {
    debug_assert!(
      requested > available,
      "BufferTooSmall: requested must be greater than available"
    );
    self.requested =
      NonZeroUsize::new(requested).expect("BufferTooSmall: requested must be non-zero");
    self.available = available;
    self
  }
}

/// An error that occurs when encoded data would exceed the maximum allowed size.
///
/// This error represents a hard limit violation where the data cannot be encoded within
/// the protocol or system constraints. Unlike [`BufferTooSmall`], this cannot be resolved
/// by simply providing a larger buffer.
///
/// - Common Causes
///   - Protocol-defined maximum message size (e.g., gRPC's 4MB default limit)
///   - System-imposed limits (e.g., maximum UDP packet size)
///   - Application-defined constraints for safety or performance
///
/// - Distinction from [`BufferTooSmall`]
///   - [`BufferTooSmall`]: "Your buffer is too small, get a bigger one"
///   - `PayloadTooLarge`: "Your data is too large, make it smaller"
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("payload size {size} bytes exceeds maximum limit of {limit} bytes")]
pub struct PayloadTooLarge {
  /// The maximum allowed size
  limit: usize,
  /// The actual size of the encoded payload
  size: usize,
}

impl PayloadTooLarge {
  /// Creates a new `PayloadTooLarge` error.
  ///
  /// # Panics
  ///
  /// In debug builds, panics if `size <= limit` (this would not be an error condition).
  pub const fn new(limit: usize, size: usize) -> Self {
    debug_assert!(
      size > limit,
      "PayloadTooLarge: size must be greater than limit"
    );

    Self { limit, size }
  }

  /// Returns the maximum allowed size in bytes.
  ///
  /// This is the hard limit that cannot be exceeded.
  #[inline]
  pub const fn limit(&self) -> usize {
    self.limit
  }

  /// Returns the actual size of the payload in bytes.
  ///
  /// This is the size that exceeded the limit.
  #[inline]
  pub const fn size(&self) -> usize {
    self.size
  }

  /// Returns how much the payload exceeds the limit.
  ///
  /// This is equivalent to `size() - limit()`.
  #[inline]
  pub const fn excess(&self) -> usize {
    self.size - self.limit
  }

  /// Returns the percentage by which the limit was exceeded.
  ///
  /// For example, if limit is 100 and size is 150, this returns 50.0 (50% over).
  #[cfg(feature = "std")]
  #[inline]
  pub fn excess_percentage(&self) -> f64 {
    ((self.size as f64 / self.limit as f64) - 1.0) * 100.0
  }
}

/// An error that occurs when there is insufficient data to decode a message.
///
/// We attempted to read from a buffer, but we ran out of room before we could read the entire
/// value.
///
/// This error can be fixed by reading more bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InsufficientData {
  available: usize,
  requested: Option<NonZeroUsize>,
}

impl core::fmt::Display for InsufficientData {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self.requested {
      Some(requested) => write!(
        f,
        "insufficient data: available {}, requested {}",
        self.available, requested
      ),
      None => write!(f, "insufficient data: available {}", self.available),
    }
  }
}

impl core::error::Error for InsufficientData {}

impl InsufficientData {
  /// Creates a new `InsufficientData` error.
  #[inline]
  pub const fn new(available: usize) -> Self {
    Self {
      available,
      requested: None,
    }
  }

  /// Creates a new `InsufficientData` error with a requested size.
  ///
  /// # Panics
  /// - Panics if `available >= requested`.
  #[inline]
  pub const fn with_requested(available: usize, requested: usize) -> Self {
    debug_assert!(
      available < requested,
      "InsufficientData: available must be less than requested"
    );

    Self {
      available,
      requested: Some(
        NonZeroUsize::new(requested).expect("InsufficientData: requested must be non-zero"),
      ),
    }
  }

  /// Returns the number of bytes available in the buffer.
  #[inline]
  pub const fn available(&self) -> usize {
    self.available
  }

  /// Returns the number of bytes requested to read.
  ///
  /// This is `None` if the requested size is unknown.
  #[inline]
  pub const fn requested(&self) -> Option<usize> {
    match self.requested {
      Some(requested) => Some(requested.get()),
      None => None,
    }
  }

  #[inline]
  const fn update(mut self, available: usize, requested: Option<NonZeroUsize>) -> Self {
    self.available = available;
    self.requested = requested;
    self
  }
}

/// The encode error
#[derive(Debug, Clone, PartialEq, Eq, derive_more::IsVariant, derive_more::Display)]
pub enum EncodeError<F: Flavor + ?Sized> {
  /// Returned when the encoded buffer is too small to hold the bytes format of the types.
  #[display("{_0}")]
  BufferTooSmall(BufferTooSmall),

  /// Returned when the data in encoded format is larger than the maximum allowed size.
  #[display("{_0}")]
  PayloadTooLarge(PayloadTooLarge),

  /// Returned when trying to encode a value at an offset that is out of bounds.
  #[display("{_0}")]
  OutOfBounds(OutOfBounds),

  /// Returned when the type cannot be encoded in the given wire type format
  #[display("cannot encode {ty} in {wire_type} format in {flavor} flavor", flavor = F::NAME)]
  IncompatibleWireType {
    /// The type of the value.
    ty: &'static str,
    /// The wire type of the value.
    wire_type: F::WireType,
  },

  /// Other encode error.
  #[display("{_0}")]
  #[cfg(any(feature = "std", feature = "alloc"))]
  Other(std::borrow::Cow<'static, str>),

  /// Other encode error.
  #[display("{_0}")]
  #[cfg(not(any(feature = "std", feature = "alloc")))]
  Other(&'static str),
}

impl<F: Flavor + ?Sized> core::error::Error for EncodeError<F> {}

impl<F: Flavor + ?Sized> From<WriteVarintError> for EncodeError<F> {
  #[inline]
  fn from(e: WriteVarintError) -> Self {
    Self::from_varint_error(e)
  }
}

impl<F: Flavor + ?Sized> From<BufferTooSmall> for EncodeError<F> {
  #[inline]
  fn from(e: BufferTooSmall) -> Self {
    Self::BufferTooSmall(e)
  }
}

impl<F: Flavor + ?Sized> From<PayloadTooLarge> for EncodeError<F> {
  #[inline]
  fn from(e: PayloadTooLarge) -> Self {
    Self::PayloadTooLarge(e)
  }
}

impl<F: Flavor + ?Sized> From<TryWriteError> for EncodeError<F> {
  #[inline]
  fn from(e: TryWriteError) -> Self {
    Self::buffer_too_small(e.requested(), e.available())
  }
}

impl<F: Flavor + ?Sized> From<TryWriteAtError> for EncodeError<F> {
  #[inline]
  fn from(e: TryWriteAtError) -> Self {
    match e {
      TryWriteAtError::OutOfBounds(out_of_bounds) => Self::OutOfBounds(out_of_bounds),
      TryWriteAtError::InsufficientSpace(insufficient_space_at) => Self::buffer_too_small(
        insufficient_space_at.requested() + insufficient_space_at.offset(),
        insufficient_space_at.available() + insufficient_space_at.offset(),
      ),
      _ => Self::other("unknown error"),
    }
  }
}

impl<F: Flavor + ?Sized> From<WriteVarintAtError> for EncodeError<F> {
  #[inline]
  fn from(e: WriteVarintAtError) -> Self {
    match e {
      WriteVarintAtError::OutOfBounds(out_of_bounds) => Self::OutOfBounds(out_of_bounds),
      WriteVarintAtError::InsufficientSpace(insufficient_space_at) => Self::buffer_too_small(
        insufficient_space_at.requested() + insufficient_space_at.offset(),
        insufficient_space_at.available() + insufficient_space_at.offset(),
      ),
      _ => Self::other("unknown error"),
    }
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<F: Flavor + ?Sized> From<std::borrow::Cow<'static, str>> for EncodeError<F> {
  fn from(value: std::borrow::Cow<'static, str>) -> Self {
    Self::other(value)
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<F: Flavor + ?Sized> From<std::string::String> for EncodeError<F> {
  fn from(value: std::string::String) -> Self {
    Self::other(value)
  }
}

impl<F: Flavor + ?Sized> From<&'static str> for EncodeError<F> {
  fn from(value: &'static str) -> Self {
    Self::other(value)
  }
}

impl<F: Flavor + ?Sized> EncodeError<F> {
  /// Creates an insufficient buffer error.
  #[inline]
  pub const fn buffer_too_small(requested: usize, available: usize) -> Self {
    Self::BufferTooSmall(BufferTooSmall::new(requested, available))
  }

  /// Creates a payload too large error.
  #[inline]
  pub const fn payload_too_large(maximum: usize, size: usize) -> Self {
    Self::PayloadTooLarge(PayloadTooLarge::new(maximum, size))
  }

  /// Creates an out of bounds error.
  ///
  /// # Panics
  /// Panics if `offset >= length`.
  #[inline]
  pub const fn out_of_bounds(offset: usize, length: usize) -> Self {
    Self::OutOfBounds(OutOfBounds::new(offset, length))
  }

  /// Creates an incompatible wire type error.
  #[inline]
  pub const fn incompatible_wire_type(ty: &'static str, wire_type: F::WireType) -> Self {
    Self::IncompatibleWireType { ty, wire_type }
  }

  /// Creates a new encoding error from a [`WriteVarintError`].
  #[inline]
  pub const fn from_varint_error(e: WriteVarintError) -> Self {
    match e {
      WriteVarintError::InsufficientSpace(e) => {
        Self::BufferTooSmall(BufferTooSmall::new(e.requested(), e.available()))
      }
      #[cfg(any(feature = "std", feature = "alloc"))]
      WriteVarintError::Other(e) => Self::Other(std::borrow::Cow::Borrowed(e)),
      #[cfg(any(feature = "std", feature = "alloc"))]
      _ => Self::Other(std::borrow::Cow::Borrowed("unknown error")),
      #[cfg(not(any(feature = "std", feature = "alloc")))]
      WriteVarintError::Other(e) => Self::Other(e),
      #[cfg(not(any(feature = "std", feature = "alloc")))]
      _ => Self::Other("unknown error"),
    }
  }

  /// Propagating buffer size information.
  ///
  /// This is used when encoding nested structures - if a field fails with a buffer
  /// error, the struct encoder can call this to update the error with the total
  /// struct size requirement instead of just the field requirement.
  pub fn propagate_buffer_info<R, A>(self, requested: R, available: A) -> Self
  where
    R: FnOnce() -> usize,
    A: FnOnce() -> usize,
  {
    match self {
      Self::BufferTooSmall(buffer_err) => {
        Self::BufferTooSmall(buffer_err.update(requested(), available()))
      }
      _ => self,
    }
  }

  /// Creates a other encoding error.
  #[cfg(any(feature = "std", feature = "alloc"))]
  #[inline]
  pub fn other<T>(value: T) -> Self
  where
    T: Into<std::borrow::Cow<'static, str>>,
  {
    Self::Other(value.into())
  }

  /// Creates a other encoding error.
  #[cfg(not(any(feature = "std", feature = "alloc")))]
  #[inline]
  pub const fn other(value: &'static str) -> Self {
    Self::Other(value)
  }
}

/// The decode error
#[derive(Debug, Clone, PartialEq, Eq, derive_more::IsVariant, derive_more::Display)]
pub enum DecodeError<F: Flavor + ?Sized> {
  /// Returned when attempting to read from a buffer, but we ran out of room before we could read the entire
  /// value.
  ///
  /// This error can be fixed by reading more bytes.
  #[display("{_0}")]
  InsufficientData(InsufficientData),

  /// When decoding, a [`Buffer`](super::buffer::Buffer) may be used as a container to store unknown data
  /// or for repeated encoded fields, it will be used to store chunks of data.
  ///
  /// This error can be fixed by increasing the size of the buffer.
  ///
  /// e.g. In the below example, the reference type and the partial reference type will be a `Buffer` type, which
  /// will store multiple chunks of `User`, each chunk contains continuous encoded data of `User` type.
  ///
  /// ```rust,ignore
  /// #[derive(Object)]
  /// struct Data {
  ///   #[grost(list(object(repeated)), tag = 1)]
  ///   field: Vec<User>,
  /// }
  /// ```
  #[display("insufficient buffer capacity, available: {available}, requested: {requested}")]
  InsufficientBufferCapacity {
    /// The required capacity of the buffer.
    requested: NonZeroUsize,
    /// The available capacity of the buffer.
    available: usize,
  },

  /// Returned when the decoded identifier is not the expected identifier.
  #[display("unexpected identifier {actual}, expected {expected}")]
  UnexpectedIdentifier {
    /// The expected identifier for encoding.
    expected: F::Identifier,
    /// The actual identifier for encoding.
    actual: F::Identifier,
  },

  /// Returned when the field is not found but is required when decoding or transforming the data.
  #[display("missing field '{name}' in {struct_name}")]
  MissingSelectedField {
    /// The structure name.
    struct_name: &'static str,
    /// The field name.
    name: &'static str,
  },

  /// Invalid tag value
  #[display("{_0}")]
  InvalidTag(ParseTagError),

  /// Returned when the decoded type overflows the maximum value.
  #[display("decoded value would overflow the target type")]
  Overflow,

  /// Other decoding error.
  #[display("{_0}")]
  #[cfg(any(feature = "std", feature = "alloc"))]
  Other(std::borrow::Cow<'static, str>),

  /// Other decoding error.
  #[display("{_0}")]
  #[cfg(not(any(feature = "std", feature = "alloc")))]
  Other(&'static str),
}

impl<F: Flavor + ?Sized> core::error::Error for DecodeError<F> {}

impl<F: Flavor + ?Sized> From<ReadVarintError> for DecodeError<F> {
  #[inline]
  fn from(e: ReadVarintError) -> Self {
    Self::from_varint_error(e)
  }
}

impl<F: Flavor + ?Sized> From<ParseTagError> for DecodeError<F> {
  #[inline]
  fn from(e: ParseTagError) -> Self {
    Self::parse_tag(e)
  }
}

impl<F: Flavor + ?Sized> From<InsufficientData> for DecodeError<F> {
  #[inline]
  fn from(e: InsufficientData) -> Self {
    Self::InsufficientData(e)
  }
}

impl<F: Flavor + ?Sized> From<TryAdvanceError> for DecodeError<F> {
  #[inline]
  fn from(e: TryAdvanceError) -> Self {
    Self::insufficient_data_with_requested(e.available(), e.requested())
  }
}

impl<F: Flavor + ?Sized> From<TryPeekError> for DecodeError<F> {
  #[inline]
  fn from(e: TryPeekError) -> Self {
    Self::insufficient_data_with_requested(e.available(), e.requested())
  }
}

impl<F: Flavor + ?Sized> From<TryReadError> for DecodeError<F> {
  #[inline]
  fn from(e: TryReadError) -> Self {
    Self::insufficient_data_with_requested(e.available(), e.requested())
  }
}

impl<F: Flavor + ?Sized> From<TrySegmentError> for DecodeError<F> {
  #[inline]
  fn from(e: TrySegmentError) -> Self {
    Self::insufficient_data_with_requested(e.available(), e.end())
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<F: Flavor + ?Sized> From<std::borrow::Cow<'static, str>> for DecodeError<F> {
  fn from(value: std::borrow::Cow<'static, str>) -> Self {
    Self::other(value)
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<F: Flavor + ?Sized> From<std::string::String> for DecodeError<F> {
  fn from(value: std::string::String) -> Self {
    Self::other(value)
  }
}

impl<F: Flavor + ?Sized> From<&'static str> for DecodeError<F> {
  fn from(value: &'static str) -> Self {
    Self::other(value)
  }
}

impl<F: Flavor + ?Sized> DecodeError<F> {
  /// Creates an insufficient data error.
  #[inline]
  pub const fn insufficient_data(available: usize) -> Self {
    Self::InsufficientData(InsufficientData::new(available))
  }

  /// Creates an insufficient data error with a requested size.
  ///
  /// # Panics
  ///
  /// Panics if `available >= requested`.
  #[inline]
  pub const fn insufficient_data_with_requested(available: usize, requested: usize) -> Self {
    Self::InsufficientData(InsufficientData::with_requested(available, requested))
  }

  /// Creates an unexpected identifier error.
  #[inline]
  pub const fn unexpected_identifier(expected: F::Identifier, actual: F::Identifier) -> Self {
    Self::UnexpectedIdentifier { expected, actual }
  }

  /// Creates a missing selected field error.
  #[inline]
  pub const fn missing_selected_field(struct_name: &'static str, field_name: &'static str) -> Self {
    Self::MissingSelectedField {
      struct_name,
      name: field_name,
    }
  }

  /// Creates a new error from the varint error.
  #[inline]
  pub const fn from_varint_error(e: ReadVarintError) -> Self {
    match e {
      ReadVarintError::InsufficientData { available } => Self::insufficient_data(available),
      ReadVarintError::Overflow => Self::Overflow,
      #[cfg(any(feature = "std", feature = "alloc"))]
      ReadVarintError::Other(e) => Self::Other(std::borrow::Cow::Borrowed(e)),
      #[cfg(any(feature = "std", feature = "alloc"))]
      _ => Self::Other(std::borrow::Cow::Borrowed("unknown error")),
      #[cfg(not(any(feature = "std", feature = "alloc")))]
      ReadVarintError::Other(e) => Self::other(e),
      #[cfg(not(any(feature = "std", feature = "alloc")))]
      _ => Self::other("unknown error"),
    }
  }

  /// Creates a new error from the tag decode error.
  #[inline]
  pub const fn from_decode_tag_error(e: DecodeTagError) -> Self {
    match e {
      DecodeTagError::Read(e) => Self::from_varint_error(e),
      DecodeTagError::Parse(e) => Self::from_parse_tag_error(e),
    }
  }

  /// Creates a invalid tag error.
  #[inline]
  pub const fn from_parse_tag_error(err: ParseTagError) -> Self {
    Self::InvalidTag(err)
  }

  /// Propagating insufficient buffer information.
  ///
  /// This is used when decoding nested structures - if a field fails with a buffer
  /// error, the struct decoder can call this to update the error with the total
  /// size requirement instead of just the field requirement.
  #[inline]
  pub fn propagate_buffer_info<R, A>(self, requested: R, available: A) -> Self
  where
    R: FnOnce() -> Option<NonZeroUsize>,
    A: FnOnce() -> usize,
  {
    match self {
      Self::InsufficientData(insufficient_data) => {
        Self::InsufficientData(insufficient_data.update(available(), requested()))
      }
      _ => self,
    }
  }

  /// Creates a other encoding error.
  #[cfg(any(feature = "std", feature = "alloc"))]
  #[inline]
  pub fn other<T>(value: T) -> Self
  where
    T: Into<std::borrow::Cow<'static, str>>,
  {
    Self::Other(value.into())
  }

  /// Creates a other encoding error.
  #[cfg(not(any(feature = "std", feature = "alloc")))]
  #[inline]
  pub const fn other(value: &'static str) -> Self {
    Self::Other(value)
  }
}
