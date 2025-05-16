use quote::{format_ident, quote};
use syn::{GenericParam, Generics, Ident, Type, Visibility, parse::Parser, parse_quote};

use super::{super::wire_format_reflection_ty, Object};

use crate::ast::{
  grost_flavor_generic, grost_lifetime,
  object::{Field, ObjectExt as _, Selection, SelectorIterMeta},
};

const GROST_SELECTED_CONST: &str = "__GROST_SELECTED__";

fn selected_const_generic_param() -> syn::Ident {
  format_ident!("{GROST_SELECTED_CONST}")
}

#[derive(Debug, Clone)]
pub struct SelectorField {
  field: syn::Field,
  default: Selection,
}

impl SelectorField {
  /// Returns the name of the field.
  #[inline]
  pub const fn name(&self) -> &Ident {
    self.field.ident.as_ref().unwrap()
  }

  /// Returns the type of the field.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.field.ty
  }

  /// Returns the visibility of the field.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.field.vis
  }

  /// Returns the field.
  #[inline]
  pub const fn field(&self) -> &syn::Field {
    &self.field
  }

  /// Returns the attributes of the field.
  #[inline]
  pub const fn attrs(&self) -> &[syn::Attribute] {
    self.field.attrs.as_slice()
  }

  /// Returns the default selection of the field.
  #[inline]
  pub const fn default_selection(&self) -> &Selection {
    &self.default
  }
}

#[derive(Debug, Clone)]
pub struct SelectorIter {
  selector: Ident,
  selector_generics: Generics,
  indexer: Ident,
  name: Ident,
  vis: Visibility,
  attrs: Vec<syn::Attribute>,
  generics: Generics,
}

impl SelectorIter {
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  pub const fn attrs(&self) -> &[syn::Attribute] {
    self.attrs.as_slice()
  }

  pub(super) fn to_token_stream(&self) -> proc_macro2::TokenStream {
    let selector_name = &self.selector;
    let iter_name = self.name();
    let vis = self.vis();
    let indexer_name = &self.indexer;
    let gl = grost_lifetime();
    let generics = self.generics();
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
    let (_, tg, _) = self.selector_generics.split_for_impl();

    quote! {
      #(#attrs)*
      #doc
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #vis struct #iter_name #generics #where_clause
      {
        selector: &#gl #selector_name #tg,
        index: ::core::option::Option<#indexer_name>,
        num: ::core::primitive::usize,
        yielded: ::core::primitive::usize,
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct Selector {
  parent_name: Ident,
  name: Ident,
  vis: Visibility,
  generics: Generics,
  attrs: Vec<syn::Attribute>,
  fields: Vec<SelectorField>,
}

impl Selector {
  #[inline]
  pub const fn fields(&self) -> &[SelectorField] {
    self.fields.as_slice()
  }

  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  #[inline]
  pub const fn attrs(&self) -> &[syn::Attribute] {
    self.attrs.as_slice()
  }

  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  pub(super) fn from_input<O>(path_to_grost: &syn::Path, input: &O) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let name = input.selector_name();

    let vis = input.vis().clone();
    let reflection_name = input.reflection_name();
    let fg = grost_flavor_generic();

    let mut generics = Generics::default();
    let original_generics = input.generics();

    // push the lifetime generic parameter first
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Lifetime(_)))
        .cloned(),
    );

    // push the original type generic parameters
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Type(_)))
        .cloned(),
    );

    generics.params.push(syn::GenericParam::Type(syn::parse2(
      quote! { #fg: ?::core::marker::Sized },
    )?));

    // push the original const generic parameters last
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Const(_)))
        .cloned(),
    );

    if let Some(where_clause) = original_generics.where_clause.as_ref() {
      generics
        .make_where_clause()
        .predicates
        .extend(where_clause.predicates.iter().cloned());
    }

    add_selector_constraints(
      &mut generics,
      path_to_grost,
      &reflection_name,
      input.fields().iter().copied(),
      &fg,
    )?;

    let fields = input
      .fields()
      .iter()
      .map(|f| {
        let ty = f.ty();
        let tag = f.meta().tag();
        let wfr = wire_format_reflection_ty(path_to_grost, &reflection_name, tag.get(), &fg);
        let wf = wire_format_ty(path_to_grost, &wfr, &fg);

        let attrs = f.meta().selector().attrs();
        let vis = f.vis();
        let name = f.name();
        let field = syn::Field::parse_named.parse2(quote! {
          #(#attrs)*
          #vis #name: <#ty as #path_to_grost::__private::Selectable<#fg, #wf>>::Selector
        })?;

        Ok(SelectorField {
          field,
          default: f.meta().selector().select().clone(),
        })
      })
      .collect::<Result<Vec<_>, darling::Error>>()?;

    let attrs = input.meta().selector().attrs().to_vec();
    Ok(Self {
      parent_name: input.name().clone(),
      name,
      vis,
      attrs,
      generics,
      fields,
    })
  }

  pub(super) fn selector_iter(
    &self,
    name: Ident,
    indexer: Ident,
    iter_meta: &SelectorIterMeta,
  ) -> darling::Result<SelectorIter> {
    let mut generics = Generics::default();
    let original_generics = self.generics.clone();

    // push the lifetime generic parameter first
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Lifetime(_)))
        .cloned(),
    );

    let lt = grost_lifetime();

    // Push our lifetime generic parameter in the front
    generics
      .params
      .push(syn::GenericParam::Lifetime(syn::LifetimeParam::new(
        lt.clone(),
      )));

    // push the original type generic parameters
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Type(_)))
        .cloned(),
    );

    // push the original const generic parameters last
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Const(_)))
        .cloned(),
    );

    if let Some(where_clause) = original_generics.where_clause.as_ref() {
      generics
        .make_where_clause()
        .predicates
        .extend(where_clause.predicates.iter().cloned());
    }

    // push our const generic parameter in the end
    let const_selected_param = selected_const_generic_param();
    generics.params.push(GenericParam::Const(syn::parse2(
      quote! { const #const_selected_param: ::core::primitive::bool = true },
    )?));

    Ok(SelectorIter {
      selector: self.name.clone(),
      selector_generics: original_generics,
      indexer,
      name,
      attrs: iter_meta.attrs().to_vec(),
      vis: self.vis.clone(),
      generics,
    })
  }

  pub(super) fn to_token_stream(&self) -> proc_macro2::TokenStream {
    let name = self.name();
    let vis = self.vis();

    let attrs = self.attrs();
    let doc = if !attrs.iter().any(|attr| attr.path().is_ident("doc")) {
      let doc = format!(" The selection type for [`{}`]", self.parent_name);
      Some(quote! {
        #[doc = #doc]
      })
    } else {
      None
    };

    let fields = self.fields().iter().map(SelectorField::field);

    let generics = self.generics();
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
}

impl<M> Object<M>
where
  M: crate::ast::object::Object,
{
  pub(super) fn derive_selector_iter(&self) -> proc_macro2::TokenStream {
    let selector_iter = self.selector_iter();
    let iter_name = selector_iter.name();
    let selector = self.selector();
    let name = selector.name();
    let indexer_name = self.indexer().name();
    let (ig, tg, where_clauses) = selector_iter.generics().split_for_impl();
    let (_, selector_tg, _) = selector_iter.selector_generics.split_for_impl();
    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #iter_name #tg #where_clauses
      {
        #[inline]
        const fn new(selector: &'__grost_lifetime__ #name #selector_tg, num: ::core::primitive::usize) -> Self {
          Self {
            selector,
            index: ::core::option::Option::Some(#indexer_name::FIRST),
            num,
            yielded: 0,
          }
        }

        /// Returns the exact remaining length of the iterator.
        #[inline]
        pub const fn remaining(&self) -> ::core::primitive::usize {
          self.num - self.yielded
        }

        /// Returns `true` if the iterator is empty.
        #[inline]
        pub const fn is_empty(&self) -> ::core::primitive::bool {
          self.remaining() == 0
        }
      }
    }
  }

  pub(super) fn derive_selector(&self) -> proc_macro2::TokenStream {
    let selector = self.selector();
    let name = selector.name();
    let path_to_grost = &self.path_to_grost;
    let fns = self.selector_field_fns(&self.path_to_grost);
    let fg = grost_flavor_generic();
    let (ig, tg, selector_where_clauses) = selector.generics().split_for_impl();

    let empty = self.selector.fields().iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();

      quote! {
        #field_name: <#ty as #path_to_grost::__private::Selector<#fg>>::NONE
      }
    });

    let all = self.selector.fields().iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();

      quote! {
        #field_name: <#ty as #path_to_grost::__private::Selector<#fg>>::ALL
      }
    });

    let default = self.selector.fields().iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();

      quote! {
        #field_name: <#ty as #path_to_grost::__private::Selector<#fg>>::DEFAULT
      }
    });

    let is_empty = self.selector.fields().iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();

      quote! {
        <#ty as #path_to_grost::__private::Selector<#fg>>::is_empty(&self.#field_name)
      }
    });

    let is_all = self.selector.fields().iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();

      quote! {
        <#ty as #path_to_grost::__private::Selector<#fg>>::is_all(&self.#field_name)
      }
    });

    let num_selected = self.selector.fields().iter().map(|f| {
      let field_name = f.name();
      let fn_name = format_ident!("is_{}_selected", field_name);
      quote! {
        if self.#fn_name() {
          num += 1;
        }
      }
    });

    let num_unselected = self.selector.fields().iter().map(|f| {
      let field_name = f.name();
      let fn_name = format_ident!("is_{}_unselected", field_name);
      quote! {
        if self.#fn_name() {
          num += 1;
        }
      }
    });

    let merge = self.selector_merge_impl(&self.path_to_grost);
    let flip = self.selector_flip_impl(&self.path_to_grost);

    let eq = self.selector.fields().iter().map(|f| {
      let field_name = f.name();
      quote! {
        self.#field_name == other.#field_name
      }
    });

    let clone = self.selector.fields().iter().map(|f| {
      let field_name = f.name();
      quote! {
        #field_name: ::core::clone::Clone::clone(&self.#field_name)
      }
    });

    let debug = self.selector.fields().iter().map(|f| {
      let field_name = f.name();
      let field_name_str = field_name.to_string();
      quote! {
        field(#field_name_str, &self.#field_name)
      }
    });

    let hash = self.selector.fields().iter().map(|f| {
      let field_name = f.name();
      quote! {
        self.#field_name.hash(state);
      }
    });

    let copy_constraints = self.selector.fields().iter().map(|f| {
      let ty = f.ty();
      quote! { #ty: ::core::marker::Copy }
    });

    let num_fields = self.fields.len();
    let name_str = name.to_string();
    let iter = self.selector_iter();
    let iter_name = iter.name();
    let selector_generics = self.selector().generics();
    let has_lifetime = selector_generics.lifetimes().count() > 0;
    let iter_lifetime_params = selector_generics.lifetimes().map(|p| {
      let lifetime = &p.lifetime;
      quote!(#lifetime)
    });
    let iter_lifetime_params_with_angle = if has_lifetime {
      Some(quote!(< #(#iter_lifetime_params),* >))
    } else {
      None
    };

    let fg = grost_flavor_generic();
    let (selected_iter_generics, unselected_iter_generics) = {
      let params = |selected: bool| {
        selector_generics.lifetimes().map(|p| {
          let lifetime = &p.lifetime;
          quote!(#lifetime)
        }).chain(
          selector_generics.type_params().map(|p| {
            let ident = &p.ident;
            quote!(#ident)
          }).chain(selector_generics.const_params().map(|p| {
            let ident = &p.ident;
            quote!(#ident)
          }))
          .chain([quote!(#selected)])
        )
      };

      let selected_iter_generics = params(true);
      let unselected_iter_generics = params(false);
      (selected_iter_generics, unselected_iter_generics)
    };

    let indexer_name = self.indexer.name();

    let is_selected = self.indexer.variants().iter().map(|f| {
      let field_name = f.field_name();
      let field_variant = f.name();
      let fn_name = format_ident!("is_{}_selected", field_name);
      quote! {
        #indexer_name::#field_variant => self.#fn_name()
      }
    });

    let is_unselected = self.indexer.variants().iter().map(|f| {
      let field_name = f.field_name();
      let field_variant = f.name();
      let fn_name = format_ident!("is_{}_unselected", field_name);
      quote! {
        #indexer_name::#field_variant => self.#fn_name()
      }
    });

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig ::core::fmt::Debug for #name #tg #selector_where_clauses
      {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
          f.debug_struct(#name_str)
            #(.#debug)*
            .finish()
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig ::core::cmp::PartialEq for #name #tg #selector_where_clauses
      {
        fn eq(&self, other: &Self) -> ::core::primitive::bool {
          #(#eq) && *
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig ::core::cmp::Eq for #name #tg #selector_where_clauses
      {}

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig ::core::hash::Hash for #name #tg #selector_where_clauses
      {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
          #(#hash)*
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig ::core::clone::Clone for #name #tg #selector_where_clauses
      {
        fn clone(&self) -> Self {
          Self {
            #(#clone),*
          }
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig ::core::marker::Copy for #name
        #tg
        #selector_where_clauses,
        #(#copy_constraints),*
      {}

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig #path_to_grost::__private::Selector<#fg> for #name #tg #selector_where_clauses
      {
        const ALL: Self = Self::all();
        const DEFAULT: Self = Self::new();
        const NONE: Self = Self::empty();

        fn selected(&self) -> ::core::primitive::usize {
          Self::selected(self)
        }

        fn unselected(&self) -> ::core::primitive::usize {
          Self::unselected(self)
        }

        fn flip(&mut self) -> &mut Self {
          #(#flip)*

          self
        }

        fn merge(&mut self, other: Self) -> &mut Self {
          #(#merge)*

          self
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig ::core::default::Default for #name #tg #selector_where_clauses
      {
        fn default() -> Self {
          Self::new()
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #name #tg #selector_where_clauses
      {
        /// The number of options in this selection type.
        pub const OPTIONS: ::core::primitive::usize = #num_fields;

        /// Returns a selector with the default values.
        #[inline]
        pub const fn new() -> Self {
          Self {
            #(#default),*
          }
        }

        /// Returns a selector which selects nothing.
        #[inline]
        pub const fn empty() -> Self {
          Self {
            #(#empty),*
          }
        }

        /// Returns a selector which selects all.
        #[inline]
        pub const fn all() -> Self {
          Self {
            #(#all),*
          }
        }

        /// Returns `true` if the selector selects nothing.
        #[inline]
        pub fn is_empty(&self) -> ::core::primitive::bool {
          #(#is_empty) && *
        }

        /// Returns `true` if the selector selects all.
        #[inline]
        pub fn is_all(&self) -> ::core::primitive::bool {
          #(#is_all) && *
        }

        /// Returns the number of selected fields.
        #[inline]
        pub fn selected(&self) -> ::core::primitive::usize {
          let mut num = 0;
          #(#num_selected)*
          num
        }

        /// Returns the number of unselected fields.
        #[inline]
        pub fn unselected(&self) -> ::core::primitive::usize {
          let mut num = 0;
          #(#num_unselected)*
          num
        }

        /// Returns an iterator over the selected fields.
        #[inline]
        pub fn iter_selected #iter_lifetime_params_with_angle (&self) -> #iter_name <'_, #(#selected_iter_generics),*>
        {
          #iter_name::new(self, self.selected())
        }

        /// Returns an iterator over the unselected fields.
        #[inline]
        pub fn iter_unselected #iter_lifetime_params_with_angle (&self) -> #iter_name <'_, #(#unselected_iter_generics),*>
        {
          #iter_name::new(self, self.unselected())
        }

        /// Returns `true` if such field is selected.
        #[inline]
        pub fn is_selected(&self, idx: #indexer_name) -> ::core::primitive::bool {
          match idx {
            #(#is_selected),*
          }
        }

        /// Returns `true` if such field is unselected.
        #[inline]
        pub fn is_unselected(&self, idx: #indexer_name) -> ::core::primitive::bool {
          match idx {
            #(#is_unselected),*
          }
        }

        #(#fns)*
      }
    }
  }

  fn selector_merge_impl(
    &self,
    path_to_grost: &syn::Path,
  ) -> impl Iterator<Item = proc_macro2::TokenStream> {
    let fg = grost_flavor_generic();
    self.selector.fields().iter().map(move |f| {
      let ty = f.ty();
      let field_name = f.name();
      quote! {
        <#ty as #path_to_grost::__private::Selector<#fg>>::merge(&mut self.#field_name, other.#field_name);
      }
    })
  }

  fn selector_flip_impl(
    &self,
    path_to_grost: &syn::Path,
  ) -> impl Iterator<Item = proc_macro2::TokenStream> {
    let fg = grost_flavor_generic();
    self.selector.fields().iter().map(move |f| {
      let ty = f.ty();
      let field_name = f.name();

      quote! {
        <#ty as #path_to_grost::__private::Selector<#fg>>::flip(&mut self.#field_name);
      }
    })
  }

  fn selector_field_fns(
    &self,
    path_to_grost: &syn::Path,
  ) -> impl Iterator<Item = proc_macro2::TokenStream> {
    self.selector.fields().iter().map(move |f| {
      let ty = f.ty();
      let field_name = f.name();
      let fg = grost_flavor_generic();
      let select_fn_name = format_ident!("select_{}", field_name);
      let select_fn_doc = format!(" Select the `{}.{}` field", self.name, field_name);
      let unselect_fn_name = format_ident!("unselect_{}", field_name);
      let unselect_fn_doc = format!(" Unselect the `{}.{}` field", self.name, field_name);
      let update_fn_name = format_ident!("update_{}", field_name);
      let update_fn_doc = format!(" Update the `{}.{}` field", self.name, field_name);
      let with_fn_name = format_ident!("with_{}", field_name);
      let with_fn_doc = format!(" Set the `{}.{}` field", self.name, field_name);
      let without_fn_name = format_ident!("without_{}", field_name);
      let without_fn_doc = format!(" Unset the `{}.{}` field", self.name, field_name);
      let maybe_fn_name = format_ident!("maybe_{}", field_name);
      let maybe_fn_doc = format!(" Set or unset the `{}.{}` field", self.name, field_name);
      let is_field_selected_fn_name = format_ident!("is_{}_selected", field_name);
      let is_field_selected_fn_doc = format!(
        " Returns `true` if the `{}.{}` field is selected",
        self.name, field_name
      );
      let is_field_unselected_fn_name = format_ident!("is_{}_unselected", field_name);
      let is_field_unselected_fn_doc = format!(
        " Returns `true` if the `{}.{}` field is unselected",
        self.name, field_name
      );

      let ref_fn_name = format_ident!("{}_ref", field_name);
      let ref_fn_doc = format!(
        " Get a reference to the selector of `{}.{}` field",
        self.name, field_name
      );
      let ref_mut_fn_name = format_ident!("{}_mut", field_name);
      let ref_mut_fn_doc = format!(
        " Get a mutable reference to the selector of `{}.{}` field",
        self.name, field_name
      );
      quote! {
        #[doc = #select_fn_doc]
        #[inline]
        pub fn #select_fn_name(&mut self) -> &mut Self {
          self.#field_name = <#ty as #path_to_grost::__private::Selector<#fg>>::DEFAULT;
          self
        }

        #[doc = #unselect_fn_doc]
        #[inline]
        pub fn #unselect_fn_name(&mut self) -> &mut Self {
          self.#field_name = <#ty as #path_to_grost::__private::Selector<#fg>>::NONE;
          self
        }

        #[doc = #update_fn_doc]
        #[inline]
        pub fn #update_fn_name(&mut self, value: #ty) -> &mut Self {
          self.#field_name = value;
          self
        }

        #[doc = #maybe_fn_doc]
        #[inline]
        pub fn #maybe_fn_name(mut self, val: #ty) -> Self {
          self.#field_name = val;
          self
        }

        #[doc = #ref_fn_doc]
        #[inline]
        pub const fn #ref_fn_name(&self) -> &#ty {
          &self.#field_name
        }

        #[doc = #ref_mut_fn_doc]
        #[inline]
        pub const fn #ref_mut_fn_name(&mut self) -> &mut #ty {
          &mut self.#field_name
        }

        #[doc = #with_fn_doc]
        #[inline]
        pub fn #with_fn_name(mut self) -> Self {
          self.#field_name = <#ty as #path_to_grost::__private::Selector<#fg>>::DEFAULT;
          self
        }

        #[doc = #without_fn_doc]
        #[inline]
        pub fn #without_fn_name(mut self) -> Self {
          self.#field_name = <#ty as #path_to_grost::__private::Selector<#fg>>::NONE;
          self
        }

        #[doc = #is_field_selected_fn_doc]
        #[inline]
        pub fn #is_field_selected_fn_name(&self) -> ::core::primitive::bool {
          !<#ty as #path_to_grost::__private::Selector<#fg>>::is_empty(&self.#field_name)
        }

        #[doc = #is_field_unselected_fn_doc]
        #[inline]
        pub fn #is_field_unselected_fn_name(&self) -> ::core::primitive::bool {
          <#ty as #path_to_grost::__private::Selector<#fg>>::is_empty(&self.#field_name)
        }
      }
    })
  }
}

fn wire_format_ty(path_to_grost: &syn::Path, wf: &syn::Type, fg: &syn::Ident) -> syn::Type {
  parse_quote! {
    <#wf as #path_to_grost::__private::reflection::Reflectable<#fg>>::Reflection
  }
}

fn selector_ty(path_to_grost: &syn::Path, wf: &syn::Type, fg: &syn::Ident) -> syn::Type {
  parse_quote! {
    #path_to_grost::__private::Selectable<
      #fg,
      #wf,
    >
  }
}

fn add_selector_constraints<'a, I>(
  generics: &mut Generics,
  path_to_grost: &syn::Path,
  field_reflection: &syn::Ident,
  mut fields: impl Iterator<Item = &'a I>,
  fg: &syn::Ident,
) -> darling::Result<()>
where
  I: crate::ast::object::Field + 'a,
{
  fields.try_for_each(move |f| {
    let ty = f.ty();
    let wfr = wire_format_reflection_ty(path_to_grost, field_reflection, f.meta().tag().get(), fg);
    let wf = wire_format_ty(path_to_grost, &wfr, fg);
    let selector_ty = selector_ty(path_to_grost, &wf, fg);

    let where_clause = generics.make_where_clause();

    where_clause.predicates.push(syn::parse2(quote! {
      #wfr: #path_to_grost::__private::reflection::Reflectable<#fg>
    })?);
    where_clause.predicates.push(syn::parse2(quote! {
      #ty: #selector_ty
    })?);

    Ok(())
  })
}
