bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  struct Flags: u8 {
    /// The return error when encountering unsupported wire type is set in decoding
    const ERR_ON_UNSUPPORT_WIRE_TYPE = 0b0000_0001;
    /// Should return an error when encountering an unknown tag, or identifier in decoding
    const ERR_ON_UNKNOWN = 0b0000_0010;
    /// Should skip the unknown identifier and its data when encoding or decoding
    const SKIP_UNKNOWN = 0b0000_0100;
    /// Should return an error when encountering duplicated keys in a set
    const ERR_ON_DUPLICATED_KEYS = 0b0000_1000;
    /// Should return an error when encountering duplicated keys in a map
    const ERR_ON_DUPLICATED_MAP_KEYS = 0b0001_0000;
    /// Should return an error when length mismatch when decoding a packed encoded collections
    const ERR_ON_LENGTH_MISMATCH = 0b0010_0000;
  }
}

/// Policy for decoding repeated fields in collections.
///
/// This enum determines the memory allocation strategy when decoding repeated fields
/// into collections (maps, sets, vectors, etc.). Each strategy represents a different
/// trade-off between processing time and memory allocation efficiency.
#[derive(
  Debug, Default, Clone, Copy, PartialEq, Eq, Hash, derive_more::IsVariant, derive_more::Display,
)]
#[non_exhaustive]
pub enum RepeatedDecodePolicy {
  /// Pre-allocate collection capacity by traversing source data twice.
  ///
  /// This strategy first counts all elements to determine the exact capacity needed,
  /// then allocates the collection with that capacity and decodes elements in a
  /// second pass.
  ///
  /// **Advantages:**
  /// - Minimizes memory allocations and reallocations
  /// - Reduces memory fragmentation
  /// - Better performance for large repeated fields
  /// - Predictable memory usage
  ///
  /// **Trade-offs:**
  /// - Requires two passes through the source data
  /// - Higher initial processing overhead
  /// - May over-allocate if source contains invalid data
  ///
  /// **Best for:** Large collections where memory efficiency is prioritized over
  /// processing speed.
  #[default]
  #[display("preallocate-capacity")]
  PreallocateCapacity,

  /// Grow collection capacity incrementally during decoding.
  ///
  /// This strategy decodes elements one by one in a single pass, allowing the
  /// collection to grow dynamically through standard reallocation mechanisms
  /// (typically doubling capacity when full).
  ///
  /// **Advantages:**
  /// - Single pass through source data
  /// - Lower initial processing overhead
  /// - Memory usage grows only as needed
  /// - Faster startup time
  ///
  /// **Trade-offs:**
  /// - May cause frequent reallocations for large collections
  /// - Potential memory fragmentation from multiple allocations
  /// - Less predictable memory usage patterns
  /// - Temporary over-allocation during growth phases
  ///
  /// **Best for:** Small to medium collections or scenarios where minimizing
  /// initial latency is important.
  #[display("grow-incrementally")]
  GrowIncrementally,
}

/// A context for encoding or decoding messages.
///
/// ## Configuration
///
/// - **Decode**
///   - `err_on_unsupported_wire_type`: If set, the decoder will return an error when encountering an unsupported wire type.
///   - `err_on_unknown`: If set, the decoder will return an error when encountering an unknown tag or identifier.
///   - `skip_unknown`: If set, the decoder will skip unknown values and moving forward.
///   - `err_on_duplicated_set_keys`: If set, the decoder will return an error when encountering duplicated keys in a set when decoding.
///   - `err_on_duplicated_map_keys`: If set, the decoder will return an error when encountering duplicated keys in a map when decoding.
///   - `err_on_length_mismatch`: If set, the decoder will return an error when the length of a packed encoded collection does not match the expected length when decoding.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Context {
  flags: Flags,
  repeated_decode_policy: RepeatedDecodePolicy,
}

#[cfg(feature = "quickcheck")]
const _: () = {
  use quickcheck::{Arbitrary, Gen};

  impl Arbitrary for Context {
    fn arbitrary(g: &mut Gen) -> Self {
      let flags = Flags::from_bits_truncate(*g.choose(&[1, 2, 4]).unwrap());
      Self {
        flags,
        repeated_decode_policy: if bool::arbitrary(g) {
          RepeatedDecodePolicy::PreallocateCapacity
        } else {
          RepeatedDecodePolicy::GrowIncrementally
        },
      }
    }
  }
};

impl Default for Context {
  fn default() -> Self {
    Self::new()
  }
}

impl Context {
  /// Creates a new context with default configuration
  #[inline]
  pub const fn new() -> Self {
    Self {
      flags: Flags::empty(),
      repeated_decode_policy: RepeatedDecodePolicy::PreallocateCapacity,
    }
  }

  /// Sets the repeated decode policy for the context.
  #[inline]
  pub const fn set_repeated_decode_policy(&mut self, policy: RepeatedDecodePolicy) -> &mut Self {
    self.repeated_decode_policy = policy;
    self
  }

  /// Sets the repeated decode policy for the context.
  #[inline]
  pub const fn with_repeated_decode_policy(mut self, policy: RepeatedDecodePolicy) -> Self {
    self.repeated_decode_policy = policy;
    self
  }

  /// Gets the repeated decode policy for the context.
  #[inline]
  pub const fn repeated_decode_policy(&self) -> RepeatedDecodePolicy {
    self.repeated_decode_policy
  }

  /// Sets the return error when encountering unsupported wire type for the context.
  #[inline]
  pub const fn set_err_on_unsupported_wire_type(&mut self) -> &mut Self {
    self.flags = self.flags.union(Flags::ERR_ON_UNSUPPORT_WIRE_TYPE);
    self
  }

  /// Sets the return error when encountering unsupported wire type for the context.
  #[inline]
  pub const fn with_err_on_unsupported_wire_type(mut self) -> Self {
    self.flags = self.flags.union(Flags::ERR_ON_UNSUPPORT_WIRE_TYPE);
    self
  }

  /// Unsets the return error when encountering unsupported wire type for the context.
  #[inline]
  pub const fn unset_err_on_unsupported_wire_type(&mut self) -> &mut Self {
    self.flags = self.flags.difference(Flags::ERR_ON_UNSUPPORT_WIRE_TYPE);
    self
  }

  /// Unsets the return error when encountering unsupported wire type for the context.
  #[inline]
  pub const fn without_err_on_unsupported_wire_type(mut self) -> Self {
    self.flags = self.flags.difference(Flags::ERR_ON_UNSUPPORT_WIRE_TYPE);
    self
  }

  /// Gets if the return error when encountering unsupported wire type is set for the context.
  #[inline]
  pub const fn err_on_unsupported_wire_type(&self) -> bool {
    self.flags.contains(Flags::ERR_ON_UNSUPPORT_WIRE_TYPE)
  }

  /// Sets the skip unknown flag for the context.
  #[inline]
  pub const fn set_skip_unknown(&mut self) -> &mut Self {
    self.flags = self.flags.union(Flags::SKIP_UNKNOWN);
    self
  }

  /// Sets the skip unknown flag for the context.
  #[inline]
  pub const fn with_skip_unknown(mut self) -> Self {
    self.flags = self.flags.union(Flags::SKIP_UNKNOWN);
    self
  }

  /// Unsets the skip unknown flag for the context.
  #[inline]
  pub const fn unset_skip_unknown(&mut self) -> &mut Self {
    self.flags = self.flags.difference(Flags::SKIP_UNKNOWN);
    self
  }

  /// Unsets the skip unknown flag for the context.
  #[inline]
  pub const fn without_skip_unknown(mut self) -> Self {
    self.flags = self.flags.difference(Flags::SKIP_UNKNOWN);
    self
  }

  /// Gets if the skip unknown flag is set for the context.
  #[inline]
  pub const fn skip_unknown(&self) -> bool {
    self.flags.contains(Flags::SKIP_UNKNOWN)
  }

  /// Sets the returning error on unknown identifier flag for the context.
  #[inline]
  pub const fn set_err_on_unknown(&mut self) -> &mut Self {
    self.flags = self.flags.union(Flags::SKIP_UNKNOWN);
    self
  }

  /// Sets the returning error on unknown identifier flag for the context.
  #[inline]
  pub const fn with_err_on_unknown(mut self) -> Self {
    self.flags = self.flags.union(Flags::SKIP_UNKNOWN);
    self
  }

  /// Unsets the returning error on unknown identifier flag for the context.
  #[inline]
  pub const fn unset_err_on_unknown(&mut self) -> &mut Self {
    self.flags = self.flags.difference(Flags::SKIP_UNKNOWN);
    self
  }

  /// Unsets the returning error on unknown identifier flag for the context.
  #[inline]
  pub const fn without_err_on_unknown(mut self) -> Self {
    self.flags = self.flags.difference(Flags::SKIP_UNKNOWN);
    self
  }

  /// Gets if the returning error on unknown identifier flag is set for the context.
  #[inline]
  pub const fn err_on_unknown(&self) -> bool {
    self.flags.contains(Flags::SKIP_UNKNOWN)
  }

  /// Sets the returning error on duplicated keys in a set flag for the context.
  #[inline]
  pub const fn set_err_on_duplicated_set_keys(&mut self) -> &mut Self {
    self.flags = self.flags.union(Flags::ERR_ON_DUPLICATED_KEYS);
    self
  }

  /// Sets the returning error on duplicated keys in a set flag for the context.
  #[inline]
  pub const fn with_err_on_duplicated_set_keys(mut self) -> Self {
    self.flags = self.flags.union(Flags::ERR_ON_DUPLICATED_KEYS);
    self
  }

  /// Unsets the returning error on duplicated keys in a set flag for the context.
  #[inline]
  pub const fn unset_err_on_duplicated_set_keys(&mut self) -> &mut Self {
    self.flags = self.flags.difference(Flags::ERR_ON_DUPLICATED_KEYS);
    self
  }

  /// Unsets the returning error on duplicated keys in a set flag for the context.
  #[inline]
  pub const fn without_err_on_duplicated_set_keys(mut self) -> Self {
    self.flags = self.flags.difference(Flags::ERR_ON_DUPLICATED_KEYS);
    self
  }

  /// Gets if the returning error on duplicated keys in a set flag is set for the context.
  #[inline]
  pub const fn err_on_duplicated_set_keys(&self) -> bool {
    self.flags.contains(Flags::ERR_ON_DUPLICATED_KEYS)
  }

  /// Sets the returning error on duplicated keys in a map flag for the context.
  #[inline]
  pub const fn set_err_on_duplicated_map_keys(&mut self) -> &mut Self {
    self.flags = self.flags.union(Flags::ERR_ON_DUPLICATED_MAP_KEYS);
    self
  }

  /// Sets the returning error on duplicated keys in a map flag for the context.
  #[inline]
  pub const fn with_err_on_duplicated_map_keys(mut self) -> Self {
    self.flags = self.flags.union(Flags::ERR_ON_DUPLICATED_MAP_KEYS);
    self
  }

  /// Unsets the returning error on duplicated keys in a map flag for the context.
  #[inline]
  pub const fn unset_err_on_duplicated_map_keys(&mut self) -> &mut Self {
    self.flags = self.flags.difference(Flags::ERR_ON_DUPLICATED_MAP_KEYS);
    self
  }

  /// Unsets the returning error on duplicated keys in a map flag for the context.
  #[inline]
  pub const fn without_err_on_duplicated_map_keys(mut self) -> Self {
    self.flags = self.flags.difference(Flags::ERR_ON_DUPLICATED_MAP_KEYS);
    self
  }

  /// Gets if the returning error on duplicated keys in a map flag is set for the context.
  #[inline]
  pub const fn err_on_duplicated_map_keys(&self) -> bool {
    self.flags.contains(Flags::ERR_ON_DUPLICATED_MAP_KEYS)
  }

  /// Sets the returning error on length mismatch when decoding a packed encoded collections flag for the context.
  #[inline]
  pub const fn set_err_on_length_mismatch(&mut self) -> &mut Self {
    self.flags = self.flags.union(Flags::ERR_ON_LENGTH_MISMATCH);
    self
  }

  /// Sets the returning error on length mismatch when decoding a packed encoded collections flag for the context.
  #[inline]
  pub const fn with_err_on_length_mismatch(mut self) -> Self {
    self.flags = self.flags.union(Flags::ERR_ON_LENGTH_MISMATCH);
    self
  }

  /// Unsets the returning error on length mismatch when decoding a packed encoded collections flag for the context.
  #[inline]
  pub const fn unset_err_on_length_mismatch(&mut self) -> &mut Self {
    self.flags = self.flags.difference(Flags::ERR_ON_LENGTH_MISMATCH);
    self
  }

  /// Unsets the returning error on length mismatch when decoding a packed encoded collections flag for the context.
  #[inline]
  pub const fn without_err_on_length_mismatch(mut self) -> Self {
    self.flags = self.flags.difference(Flags::ERR_ON_LENGTH_MISMATCH);
    self
  }

  /// Gets if the returning error on length mismatch when decoding a packed encoded collections flag is set for the context.
  #[inline]
  pub const fn err_on_length_mismatch(&self) -> bool {
    self.flags.contains(Flags::ERR_ON_LENGTH_MISMATCH)
  }
}

impl Context {
  #[inline]
  pub(crate) fn err_length_mismatch(
    &self,
    expected: usize,
    actual: usize,
  ) -> Result<(), super::DecodeError> {
    if self.err_on_length_mismatch() && expected != actual {
      #[cfg(any(feature = "std", feature = "alloc"))]
      let err = format!("expected {expected} elements, but got {actual} elements");
      #[cfg(not(any(feature = "std", feature = "alloc")))]
      let err = "expected elements not match actual elements";
      Err(super::DecodeError::other(err))
    } else {
      Ok(())
    }
  }

  #[inline]
  pub(crate) fn err_duplicated_set_keys(&self, duplicated: bool) -> Result<(), super::DecodeError> {
    if duplicated && self.err_on_duplicated_set_keys() {
      Err(super::DecodeError::other("duplicated keys in set"))
    } else {
      Ok(())
    }
  }

  #[inline]
  pub(crate) fn err_duplicated_map_keys(&self, duplicated: bool) -> Result<(), super::DecodeError> {
    if duplicated && self.err_on_duplicated_map_keys() {
      Err(super::DecodeError::other("duplicated keys in map"))
    } else {
      Ok(())
    }
  }
}
