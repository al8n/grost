use quote::ToTokens;

use super::*;

impl Object {
  /// Generates a partial reference object of the object
  pub fn generate_partial_ref_object(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let field_reflection = self.field_reflection_name();
    let name = self.partial_ref_name();
    let vis = self.visibility.as_ref();
    let where_clauses = constraints(path_to_grost, &field_reflection, self.fields());
    let fields = self.fields.iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty();
      let tag = f.tag();
      let wf = wire_format_reflection_ty(&field_reflection, tag);
      let encoded_state = encode_state_ty(path_to_grost, &wf);

      quote! {
        #field_name: ::core::option::Option<<#ty as #encoded_state>::Output>,
      }
    });

    quote! {
      #[allow(clippy::type_complexity, non_camel_case_types)]
      #vis struct #name<'__grost_lifetime__, __GROST_FLAVOR__>
      where
        #(#where_clauses)*
      {
        #(#fields)*
      }
    }
  }

  /// Derives implementations for the partial reference object
  pub fn derive_partial_ref_object(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name = self.partial_ref_name();
    let fields_init = self.fields.iter().map(|f| {
      let field_name = f.name();
      quote! {
        #field_name: ::core::option::Option::None,
      }
    });
    let field_reflection_name = self.field_reflection_name();
    let where_clauses =
      constraints(path_to_grost, &field_reflection_name, self.fields()).collect::<Vec<_>>();
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
        let encoded_state = encode_state_ty(path_to_grost, &wf);

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
      // #[automatically_derived]
      // #[allow(non_camel_case_types)]
      // impl #path_to_grost::__private::Referenceable<#flavor_ty, #path_to_grost::__private::flavors::network::LengthDelimited> for #struct_name {
      //   type Ref<'__grost_lifetime__> = #name<'__grost_lifetime__> where Self: '__grost_lifetime__;
      // }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl<'__grost_lifetime__, __GROST_FLAVOR__> ::core::default::Default for #name<'__grost_lifetime__, __GROST_FLAVOR__>
      where
        #(#where_clauses)*
      {
        fn default() -> Self {
          Self::new()
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl<'__grost_lifetime__, __GROST_FLAVOR__> #name<'__grost_lifetime__, __GROST_FLAVOR__>
      where
        #(#where_clauses)*
      {
        /// Creates an empty partial struct.
        #[inline]
        pub const fn new() -> Self {
          Self {
            #(#fields_init)*
          }
        }

        #(#fields_accessors)*
      }
    }
  }
}

fn wire_format_reflection_ty(field_reflection: impl ToTokens, tag: u32) -> syn::Type {
  parse_quote! {
    #field_reflection<
      ::grost::__private::reflection::WireFormatReflection,
      __GROST_FLAVOR__,
      #tag,
    >
  }
}

fn encode_state_ty(path_to_grost: &syn::Path, wf: &syn::Type) -> syn::Type {
  parse_quote! {
    #path_to_grost::__private::convert::State<
      #path_to_grost::__private::convert::Encoded<
        '__grost_lifetime__,
        __GROST_FLAVOR__,
        <#wf as #path_to_grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
      >
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
    let wf = wire_format_reflection_ty(&field_reflection, f.tag());
    let encoded_state = encode_state_ty(path_to_grost, &wf);
    quote! {
      #wf: #path_to_grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
      #ty: #encoded_state,
      <#ty as #encoded_state>::Output: ::core::marker::Sized,
    }
  })
}
