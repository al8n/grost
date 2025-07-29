macro_rules! try_op_error {
  (
    #[doc = $doc:literal]
    #[error($msg:literal)]
    $op:ident
  ) => {
    paste::paste! {
      #[doc = $doc]
      #[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
      #[error($msg)]
      pub struct [< Try $op:camel Error >] {
        requested: usize,
        available: usize,
      }

      impl [< Try $op:camel Error >] {
        #[doc = "Creates a new `Try" $op:camel "Error` with the requested and available bytes."]
        #[inline]
        pub const fn new(requested: usize, available: usize) -> Self {
          Self {
            requested,
            available,
          }
        }

        /// Returns the number of bytes requested.
        #[inline]
        pub const fn requested(&self) -> usize {
          self.requested
        }

        /// Returns the number of bytes available.
        #[inline]
        pub const fn available(&self) -> usize {
          self.available
        }
      }
    }
  };
}

try_op_error!(
  #[doc = "An error that occurs when trying to advance the buffer."]
  #[error(
    "Not enough bytes remaining in buffer to advance (requested {requested} but only {available} available)"
  )]
  advance
);

try_op_error!(
  #[doc = "An error that occurs when trying to read a value from the buffer."]
  #[error(
    "Not enough bytes remaining in buffer to read value (requested {requested} but only {available} available)"
  )]
  read
);

try_op_error!(
  #[doc = "An error that occurs when trying to peek a value from the buffer."]
  #[error(
    "Not enough bytes remaining in buffer to peek value (requested {requested} but only {available} available)"
  )]
  peek
);

try_op_error!(
  #[doc = "An error that occurs when trying to write to the buffer."]
  #[error(
    "Not enough bytes remaining in buffer to write value (requested {requested} but only {available} available)"
  )]
  write
);

/// An error that occurs when trying to create a segment from the read buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("Segment range out of bounds (range {start}..{end} but only {buffer_len} is available)")]
pub struct TrySegmentError {
  start: usize,
  end: usize,
  buffer_len: usize,
}

impl TrySegmentError {
  /// Creates a new `TrySegmentError`.
  #[inline]
  pub const fn new(start: usize, end: usize, buffer_len: usize) -> Self {
    Self {
      start,
      end,
      buffer_len,
    }
  }

  /// Returns the start of the requested range.
  #[inline]
  pub const fn start(&self) -> usize {
    self.start
  }

  /// Returns the end of the requested range.
  #[inline]
  pub const fn end(&self) -> usize {
    self.end
  }

  /// Returns the length of the buffer.
  #[inline]
  pub const fn available(&self) -> usize {
    self.buffer_len
  }
}

#[cfg(feature = "bytes_1")]
const _: () = {
  use bytes_1::TryGetError;

  impl From<TryGetError> for TryAdvanceError {
    fn from(e: TryGetError) -> Self {
      TryAdvanceError::new(e.requested, e.available)
    }
  }

  impl From<TryGetError> for TryReadError {
    fn from(e: TryGetError) -> Self {
      TryReadError::new(e.requested, e.available)
    }
  }
};
