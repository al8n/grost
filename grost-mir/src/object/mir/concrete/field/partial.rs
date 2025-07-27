use syn::{Attribute, Type, WherePredicate, punctuated::Punctuated, token::Comma};

use quote::quote;

#[derive(Debug, Clone)]
pub struct ConcretePartialField {
  ty: Type,
  nullable_type: Type,
  attrs: Vec<Attribute>, 
  // convert: ConvertAttribute,
}

impl ConcretePartialField {
  /// Returns the specified type of the partial field, if any.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the nullable type of the partial field, which is `Option<_>`.
  #[inline]
  pub const fn nullable_type(&self) -> &Type {
    &self.nullable_type
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

    let nullable_type = syn::parse2(quote! {
      ::core::option::Option<#ty>
    })?;

    Ok(Self {
      ty,
      nullable_type,
      constraints,
      attrs: attrs.to_vec(),
    })
  }
}
