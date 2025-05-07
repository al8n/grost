use heck::ToShoutySnakeCase;
use indexmap::IndexMap;
use quote::{ToTokens, format_ident};
use syn::Ident;

use crate::FlavorGenerator;

use super::*;

impl Struct {
  pub(crate) fn generate_selector_defination(
    &self,
    path_to_grost: &syn::Path,
  ) -> proc_macro2::TokenStream {
    let name = self.selector_name();
    let vis = self.visibility.as_ref();
    let doc = format!(" The selection type for {}", self.name.name_str());

    let selection_fields = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      if ty.primitive_selection_type() {
        quote! {
          #field_name: ::core::primitive::bool
        }
      } else {
        quote! {
          #field_name: <#rust_ty as #path_to_grost::__private::Selectable<F>>::Selector
        }
      }
    });

    quote! {
      #[doc = #doc]
      #vis struct #name<F: ?::core::marker::Sized> {
        _m: ::core::marker::PhantomData<F>,
        #(#selection_fields),*
      }
    }
  }

  pub(crate) fn generate_selector_iter_defination(&self) -> proc_macro2::TokenStream {
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
      #vis struct #iter_name<'a, F: ?::core::marker::Sized, const N: ::core::primitive::bool = true> {
        selector: &'a #name<F>,
        index: ::core::option::Option<#indexer_name>,
        num: ::core::primitive::usize,
        yielded: ::core::primitive::usize,
      }
    }
  }

  pub(crate) fn generate_selector_iter_impl(&self) -> proc_macro2::TokenStream {
    let iter_name = self.selector_iter_name();
    let name = self.selector_name();
    let indexer_name = self.indexer_name();
    quote! {
      impl<'a, F: ?::core::marker::Sized, const N: ::core::primitive::bool> #iter_name<'a, F, N> {
        #[inline]
        const fn new(selector: &'a #name<F>, num: ::core::primitive::usize) -> Self {
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

  pub(crate) fn generate_selector_impl(
    &self,
    path_to_grost: &syn::Path,
  ) -> proc_macro2::TokenStream {
    let name = self.selector_name();
    let fields = &self.fields;

    let fns = fields.iter().map(move |f| {
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
        quote! {
          #[doc = #select_fn_doc]
          #[inline]
          pub fn #select_fn_name(&mut self, val: <#rust_ty as #path_to_grost::__private::Selectable<F>>::Selector) -> &mut Self {
            self.#field_name = val;
            self
          }
  
          #[doc = #unselect_fn_doc]
          #[inline]
          pub fn #unselect_fn_name(&mut self) -> &mut Self {
            self.#field_name = <<#rust_ty as #path_to_grost::__private::Selectable<F>>::Selector as #path_to_grost::__private::Selector<F>>::NONE;
            self
          }

          #[doc = #ref_fn_doc]
          #[inline]
          pub const fn #ref_fn_name(&self) -> &<#rust_ty as #path_to_grost::__private::Selectable<F>>::Selector {
            &self.#field_name
          }

          #[doc = #ref_mut_fn_doc]
          #[inline]
          pub const fn #ref_mut_fn_name(&mut self) -> &mut <#rust_ty as #path_to_grost::__private::Selectable<F>>::Selector {
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
    });

    let empty = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();

      if ty.primitive_selection_type() {
        quote! {
          #field_name: false
        }
      } else {
        quote! {
          #field_name: <<#rust_ty as #path_to_grost::__private::Selectable<F>>::Selector as #path_to_grost::__private::Selector<F>>::NONE
        }
      } 
    });

    let all = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();

      if ty.primitive_selection_type() {
        quote! {
          #field_name: true
        }
      } else {
        quote! {
          #field_name: <<#rust_ty as #path_to_grost::__private::Selectable<F>>::Selector as #path_to_grost::__private::Selector<F>>::ALL
        }
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

    let merge = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();
      if ty.primitive_selection_type() {
        quote! {
          self.#field_name |= other.#field_name;
        }
      } else {
        quote! {
          <<#rust_ty as #path_to_grost::__private::Selectable<F>>::Selector as #path_to_grost::__private::Selector<F>>::merge(&mut self.#field_name, other.#field_name);
        }
      }
    });

    let flip = self.fields.iter().map(|f| {
      let ty = f.ty();
      let field_name = f.name();
      let rust_ty = ty.ty();

      if ty.primitive_selection_type() {
        quote! {
          self.#field_name = !self.#field_name;
        }
      } else {
        quote! {
          <<#rust_ty as #path_to_grost::__private::Selectable<F>>::Selector as #path_to_grost::__private::Selector<F>>::flip(&mut self.#field_name);
        }
      }
    });

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

    quote! {
      #[automatically_derived]
      impl<F: ?::core::marker::Sized> #name<F> {
        fn debug_helper(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
          let num_selected = self.selected();
          let mut idx = 0;
          ::core::write!(f, ::core::concat!("("))?;
          #(#debug)*
          ::core::write!(f, ")")
        }
      }

      #[automatically_derived]
      impl<F: ?::core::marker::Sized> ::core::fmt::Debug for #name<F> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
          ::core::write!(f, #name_str)?;
          self.debug_helper(f)
        }
      }

      #[automatically_derived]
      impl<F: ?::core::marker::Sized> ::core::cmp::PartialEq for #name<F> {
        fn eq(&self, other: &Self) -> ::core::primitive::bool {
          #(#eq) && *
        }
      }

      #[automatically_derived]
      impl<F: ?::core::marker::Sized> ::core::cmp::Eq for #name<F> {}

      #[automatically_derived]
      impl<F: ?::core::marker::Sized> ::core::hash::Hash for #name<F> {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
          #(#hash)*
        }
      }

      #[automatically_derived]
      impl<F: ?::core::marker::Sized> ::core::clone::Clone for #name<F> {
        fn clone(&self) -> Self {
          *self
        }
      }

      #[automatically_derived]
      impl<F: ?::core::marker::Sized> ::core::marker::Copy for #name<F> {}

      #[automatically_derived]
      impl<F: ?::core::marker::Sized> #path_to_grost::__private::Selectable<F> for #struct_name {
        type Selector = #name<F>;
      }

      #[automatically_derived]
      impl<F: ?::core::marker::Sized> #path_to_grost::__private::Selector<F> for #name<F> {
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
      impl<F: ?::core::marker::Sized> #name<F> {
        /// The number of options in this selection type.
        pub const OPTIONS: ::core::primitive::usize = #num_fields;

        /// Returns a selector which selects nothing.
        #[inline]
        pub const fn empty() -> Self {
          Self {
            _m: ::core::marker::PhantomData,
            #(#empty),*
          }
        }

        /// Returns a selector which selects all.
        #[inline]
        pub const fn all() -> Self {
          Self {
            _m: ::core::marker::PhantomData,
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
        pub const fn iter_selected(&self) -> #iter_name<F, true>
        {
          #iter_name::new(self, self.selected())
        }

        /// Returns an iterator over the unselected fields.
        #[inline]
        pub const fn iter_unselected(&self) -> #iter_name<F, false>
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

      // #path_to_grost::__private::selectable_scalar!(#path_to_grost::__private::flavors::Select: #name<#path_to_grost::__private::flavors::Network>);
      // #path_to_grost::__private::partial_encode_scalar!(#path_to_grost::__private::flavors::Select: #name<#path_to_grost::__private::flavors::Network> as #path_to_grost::__private::flavors::network::LengthDelimited);
      // #path_to_grost::__private::decode_owned_scalar!(#path_to_grost::__private::flavors::Select: #name<#path_to_grost::__private::flavors::Network> as #path_to_grost::__private::flavors::network::LengthDelimited);

      #[automatically_derived]
      impl<F: ?::core::marker::Sized> #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::Select, #path_to_grost::__private::flavors::selector::Zst> for #name<F> {
        fn encode(&self, _: &<#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::EncodeError> {
          const SELECT_NONE: ::core::primitive::u8 = #path_to_grost::__private::flavors::selector::SelectorIdentifier::none().as_u8();
          const SELECT_ALL: ::core::primitive::u8 = #path_to_grost::__private::flavors::selector::SelectorIdentifier::all().as_u8();

          if self.is_empty() {
            if buf.is_empty() {
              return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(1, 0));
            }

            buf[0] = SELECT_NONE;
            return ::core::result::Result::Ok(1);
          }

          if self.is_all() {
            if buf.is_empty() {
              return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(1, 0));
            }

            buf[0] = SELECT_ALL;
            return ::core::result::Result::Ok(1);
          }

          ::core::result::Result::Err(#path_to_grost::__private::EncodeError::custom("only select all fields or no fields can be encoded as zst"))
        }

        fn encoded_len(&self, _: &<#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::Context) -> ::core::primitive::usize {
          1
        }

        fn encoded_length_delimited_len(&self, ctx: &<#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::Context) -> ::core::primitive::usize {
          <Self as #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::Select, #path_to_grost::__private::flavors::selector::Zst>>::encoded_len(self, ctx)
        }

        fn encode_length_delimited(&self, ctx: &<#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::EncodeError> {
          <Self as #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::Select, #path_to_grost::__private::flavors::selector::Zst>>::encode(self, ctx, buf)
        }

        fn encode_identified(
          &self,
          ctx: &<#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::Context,
          identifier: &<#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::Identifier,
          buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::EncodeError> {
          if identifier.wire_type() != #path_to_grost::__private::flavors::selector::SelectorWireType::Zst {
            return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::unsupported_wire_type(
              ::core::any::type_name::<Self>(),
              identifier.wire_type(),
            ));
          }

          <Self as #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::Select, #path_to_grost::__private::flavors::selector::Zst>>::encode(self, ctx, buf)
        }

        fn encoded_identified_len(
          &self,
          ctx: &<#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::Context,
          identifier: &<#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::Identifier,
        ) -> ::core::primitive::usize {
          <Self as #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::Select, #path_to_grost::__private::flavors::selector::Zst>>::encoded_len(self, ctx)
        }
      }
    }
  }

  fn generate_codec<F>(&self, path_to_grost: &syn::Path, flavors: &F) -> proc_macro2::TokenStream
  where
    F: FlavorGenerator + ?Sized,
  {
    let name = self.selector_name();
    let flavor_ty = flavors.ty();

    quote! {
      const _: () = {
        const ALL_TAG: ::core::primitive::u8 = 1;
        const NONE_TAG: ::core::primitive::u8 = 2;
        const SELECT_TAG: ::core::primitive::u8 = 3;
        const UNSELECT_TAG: ::core::primitive::u8 = 4;
        const SELECT_ONE_TAG: ::core::primitive::u8 = 5;
        const UNSELECT_ONE_TAG: ::core::primitive::u8 = 6;

        impl #name {
          /// Encode the selection into a buffer.
          ///
          /// Returns the number of bytes written to the buffer.
          #[inline]
          pub fn encode(&self, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::EncodeError> {
            use ::core::iter::Iterator;

            macro_rules! encode {
              (@many $buf_len:ident, $fn:ident, $tag:ident) => {{
                buf[0] = $tag;
                let mut offset = 1;
                let data_size = self.$fn::<#flavor_ty>().map(|f| #path_to_grost::__private::varing::encoded_u32_varint_len(f.tag().get())).sum::<::core::primitive::usize>();
                let data_size_len = #path_to_grost::__private::varing::encoded_u32_varint_len(data_size as ::core::primitive::u32);
                let total_len = offset + data_size_len + data_size;

                if $buf_len < total_len {
                  return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(total_len, $buf_len));
                }

                offset += #path_to_grost::__private::varing::encode_u32_varint_to(data_size as ::core::primitive::u32, &mut buf[offset..])
                  .map_err(|e| #path_to_grost::__private::EncodeError::from_varint_error(e).update(total_len, $buf_len))?;

                for tag in self.$fn::<#flavor_ty>().map(|f| f.tag().get()) {
                  offset += #path_to_grost::__private::varing::encode_u32_varint_to(tag, &mut buf[offset..])
                    .map_err(|e| #path_to_grost::__private::EncodeError::from_varint_error(e).update(total_len, $buf_len))?;
                }

                ::core::result::Result::Ok(offset)
              }};
              (@single $buf_len:ident, $fn:ident, $tag:ident) => {{
                buf[0] = $tag;
                let selected = self.$fn::<#flavor_ty>().next().unwrap().tag().get();
                #path_to_grost::__private::varing::encode_u32_varint_to(selected, &mut buf[1..])
                  .map_err(|e| #path_to_grost::__private::EncodeError::from_varint_error(e).update(self.encoded_len(), $buf_len))
              }};
            }

            if self.is_empty() {
              if buf.is_empty() {
                return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(1, buf.len()));
              }

              buf[0] = NONE_TAG;
              return ::core::result::Result::Ok(1);
            }

            if self.is_all() {
              if buf.is_empty() {
                return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(1, buf.len()));
              }

              buf[0] = ALL_TAG;
              return ::core::result::Result::Ok(1);
            }

            let num_selected = self.num_selected();
            let num_unselected = self.num_unselected();
            let buf_len = buf.len();
            if buf_len < 2 {
              return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(self.encoded_len(), buf_len));
            }

            if num_selected >= num_unselected {
              if num_selected == 1 {
                encode!{ @single buf_len, iter_selected, SELECT_ONE_TAG }
              } else {
                encode!{ @many buf_len, iter_selected, SELECT_TAG }
              }
            } else if num_unselected == 1 {
              encode!{ @single buf_len, iter_unselected, UNSELECT_ONE_TAG }
            } else {
              encode!{ @many buf_len, iter_unselected, UNSELECT_TAG }
            }
          }

          /// Returns the length of the encoded selection.
          #[inline]
          pub fn encoded_len(&self) -> ::core::primitive::usize {
            use ::core::iter::Iterator;

            macro_rules! len {
              (@many $fn:ident) => {{
                let data_size = self.$fn::<#flavor_ty>().map(|f| #path_to_grost::__private::varing::encoded_u32_varint_len(f.tag().get())).sum::<::core::primitive::usize>();
                let data_size_len = #path_to_grost::__private::varing::encoded_u32_varint_len(data_size as ::core::primitive::u32);
                1 + data_size_len + data_size
              }};
              (@single $fn:ident) => {{
                let selected = self.$fn::<#flavor_ty>().next().unwrap().tag().get();
                1 + #path_to_grost::__private::varing::encoded_u32_varint_len(selected)
              }};
            }

            if self.is_empty() {
              return 1;
            }

            if self.is_all() {
              return 1;
            }

            let num_selected = self.num_selected();
            let num_unselected = self.num_unselected();

            if num_selected >= num_unselected {
              if num_selected == 1 {
                len!{ @single iter_selected }
              } else {
                len!{ @many iter_selected }
              }
            } else if num_unselected == 1 {
              len!{ @single iter_unselected }
            } else {
              len!{ @many iter_unselected }
            }
          }

          /// Decodes the selection from a buffer.
          pub fn decode<'a, F, UB>(src: &'a [u8]) -> ::core::result::Result<(::core::primitive::usize, #path_to_grost::__private::SelectionSet<Self, UB>), #path_to_grost::__private::DecodeError<F>>
          where
            F: #path_to_grost::__private::Flavor + ?::core::marker::Sized,
            UB: #path_to_grost::__private::Buffer<#path_to_grost::__private::Unknown<F, &'a [::core::primitive::u8]>> + 'a,
          {
            if src.is_empty() {
              return ::core::result::Result::Err(#path_to_grost::__private::DecodeError::buffer_underflow());
            }

            let tag = src[0];
            match tag {
              NONE_TAG => {
                ::core::result::Result::Ok((1, #path_to_grost::__private::SelectionSet::new(Self::empty(), ::core::option::Option::None)))
              }
              ALL_TAG => {
                ::core::result::Result::Ok((1, #path_to_grost::__private::SelectionSet::new(Self::all(), ::core::option::Option::None)))
              }
              SELECT_TAG => {
                let (read, data_size) = #path_to_grost::__private::varing::decode_u32_varint(&src[1..])?;

                let mut offset = 1 + read;
                let total = offset + data_size as usize;
                if total > src.len() {
                  return ::core::result::Result::Err(#path_to_grost::__private::DecodeError::buffer_underflow());
                }

                let mut selection = Self::empty();

                while offset < total {
                  let (read, tag) = #path_to_grost::__private::varing::decode_u32_varint(&src[offset..])?;
                  offset += read;


                }

                ::core::result::Result::Ok((total, #path_to_grost::__private::SelectionSet::new(selection, ::core::option::Option::None)))
              }
              UNSELECT_TAG => {
                let (read, data_size) = #path_to_grost::__private::varing::decode_u32_varint(&src[1..])?;
                let mut offset = 1 + read;
                if offset + data_size as usize > src.len() {
                  return ::core::result::Result::Err(#path_to_grost::__private::DecodeError::buffer_underflow());
                }

                ::core::todo!()
              }
              SELECT_ONE_TAG => {
                let (read, tag) = #path_to_grost::__private::varing::decode_u32_varint(&src[1..])?;
                ::core::todo!()
              }
              UNSELECT_ONE_TAG => {
                let (read, tag) = #path_to_grost::__private::varing::decode_u32_varint(&src[1..])?;
                ::core::todo!()
              },
              _ => {

              }
            }
          }
        };
      };
    }
  }
}
