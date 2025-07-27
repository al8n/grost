use crate::utils::Invokable;

use super::meta::IdentifierFromMeta;

impl From<IdentifierFromMeta> for IdentifierOptions {
  fn from(meta: IdentifierFromMeta) -> Self {
    Self {
      constructor: meta.constructor,
      encode: meta.encode,
    }
  }
}

#[derive(Debug, Clone)]
pub struct IdentifierOptions {
  pub(crate) constructor: Invokable,
  pub(crate) encode: Invokable,
}

impl IdentifierOptions {
  pub const fn constructor(&self) -> &Invokable {
    &self.constructor
  }

  pub const fn encode(&self) -> &Invokable {
    &self.encode
  }
}
