use heck::ToUpperCamelCase;

use crate::Field;

use super::*;

impl Network {
  pub(super) fn generate_struct_index(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
    let indexer_name = struct_.indexer_name();
    let struct_name = struct_.name();
    let selector_name = struct_.selector_name();
    let select_by_index = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty();
      let field_reflection_name = self.field_reflection_name(f.name().name_str());
      let field_variant = format_ident!("{}", field_name.name_str().to_upper_camel_case());
      if ty.primitive_selection_type() {
        quote! {
          #indexer_name::#field_variant => match (select, self.#field_name) {
            (true, true) => ::core::option::Option::Some(&#struct_name::#field_reflection_name),
            (true, false) => ::core::option::Option::None,
            (false, true) => ::core::option::Option::None,
            (false, false) => ::core::option::Option::Some(&#struct_name::#field_reflection_name),
          }
        }
      } else {
        quote! {
          #indexer_name::#field_variant => match (select, self.#field_name.is_empty()) {
            (true, false) => ::core::option::Option::Some(&#struct_name::#field_reflection_name),
            (true, true) => ::core::option::Option::None,
            (false, false) => ::core::option::Option::None,
            (false, true) => ::core::option::Option::Some(&#struct_name::#field_reflection_name),
          }
        }
      }
    });
    let index = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let field_variant = format_ident!("{}", field_name.name_str().to_upper_camel_case());
      let ty = f.ty();
      if ty.primitive_selection_type() {
        quote! {
          #indexer_name::#field_variant => self.#field_name
        }
      } else {
        quote! {
          #indexer_name::#field_variant => !self.#field_name.is_empty()
        }
      }
    });

    quote! {
      #[automatically_derived]
      impl #selector_name<#path_to_grost::__private::flavors::Network> {
        fn __field_reflection_by_index_network_flavor(&self, idx: #indexer_name, select: ::core::primitive::bool) -> ::core::option::Option<&'static #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>> {
          match idx {
            #(#select_by_index),*
          }
        }
      }
    }
  }
}
