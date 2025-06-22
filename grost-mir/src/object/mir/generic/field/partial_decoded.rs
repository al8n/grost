use syn::{Attribute, Type, WherePredicate, punctuated::Punctuated, token::Comma};

#[derive(Debug, Clone)]
pub struct GenericPartialRefField {
  pub(super) ty: Type,
  pub(super) optional_type: Type,
  pub(super) attrs: Vec<Attribute>,
  pub(super) constraints: Punctuated<WherePredicate, Comma>,
  pub(super) copy: bool,
}

impl GenericPartialRefField {
  /// Returns the type of the partial decoded field.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the optional type of the partial decoded field, which is `Option<_>`.
  #[inline]
  pub const fn optional_type(&self) -> &Type {
    &self.optional_type
  }

  /// Returns the constraints of the partial decoded field.
  #[inline]
  pub const fn type_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }

  /// Returns the attributes of the partial decoded field.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial decoded field is copyable.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }
}
