use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use quote::{ToTokens, format_ident, quote};
use smol_str::SmolStr;
use syn::{Attribute, Ident, Visibility, parse_quote};

use crate::{SafeIdent, flavors::FlavorGeneratorExt};

pub use field::Field;

/// The field of a struct
pub mod field;

mod index;
mod selection;
mod reflection;

pub struct Struct {
  name: SafeIdent,
  schema_name: SmolStr,
  selector_name: SafeIdent,
  description: Option<SmolStr>,
  fields: Vec<Field>,
  attrs: Vec<Attribute>,
  visibility: Option<Visibility>,
}

impl Struct {
  pub fn new(name: SafeIdent, mut fields: Vec<Field>) -> Self {
    fields.sort_by_key(|f| f.tag());
    Self {
      selector_name: SafeIdent::new(format!("{}Selector", name.name_str()).as_str()),
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
  pub fn with_selector_name(mut self, name: SafeIdent) -> Self {
    self.selector_name = name;
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

  pub fn selector_name(&self) -> &SafeIdent {
    &self.selector_name
  }

  pub fn selector_iter_name(&self) -> Ident {
    format_ident!("{}Iter", self.selector_name.name_str())
  }

  pub fn indexer_name(&self) -> Ident {
    format_ident!("{}FieldIndexer", self.name.name_str())
  }

  pub fn index_name(&self) -> Ident {
    format_ident!("{}FieldIndex", self.name.name_str())
  }

  pub fn name(&self) -> &SafeIdent {
    &self.name
  }

  pub fn schema_name(&self) -> &str {
    &self.schema_name
  }

  pub fn fields(&self) -> &[Field] {
    &self.fields
  }

  pub fn relfection_mod_name(&self) -> Ident {
    format_ident!("{}_reflection", self.name.name_str().to_snake_case())
  }

  pub fn relfection_name(&self) -> Ident {
    format_ident!("{}Reflection", self.name.name_str())
  }

  pub fn field_reflection_name(&self, field_name: &str) -> Ident {
    let struct_name = self.name.name_str();
    format_ident!("{struct_name}{}FieldReflection", field_name.to_upper_camel_case())
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
}
