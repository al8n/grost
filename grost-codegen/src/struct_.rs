use quote::quote;
use smol_str::SmolStr;
use syn::{Attribute, Visibility, parse_quote};

use crate::SafeIdent;

pub use field::Field;

/// The field of a struct
pub mod field;

pub struct Struct {
  name: SafeIdent,
  selection_name: SafeIdent,
  description: Option<SmolStr>,
  fields: Vec<Field>,
  attrs: Vec<Attribute>,
  visibility: Option<Visibility>,
}

impl Struct {
  pub fn new(name: SafeIdent) -> Self {
    Self {
      selection_name: SafeIdent::new(format!("{}Selection", name.name_str()).as_str()),
      name,
      description: None,
      fields: Vec::new(),
      attrs: Vec::new(),
      visibility: Some(parse_quote!(pub)),
    }
  }

  /// Change the default selection type name
  pub fn with_selection_type_name(mut self, name: SafeIdent) -> Self {
    self.selection_name = name;
    self
  }

  pub fn with_description(mut self, description: impl Into<SmolStr>) -> Self {
    self.description = Some(description.into());
    self
  }

  pub fn with_visibility(mut self, visibility: Visibility) -> Self {
    self.visibility = Some(visibility);
    self
  }

  pub fn with_attrs(mut self, attrs: Vec<Attribute>) -> Self {
    self.attrs = attrs;
    self
  }

  pub fn with_fields(mut self, fields: Vec<Field>) -> Self {
    self.fields = fields;
    self
  }

  pub fn add_field(&mut self, field: Field) -> &mut Self {
    self.fields.push(field);
    self
  }

  pub(crate) fn struct_defination(&self) -> proc_macro2::TokenStream {
    let name = &self.name;
    let description = self.description.as_ref().map(|d| {
      let s = d.as_str();
      quote! {
        #[doc = #s]
      }
    });
    let visibility = self.visibility.as_ref();
    let attrs = &self.attrs;
    let fields = self.fields.iter().map(|f| f.field_defination());

    quote! {
      #[derive(::core::fmt::Debug, ::core::clone::Clone)]
      #description
      #(#attrs)*
      #visibility struct #name {
        #(#fields),*
      }
    }
  }

  pub(crate) fn struct_basic(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name = &self.name;
    let default_fields = self.fields.iter().map(|f| {
      let name = f.name();
      let default = f.default();
      if let Some(default) = default {
        quote! {
          #name: #default
        }
      } else {
        quote! {
          #name: ::core::default::Default::default()
        }
      }
    });

    let accessors = self.fields.iter().map(|f| f.field_accessors());
    let consts = self.fields.iter().map(|f| f.field_consts(path_to_grost));

    quote! {
      impl ::core::default::Default for #name {
        fn default() -> Self {
          Self::new()
        }
      }

      impl #name {
        #(#consts)*

        /// Returns a new default instance of the struct
        pub fn new() -> Self {
          Self {
            #(#default_fields),*
          }
        }

        #(#accessors)*
      }
    }
  }

  pub(crate) fn struct_message(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name = &self.name;
    let selection_name = &self.selection_name;

    quote! {
      impl #path_to_grost::__private::Wirable for #name {}

      impl #path_to_grost::__private::Encode for #name {
        #[inline]
        fn encode(&self, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::EncodeError> {
          ::core::todo!()
        }

        #[inline]
        fn encoded_len(&self) -> ::core::primitive::usize {
          ::core::todo!()
        }
      }

      impl #path_to_grost::__private::PartialEncode for #name {
        type Selection = #selection_name;

        #[inline]
        fn partial_encode(&self, buf: &mut [::core::primitive::u8], selection: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::EncodeError> {
          ::core::todo!()
        }

        #[inline]
        fn partial_encoded_len(&self, selection: &Self::Selection,) -> ::core::primitive::usize {
          ::core::todo!()
        }
      }

      impl<'de> #path_to_grost::__private::Decode<'de> for #name {
        #[inline]
        fn decode<B>(src: &'de [::core::primitive::u8], _: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), #path_to_grost::__private::DecodeError>
        where
          B: #path_to_grost::__private::UnknownRefBuffer<'de>,
        {
          ::core::todo!()
        }
      }
    }
  }
}
