use heck::ToUpperCamelCase;
use quote::{format_ident, quote};

use crate::{Object, network::Network};

impl Network {
  pub(crate) fn generate_struct_index(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
  ) -> proc_macro2::TokenStream {
    let indexer_name = struct_.indexer_name();
    let selector_name = struct_.selector_name();

    let index_trait_select = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty();
      let field_variant = format_ident!("{}", field_name.name_str().to_upper_camel_case());
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();
      let reflection = quote! {
        const REFLECTION: ::core::option::Option<&#path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>> = ::core::option::Option::Some(
          <
            #field_reflection<
              #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::Network,
            >
          >::REFLECTION
        );
      };

      if ty.primitive_selection_type() {
        quote! {
          #indexer_name::#field_variant => {
            #reflection

            match (select, self.#field_name) {
              (true, true) => &REFLECTION,
              (true, false) => NONE,
              (false, true) => NONE,
              (false, false) => &REFLECTION,
            }
          }
        }
      } else {
        quote! {
          #indexer_name::#field_variant => {
            #reflection

            match (select, self.#field_name.is_empty()) {
              (true, false) => &REFLECTION,
              (true, true) => NONE,
              (false, false) => NONE,
              (false, true) => &REFLECTION,
            }
          }
        }
      }
    });

    let index_selector = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty();
      let field_variant = format_ident!("{}", field_name.name_str().to_upper_camel_case());
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();
      let reflection = quote! {
        const REFLECTION: ::core::option::Option<&#path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>> = ::core::option::Option::Some(
          <
            #field_reflection<
              #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::Network,
            >
          >::REFLECTION
        );
      };
      if ty.primitive_selection_type() {
        quote! {
          #indexer_name::#field_variant => {
            #reflection

            if self.#field_name {
              &REFLECTION
            } else {
              NONE
            }
          }
        }
      } else {
        quote! {
          #indexer_name::#field_variant => {
            #reflection

            if self.#field_name.is_empty() {
              NONE
            } else {
              &REFLECTION
            }
          }
        }
      }
    });

    quote! {
      #[automatically_derived]
      impl ::core::ops::Index<(#indexer_name, ::core::primitive::bool)> for #selector_name<#path_to_grost::__private::flavors::Network> {
        type Output = ::core::option::Option<&'static #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>>;

        fn index(
          &self,
          (indexer, select): (#indexer_name, ::core::primitive::bool),
        ) -> &Self::Output {
          const NONE: &::core::option::Option<&'static #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>> = &::core::option::Option::None;

          match indexer {
            #(#index_trait_select),*
          }
        }
      }

      #[automatically_derived]
      impl ::core::ops::Index<#indexer_name> for #selector_name<#path_to_grost::__private::flavors::Network> {
        type Output = ::core::option::Option<&'static #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>>;

        fn index(
          &self,
          indexer: #indexer_name,
        ) -> &Self::Output {
          const NONE: &::core::option::Option<&#path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>> = &::core::option::Option::None;

          match indexer {
            #(#index_selector),*
          }
        }
      }
    }
  }
}
