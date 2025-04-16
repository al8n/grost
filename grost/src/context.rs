use grost_types::Tag;

bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  struct Flags: u8 {
    /// The length delimiter is set
    const LENGTH_DELIMITER = 0b0000_0001;
    /// The fixed flag is set
    const FIXED = 0b0000_0010;
  }
}

/// A context for encoding or decoding messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Context {
  tag: Option<Tag>,
  flags: Flags,
}

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
      tag: None,
      flags: Flags::empty(),
    }
  }

  /// Sets the tag for the context.
  #[inline]
  pub const fn set_tag(&mut self, tag: Tag) -> &mut Self {
    self.tag = Some(tag);
    self
  }

  /// Unsets the tag for the context.
  #[inline]
  pub const fn unset_tag(&mut self) -> &mut Self {
    self.tag = None;
    self
  }

  /// Sets or unsets the length delimiter for the context.
  #[inline]
  pub const fn optional_tag(&mut self, tag: Option<Tag>) -> &mut Self {
    self.tag = tag;
    self
  }

  /// Sets the tag for the context.
  #[inline]
  pub const fn with_tag(mut self, tag: Tag) -> Self {
    self.tag = Some(tag);
    self
  }

  /// Unsets the tag for the context.
  #[inline]
  pub const fn without_tag(mut self) -> Self {
    self.tag = None;
    self
  }

  /// Sets or unsets the length delimiter for the context.
  #[inline]
  pub const fn maybe_tag(mut self, tag: Option<Tag>) -> Self {
    self.tag = tag;
    self
  }

  /// Gets the tag for the context.
  #[inline]
  pub const fn tag(&self) -> Option<Tag> {
    self.tag
  }

  /// Sets the length delimiter for the context.
  #[inline]
  pub const fn set_length_delimiter(&mut self) -> &mut Self {
    self.flags = self.flags.union(Flags::LENGTH_DELIMITER);
    self
  }

  /// Sets the length delimiter for the context.
  #[inline]
  pub const fn with_length_delimiter(mut self) -> Self {
    self.flags = self.flags.union(Flags::LENGTH_DELIMITER);
    self
  }

  /// Unsets the length delimiter for the context.
  #[inline]
  pub const fn unset_length_delimiter(&mut self) -> &mut Self {
    self.flags = self.flags.difference(Flags::LENGTH_DELIMITER);
    self
  }

  /// Unsets the length delimiter for the context.
  #[inline]
  pub const fn without_length_delimiter(mut self) -> Self {
    self.flags = self.flags.difference(Flags::LENGTH_DELIMITER);
    self
  }

  /// Gets if the length delimiter is set for the context.
  #[inline]
  pub const fn length_delimiter(&self) -> bool {
    self.flags.contains(Flags::LENGTH_DELIMITER)
  }

  /// Sets the fixed flag for the context.
  #[inline]
  pub const fn set_fixed(&mut self) -> &mut Self {
    self.flags = self.flags.union(Flags::FIXED);
    self
  }

  /// Sets the fixed flag for the context.
  #[inline]
  pub const fn with_fixed(mut self) -> Self {
    self.flags = self.flags.union(Flags::FIXED);
    self
  }

  /// Unsets the fixed flag for the context.
  #[inline]
  pub const fn unset_fixed(&mut self) -> &mut Self {
    self.flags = self.flags.difference(Flags::FIXED);
    self
  }

  /// Unsets the fixed flag for the context.
  #[inline]
  pub const fn without_fixed(mut self) -> Self {
    self.flags = self.flags.difference(Flags::FIXED);
    self
  }

  /// Gets if the fixed flag is set for the context.
  #[inline]
  pub const fn fixed(&self) -> bool {
    self.flags.contains(Flags::FIXED)
  }
}
