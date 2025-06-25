use syn::{Attribute, Type, WherePredicate, punctuated::Punctuated, token::Comma};

use crate::object::meta::concrete::PartialFieldFromMeta;

use super::PartialFieldConvertOptions;

impl PartialFieldFromMeta {
  /// Finalizes the partial field meta and returns the attribute
  pub(super) fn finalize(self) -> darling::Result<PartialFieldOptions> {
    Ok(PartialFieldOptions {
      ty: self.ty,
      copy: self.copy,
      attrs: self.attrs,
      partial_transform_ref: self.partial_transform_ref.finalize()?,
      transform_ref: self.transform_ref.finalize()?,
      partial_transform: self.partial_transform.finalize()?,
    })
  }
}

#[derive(Debug, Clone)]
pub(super) struct PartialFieldOptions {
  pub(super) ty: Option<Type>,
  pub(super) copy: bool,
  pub(super) attrs: Vec<Attribute>,
  pub(super) transform_ref: PartialFieldConvertOptions,
  pub(super) partial_transform_ref: PartialFieldConvertOptions,
  pub(super) partial_transform: PartialFieldConvertOptions,
}

#[derive(Debug, Clone)]
pub struct PartialField {
  pub(super) ty: Type,
  pub(super) optional_type: Type,
  pub(super) state_type: Option<Type>,
  pub(super) attrs: Vec<Attribute>,
  pub(super) constraints: Punctuated<WherePredicate, Comma>,
  pub(super) transform_ref: PartialFieldConvertOptions,
  pub(super) partial_transform_ref: PartialFieldConvertOptions,
  pub(super) partial_transform: PartialFieldConvertOptions,
}

impl PartialField {
  /// Returns the specified type of the partial field, if any.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the optional type of the partial field, which is `Option<_>`.
  #[inline]
  pub const fn optional_type(&self) -> &Type {
    &self.optional_type
  }

  /// Returns the partial state type of the partial field, if any.
  #[inline]
  pub const fn state_type(&self) -> Option<&Type> {
    self.state_type.as_ref()
  }

  /// Returns the attributes of the partial field.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the transformation options for the partial field.
  #[inline]
  pub const fn transform_ref(&self) -> &PartialFieldConvertOptions {
    &self.transform_ref
  }

  /// Returns the partial transformation options for the partial field.
  #[inline]
  pub const fn partial_transform_ref(&self) -> &PartialFieldConvertOptions {
    &self.partial_transform_ref
  }

  /// Returns the partial transformation options for the partial field.
  #[inline]
  pub const fn partial_transform(&self) -> &PartialFieldConvertOptions {
    &self.partial_transform
  }

  /// Returns the type constraints of the partial field.
  #[inline]
  pub const fn type_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }
}
