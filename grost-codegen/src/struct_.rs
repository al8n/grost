use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use quote::{ToTokens, format_ident, quote};
use smol_str::SmolStr;
use syn::{Attribute, Visibility, parse_quote};

use crate::{SafeIdent, flavors::FlavorGeneratorExt};

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
  pub fn new(name: SafeIdent, mut fields: Vec<Field>) -> Self {
    fields.sort_by_key(|f| f.tag());
    Self {
      selection_name: SafeIdent::new(format!("{}Selection", name.name_str()).as_str()),
      schema_name: name.original_str().into(),
      name,
      description: None,
      fields,
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

  pub fn generate_reflection<F>(
    &self,
    path_to_grost: &syn::Path,
    flavor: &F,
  ) -> proc_macro2::TokenStream
  where
    F: super::FlavorGenerator + ?Sized,
  {
    let name = self.name.name();
    let name_str = self.name.name_str();
    let schema_name = self.schema_name.as_str();
    let fields = &self.fields;
    let fields_reflection_defination = fields
      .iter()
      .map(|f| f.field_reflections(path_to_grost, flavor));
    let field_reflections = fields.iter().map(|f| {
      let ident = flavor.field_reflection_name(f.name().name_str());
      quote! { Self::#ident }
    });

    let reflection_name = flavor.struct_reflection_name();
    let reflection_doc = format!(
      " The reflection of the struct `{name_str}` for [`{}`]({}) flavor.",
      flavor.name().to_upper_camel_case(),
      flavor.ty().to_token_stream().to_string().replace(" ", "")
    );
    let flavor_ty = flavor.ty();

    quote! {
      impl #name {
        #(#fields_reflection_defination)*

        #[doc = #reflection_doc]
        pub const #reflection_name: #path_to_grost::__private::reflection::StructReflection<#flavor_ty> = #path_to_grost::__private::reflection::StructReflectionBuilder::<#flavor_ty> {
          name: #name_str,
          schema_name: #schema_name,
          fields: &[
            #(#field_reflections),*
          ],
        }.build();
      }
    }
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
    let fields = &self.fields;
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

    let name_str = self.name.name_str();
    let schema_name = self.schema_name.as_str();

    quote! {
      impl ::core::default::Default for #name {
        fn default() -> Self {
          Self::new()
        }
      }

      impl #name {
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
        fn partial_encode(&self, buf: &mut [::core::primitive::u8], selector: &Self::Selector) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::EncodeError> {
          ::core::todo!()
        }

        #[inline]
        fn partial_encoded_len(&self, selector: &Self::Selector,) -> ::core::primitive::usize {
          ::core::todo!()
        }
      }

      impl<'de> #path_to_grost::__private::Decode<'de> for #name {
        #[inline]
        fn decode<B>(src: &'de [::core::primitive::u8], _: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), #path_to_grost::__private::DecodeError>
        where
          B: #path_to_grost::__private::UnknownRefBytesBuffer<'de>,
        {
          ::core::todo!()
        }
      }
    }
  }
}
