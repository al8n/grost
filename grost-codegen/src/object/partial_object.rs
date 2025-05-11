use super::*;

impl Object {
  /// Generates a partial object of the object
  pub fn generate_partial_object(&self) -> proc_macro2::TokenStream {
    let name = format_ident!("Partial{}", self.name.name_str());
    let visibility = self.visibility.as_ref();
    let fields = self.fields.iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty().repr().partial_ty();
      quote! {
        #field_name: ::core::option::Option<#ty>,
      }
    });
    let doc = format!(" The partial struct of [`{}`]", self.name.name_str());

    quote! {
      #[doc = #doc]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #visibility struct #name {
        #(#fields)*
      }
    }
  }

  /// Derives implementations for the partial object
  pub fn derive_partial_object(&self, _path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name = self.partial_struct_name();
    let fields = self.fields.iter().map(|f| {
      let field_name = f.name();
      quote! {
        #field_name: ::core::option::Option::None,
      }
    });

    let fields_accessors = self.fields.iter().map(|f| {
      let field_name = f.name();
      let ref_fn = format_ident!("{}_ref", field_name.name_str());
      let ref_mut_fn = format_ident!("{}_mut", field_name.name_str());
      let set_fn = format_ident!("set_{}", field_name.name_str());
      let take_fn = format_ident!("take_{}", field_name.name_str());
      let without_fn = format_ident!("without_{}", field_name.name_str());
      let with_fn = format_ident!("with_{}", field_name.name_str());
      let clear_fn = format_ident!("clear_{}", field_name.name_str());
      let ty = f.ty();
      let constable = ty.copy().then(|| quote! { const });
      let ty = ty.repr().partial_ty();

      quote! {
        #[inline]
        pub const fn #ref_fn(&self) -> ::core::option::Option<&#ty> {
          self.#field_name.as_ref()
        }

        #[inline]
        pub const fn #ref_mut_fn(&mut self) -> ::core::option::Option<&mut #ty> {
          self.#field_name.as_mut()
        }

        #[inline]
        pub const fn #take_fn(&mut self) -> ::core::option::Option<#ty> {
          self.#field_name.take()
        }

        #[inline]
        pub #constable fn #clear_fn(&mut self) -> &mut Self {
          self.#field_name = ::core::option::Option::None;
          self
        }

        #[inline]
        pub #constable fn #set_fn(&mut self, value: #ty) -> &mut Self {
          self.#field_name = ::core::option::Option::Some(value);
          self
        }

        #[inline]
        pub #constable fn #with_fn(mut self, value: #ty) -> Self {
          self.#field_name = ::core::option::Option::Some(value);
          self
        }

        #[inline]
        pub #constable fn #without_fn(mut self) -> Self {
          self.#field_name = ::core::option::Option::None;
          self
        }
      }
    });

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl ::core::default::Default for #name {
        fn default() -> Self {
          Self::new()
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #name {
        /// Creates an empty partial struct.
        pub const fn new() -> Self {
          Self {
            #(#fields)*
          }
        }

        #(#fields_accessors)*
      }
    }
  }
}
