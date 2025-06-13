use syn::{Attribute, Type};

use crate::object::meta::PartialDecodedFieldFromMeta;

impl PartialDecodedFieldFromMeta {
  /// Finalizes the partial decoded field meta and returns the attribute
  pub fn finalize(self) -> PartialDecodedFieldAttribute {
    PartialDecodedFieldAttribute {
      copy: self.copy,
      attrs: self.attrs,
      ty: self.ty,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialDecodedFieldAttribute {
  pub(crate) copy: bool,
  pub(crate) attrs: Vec<Attribute>,
  pub(crate) ty: Option<Type>,
}

impl PartialDecodedFieldAttribute {
  /// Returns the attributes of the partial reference object field
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial reference object field is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the type of the partial decoded object field
  pub const fn ty(&self) -> Option<&Type> {
    self.ty.as_ref()
  }
}
