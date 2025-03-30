/// Invalid tag error
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("tag {0} overflow the maximum tag value {max}", max = (1u32 << 29) - 1)]
pub struct TagOverflow(u32);

/// Tag is a unique identifier for a fie
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[display("{_0}")]
pub struct Tag(pub(super) u32);

impl Tag {
  /// Create a new tag
  #[inline]
  pub const fn new(tag: u32) -> Result<Self, TagOverflow> {
    const MAX: u32 = (1u32 << 29) - 1;

    if tag >= MAX {
      Err(TagOverflow(tag))
    } else {
      Ok(Self(tag))
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
