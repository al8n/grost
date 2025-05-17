use quote::ToTokens;

use crate::{grost_flavor_param, grost_lifetime, grost_unknown_buffer_param};

use super::*;

impl Object {
  /// Generates a partial reference object of the object
  pub fn generate_partial_decoded_object(
    &self,
    path_to_grost: &syn::Path,
  ) -> proc_macro2::TokenStream {
    let field_reflection = self.field_reflection_name();
    let name = self.partial_decoded_name();
    let vis = self.visibility.as_ref();
    let lg = grost_lifetime();
    let fg = grost_flavor_param();
    let ubg = grost_unknown_buffer_param();
    let where_clauses = constraints(path_to_grost, &field_reflection, self.fields(), &fg, &lg);
    let fields = self.fields.iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty();
      let tag = f.tag();
      let wf = wire_format_reflection_ty(&field_reflection, tag);
      let encoded_state = encode_state_ty(path_to_grost, &wf, &fg, &lg);

      quote! {
        #field_name: ::core::option::Option<<#ty as #encoded_state>::Output>,
      }
    });

    quote! {
      #[allow(clippy::type_complexity, non_camel_case_types)]
      #vis struct #name<'__grost_lifetime__, #fg, #ubg = ()>
      where
        #(#where_clauses)*
      {
        #(#fields)*
        __grost_unknown_buffer__: ::core::option::Option<#ubg>,
      }
    }
  }

  /// Derives implementations for the partial reference object
  pub fn derive_partial_decoded_object(
    &self,
    path_to_grost: &syn::Path,
  ) -> proc_macro2::TokenStream {
    let name = self.partial_decoded_name();
    let fields_init = self.fields.iter().map(|f| {
      let field_name = f.name();
      quote! {
        #field_name: ::core::option::Option::None,
      }
    });
    let field_reflection_name = self.field_reflection_name();
    let ubg = grost_unknown_buffer_param();
    let fg = grost_flavor_param();
    let lg = grost_lifetime();
    let where_clauses = constraints(
      path_to_grost,
      &field_reflection_name,
      self.fields(),
      &fg,
      &lg,
    )
    .collect::<Vec<_>>();
    let fields_accessors = self.fields.iter()
      .map(|f| {
        let field_name = f.name();
        let ref_fn = format_ident!("{}_ref", field_name.name_str());
        let ref_mut_fn = format_ident!("{}_mut", field_name.name_str());
        let set_fn = format_ident!("set_{}", field_name.name_str());
        let take_fn = format_ident!("take_{}", field_name.name_str());
        let without_fn = format_ident!("without_{}", field_name.name_str());
        let with_fn = format_ident!("with_{}", field_name.name_str());
        let clear_fn = format_ident!("clear_{}", field_name.name_str());
        let ty = f.ty();
        let tag = f.tag();
        let wf = wire_format_reflection_ty(&field_reflection_name, tag);
        let encoded_state = encode_state_ty(path_to_grost, &wf, &fg, &lg);

        quote! {
          #[inline]
          pub const fn #ref_fn(&self) -> ::core::option::Option<&<#ty as #encoded_state>::Output> {
            self.#field_name.as_ref()
          }

          #[inline]
          pub const fn #ref_mut_fn(&mut self) -> ::core::option::Option<&mut <#ty as #encoded_state>::Output> {
            self.#field_name.as_mut()
          }

          #[inline]
          pub const fn #take_fn(&mut self) -> ::core::option::Option<<#ty as #encoded_state>::Output> {
            self.#field_name.take()
          }

          #[inline]
          pub fn #clear_fn(&mut self) -> &mut Self {
            self.#field_name = ::core::option::Option::None;
            self
          }

          #[inline]
          pub fn #set_fn(&mut self, value: <#ty as #encoded_state>::Output) -> &mut Self {
            self.#field_name = ::core::option::Option::Some(value);
            self
          }

          #[inline]
          pub fn #with_fn(mut self, value: <#ty as #encoded_state>::Output) -> Self {
            self.#field_name = ::core::option::Option::Some(value);
            self
          }

          #[inline]
          pub fn #without_fn(mut self) -> Self {
            self.#field_name = ::core::option::Option::None;
            self
          }
        }
      });

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl<#lg, #fg, #ubg> ::core::default::Default for #name<#lg, #fg, #ubg>
      where
        #(#where_clauses)*
      {
        fn default() -> Self {
          Self::new()
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl<#lg, #fg, #ubg,> #name<#lg, #fg, #ubg>
      where
        #(#where_clauses)*
      {
        /// Creates an empty partial struct.
        #[inline]
        pub const fn new() -> Self {
          Self {
            #(#fields_init)*
            __grost_unknown_buffer__: ::core::option::Option::None,
          }
        }

        #(#fields_accessors)*
      }
    }
  }
}

fn encode_state_ty(
  path_to_grost: &syn::Path,
  wf: &syn::Type,
  flavor_generic: &syn::Ident,
  lifetime: &syn::Lifetime,
) -> syn::Type {
  parse_quote! {
    #path_to_grost::__private::convert::State<
      #path_to_grost::__private::convert::Decoded<
        #lifetime,
        #flavor_generic,
        <#wf as #path_to_grost::__private::reflection::Reflectable<#flavor_generic>>::Reflection,
      >
    >
  }
}

fn constraints(
  path_to_grost: &syn::Path,
  field_reflection: impl ToTokens,
  fields: &[Field],
  flavor_generic: &syn::Ident,
  lifetime: &syn::Lifetime,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
  fields.iter().map(move |f| {
    let ty = f.ty();
    let wf = wire_format_reflection_ty(&field_reflection, f.tag());
    let encoded_state = encode_state_ty(path_to_grost, &wf, flavor_generic, lifetime);
    quote! {
      #wf: #path_to_grost::__private::reflection::Reflectable<#flavor_generic>,
      #ty: #encoded_state,
      <#ty as #encoded_state>::Output: ::core::marker::Sized,
    }
  })
}
