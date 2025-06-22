use crate::utils::Invokable;

use super::meta::TagFromMeta;

impl From<TagFromMeta> for TagOptions {
  fn from(meta: TagFromMeta) -> Self {
    Self {
      constructor: meta.constructor,
      encode: meta.encode,
    }
  }
}

#[derive(Debug, Clone)]
pub struct TagOptions {
  pub(crate) constructor: Invokable,
  pub(crate) encode: Invokable,
}

impl TagOptions {
  pub const fn constructor(&self) -> &Invokable {
    &self.constructor
  }

  pub const fn encode(&self) -> &Invokable {
    &self.encode
  }
}
