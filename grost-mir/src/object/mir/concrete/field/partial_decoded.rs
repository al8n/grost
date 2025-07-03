use syn::{Attribute, Type, WherePredicate, punctuated::Punctuated, token::Comma};

use crate::object::ConvertAttribute;

#[derive(Debug, Clone)]
pub struct ConcretePartialRefField {
  pub(super) ty: Type,
  pub(super) nullable_type: Type,
  pub(super) partial_ref_state_type: Option<Type>,
  pub(super) decode_trait_type: Type,
  pub(super) attrs: Vec<Attribute>,
  pub(super) constraints: Punctuated<WherePredicate, Comma>,
  pub(super) copy: bool,
  pub(super) convert: ConvertAttribute,
}

impl ConcretePartialRefField {
  /// Returns the type of the partial decoded field.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the nullable type of the partial decoded field, which is `Option<_>`.
  #[inline]
  pub const fn nullable_type(&self) -> &Type {
    &self.nullable_type
  }

  /// Returns the decoded state type of the partial decoded field.
  ///
  /// Returns `Some(_)` only if the field use generics,
  #[inline]
  pub const fn partial_ref_state_type(&self) -> Option<&Type> {
    self.partial_ref_state_type.as_ref()
  }

  /// Returns the field decode trait type for this partial decoded field.
  #[inline]
  pub const fn decode_trait_type(&self) -> &Type {
    &self.decode_trait_type
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

  /// Returns the convert attribute about how to convert the partial decoded field to the partial field.
  #[inline]
  pub const fn convert(&self) -> &ConvertAttribute {
    &self.convert
  }
}
