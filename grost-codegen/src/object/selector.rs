use heck::ToShoutySnakeCase;
use indexmap::IndexMap;
use quote::{ToTokens, format_ident};
use syn::Ident;

use crate::DeriveGenerator;

use super::*;

impl Object {
  pub(crate) fn selector_defination<F>(
    &self,
    path_to_grost: &syn::Path,
    flavor: &F,
  ) -> proc_macro2::TokenStream
  where
    F: DeriveGenerator + ?Sized,
  {
    let name = self.selector_name();
    let vis = self.visibility.as_ref();
    let doc = format!(" The selection type for {}", self.name.name_str());

    let flavor_ty = flavor.ty();
    let selection_fields = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      let wf = f.get_wire_format_or_default(path_to_grost, flavor);
      quote! {
        #field_name: <#rust_ty as #path_to_grost::__private::Selectable<#flavor_ty, #wf>>::Selector
      }
    });

    quote! {
      #[doc = #doc]
      #[allow(non_camel_case_types)]
      #vis struct #name {
        #(#selection_fields),*
      }
    }
  }

  pub(crate) fn selector_iter_defination(&self) -> proc_macro2::TokenStream {
    let name = self.selector_name();
    let name_str = name.name_str();
    let iter_name = self.selector_iter_name();
    let vis = self.visibility.as_ref();
    let iter_doc = format!(
      " An iterator over the selected fields of the [`{}`]",
      name_str,
    );
    let indexer_name = self.indexer_name();

    quote! {
      #[doc = #iter_doc]
      #[allow(non_camel_case_types)]
      #vis struct #iter_name<'__grost_selector_iter__, const N: ::core::primitive::bool = true> {
        selector: &'__grost_selector_iter__ #name,
        index: ::core::option::Option<#indexer_name>,
        num: ::core::primitive::usize,
        yielded: ::core::primitive::usize,
      }
    }
  }

  pub(crate) fn selector_iter_impl<F>(&self, flavor: &F) -> proc_macro2::TokenStream
  where
    F: DeriveGenerator + ?Sized,
  {
    let iter_name = self.selector_iter_name();
    let name = self.selector_name();
    let indexer_name = self.indexer_name();
    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<'__grost_selector_iter__, const N: ::core::primitive::bool> #iter_name<'__grost_selector_iter__, N> {
        #[inline]
        const fn new(selector: &'__grost_selector_iter__ #name, num: ::core::primitive::usize) -> Self {
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

  pub(crate) fn selector_impl<F>(
    &self,
    path_to_grost: &syn::Path,
    generator: &F,
  ) -> proc_macro2::TokenStream
  where
    F: DeriveGenerator + ?Sized,
  {
    let name = self.selector_name();
    let flavor_ty = generator.ty();

    let fns = self.selector_field_fns(path_to_grost, generator);

    let empty = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      let wf = f.get_wire_format_or_default(path_to_grost, generator);

      quote! {
        #field_name: <<#rust_ty as #path_to_grost::__private::Selectable<#flavor_ty, #wf>>::Selector as #path_to_grost::__private::Selector<#flavor_ty>>::NONE
      }
    });

    let all = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      let wf = f.get_wire_format_or_default(path_to_grost, generator);

      quote! {
        #field_name: <<#rust_ty as #path_to_grost::__private::Selectable<#flavor_ty, #wf>>::Selector as #path_to_grost::__private::Selector<#flavor_ty>>::ALL
      }
    });

    let is_empty = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();

      if ty.primitive_selection_type() {
        quote! {
          !self.#field_name
        }
      } else {
        quote! {
          self.#field_name.is_empty()
        }
      }
    });

    let is_all = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();

      if ty.primitive_selection_type() {
        quote! {
          self.#field_name
        }
      } else {
        quote! {
          self.#field_name.is_all()
        }
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

    let merge = self.selector_merge_impl(path_to_grost, generator);

    let flip = self.selector_flip_impl(path_to_grost, generator);

    let debug = self.fields.iter().map(|f| {
      let field_name = f.name();
      let field_name_str = f.schema_name();
      let ty = f.ty();
      if ty.primitive_selection_type() {
        quote! {
          if self.#field_name {
            if idx != num_selected - 1 {
              ::core::write!(f, ::core::concat!(#field_name_str, " & "))?;
            } else {
              ::core::write!(f, #field_name_str)?;
            }
            idx += 1;
          }
        }
      } else {
        quote! {
          if !self.#field_name.is_empty() {
            if idx != num_selected - 1 {
              ::core::write!(f, #field_name_str)?;
              self.#field_name.debug_helper(f)?;
              ::core::write!(f, " & ")?;
            } else {
              ::core::write!(f, #field_name_str)?;
              self.#field_name.debug_helper(f)?;
            }
            idx += 1;
          }
        }
      }
    });

    let eq = self.fields.iter().map(|f| {
      let field_name = f.name();
      quote! {
        self.#field_name == other.#field_name
      }
    });

    let hash = self.fields.iter().map(|f| {
      let field_name = f.name();
      quote! {
        self.#field_name.hash(state);
      }
    });

    let struct_name = &self.name;
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

    let flavor_ty = generator.ty();

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #name {
        fn debug_helper(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
          let num_selected = self.selected();
          let mut idx = 0;
          ::core::write!(f, ::core::concat!("("))?;
          #(#debug)*
          ::core::write!(f, ")")
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl ::core::fmt::Debug for #name {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
          ::core::write!(f, #name_str)?;
          self.debug_helper(f)
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl ::core::cmp::PartialEq for #name {
        fn eq(&self, other: &Self) -> ::core::primitive::bool {
          #(#eq) && *
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl ::core::cmp::Eq for #name {}

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl ::core::hash::Hash for #name {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
          #(#hash)*
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl ::core::clone::Clone for #name {
        fn clone(&self) -> Self {
          *self
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl ::core::marker::Copy for #name {}

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #path_to_grost::__private::Selector<#flavor_ty> for #name {
        const ALL: Self = Self::all();
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
      #[allow(non_camel_case_types)]
      impl #name {
        /// The number of options in this selection type.
        pub const OPTIONS: ::core::primitive::usize = #num_fields;

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
        pub const fn is_empty(&self) -> ::core::primitive::bool {
          #(#is_empty) && *
        }

        /// Returns `true` if the selector selects all.
        #[inline]
        pub const fn is_all(&self) -> ::core::primitive::bool {
          #(#is_all) && *
        }

        /// Returns the number of selected fields.
        #[inline]
        pub const fn selected(&self) -> ::core::primitive::usize {
          let mut num = 0;
          #(#num_selected)*
          num
        }

        /// Returns the number of unselected fields.
        #[inline]
        pub const fn unselected(&self) -> ::core::primitive::usize {
          let mut num = 0;
          #(#num_unselected)*
          num
        }

        /// Returns an iterator over the selected fields.
        #[inline]
        pub const fn iter_selected(&self) -> #iter_name<true>
        {
          #iter_name::new(self, self.selected())
        }

        /// Returns an iterator over the unselected fields.
        #[inline]
        pub const fn iter_unselected(&self) -> #iter_name<false>
        {
          #iter_name::new(self, self.unselected())
        }

        /// Returns `true` if such field is selected.
        #[inline]
        pub const fn is_selected(&self, idx: #indexer_name) -> ::core::primitive::bool {
          match idx {
            #(#is_selected),*
          }
        }

        /// Returns `true` if such field is unselected.
        #[inline]
        pub const fn is_unselected(&self, idx: #indexer_name) -> ::core::primitive::bool {
          match idx {
            #(#is_unselected),*
          }
        }

        #(#fns)*
      }
    }
  }

  fn selector_merge_impl<F>(
    &self,
    path_to_grost: &syn::Path,
    generator: &F,
  ) -> impl Iterator<Item = proc_macro2::TokenStream>
  where
    F: DeriveGenerator + ?Sized,
  {
    let flavor_ty = generator.ty();
    self.fields.iter().map(move |f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      let wf = f.get_wire_format_or_default(path_to_grost, generator);
      quote! {
        <<#rust_ty as #path_to_grost::__private::Selectable<#flavor_ty, #wf>>::Selector as #path_to_grost::__private::Selector<#flavor_ty>>::merge(&mut self.#field_name, other.#field_name);
      }
    })
  }

  fn selector_flip_impl<F>(
    &self,
    path_to_grost: &syn::Path,
    generator: &F,
  ) -> impl Iterator<Item = proc_macro2::TokenStream>
  where
    F: DeriveGenerator + ?Sized,
  {
    let flavor_ty = generator.ty();
    self.fields.iter().map(move |f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      let wf = f.get_wire_format_or_default(path_to_grost, generator);

      quote! {
        <<#rust_ty as #path_to_grost::__private::Selectable<#flavor_ty, #wf>>::Selector as #path_to_grost::__private::Selector<#flavor_ty>>::flip(&mut self.#field_name);
      }
    })
  }

  fn selector_field_fns<F>(
    &self,
    path_to_grost: &syn::Path,
    generator: &F,
  ) -> impl Iterator<Item = proc_macro2::TokenStream>
  where
    F: DeriveGenerator + ?Sized,
  {
    let flavor_ty = generator.ty();
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

      if !ty.primitive_selection_type() {
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
        let wf = f.get_wire_format_or_default(path_to_grost, generator);
        quote! {
          #[doc = #select_fn_doc]
          #[inline]
          pub fn #select_fn_name(&mut self, val: <#rust_ty as #path_to_grost::__private::Selectable<#flavor_ty, #wf>>::Selector) -> &mut Self {
            self.#field_name = val;
            self
          }

          #[doc = #unselect_fn_doc]
          #[inline]
          pub fn #unselect_fn_name(&mut self) -> &mut Self {
            self.#field_name = <<#rust_ty as #path_to_grost::__private::Selectable<#flavor_ty, #wf>>::Selector as #path_to_grost::__private::Selector<#flavor_ty>>::NONE;
            self
          }

          #[doc = #ref_fn_doc]
          #[inline]
          pub const fn #ref_fn_name(&self) -> &<#rust_ty as #path_to_grost::__private::Selectable<#flavor_ty, #wf>>::Selector {
            &self.#field_name
          }

          #[doc = #ref_mut_fn_doc]
          #[inline]
          pub const fn #ref_mut_fn_name(&mut self) -> &mut <#rust_ty as #path_to_grost::__private::Selectable<#flavor_ty, #wf>>::Selector {
            &mut self.#field_name
          }

          #[doc = #is_field_selected_fn_doc]
          #[inline]
          pub const fn #is_field_selected_fn_name(&self) -> ::core::primitive::bool {
            !self.#field_name.is_empty()
          }

          #[doc = #is_field_unselected_fn_doc]
          #[inline]
          pub const fn #is_field_unselected_fn_name(&self) -> ::core::primitive::bool {
            self.#field_name.is_empty()
          }
        }
      } else {
        quote! {
          #[doc = #select_fn_doc]
          #[inline]
          pub const fn #select_fn_name(&mut self) -> &mut Self {
            self.#field_name = true;
            self
          }

          #[doc = #unselect_fn_doc]
          #[inline]
          pub const fn #unselect_fn_name(&mut self) -> &mut Self {
            self.#field_name = false;
            self
          }

          #[doc = #update_fn_doc]
          #[inline]
          pub const fn #update_fn_name(&mut self, value: ::core::primitive::bool) -> &mut Self {
            self.#field_name = value;
            self
          }

          #[doc = #toggle_fn_doc]
          #[inline]
          pub const fn #toggle_fn_name(&mut self) -> &mut Self {
            self.#field_name = !self.#field_name;
            self
          }

          #[doc = #with_fn_doc]
          #[inline]
          pub const fn #with_fn_name(mut self) -> Self {
            self.#field_name = true;
            self
          }

          #[doc = #without_fn_doc]
          #[inline]
          pub const fn #without_fn_name(mut self) -> Self {
            self.#field_name = false;
            self
          }

          #[doc = #maybe_fn_doc]
          #[inline]
          pub const fn #maybe_fn_name(mut self, val: ::core::primitive::bool) -> Self {
            self.#field_name = val;
            self
          }

          #[doc = #is_field_selected_fn_doc]
          #[inline]
          pub const fn #is_field_selected_fn_name(&self) -> ::core::primitive::bool {
            self.#field_name
          }

          #[doc = #is_field_unselected_fn_doc]
          #[inline]
          pub const fn #is_field_unselected_fn_name(&self) -> ::core::primitive::bool {
            !self.#field_name
          }
        }
      }
    })
  }
}
