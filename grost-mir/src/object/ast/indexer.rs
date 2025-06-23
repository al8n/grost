use syn::{Attribute, Ident};

use crate::object::meta::IndexerFromMeta;

impl From<IndexerFromMeta> for IndexerOptions {
  fn from(meta: IndexerFromMeta) -> Self {
    Self {
      name: meta.name,
      attrs: meta.attrs,
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct IndexerOptions {
  pub(crate) name: Option<Ident>,
  pub(crate) attrs: Vec<Attribute>,
}

impl IndexerOptions {
  /// Returns the name of the indexer
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the indexer
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}
