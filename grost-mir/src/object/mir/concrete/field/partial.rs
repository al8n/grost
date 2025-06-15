use syn::{Attribute, Path, Type, WherePredicate, punctuated::Punctuated, token::Comma};

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
    path_to_grost: &Path,
    ty: &Type,
    partial_type: Option<&Type>,
    attrs: &[Attribute],
    use_generics: bool,
  ) -> darling::Result<Self> {
    let mut constraints = Punctuated::new();
    let ty = match partial_type {
      Some(ty) => ty.clone(),
      None => {
        if use_generics {
          constraints.push(syn::parse2(quote! {
            #ty: #path_to_grost::__private::convert::State<
              #path_to_grost::__private::convert::Flatten
            >
          })?);
          constraints.push(syn::parse2(quote! {
            <#ty as #path_to_grost::__private::convert::State<#path_to_grost::__private::convert::Flatten>>::Output: ::core::marker::Sized
          })?);
        }

        syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flatten
          >>::Output
        })?
      }
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
