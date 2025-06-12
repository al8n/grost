use quote::{format_ident, quote};
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
      if !type_constraints.is_empty() {
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

impl<M, F> super::ConcreteObject<M, F> {
  pub(super) fn derive_selector_defination(&self) -> proc_macro2::TokenStream {
    let name = self.selector().name();
    let vis = self.vis();

    let attrs = self.attrs();
    let doc = if !attrs.iter().any(|attr| attr.path().is_ident("doc")) {
      let doc = format!(" The selection type for [`{}`]", self.name());
      Some(quote! {
        #[doc = #doc]
      })
    } else {
      None
    };

    let fields = self.fields().iter().map(|f| {
      let name = f.name();
      let vis = f.vis();
      match f {
        ConcreteField::Skipped(skipped_field) => {
          let ty = skipped_field.ty();
          quote! {
            #vis #name: ::core::marker::PhantomData<#ty>
          }
        }
        ConcreteField::Tagged(concrete_tagged_field) => {
          let ty = concrete_tagged_field.selector().ty();
          let attrs = concrete_tagged_field.attrs();

          quote! {
            #(#attrs)*
            #vis #name: #ty
          }
        }
      }
    });

    let generics = self.selector().generics();
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

  pub(super) fn derive_selector(&self) -> proc_macro2::TokenStream {
    let path_to_grost = self.path_to_grost();
    let selector = self.selector();
    let flavor_ty = self.flavor_type();
    let generics = selector.generics();
    let (ig, tg, wc) = generics.split_for_impl();
    let name = selector.name();
    let name_str = name.to_string();
    let indexer_name = self.indexer().name();

    let default = self
      .fields()
      .iter()
      .map(|f| {
        let field_name = f.name();

        match f {
          ConcreteField::Skipped(_) => {
            quote! {
              #field_name: ::core::marker::PhantomData
            }
          },
          ConcreteField::Tagged(f) => {
            let ty = f.selector().ty();

            quote! {
              #field_name: <#ty as #path_to_grost::__private::selection::Selector<#flavor_ty>>::DEFAULT
            }
          },
        }
      });

    let empty = self.fields().iter().map(|f| {
      let field_name = f.name();

      match f {
        ConcreteField::Skipped(_) => {
          quote! {
            #field_name: ::core::marker::PhantomData
          }
        }
        ConcreteField::Tagged(f) => {
          let ty = f.selector().ty();

          quote! {
            #field_name: <#ty as #path_to_grost::__private::selection::Selector<#flavor_ty>>::NONE
          }
        }
      }
    });

    let all = self.fields().iter().map(|f| {
      let field_name = f.name();

      match f {
        ConcreteField::Skipped(_) => {
          quote! {
            #field_name: ::core::marker::PhantomData
          }
        }
        ConcreteField::Tagged(f) => {
          let ty = f.selector().ty();

          quote! {
            #field_name: <#ty as #path_to_grost::__private::selection::Selector<#flavor_ty>>::ALL
          }
        }
      }
    });

    let is_empty = self.fields().iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let ty = f.selector().ty();
        let field_name = f.name();

        quote! {
          <#ty as #path_to_grost::__private::selection::Selector<#flavor_ty>>::is_empty(&self.#field_name)
        }
      });

    let is_all = self.fields().iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let ty = f.selector().ty();
        let field_name = f.name();

        quote! {
          <#ty as #path_to_grost::__private::selection::Selector<#flavor_ty>>::is_all(&self.#field_name)
        }
      });

    let num_selected = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();
        let fn_name = format_ident!("is_{}_selected", field_name);
        quote! {
          if self.#fn_name() {
            num += 1;
          }
        }
      });

    let num_unselected = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();
        let fn_name = format_ident!("is_{}_unselected", field_name);
        quote! {
          if self.#fn_name() {
            num += 1;
          }
        }
      });

    let is_selected = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();
        let field_variant = &f.index().variant().ident;
        let fn_name = format_ident!("is_{}_selected", field_name);
        quote! {
          #indexer_name::#field_variant => self.#fn_name()
        }
      });

    let is_unselected = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();
        let field_variant = &f.index().variant().ident;
        let fn_name = format_ident!("is_{}_unselected", field_name);
        quote! {
          #indexer_name::#field_variant => self.#fn_name()
        }
      });

    let merge = self.fields().iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();
        let ty = f.selector().ty();
        quote! {
          <#ty as #path_to_grost::__private::selection::Selector<#flavor_ty>>::merge(&mut self.#field_name, other.#field_name);
        }
      });
    let flip = self.fields().iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();
        let ty = f.selector().ty();
        quote! {
          <#ty as #path_to_grost::__private::selection::Selector<#flavor_ty>>::flip(&mut self.#field_name);
        }
      });

    let eq = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();
        quote! {
          self.#field_name == other.#field_name
        }
      });

    let clone = self.fields().iter().map(|f| {
      let field_name = f.name();

      match f {
        ConcreteField::Skipped(_) => {
          quote! {
            #field_name: ::core::marker::PhantomData
          }
        }
        ConcreteField::Tagged(_) => {
          quote! {
            #field_name: ::core::clone::Clone::clone(&self.#field_name)
          }
        }
      }
    });

    let debug = self.fields().iter().map(|f| {
      let field_name = f.name();
      let field_name_str = field_name.to_string();
      quote! {
        field(#field_name_str, &self.#field_name)
      }
    });

    let hash = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();
        quote! {
          self.#field_name.hash(state);
        }
      });

    let copy_constraints = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let ty = f.selector().ty();
        quote! { #ty: ::core::marker::Copy }
      });

    let fns = selector_field_fns(self);

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig ::core::fmt::Debug for #name #tg #wc
      {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
          f.debug_struct(#name_str)
            #(.#debug)*
            .finish()
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig ::core::cmp::PartialEq for #name #tg #wc
      {
        fn eq(&self, other: &Self) -> ::core::primitive::bool {
          #(#eq) && *
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig ::core::cmp::Eq for #name #tg #wc
      {}

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig ::core::hash::Hash for #name #tg #wc
      {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
          #(#hash)*
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig ::core::clone::Clone for #name #tg #wc
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
        #wc,
        #(#copy_constraints),*
      {}

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig #path_to_grost::__private::selection::Selector<#flavor_ty> for #name #tg #wc
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
      impl #ig #name #tg #wc {
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

        // /// Returns an iterator over the selected fields.
        // #[inline]
        // pub fn iter_selected #iter_lifetime_params_with_angle (&self) -> #iter_name <'_, #(#selected_iter_generics),*>
        // {
        //   #iter_name::new(self, self.selected())
        // }

        // /// Returns an iterator over the unselected fields.
        // #[inline]
        // pub fn iter_unselected #iter_lifetime_params_with_angle (&self) -> #iter_name <'_, #(#unselected_iter_generics),*>
        // {
        //   #iter_name::new(self, self.unselected())
        // }

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

  pub(super) fn derive_selector_iter_defination(&self) -> proc_macro2::TokenStream {
    quote! {}
  }

  pub(super) fn derive_selector_iter(&self) -> proc_macro2::TokenStream {
    quote! {}
  }
}

fn selector_field_fns<M, F>(
  object: &super::ConcreteObject<M, F>,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
  let path_to_grost = object.path_to_grost();
  object.fields().iter().filter_map(|f| f.try_unwrap_tagged_ref().ok()).map(move |f| {
      let ty = f.selector().ty();
      let object_name = object.name();
      let field_name = f.name();
      let flavor_ty = object.flavor_type();
      let select_fn_name = format_ident!("select_{}", field_name);
      let select_fn_doc = format!(" Select the `{}.{}` field", object_name, field_name);
      let unselect_fn_name = format_ident!("unselect_{}", field_name);
      let unselect_fn_doc = format!(" Unselect the `{}.{}` field", object_name, field_name);
      let update_fn_name = format_ident!("update_{}", field_name);
      let update_fn_doc = format!(" Update the `{}.{}` field", object_name, field_name);
      let with_fn_name = format_ident!("with_{}", field_name);
      let with_fn_doc = format!(" Set the `{}.{}` field", object_name, field_name);
      let without_fn_name = format_ident!("without_{}", field_name);
      let without_fn_doc = format!(" Unset the `{}.{}` field", object_name, field_name);
      let maybe_fn_name = format_ident!("maybe_{}", field_name);
      let maybe_fn_doc = format!(" Set or unset the `{}.{}` field", object_name, field_name);
      let is_field_selected_fn_name = format_ident!("is_{}_selected", field_name);
      let is_field_selected_fn_doc = format!(
        " Returns `true` if the `{}.{}` field is selected",
        object_name, field_name
      );
      let is_field_unselected_fn_name = format_ident!("is_{}_unselected", field_name);
      let is_field_unselected_fn_doc = format!(
        " Returns `true` if the `{}.{}` field is unselected",
        object_name, field_name
      );

      let ref_fn_name = format_ident!("{}_ref", field_name);
      let ref_fn_doc = format!(
        " Get a reference to the selector of `{}.{}` field",
        object_name, field_name
      );
      let ref_mut_fn_name = format_ident!("{}_mut", field_name);
      let ref_mut_fn_doc = format!(
        " Get a mutable reference to the selector of `{}.{}` field",
        object_name, field_name
      );
      quote! {
        #[doc = #select_fn_doc]
        #[inline]
        pub fn #select_fn_name(&mut self) -> &mut Self {
          self.#field_name = <#ty as #path_to_grost::__private::selection::Selector<#flavor_ty>>::DEFAULT;
          self
        }

        #[doc = #unselect_fn_doc]
        #[inline]
        pub fn #unselect_fn_name(&mut self) -> &mut Self {
          self.#field_name = <#ty as #path_to_grost::__private::selection::Selector<#flavor_ty>>::NONE;
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
          self.#field_name = <#ty as #path_to_grost::__private::selection::Selector<#flavor_ty>>::DEFAULT;
          self
        }

        #[doc = #without_fn_doc]
        #[inline]
        pub fn #without_fn_name(mut self) -> Self {
          self.#field_name = <#ty as #path_to_grost::__private::selection::Selector<#flavor_ty>>::NONE;
          self
        }

        #[doc = #is_field_selected_fn_doc]
        #[inline]
        pub fn #is_field_selected_fn_name(&self) -> ::core::primitive::bool {
          !<#ty as #path_to_grost::__private::selection::Selector<#flavor_ty>>::is_empty(&self.#field_name)
        }

        #[doc = #is_field_unselected_fn_doc]
        #[inline]
        pub fn #is_field_unselected_fn_name(&self) -> ::core::primitive::bool {
          <#ty as #path_to_grost::__private::selection::Selector<#flavor_ty>>::is_empty(&self.#field_name)
        }
      }
    })
}
