use syn::Attribute;

use crate::meta::object::{Selection, field::SelectorFieldFromMeta};

#[derive(Debug, Clone)]
pub struct SelectorFieldAttribute {
  attrs: Vec<Attribute>,
  select: Selection,
}

impl SelectorFieldAttribute {
  /// Returns the attributes of the selector field
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the selection of the selector field
  pub const fn select(&self) -> &Selection {
    &self.select
  }
}

impl SelectorFieldFromMeta {
  /// Finalizes the selector field meta and returns the attribute
  pub(super) fn finalize(self) -> SelectorFieldAttribute {
    SelectorFieldAttribute {
      attrs: self.attrs,
      select: self.select,
    }
  }
}
