use syn::{Attribute, Ident};

use crate::object::meta::IndexerFromMeta;

impl From<IndexerFromMeta> for IndexerAttribute {
  fn from(meta: IndexerFromMeta) -> Self {
    Self {
      name: meta.name,
      attrs: meta.attrs,
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct IndexerAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
}

impl IndexerAttribute {
  /// Returns the name of the indexer
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the indexer
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}
