use indexmap::IndexMap;
use quote::quote;
use syn::{Attribute, ConstParam, GenericParam, Generics, Ident, Type};

use crate::utils::grost_lifetime;

use super::{super::{super::ast::GenericObject as GenericObjectAst, grost_selected_param}, GenericField, ObjectFlavor, };

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

#[derive(Debug, Clone)]
pub struct GenericSelectorIter {
  name: Ident,
  ty: Type,
  selected_type: Type,
  unselected_type: Type,
  generics: Generics,
  selected_generics: Generics,
  unselected_generics: Generics,
  selected: ConstParam,
  flavors: IndexMap<Ident, SelectorIterFlavor>,
}

impl GenericSelectorIter {
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

  /// Returns the selected constant parameter.
  #[inline]
  pub const fn selected(&self) -> &ConstParam {
    &self.selected
  }

  /// Returns the flavors of the selector iterator.
  #[inline]
  pub const fn flavors(&self) -> &IndexMap<Ident, SelectorIterFlavor> {
    &self.flavors
  }
}

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

  fn try_new<M, F>(
    object: &GenericObjectAst<M, F>,
    flavor_name: &Ident,
    flavor: &ObjectFlavor,
    fields: &[GenericField<F>],
  ) -> darling::Result<Self> {
    let selector = object.selector();
    let selector_name = selector.name();
    let flavor_type = flavor.ty();

    let original_generics = object.generics();
    let mut generics = original_generics.clone();

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

    let params = generics
      .lifetimes()
      .map(|lp| {
        let lt = &lp.lifetime;
        quote!(#lt)
      })
      .chain(generics.type_params().map(|tp| {
        let tp = &tp.ident;
        quote!(#tp)
      }))
      .chain([quote!(#flavor_type)])
      .chain(generics.const_params().map(|cp| {
        let cp = &cp.ident;
        quote!(#cp)
      }));

    let ty: Type = syn::parse2(quote! {
      #selector_name <#(#params),*>
    })?;
    Ok(Self { ty, generics })
  }
}

#[derive(Debug, Clone)]
pub struct GenericSelector {
  name: Ident,
  ty: Type,
  attrs: Vec<Attribute>,
  generics: Generics,
  flavors: IndexMap<Ident, SelectorFlavor>,
}

impl GenericSelector {
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

  /// Returns the type of the selector.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the generics of the selector.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the flavors specified selectors
  #[inline]
  pub const fn flavors(&self) -> &IndexMap<Ident, SelectorFlavor> {
    &self.flavors
  }

  pub(super) fn from_ast<M, F>(
    object: &GenericObjectAst<M, F>,
    fields: &[GenericField<F>],
  ) -> darling::Result<Self> {
    let selector = object.selector();
    let selector_name = selector.name();
    let flavor_param = selector.flavor();

    let mut generics = Generics::default();
    let original_generics = object.generics();

    // push the lifetime generic parameter first
    generics.params.extend(
      original_generics
        .lifetimes()
        .map(|lp| GenericParam::Lifetime(lp.clone())),
    );

    // push the original type generic parameters
    generics.params.extend(
      original_generics
        .type_params()
        .map(|tp| GenericParam::Type(tp.clone())),
    );

    generics
      .params
      .push(syn::GenericParam::Type(flavor_param.clone()));

    // push the original const generic parameters last
    generics.params.extend(
      original_generics
        .const_params()
        .map(|cp| GenericParam::Const(cp.clone())),
    );

    if let Some(where_clause) = original_generics.where_clause.as_ref() {
      generics
        .make_where_clause()
        .predicates
        .extend(where_clause.predicates.iter().cloned());
    }

    fields
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .for_each(|f| {
        let type_constraints = f.selector().type_constraints();
        if !type_constraints.is_empty() {
          generics
            .make_where_clause()
            .predicates
            .extend(type_constraints.iter().cloned());
        }
      });

    let (_, tg, _) = generics.split_for_impl();
    let ty: Type = syn::parse2(quote! {
      #selector_name #tg
    })?;

    let flavors = object
      .flavors()
      .iter()
      .map(|(name, flavor)| {
        SelectorFlavor::try_new(object, name, flavor, fields).map(|flavor| (name.clone(), flavor))
      })
      .collect::<darling::Result<IndexMap<_, _>>>()?;

    Ok(Self {
      name: selector_name.clone(),
      ty,
      attrs: object.attrs().to_vec(),
      generics,
      flavors,
    })
  }

  pub fn selector_iter<M, F>(
    &self,
    object: &GenericObjectAst<M, F>,
  ) -> darling::Result<GenericSelectorIter> {
    let selector_iter = object.selector_iter();
    let selector_iter_name = selector_iter.name();
    let selected = grost_selected_param();
    let lifetime = grost_lifetime();

    let flavor_param = selector_iter.flavor();
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

    generics
      .params
      .push(syn::GenericParam::Type(flavor_param.clone()));

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

    let flavors = object
      .flavors()
      .iter()
      .map(|(name, flavor)| {
        self
          .flavor_selector_iter(object, flavor.ty())
          .map(|flavor| (name.clone(), flavor))
      })
      .collect::<darling::Result<IndexMap<_, _>>>()?;

    Ok(GenericSelectorIter {
      name: selector_iter_name.clone(),
      ty,
      selected_type,
      unselected_type,
      generics,
      selected_generics,
      unselected_generics,
      selected,
      flavors,
    })
  }

  fn flavor_selector_iter<M, F>(
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
