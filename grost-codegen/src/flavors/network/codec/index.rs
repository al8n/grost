use heck::ToUpperCamelCase;
use quote::{ToTokens, format_ident, quote};

use crate::{Field, FlavorGeneratorExt, Struct, network::Network};

use super::*;

impl Network {
  pub(crate) fn generate_struct_index(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
    let indexer_name = struct_.indexer_name();
    let index_name = struct_.index_name();
    let struct_name = struct_.name();
    let selector_name = struct_.selector_name();

    let reflection_mod_name = struct_.reflection_mod_name();
    let reflection_name = struct_.reflection_name();
    let index_trait_select = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty();
      let field_variant = format_ident!("{}", field_name.name_str().to_upper_camel_case());
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();
      let reflection = quote! {
        const REFLECTION: ::core::option::Option<&#path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>> = ::core::option::Option::Some(
          <
            #reflection_mod_name::#field_reflection<
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
            #reflection_mod_name::#field_reflection<
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

    let field_reflection_indexing = self.generate_index_field_reflection(
      struct_,
      path_to_grost,
      &indexer_name,
      &index_name,
      struct_name.name(),
    );

    let field_tag_indexing = self.generate_index_tag(
      struct_,
      path_to_grost,
      &indexer_name,
      &index_name,
      struct_name.name(),
    );

    let field_wire_type_indexing = self.generate_index_wire_type(
      struct_,
      path_to_grost,
      &indexer_name,
      &index_name,
      struct_name.name(),
    );

    let field_identifier_indexing = self.generate_index_identifier(
      struct_,
      path_to_grost,
      &indexer_name,
      &index_name,
      struct_name.name(),
    );

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

      // #field_reflection_indexing
      // #field_tag_indexing
      // #field_wire_type_indexing
      // #field_identifier_indexing
    }
  }

  fn generate_index_identifier(
    &self,
    struct_: &Struct,
    path_to_grost: &syn::Path,
    indexer_name: &syn::Ident,
    index_name: &syn::Ident,
    struct_name: &syn::Ident,
  ) -> proc_macro2::TokenStream {
    let index_identifier = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let field_variant = format_ident!("{}", field_name.name_str().to_upper_camel_case());
      let name = self.field_identifier_const_name(f.name().name_str());
      quote! {
        #indexer_name::#field_variant => &#name
      }
    });

    let output = quote! {#path_to_grost::__private::flavors::network::Identifier};
    let debug = self.generate_index_debug(path_to_grost, index_name, &output);
    let deref = self.generate_index_deref(path_to_grost, index_name, &output, true);
    let display = self.generate_index_display(path_to_grost, index_name, &output);

    quote! {
      #[automatically_derived]
      impl ::core::ops::Index<()> for #index_name<#path_to_grost::__private::flavors::network::Identifier,  #path_to_grost::__private::flavors::Network> {
        type Output = #path_to_grost::__private::network::Identifier;

        fn index(
          &self,
          _: (),
        ) -> &Self::Output {
          match ::core::convert::AsRef::<#indexer_name>::as_ref(self) {
            #(#index_identifier),*
          }
        }
      }

      #debug
      #deref
      #display
    }
  }

  fn generate_index_wire_type(
    &self,
    struct_: &Struct,
    path_to_grost: &syn::Path,
    indexer_name: &syn::Ident,
    index_name: &syn::Ident,
    struct_name: &syn::Ident,
  ) -> proc_macro2::TokenStream {
    let index_wire_type = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let field_variant = format_ident!("{}", field_name.name_str().to_upper_camel_case());
      let name = self.field_wire_type_const_name(f.name().name_str());
      quote! {
        #indexer_name::#field_variant => &#name
      }
    });

    let output = quote! {#path_to_grost::__private::flavors::network::WireType};
    let debug = self.generate_index_debug(path_to_grost, index_name, &output);
    let deref = self.generate_index_deref(path_to_grost, index_name, &output, true);
    let display = self.generate_index_display(path_to_grost, index_name, &output);

    quote! {
      #[automatically_derived]
      impl ::core::ops::Index<()> for #index_name<#path_to_grost::__private::flavors::network::WireType,  #path_to_grost::__private::flavors::Network> {
        type Output = #path_to_grost::__private::network::WireType;

        fn index(
          &self,
          _: (),
        ) -> &Self::Output {
          match ::core::convert::AsRef::<#indexer_name>::as_ref(self) {
            #(#index_wire_type),*
          }
        }
      }

      #debug
      #deref
      #display
    }
  }

  fn generate_index_tag(
    &self,
    struct_: &Struct,
    path_to_grost: &syn::Path,
    indexer_name: &syn::Ident,
    index_name: &syn::Ident,
    struct_name: &syn::Ident,
  ) -> proc_macro2::TokenStream {
    let index_tag = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let field_variant = format_ident!("{}", field_name.name_str().to_upper_camel_case());
      let tag_name = self.field_tag_const_name(f.name().name_str());
      quote! {
        #indexer_name::#field_variant => &#tag_name
      }
    });
    let output = quote! {#path_to_grost::__private::flavors::network::Tag};
    let debug = self.generate_index_debug(path_to_grost, index_name, &output);
    let deref = self.generate_index_deref(path_to_grost, index_name, &output, true);
    let display = self.generate_index_display(path_to_grost, index_name, &output);

    quote! {
      #[automatically_derived]
      impl ::core::ops::Index<()> for #index_name<#path_to_grost::__private::flavors::network::Tag,  #path_to_grost::__private::flavors::Network> {
        type Output = #path_to_grost::__private::network::Tag;

        fn index(
          &self,
          _: (),
        ) -> &Self::Output {
          match ::core::convert::AsRef::<#indexer_name>::as_ref(self) {
            #(#index_tag),*
          }
        }
      }

      #debug
      #deref
      #display
    }
  }

  fn generate_index_field_reflection(
    &self,
    struct_: &Struct,
    path_to_grost: &syn::Path,
    indexer_name: &syn::Ident,
    index_name: &syn::Ident,
    struct_name: &syn::Ident,
  ) -> proc_macro2::TokenStream {
    let index_field_reflection = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let field_variant = format_ident!("{}", field_name.name_str().to_upper_camel_case());
      let r = self.field_reflection_name(f.name().name_str());
      quote! {
        #indexer_name::#field_variant => &#struct_name::#r
      }
    });
    let output = quote! {#path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>};
    let debug = self.generate_index_debug(path_to_grost, index_name, &output);
    let deref = self.generate_index_deref(path_to_grost, index_name, &output, false);

    quote! {
      #[automatically_derived]
      impl ::core::ops::Index<()> for #index_name<#path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>, #path_to_grost::__private::flavors::Network> {
        type Output = &'static #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>;

        fn index(
          &self,
          indexer: (),
        ) -> &Self::Output {
          match ::core::convert::AsRef::<#indexer_name>::as_ref(self) {
            #(#index_field_reflection),*
          }
        }
      }

      #deref
      #debug
    }
  }

  fn generate_index_deref(
    &self,
    path_to_grost: &syn::Path,
    index_name: &syn::Ident,
    output: impl ToTokens,
    reference: bool,
  ) -> proc_macro2::TokenStream {
    let reference = reference.then(|| quote! {&});
    quote! {
      #[automatically_derived]
      impl ::core::ops::Deref for #index_name<#output, #path_to_grost::__private::flavors::Network> {
        type Target = #output;

        fn deref(
          &self,
        ) -> &Self::Target {
          #reference self[()]
        }
      }
    }
  }

  fn generate_index_debug(
    &self,
    path_to_grost: &syn::Path,
    index_name: &syn::Ident,
    output: impl ToTokens,
  ) -> proc_macro2::TokenStream {
    quote! {
      #[automatically_derived]
      impl ::core::fmt::Debug for #index_name<#output, #path_to_grost::__private::flavors::Network> {
        fn fmt(
          &self,
          f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
          ::core::fmt::Debug::fmt(
            &self[()],
            f,
          )
        }
      }
    }
  }

  fn generate_index_display(
    &self,
    path_to_grost: &syn::Path,
    index_name: &syn::Ident,
    output: impl ToTokens,
  ) -> proc_macro2::TokenStream {
    quote! {
      #[automatically_derived]
      impl ::core::fmt::Display for #index_name<#output, #path_to_grost::__private::flavors::Network> {
        fn fmt(
          &self,
          f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
          ::core::fmt::Display::fmt(
            &self[()],
            f,
          )
        }
      }
    }
  }
}
