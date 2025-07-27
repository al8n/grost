use quote::quote;
use syn::{ConstParam, GenericParam, Generics, Ident, Type};

use crate::{object::mir::grost_selected_param, utils::grost_lifetime};

use super::super::{GenericField, GenericObjectAst};

#[derive(Debug, Clone)]
pub struct SelectorFlavor {
  ty: Type,
  generics: Generics,
}

impl SelectorFlavor {
  /// Returns the selector type of this flavor.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the selector generics of this flavor.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  pub(super) fn from_ast<M, F>(
    flavor_name: &Ident,
    flavor_type: &Type,
    object: &GenericObjectAst<M, F>,
    fields: &[GenericField<F>],
  ) -> darling::Result<Self> {
    let selector = object.selector();
    let selector_name = selector.name();

    let original_generics = object.generics();
    let mut generics = Generics::default();
    let mut ty_params = Vec::new();

    // push the lifetime generic parameter first
    generics.params.extend(
      original_generics
        .lifetimes()
        .cloned()
        .map(GenericParam::from),
    );
    ty_params.extend(original_generics.lifetimes().map(|l| {
      let lt = &l.lifetime;
      quote! { #lt }
    }));

    // push the original type generic parameters
    generics.params.extend(
      original_generics
        .type_params()
        .cloned()
        .map(GenericParam::from),
    );
    ty_params.extend(original_generics.type_params().map(|t| {
      let ident = &t.ident;
      quote! { #ident }
    }));
    ty_params.push(quote! { #flavor_type });

    // push the original const generic parameters last
    generics.params.extend(
      original_generics
        .const_params()
        .cloned()
        .map(GenericParam::from),
    );
    ty_params.extend(original_generics.const_params().map(|c| {
      let ident = &c.ident;
      quote! { #ident }
    }));

    fields
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .for_each(|f| {
        let type_constraints = f
          .flavors()
          .get(flavor_name)
          .expect("Field flavor already checked when constructing the AST")
          .selector()
          .type_constraints();
        if !type_constraints.is_empty() {
          generics
            .make_where_clause()
            .predicates
            .extend(type_constraints.iter().cloned());
        }
      });

    let ty: Type = syn::parse2(quote! {
      #selector_name <#(#ty_params),*>
    })?;
    Ok(Self { ty, generics })
  }

  pub(super) fn selector_iter<M, F>(
    &self,
    object: &GenericObjectAst<M, F>,
    flavor_type: &Type,
  ) -> darling::Result<SelectorIterFlavor> {
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

    let params = selected_generics
      .lifetimes()
      .map(|lp| {
        let lt = &lp.lifetime;
        quote!(#lt)
      })
      .chain(selected_generics.type_params().map(|tp| {
        let tp = &tp.ident;
        quote!(#tp)
      }))
      .chain([quote!(#flavor_type)])
      .chain(selected_generics.const_params().map(|cp| {
        let cp = &cp.ident;
        quote!(#cp)
      }));
    let selected_type: Type = syn::parse2(quote! {
      #selector_iter_name <#(#params),*, true>
    })?;

    let params = unselected_generics
      .lifetimes()
      .map(|lp| {
        let lt = &lp.lifetime;
        quote!(#lt)
      })
      .chain(unselected_generics.type_params().map(|tp| {
        let tp = &tp.ident;
        quote!(#tp)
      }))
      .chain([quote!(#flavor_type)])
      .chain(unselected_generics.const_params().map(|cp| {
        let cp = &cp.ident;
        quote!(#cp)
      }));
    let unselected_type: Type = syn::parse2(quote! {
      #selector_iter_name <#(#params),*, false>
    })?;

    Ok(SelectorIterFlavor {
      ty,
      selected_type,
      unselected_type,
      generics,
      selected_generics,
      unselected_generics,
      selected,
    })
  }
}

#[derive(Debug, Clone)]
pub struct SelectorIterFlavor {
  ty: Type,
  selected_type: Type,
  unselected_type: Type,
  generics: Generics,
  selected_generics: Generics,
  unselected_generics: Generics,
  selected: ConstParam,
}

impl SelectorIterFlavor {
  /// Returns the type of the selector iterator flavor.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the selected type of the selector iterator flavor.
  #[inline]
  pub const fn selected_type(&self) -> &Type {
    &self.selected_type
  }

  /// Returns the unselected type of the selector iterator flavor.
  #[inline]
  pub const fn unselected_type(&self) -> &Type {
    &self.unselected_type
  }

  /// Returns the generics of the selector iterator flavor.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the generics for the selected iterator flavor.
  #[inline]
  pub const fn selected_generics(&self) -> &Generics {
    &self.selected_generics
  }

  /// Returns the generics for the unselected iterator flavor.
  #[inline]
  pub const fn unselected_generics(&self) -> &Generics {
    &self.unselected_generics
  }

  /// Returns the selected constant parameter.
  #[inline]
  pub const fn selected(&self) -> &ConstParam {
    &self.selected
  }
}
