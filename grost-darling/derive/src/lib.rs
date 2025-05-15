use std::collections::HashSet;

use darling::FromDeriveInput;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse_macro_input, parse_quote};

mod object;

fn default_path() -> syn::Path {
  syn::parse_str("::grost_darling").unwrap()
}

fn default_grost_path() -> syn::Path {
  parse_quote! { ::grost_darling::__private::default_grost_path }
}

#[proc_macro_derive(Object, attributes(grost))]
pub fn object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input_struct = parse_macro_input!(input as DeriveInput);

  let input = match object::ObjectDeriveInput::from_derive_input(&input_struct) {
    Ok(input) => input,
    Err(e) => return e.write_errors().into(),
  };

  input.to_token_stream().into()
}

#[proc_macro_derive(ObjectField, attributes(grost))]
pub fn field(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input_struct = parse_macro_input!(input as DeriveInput);

  let input = match object::ObjectFieldDeriveInput::from_derive_input(&input_struct) {
    Ok(input) => input,
    Err(e) => return e.write_errors().into(),
  };

  input.to_token_stream().into()
}

#[derive(Debug, Default, Clone)]
struct Attributes(Vec<syn::Attribute>);

impl From<Attributes> for Vec<syn::Attribute> {
  fn from(attrs: Attributes) -> Self {
    attrs.0
  }
}

impl Attributes {
  /// Consumes the `Attributes` and returns the inner vector of attributes
  pub fn into_inner(self) -> Vec<syn::Attribute> {
    self.0
  }
}

impl darling::FromMeta for Attributes {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    use syn::parse::Parser;

    let mut attributes = Vec::new();
    for item in items {
      match item {
        darling::ast::NestedMeta::Lit(lit) => {
          return Err(darling::Error::unexpected_lit_type(lit));
        }
        darling::ast::NestedMeta::Meta(meta) => {
          let attr = syn::Attribute::parse_outer
            .parse2(quote! { #[#meta] })
            .map_err(|e| darling::Error::from(e).with_span(meta))?;
          attributes.extend(attr);
        }
      }
    }

    Ok(Self(attributes))
  }
}

#[derive(Debug, Default, Clone)]
struct DarlingAttributes(HashSet<syn::Ident>);

impl From<DarlingAttributes> for HashSet<syn::Ident> {
  fn from(attrs: DarlingAttributes) -> Self {
    attrs.0
  }
}

impl DarlingAttributes {
  /// Consumes the `DarlingAttributes` and returns the inner vector of attributes
  pub fn into_inner(self) -> HashSet<syn::Ident> {
    self.0
  }
}

impl darling::FromMeta for DarlingAttributes {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let mut attributes = HashSet::new();
    for item in items {
      match item {
        darling::ast::NestedMeta::Lit(lit) => {
          return Err(darling::Error::unexpected_lit_type(lit));
        }
        darling::ast::NestedMeta::Meta(meta) => {
          if let syn::Meta::Path(path) = meta {
            if let Some(ident) = path.get_ident() {
              attributes.insert(ident.clone());
            } else {
              return Err(darling::Error::custom("missing an ident"));
            }
          }
        }
      }
    }

    Ok(Self(attributes))
  }
}
