use heck::ToShoutySnakeCase;
use quote::{format_ident, quote};
use smol_str::SmolStr;
use syn::{Attribute, Visibility, parse_quote};

use crate::SafeIdent;

pub use field::Field;

/// The field of a struct
pub mod field;

mod selection;

pub struct Struct {
  name: SafeIdent,
  schema_name: SmolStr,
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
      schema_name: name.original_str().into(),
      name,
      description: None,
      fields: Vec::new(),
      attrs: Vec::new(),
      visibility: Some(parse_quote!(pub)),
    }
  }

  pub fn with_schema_name(mut self, name: impl Into<SmolStr>) -> Self {
    self.schema_name = name.into();
    self
  }

  /// Change the default selection type name
  pub fn with_selection_name(mut self, name: SafeIdent) -> Self {
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

  pub fn selection_name(&self) -> &SafeIdent {
    &self.selection_name
  }

  pub fn name(&self) -> &SafeIdent {
    &self.name
  }

  pub fn schema_name(&self) -> &str {
    &self.schema_name
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
    let mut fields = self.fields.clone();
    fields.sort_by_key(|f| f.tag());

    let default_fields = fields.iter().map(|f| {
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

    let accessors = fields.iter().map(|f| f.field_accessors());
    let consts = fields.iter().map(|f| f.field_consts(path_to_grost));
    let field_reflections = fields.iter().map(|f| {
      let ident = format_ident!("{}_REFLECTION", f.name().name_str().to_shouty_snake_case());
      quote! { Self::#ident }
    });

    let name_str = self.name.name_str();
    let schema_name = self.schema_name.as_str();

    quote! {
      impl ::core::default::Default for #name {
        fn default() -> Self {
          Self::new()
        }
      }

      impl #name {
        #(#consts)*

        /// The reflection of the struct
        pub const REFLECTION: #path_to_grost::__private::StructReflection = #path_to_grost::__private::StructReflectionBuilder {
          name: #name_str,
          schema_name: #schema_name,
          fields: &[
            #(#field_reflections),*
          ],
        }.build();

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
