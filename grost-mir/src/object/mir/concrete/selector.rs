use quote::quote;
use syn::{Attribute, ConstParam, GenericParam, Generics, Ident, Type};

use crate::utils::grost_lifetime;

use super::{super::grost_selected_param, ConcreteField, ConcreteObjectAst};

#[derive(Debug, Clone)]
pub struct ConcreteSelectorIter {
  name: Ident,
  ty: Type,
  selected_type: Type,
  unselected_type: Type,
  generics: Generics,
  selected_generics: Generics,
  unselected_generics: Generics,
  selected: ConstParam,
}

impl ConcreteSelectorIter {
  /// Returns the name of the selector iterator.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the type of the selector iterator.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the selected type of the selector iterator.
  #[inline]
  pub const fn selected_type(&self) -> &Type {
    &self.selected_type
  }

  /// Returns the unselected type of the selector iterator.
  #[inline]
  pub const fn unselected_type(&self) -> &Type {
    &self.unselected_type
  }

  /// Returns the generics of the selector iterator.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the generics for the selected iterator.
  #[inline]
  pub const fn selected_generics(&self) -> &Generics {
    &self.selected_generics
  }

  /// Returns the generics for the unselected iterator.
  #[inline]
  pub const fn unselected_generics(&self) -> &Generics {
    &self.unselected_generics
  }

  /// Returns the selected constant parameter of the selector iterator.
  #[inline]
  pub const fn selected(&self) -> &ConstParam {
    &self.selected
  }
}

#[derive(Debug, Clone)]
pub struct ConcreteSelector {
  name: Ident,
  attrs: Vec<Attribute>,
  generics: Generics,
  flavor_type: Type,
}

impl ConcreteSelector {
  /// Returns the name of the selector.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the selector.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the generics of the selector.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the flavor type of the selector.
  #[inline]
  pub const fn flavor_type(&self) -> &Type {
    &self.flavor_type
  }

  pub(super) fn from_ast<M, F>(
    object: &ConcreteObjectAst<M, F>,
    fields: &[ConcreteField<F>],
  ) -> darling::Result<Self> {
    let selector = object.selector();

    let object_generics = object.generics();
    let mut generics = object_generics.clone();

    for field in fields.iter().filter_map(|f| f.try_unwrap_tagged_ref().ok()) {
      let type_constraints = field.selector().type_constraints();
      if type_constraints.is_empty() {
        generics
          .make_where_clause()
          .predicates
          .extend(type_constraints.iter().cloned());
      }
    }

    Ok(Self {
      name: selector.name().clone(),
      attrs: selector.attrs().to_vec(),
      generics,
      flavor_type: object.flavor().ty().clone(),
    })
  }

  pub(super) fn selector_iter<M, F>(
    &self,
    object: &ConcreteObjectAst<M, F>,
  ) -> darling::Result<ConcreteSelectorIter> {
    let selector_iter = object.selector_iter();
    let selector_iter_name = selector_iter.name();
    let selected = grost_selected_param();
    let lifetime = grost_lifetime();

    let original_generics = object.generics();
    let mut generics = Generics::default();

    // push the lifetime generic parameter first
    generics.params.extend(
      original_generics
        .lifetimes()
        .map(|lp| GenericParam::Lifetime(lp.clone())),
    );

    generics
      .params
      .push(GenericParam::Lifetime(lifetime.clone()));

    // push the original type generic parameters
    generics.params.extend(
      original_generics
        .type_params()
        .map(|tp| GenericParam::Type(tp.clone())),
    );

    // push the original const generic parameters last
    generics.params.extend(
      original_generics
        .const_params()
        .map(|cp| GenericParam::Const(cp.clone())),
    );

    let selected_generics = generics.clone();
    let unselected_generics = generics.clone();

    generics.params.push(GenericParam::Const(selected.clone()));

    let (_, tg, _) = generics.split_for_impl();
    let ty: Type = syn::parse2(quote! {
      #selector_iter_name #tg
    })?;

    let params = selected_generics.params.iter();
    let selected_type: Type = syn::parse2(quote! {
      #selector_iter_name <#(#params),*, true>
    })?;

    let params = unselected_generics.params.iter();
    let unselected_type: Type = syn::parse2(quote! {
      #selector_iter_name <#(#params),*, false>
    })?;

    Ok(ConcreteSelectorIter {
      ty,
      selected_type,
      unselected_type,
      name: selector_iter.name().clone(),
      generics,
      selected_generics,
      unselected_generics,
      selected,
    })
  }
}
