use syn::{Attribute, Path, Type, WherePredicate, punctuated::Punctuated, token::Comma};

use quote::quote;

#[derive(Debug, Clone)]
pub struct GenericPartialField {
  ty: Type,
  nullable_type: Type,
  attrs: Vec<Attribute>,
  constraints: Punctuated<WherePredicate, Comma>,
}

impl GenericPartialField {
  /// Returns the type of the generic partial field.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the attributes of the generic partial field.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the nullable type of the generic partial field, which is `Option<_>`.
  #[inline]
  pub const fn nullable_type(&self) -> &Type {
    &self.nullable_type
  }

  /// Returns the constraints of the generic partial field.
  #[inline]
  pub const fn type_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
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
            #ty: #path_to_grost::__private::state::State<
              #path_to_grost::__private::convert::Flattened
            >
          })?);
          constraints.push(syn::parse2(quote! {
            <#ty as #path_to_grost::__private::state::State<#path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::Inner>>>::Output: ::core::marker::Sized
          })?);
        }

        syn::parse2(quote! {
          <#ty as #path_to_grost::__private::state::State<
            #path_to_grost::__private::convert::Flattened
          >>::Output
        })?
      }
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
