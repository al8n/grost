use quote::quote;
use syn::{Attribute, Type, WherePredicate, punctuated::Punctuated, token::Comma};

use crate::object::{FieldSelection, ast::field::SelectorFieldOptions};

use super::{selectable, selector};

#[derive(Debug, Clone)]
pub struct SelectorField {
  pub(super) ty: Type,
  pub(super) selectable: Type,
  pub(super) attrs: Vec<Attribute>,
  pub(super) default: FieldSelection,
  pub(super) constraints: Punctuated<WherePredicate, Comma>,
}

impl SelectorField {
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

  pub(super) fn try_new<T, S, M>(
    object: &super::RawObject<T, S, M>,
    use_generics: bool,
    field_ty: &Type,
    opts: SelectorFieldOptions,
    mut type_constraints: Punctuated<WherePredicate, Comma>,
  ) -> darling::Result<Self> {
    let path_to_grost = &object.path_to_grost;
    let flavor_type = &object.flavor_type;
    let selectable = syn::parse2(selectable(path_to_grost, flavor_type))?;
    let ty = syn::parse2(selector(path_to_grost, flavor_type, field_ty))?;

    if use_generics {
      type_constraints.push(syn::parse2(quote! {
        #field_ty: #selectable
      })?);
    }

    Ok(Self {
      ty,
      selectable,
      attrs: opts.attrs,
      constraints: type_constraints,
      default: opts.select,
    })
  }
}
