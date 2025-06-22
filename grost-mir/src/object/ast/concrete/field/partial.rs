use syn::{Attribute, Type};
use quote::quote;

use super::{PartialFieldOptions, FieldConvertOptions};

#[derive(Debug, Clone)]
pub struct PartialField {
  pub(super) ty: Type,
  pub(super) optional_type: Type,
  pub(super) attrs: Vec<Attribute>,
  pub(super) transform: FieldConvertOptions,
  pub(super) partial_transform: FieldConvertOptions,
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

  /// Returns the attributes of the partial field.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  pub(super) fn from_options(
    ty: &Type,
    opts: PartialFieldOptions,
  ) -> darling::Result<Self> {
    let optional_type = syn::parse2(quote! {
      ::core::option::Option<#ty>
    })?;

    Ok(Self {
      ty: ty.clone(),
      optional_type,
      attrs: opts.attrs,
      transform: opts.transform,
      partial_transform: opts.partial_transform
    })
  }
}
