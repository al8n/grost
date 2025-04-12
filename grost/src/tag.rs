/// Invalid tag error
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("tag value {0} is not in range 1..={max}", max = (1u32 << 29) - 1)]
pub struct TagError(u32);

/// Tag is a unique identifier for a fie
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[display("{_0}")]
pub struct Tag(pub(super) u32);

impl Tag {
  /// The maximum Tag value is `2^29 - 1` (536870911).
  pub const MAX: Self = Self((1u32 << 29) - 1);
  /// The minimum Tag value is `1`.
  pub const MIN: Self = Self(1);

  /// Try to create a new tag with the given value,
  /// returning an error if the value is not in range `1..=536870911`.
  #[inline]
  pub const fn try_new(tag: u32) -> Result<Self, TagError> {
    const MAX: u32 = (1u32 << 29) - 1;

    if tag > MAX || tag == 0 {
      Err(TagError(tag))
    } else {
      Ok(Self(tag))
    }
  }

  /// Create a new tag.
  ///
  /// ## Panics
  ///
  /// This function will panic if the value is not in range `1..=536870911`.
  #[inline]
  pub const fn new(tag: u32) -> Self {
    const MAX: u32 = (1u32 << 29) - 1;

    if tag > MAX || tag == 0 {
      panic!("the tag value must in range 1..=536870911");
    } else {
      Self(tag)
    }
  }

  /// Returns the tag as a `u32`.
  #[inline]
  pub const fn get(&self) -> u32 {
    self.0
  }
}

impl From<Tag> for u32 {
  fn from(tag: Tag) -> u32 {
    tag.0
  }
}
