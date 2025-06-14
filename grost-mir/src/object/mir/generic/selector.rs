use indexmap::IndexMap;
use quote::quote;
use syn::{Attribute, ConstParam, GenericParam, Generics, Ident, LifetimeParam, Type};

use crate::utils::grost_lifetime;

use super::{
  super::{super::ast::GenericObject as GenericObjectAst, grost_selected_param},
  GenericField, ObjectFlavor,
};

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
  lifetime_param: LifetimeParam,
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

  /// Returns the lifetime parameter of the selector iterator.
  #[inline]
  pub const fn lifetime(&self) -> &LifetimeParam {
    &self.lifetime_param
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
    let flavor_type = flavor.flavor_type();

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
    flavors: &IndexMap<Ident, ObjectFlavor>,
    fields: &[GenericField<F>],
  ) -> darling::Result<Self> {
    let path_to_grost = &object.path_to_grost;
    let selector = &object.selector;
    let selector_name = selector.name();
    let flavor_param = &object.flavor_param;

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
      let where_clauses = generics.make_where_clause();
      where_clauses
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

    let fpi = &flavor_param.ident;
    generics
      .make_where_clause()
      .predicates
      .push(syn::parse2(quote! {
        #fpi: #path_to_grost::__private::flavors::Flavor
      })?);

    let (_, tg, _) = generics.split_for_impl();
    let ty: Type = syn::parse2(quote! {
      #selector_name #tg
    })?;

    let flavors = flavors
      .iter()
      .map(|(name, flavor)| {
        SelectorFlavor::try_new(object, name, flavor, fields).map(|flavor| (name.clone(), flavor))
      })
      .collect::<darling::Result<IndexMap<_, _>>>()?;

    Ok(Self {
      name: selector_name.clone(),
      ty,
      attrs: object.selector().attrs().to_vec(),
      generics,
      flavors,
    })
  }

  pub(super) fn selector_iter<M, F>(
    &self,
    object: &GenericObjectAst<M, F>,
  ) -> darling::Result<GenericSelectorIter> {
    let selector_iter = object.selector_iter();
    let selector_iter_name = selector_iter.name();
    let selected = grost_selected_param();
    let lifetime = grost_lifetime();
    let original_generics = self.generics();
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
    generics.where_clause = original_generics.where_clause.clone();

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
      lifetime_param: lifetime,
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

impl<M, F> super::GenericObject<M, F> {
  pub(super) fn derive_selector_defination(&self) -> proc_macro2::TokenStream {
    let selector = self.selector();
    let name = selector.name();
    let vis = self.vis();

    let attrs = selector.attrs();
    let doc = if !attrs.iter().any(|attr| attr.path().is_ident("doc")) {
      let doc = format!(" The selection type for [`{}`]", self.name());
      Some(quote! {
        #[doc = #doc]
      })
    } else {
      None
    };

    let fields = self.fields().iter().filter_map(|f| {
      let name = f.name();
      match f {
        GenericField::Skipped(skipped_field) => {
          if !skipped_field.lifetime_params_usages().is_empty()
            || !skipped_field.type_params_usages().is_empty()
          {
            let name = skipped_field.name();
            let ty = skipped_field.ty();
            Some(quote! {
              #name: ::core::marker::PhantomData<#ty>
            })
          } else {
            None
          }
        }
        GenericField::Tagged(generic_tagged_field) => {
          let ty = generic_tagged_field.selector().ty();
          let vis = generic_tagged_field.vis();
          let attrs = generic_tagged_field.selector().attrs();
          Some(quote! {
            #(#attrs)*
            #vis #name: #ty
          })
        }
      }
    });

    let generics = selector.generics();
    let where_clause = generics.where_clause.as_ref();
    quote! {
      #(#attrs)*
      #doc
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #vis struct #name #generics #where_clause
      {
        #(#fields),*
      }
    }
  }

  pub(super) fn derive_selector_iter_defination(&self) -> proc_macro2::TokenStream {
    let selector = self.selector();
    let selector_name = selector.name();
    let selector_ty = selector.ty();
    let iter = self.selector_iter();
    let iter_name = iter.name();
    let vis = self.vis();
    let indexer_name = self.indexer().name();
    let generics = iter.generics();
    let where_clause = generics.where_clause.as_ref();
    let attrs = self.attrs();
    let doc = if !attrs.iter().any(|attr| attr.path().is_ident("doc")) {
      let doc = format!(" An iterator over the selected fields of the [`{selector_name}`]",);
      Some(quote! {
        #[doc = #doc]
      })
    } else {
      None
    };
    let gl = &iter.lifetime().lifetime;
    quote! {
      #(#attrs)*
      #doc
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #vis struct #iter_name #generics #where_clause
      {
        selector: &#gl #selector_ty,
        index: ::core::option::Option<#indexer_name>,
        num: ::core::primitive::usize,
        yielded: ::core::primitive::usize,
      }
    }
  }
}
