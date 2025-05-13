use heck::ToShoutySnakeCase;
use indexmap::IndexMap;
use quote::{ToTokens, format_ident};
use syn::Ident;

use crate::FlavorGenerator;

use super::*;

impl Object {
  /// Generates the selector for the object
  pub fn generate_selector(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name = self.selector_name();
    let vis = self.visibility.as_ref();
    let doc = format!(" The selection type for {}", self.name.name_str());
    let field_reflection_ident = self.field_reflection_name();
    let where_clauses = constraints(path_to_grost, &field_reflection_ident, self.fields());
    let selection_fields = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      let wf = wire_format_reflection_ty(&field_reflection_ident, f.tag());
      let wf = wire_format_ty(path_to_grost, &wf);
      quote! {
        #field_name: <#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector
      }
    });

    quote! {
      #[doc = #doc]
      #[allow(non_camel_case_types)]
      #[derive(::core::fmt::Debug)]
      #vis struct #name<__GROST_FLAVOR__>
      where
        #(#where_clauses)*
      {
        #(#selection_fields),*
      }
    }
  }

  /// Generates the iterator type of the selector for the object
  pub fn generate_selector_iter(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name = self.selector_name();
    let name_str = name.name_str();
    let iter_name = self.selector_iter_name();
    let vis = self.visibility.as_ref();
    let iter_doc = format!(
      " An iterator over the selected fields of the [`{}`]",
      name_str,
    );
    let indexer_name = self.indexer_name();
    let field_reflection_ident = self.field_reflection_name();
    let where_clauses = constraints(path_to_grost, &field_reflection_ident, self.fields());

    quote! {
      #[doc = #iter_doc]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #vis struct #iter_name<'__grost_lifetime__, __GROST_FLAVOR__, const N: ::core::primitive::bool = true>
      where
        #(#where_clauses)*
      {
        selector: &'__grost_lifetime__ #name<__GROST_FLAVOR__>,
        index: ::core::option::Option<#indexer_name>,
        num: ::core::primitive::usize,
        yielded: ::core::primitive::usize,
      }
    }
  }

  /// Derives implementations for the iterator of the selector
  /// of the object
  pub fn derive_selector_iter(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let iter_name = self.selector_iter_name();
    let name = self.selector_name();
    let indexer_name = self.indexer_name();
    let field_reflection_ident = self.field_reflection_name();
    let where_clauses = constraints(path_to_grost, &field_reflection_ident, self.fields());
    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl<'__grost_lifetime__, __GROST_FLAVOR__, const N: ::core::primitive::bool> #iter_name<'__grost_lifetime__, __GROST_FLAVOR__, N>
      where
        #(#where_clauses)*
      {
        #[inline]
        const fn new(selector: &'__grost_lifetime__ #name<__GROST_FLAVOR__>, num: ::core::primitive::usize) -> Self {
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

  /// Derives implementations for the selector of the object
  pub fn derive_selector(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name = self.selector_name();

    let field_reflection_ident = self.field_reflection_name();
    let where_clauses =
      constraints(path_to_grost, &field_reflection_ident, self.fields()).collect::<Vec<_>>();

    let fns = self.selector_field_fns(path_to_grost, &field_reflection_ident);

    let empty = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      let wfr = wire_format_reflection_ty(&field_reflection_ident, f.tag());
      let wf = wire_format_ty(path_to_grost, &wfr);

      quote! {
        #field_name: <<#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector as #path_to_grost::__private::Selector<__GROST_FLAVOR__>>::NONE
      }
    });

    let all = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      let wfr = wire_format_reflection_ty(&field_reflection_ident, f.tag());
      let wf = wire_format_ty(path_to_grost, &wfr);

      quote! {
        #field_name: <<#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector as #path_to_grost::__private::Selector<__GROST_FLAVOR__>>::ALL
      }
    });

    let default = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      let wfr = wire_format_reflection_ty(&field_reflection_ident, f.tag());
      let wf = wire_format_ty(path_to_grost, &wfr);

      quote! {
        #field_name: <<#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector as #path_to_grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT
      }
    });

    let is_empty = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      let wfr = wire_format_reflection_ty(&field_reflection_ident, f.tag());
      let wf = wire_format_ty(path_to_grost, &wfr);

      quote! {
        <<#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector as #path_to_grost::__private::Selector<__GROST_FLAVOR__>>::is_empty(&self.#field_name)
      }
    });

    let is_all = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      let wfr = wire_format_reflection_ty(&field_reflection_ident, f.tag());
      let wf = wire_format_ty(path_to_grost, &wfr);

      quote! {
        <<#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector as #path_to_grost::__private::Selector<__GROST_FLAVOR__>>::is_all(&self.#field_name)
      }
    });

    let num_selected = self.fields.iter().map(|f| {
      let field_name = f.name();
      let fn_name = format_ident!("is_{}_selected", field_name.name_str());
      quote! {
        if self.#fn_name() {
          num += 1;
        }
      }
    });

    let num_unselected = self.fields.iter().map(|f| {
      let field_name = f.name();
      let fn_name = format_ident!("is_{}_unselected", field_name.name_str());
      quote! {
        if self.#fn_name() {
          num += 1;
        }
      }
    });

    let merge = self.selector_merge_impl(path_to_grost, &field_reflection_ident);
    let flip = self.selector_flip_impl(path_to_grost, &field_reflection_ident);

    let eq = self.fields.iter().map(|f| {
      let field_name = f.name();
      quote! {
        self.#field_name == other.#field_name
      }
    });

    let clone = self.fields.iter().map(|f| {
      let field_name = f.name();
      quote! {
        #field_name: ::core::clone::Clone::clone(&self.#field_name)
      }
    });

    let hash = self.fields.iter().map(|f| {
      let field_name = f.name();
      quote! {
        self.#field_name.hash(state);
      }
    });

    let num_fields = self.fields.len();
    let name_str = name.name_str();
    let iter_name = self.selector_iter_name();
    let indexer_name = self.indexer_name();

    let is_selected = self.fields().iter().map(|f| {
      let field_name = f.name();
      let field_variant = format_ident!("{}", field_name.name_str().to_upper_camel_case());
      let fn_name = format_ident!("is_{}_selected", field_name.name_str());
      quote! {
        #indexer_name::#field_variant => self.#fn_name()
      }
    });

    let is_unselected = self.fields().iter().map(|f| {
      let field_name = f.name();
      let field_variant = format_ident!("{}", field_name.name_str().to_upper_camel_case());
      let fn_name = format_ident!("is_{}_unselected", field_name.name_str());
      quote! {
        #indexer_name::#field_variant => self.#fn_name()
      }
    });

    quote! {
      // #[automatically_derived]
      // #[allow(non_camel_case_types)]
      // impl<__GROST_FLAVOR__> #name<__GROST_FLAVOR__>
      // where
      //   #(#where_clauses)*
      // {
      //   fn debug_helper(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
      //     use #path_to_grost::__private::Selector;

      //     let num_selected = self.selected();
      //     let mut idx = 0;
      //     ::core::write!(f, ::core::concat!("("))?;
      //     #(#debug)*
      //     ::core::write!(f, ")")
      //   }
      // }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<__GROST_FLAVOR__> ::core::fmt::Debug for #name<__GROST_FLAVOR__>
      where
        #(#where_clauses)*
      {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
          ::core::write!(f, #name_str)
          // self.debug_helper(f)
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<__GROST_FLAVOR__> ::core::cmp::PartialEq for #name<__GROST_FLAVOR__>
      where
        #(#where_clauses)*
      {
        fn eq(&self, other: &Self) -> ::core::primitive::bool {
          #(#eq) && *
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<__GROST_FLAVOR__> ::core::cmp::Eq for #name<__GROST_FLAVOR__>
      where
        #(#where_clauses)*
      {}

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<__GROST_FLAVOR__> ::core::hash::Hash for #name<__GROST_FLAVOR__>
      where
        #(#where_clauses)*
      {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
          #(#hash)*
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<__GROST_FLAVOR__> ::core::clone::Clone for #name<__GROST_FLAVOR__>
      where
        #(#where_clauses)*
      {
        fn clone(&self) -> Self {
          Self {
            #(#clone),*
          }
        }
      }

      // #[automatically_derived]
      // #[allow(non_camel_case_types)]
      // impl<__GROST_FLAVOR__> ::core::marker::Copy for #name<__GROST_FLAVOR__>
      // where
      //   #(#where_clauses)*
      // {}

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<__GROST_FLAVOR__> #path_to_grost::__private::Selector<__GROST_FLAVOR__> for #name<__GROST_FLAVOR__>
      where
        #(#where_clauses)*
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
      impl<__GROST_FLAVOR__> ::core::default::Default for #name<__GROST_FLAVOR__>
      where
        #(#where_clauses)*
      {
        fn default() -> Self {
          Self::new()
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl<__GROST_FLAVOR__> #name<__GROST_FLAVOR__>
      where
        #(#where_clauses)*
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
        pub fn iter_selected(&self) -> #iter_name<__GROST_FLAVOR__, true>
        {
          #iter_name::new(self, self.selected())
        }

        /// Returns an iterator over the unselected fields.
        #[inline]
        pub fn iter_unselected(&self) -> #iter_name<__GROST_FLAVOR__, false>
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
    field_reflection_ident: &Ident,
  ) -> impl Iterator<Item = proc_macro2::TokenStream> {
    self.fields.iter().map(move |f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      let wfr = wire_format_reflection_ty(field_reflection_ident, f.tag());
      let wf = wire_format_ty(path_to_grost, &wfr);
      quote! {
        <<#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector as #path_to_grost::__private::Selector<__GROST_FLAVOR__>>::merge(&mut self.#field_name, other.#field_name);
      }
    })
  }

  fn selector_flip_impl(
    &self,
    path_to_grost: &syn::Path,
    field_reflection_ident: &Ident,
  ) -> impl Iterator<Item = proc_macro2::TokenStream> {
    self.fields.iter().map(move |f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      let wfr = wire_format_reflection_ty(field_reflection_ident, f.tag());
      let wf = wire_format_ty(path_to_grost, &wfr);

      quote! {
        <<#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector as #path_to_grost::__private::Selector<__GROST_FLAVOR__>>::flip(&mut self.#field_name);
      }
    })
  }

  fn selector_field_fns(
    &self,
    path_to_grost: &syn::Path,
    field_reflection_ident: &Ident,
  ) -> impl Iterator<Item = proc_macro2::TokenStream> {
    self.fields.iter().map(move |f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      let select_fn_name = format_ident!("select_{}", field_name.name_str());
      let select_fn_doc = format!(
        " Select the `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let unselect_fn_name = format_ident!("unselect_{}", field_name.name_str());
      let unselect_fn_doc = format!(
        " Unselect the `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let update_fn_name = format_ident!("update_{}", field_name.name_str());
      let update_fn_doc = format!(
        " Update the `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let toggle_fn_name = format_ident!("toggle_{}", field_name.name_str());
      let toggle_fn_doc = format!(
        " Toggle the `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let with_fn_name = format_ident!("with_{}", field_name.name_str());
      let with_fn_doc = format!(
        " Set the `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let without_fn_name = format_ident!("without_{}", field_name.name_str());
      let without_fn_doc = format!(
        " Unset the `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let maybe_fn_name = format_ident!("maybe_{}", field_name.name_str());
      let maybe_fn_doc = format!(
        " Set or unset the `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let is_field_selected_fn_name = format_ident!("is_{}_selected", field_name.name_str());
      let is_field_selected_fn_doc = format!(
        " Returns `true` if the `{}.{}` field is selected",
        self.name.name_str(),
        field_name.name_str()
      );
      let is_field_unselected_fn_name = format_ident!("is_{}_unselected", field_name.name_str());
      let is_field_unselected_fn_doc = format!(
        " Returns `true` if the `{}.{}` field is unselected",
        self.name.name_str(),
        field_name.name_str()
      );

      let ref_fn_name = format_ident!("{}_ref", field_name.name_str());
      let ref_fn_doc = format!(
        " Get a reference to the selector of `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let ref_mut_fn_name = format_ident!("{}_mut", field_name.name_str());
      let ref_mut_fn_doc = format!(
        " Get a mutable reference to the selector of `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let wfr = wire_format_reflection_ty(field_reflection_ident, f.tag());
      let wf = wire_format_ty(path_to_grost, &wfr);

      quote! {
        #[doc = #select_fn_doc]
        #[inline]
        pub fn #select_fn_name(&mut self) -> &mut Self {
          self.#field_name = <<#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector as #path_to_grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
          self
        }

        #[doc = #unselect_fn_doc]
        #[inline]
        pub fn #unselect_fn_name(&mut self) -> &mut Self {
          self.#field_name = <<#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector as #path_to_grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
          self
        }

        #[doc = #update_fn_doc]
        #[inline]
        pub fn #update_fn_name(&mut self, value: <#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector) -> &mut Self {
          self.#field_name = value;
          self
        }

        #[doc = #maybe_fn_doc]
        #[inline]
        pub fn #maybe_fn_name(mut self, val: <#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector) -> Self {
          self.#field_name = val;
          self
        }

        #[doc = #ref_fn_doc]
        #[inline]
        pub const fn #ref_fn_name(&self) -> &<#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector {
          &self.#field_name
        }

        #[doc = #ref_mut_fn_doc]
        #[inline]
        pub const fn #ref_mut_fn_name(&mut self) -> &mut <#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector {
          &mut self.#field_name
        }

        #[doc = #with_fn_doc]
        #[inline]
        pub fn #with_fn_name(mut self) -> Self {
          self.#field_name = <<#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector as #path_to_grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
          self
        }

        #[doc = #without_fn_doc]
        #[inline]
        pub fn #without_fn_name(mut self) -> Self {
          self.#field_name = <<#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector as #path_to_grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
          self
        }

        #[doc = #is_field_selected_fn_doc]
        #[inline]
        pub fn #is_field_selected_fn_name(&self) -> ::core::primitive::bool {
          !<<#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector as #path_to_grost::__private::Selector<__GROST_FLAVOR__>>::is_empty(&self.#field_name)
        }

        #[doc = #is_field_unselected_fn_doc]
        #[inline]
        pub fn #is_field_unselected_fn_name(&self) -> ::core::primitive::bool {
          <<#rust_ty as #path_to_grost::__private::Selectable<__GROST_FLAVOR__, #wf>>::Selector as #path_to_grost::__private::Selector<__GROST_FLAVOR__>>::is_empty(&self.#field_name)
        }
      }

      // if !ty.primitive_selection_type() {

      // } else {
      //   quote! {
      //     #[doc = #select_fn_doc]
      //     #[inline]
      //     pub const fn #select_fn_name(&mut self) -> &mut Self {
      //       self.#field_name = true;
      //       self
      //     }

      //     #[doc = #unselect_fn_doc]
      //     #[inline]
      //     pub const fn #unselect_fn_name(&mut self) -> &mut Self {
      //       self.#field_name = false;
      //       self
      //     }

      //     #[doc = #update_fn_doc]
      //     #[inline]
      //     pub const fn #update_fn_name(&mut self, value: ::core::primitive::bool) -> &mut Self {
      //       self.#field_name = value;
      //       self
      //     }

      //     #[doc = #toggle_fn_doc]
      //     #[inline]
      //     pub const fn #toggle_fn_name(&mut self) -> &mut Self {
      //       self.#field_name = !self.#field_name;
      //       self
      //     }

      //     #[doc = #with_fn_doc]
      //     #[inline]
      //     pub const fn #with_fn_name(mut self) -> Self {
      //       self.#field_name = true;
      //       self
      //     }

      //     #[doc = #without_fn_doc]
      //     #[inline]
      //     pub const fn #without_fn_name(mut self) -> Self {
      //       self.#field_name = false;
      //       self
      //     }

      //     #[doc = #maybe_fn_doc]
      //     #[inline]
      //     pub const fn #maybe_fn_name(mut self, val: ::core::primitive::bool) -> Self {
      //       self.#field_name = val;
      //       self
      //     }

      //     #[doc = #is_field_selected_fn_doc]
      //     #[inline]
      //     pub const fn #is_field_selected_fn_name(&self) -> ::core::primitive::bool {
      //       self.#field_name
      //     }

      //     #[doc = #is_field_unselected_fn_doc]
      //     #[inline]
      //     pub const fn #is_field_unselected_fn_name(&self) -> ::core::primitive::bool {
      //       !self.#field_name
      //     }
      //   }
      // }
    })
  }
}

fn wire_format_ty(path_to_grost: &syn::Path, wf: &syn::Type) -> syn::Type {
  parse_quote! {
    <#wf as #path_to_grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection
  }
}

fn selector_ty(path_to_grost: &syn::Path, wf: &syn::Type) -> syn::Type {
  parse_quote! {
    #path_to_grost::__private::Selectable<
      __GROST_FLAVOR__,
      #wf,
    >
  }
}

fn constraints(
  path_to_grost: &syn::Path,
  field_reflection: impl ToTokens,
  fields: &[Field],
) -> impl Iterator<Item = proc_macro2::TokenStream> {
  fields.iter().map(move |f| {
    let ty = f.ty();
    let wfr = wire_format_reflection_ty(&field_reflection, f.tag());
    let wf = wire_format_ty(path_to_grost, &wfr);
    let selector_ty = selector_ty(path_to_grost, &wf);
    quote! {
      #wfr: #path_to_grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
      #ty: #selector_ty,
    }
  })
}
