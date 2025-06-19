use syn::{Attribute, Type, WherePredicate, punctuated::Punctuated, token::Comma};

use quote::quote;

#[derive(Debug, Clone)]
pub struct ConcretePartialField {
  ty: Type,
  optional_type: Type,
  attrs: Vec<Attribute>,
  constraints: Punctuated<WherePredicate, Comma>,
}

impl ConcretePartialField {
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

  /// Returns the constraints of the partial field.
  #[inline]
  pub const fn type_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }

  /// Returns the attributes of the partial field.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  pub(super) fn from_ast(
    ty: &Type,
    partial_type: Option<&Type>,
    attrs: &[Attribute],
  ) -> darling::Result<Self> {
    let constraints = Punctuated::new();
    let ty = match partial_type {
      Some(ty) => ty.clone(),
      None => ty.clone(),
    };

    let optional_type = syn::parse2(quote! {
      ::core::option::Option<#ty>
    })?;

    Ok(Self {
      ty,
      optional_type,
      constraints,
      attrs: attrs.to_vec(),
    })
  }
}
