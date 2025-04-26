bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  struct Flags: u8 {
    /// The return error when encountering unsupported wire type is set
    const ERR_ON_UNSUPPORT_WIRE_TYPE = 0b0000_0001;
    /// Should return an error when encountering an unknown tag, or identifier
    const ERR_ON_UNKNOWN = 0b0000_0010;
    /// Should skip the unknown identifier
    const SKIP_UNKNOWN = 0b0000_0100;
  }
}

/// A context for encoding or decoding messages.
///
/// ## Configuration
/// - **Encode**
///   - `maximum_message_size`: The maximum size of the message can be encoded when encoding.
///
/// - **Decode**
///   - `err_on_unsupported_wire_type`: If set, the decoder will return an error when encountering an unsupported wire type.
///   - `err_on_unknown`: If set, the decoder will return an error when encountering an unknown tag or identifier.
///   - `skip_unknown`: If set, the decoder will skip unknown values and moving forward.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Context {
  flags: Flags,
  maximum_message_size: usize,
}

#[cfg(feature = "quickcheck")]
const _: () = {
  use quickcheck::{Arbitrary, Gen};

  impl Arbitrary for Context {
    fn arbitrary(g: &mut Gen) -> Self {
      let flags = Flags::from_bits_truncate(*g.choose(&[1, 2, 4]).unwrap());
      Self {
        flags,
        maximum_message_size: u32::MAX as usize,
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
      maximum_message_size: u32::MAX as usize,
    }
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

  /// Gets the maximum message size for the context.
  #[inline]
  pub const fn maximum_message_size(&self) -> usize {
    self.maximum_message_size
  }

  /// Sets the maximum message size for the context.
  ///
  /// The maximum message size is clamped to the range of 512 to `u32::MAX`.
  #[inline]
  pub const fn set_maximum_message_size(&mut self, maximum_message_size: usize) -> &mut Self {
    self.maximum_message_size = if maximum_message_size > u32::MAX as usize {
      u32::MAX as usize
    } else if maximum_message_size < 512 {
      512
    } else {
      maximum_message_size
    };
    self
  }

  /// Sets the maximum message size for the context.
  ///
  /// The maximum message size is clamped to the range of 512 to `u32::MAX`.
  #[inline]
  pub const fn with_maximum_message_size(mut self, maximum_message_size: usize) -> Self {
    self.maximum_message_size = if maximum_message_size > u32::MAX as usize {
      u32::MAX as usize
    } else if maximum_message_size < 512 {
      512
    } else {
      maximum_message_size
    };
    self
  }
}
