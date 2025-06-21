use syn::{Attribute, Type};

use crate::object::meta::PartialFieldFromMeta;

impl PartialFieldFromMeta {
  /// Finalizes the partial field meta and returns the attribute
  pub fn finalize(self) -> PartialFieldOptions {
    PartialFieldOptions {
      attrs: self.attrs,
      ty: self.ty,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialFieldOptions {
  attrs: Vec<Attribute>,
  ty: Option<Type>,
}

impl PartialFieldOptions {
  /// Returns the attributes of the partial object field
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the type of the partial object field
  pub const fn ty(&self) -> Option<&Type> {
    self.ty.as_ref()
  }
}
