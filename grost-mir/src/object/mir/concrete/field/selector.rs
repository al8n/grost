use syn::{Attribute, Type, WherePredicate, punctuated::Punctuated, token::Comma};

use crate::object::FieldSelection;

#[derive(Debug, Clone)]
pub struct ConcreteSelectorField {
  pub(super) ty: Type,
  pub(super) selectable: Type,
  pub(super) attrs: Vec<Attribute>,
  pub(super) default: FieldSelection,
  pub(super) constraints: Punctuated<WherePredicate, Comma>,
}

impl ConcreteSelectorField {
  /// Returns the attributes of the selector field
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the type of the selector field
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the selectable of the selector field
  #[inline]
  pub const fn selectable(&self) -> &Type {
    &self.selectable
  }

  /// Returns the default selection for this field
  #[inline]
  pub const fn selection(&self) -> &FieldSelection {
    &self.default
  }

  /// Returns the type constraints for the selector field
  #[inline]
  pub const fn type_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }
}
