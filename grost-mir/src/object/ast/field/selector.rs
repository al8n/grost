use syn::Attribute;

use crate::object::meta::{FieldSelection, SelectorFieldFromMeta};

#[derive(Debug, Clone)]
pub struct SelectorFieldOptions {
  pub(crate) attrs: Vec<Attribute>,
  pub(crate) select: FieldSelection,
}

impl SelectorFieldOptions {
  /// Returns the attributes of the selector field
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the selection of the selector field
  pub const fn select(&self) -> &FieldSelection {
    &self.select
  }
}

impl SelectorFieldFromMeta {
  /// Finalizes the selector field meta and returns the attribute
  pub fn finalize(self) -> SelectorFieldOptions {
    SelectorFieldOptions {
      attrs: self.attrs,
      select: self.select,
    }
  }
}
