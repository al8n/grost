use darling::FromDeriveInput;
use heck::ToUpperCamelCase;
use indexmap::IndexMap;
use quote::{ToTokens, format_ident, quote};
use smol_str::SmolStr;
use syn::{
  Attribute, DeriveInput, Ident, LitInt, LitStr, Type, Visibility, meta::ParseNestedMeta,
  parse_quote,
};

use crate::{FlavorGenerator, SafeIdent};

pub use field::Field;

/// The field of a struct
pub mod field;

mod index;
/// The parser for parsing from token stream to object
mod parser;
mod partial_object;
mod partial_ref_object;
mod reflection;
mod selector;

#[derive(Default)]
pub struct ObjectAttributeArgs {
  schema_name: Option<LitStr>,
  path_to_grost: Option<syn::Path>,
  // parse_flavor_generator: Box<dyn Fn() -> Result<Box<dyn FlavorGenerator>, syn::Error>>,
}

impl ObjectAttributeArgs {
  pub fn parse(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
    match () {
      () if meta.path.is_ident("schema_name") => {
        self.schema_name = Some(meta.value()?.parse()?);
        Ok(())
      }
      () if meta.path.is_ident("crate") => {
        let val: LitStr = meta.value()?.parse()?;
        self.path_to_grost = Some(syn::parse_str(&val.value()).map_err(|e| meta.error(e))?);
        Ok(())
      }
      _ => Err(meta.error("Unsupported grost attribute")),
    }
  }
}

pub struct Object {
  name: SafeIdent,
  schema_name: SmolStr,
  selector_name: SafeIdent,
  description: Option<SmolStr>,
  fields: Vec<Field>,
  attrs: Vec<Attribute>,
  visibility: Option<Visibility>,
}

impl PartialEq for Object {
  fn eq(&self, other: &Self) -> bool {
    self.name.name().eq(other.name.name())
  }
}

impl Eq for Object {}

impl core::hash::Hash for Object {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.name.name().hash(state);
  }
}

impl Object {
  pub fn from_attribute_macro_input(
    args: ObjectAttributeArgs,
    input: DeriveInput,
  ) -> Result<(), syn::Error> {
    // let mut obj = Self::new(SafeIdent::new(input.ident.to_string().as_str()), vec![]);
    // if let Some(s) = &args.schema_name {
    //   obj = obj.with_schema_name(s.value());
    // }
    // let data_struct = match &input.data {
    //   syn::Data::Struct(data_struct) => data_struct,
    //   syn::Data::Enum(_) => {
    //     return Err(syn::Error::new_spanned(
    //       input,
    //       "enum is not supported by object attribute macro",
    //     ));
    //   }
    //   syn::Data::Union(_) => {
    //     return Err(syn::Error::new_spanned(
    //       input,
    //       "union is not supported by object attribute macro",
    //     ));
    //   }
    // };
    // let fields = match &data_struct.fields {
    //   syn::Fields::Named(fields) => &fields.named,
    //   syn::Fields::Unnamed(_) => {
    //     return Err(syn::Error::new_spanned(
    //       input,
    //       "tuple structs are not supported by object attribute macro",
    //     ));
    //   }
    //   syn::Fields::Unit => {
    //     return Err(syn::Error::new_spanned(
    //       input,
    //       "unit structs are not supported by object attribute macro",
    //     ));
    //   }
    // };

    // for f in fields {
    //   let mut has_grost = false;
    //   for attr in f.attrs.iter() {
    //     if attr.path().is_ident("grost") {
    //       has_grost = true;
    //       // parse_field_attributes(f.ident.as_ref().unwrap(), attr)?;
    //     }
    //   }

    //   if !has_grost {
    //     return Err(syn::Error::new_spanned(
    //       f,
    //       format!("missing tag for field `{}`", f.ident.as_ref().unwrap()),
    //     ));
    //   }
    // }

    // input.generics.type_params()

    let input = parser::ObjectDeriveInput::from_derive_input(&input)?;

    Ok(())
  }

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
    format_ident!("{}FieldIndex", self.name.name_str())
  }

  pub fn partial_ref_name(&self) -> Ident {
    format_ident!("Partial{}Ref", self.name.name_str())
  }

  pub fn name(&self) -> &SafeIdent {
    &self.name
  }

  pub fn partial_struct_name(&self) -> syn::Ident {
    format_ident!("Partial{}", self.name.name_str())
  }

  pub fn schema_name(&self) -> &str {
    &self.schema_name
  }

  pub fn fields(&self) -> &[Field] {
    &self.fields
  }

  pub fn reflection_name(&self) -> Ident {
    format_ident!("{}Reflection", self.name.name_str())
  }

  pub fn field_reflection_name(&self) -> Ident {
    let struct_name = self.name.name_str();
    format_ident!("{struct_name}FieldReflection")
  }

  /// Generate the object struct
  pub fn generate_object(&self) -> proc_macro2::TokenStream {
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

  /// Derives implementations for the object
  pub fn derive_object(&self) -> proc_macro2::TokenStream {
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
      #[automatically_derived]
      impl ::core::default::Default for #name {
        fn default() -> Self {
          Self::new()
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
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

fn wire_format_reflection_ty(field_reflection: impl ToTokens, tag: u32) -> syn::Type {
  parse_quote! {
    #field_reflection<
      ::grost::__private::reflection::WireFormatReflection,
      __GROST_FLAVOR__,
      #tag,
    >
  }
}
