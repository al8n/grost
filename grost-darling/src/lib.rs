use darling::FromMeta;
use syn::{Attribute, Ident, parse::Parser};
use quote::quote;

/// The meta for the object
pub mod object;

/// The meta for the schema
#[derive(Default, Debug, Clone, FromMeta)]
pub struct SchemaMeta {
  #[darling(default)]
  name: Option<String>,
  #[darling(default)]
  description: Option<String>,
}

impl SchemaMeta {
  /// Returns the name of the schema
  pub const fn name(&self) -> Option<&str> {
    match self.name.as_ref() {
      Some(name) => Some(name.as_str()),
      None => None,
    }
  }

  /// Returns the description of the schema
  pub const fn description(&self) -> Option<&str> {
    match self.description.as_ref() {
      Some(description) => Some(description.as_str()),
      None => None,
    }
  }
}

#[derive(Debug, Default, Clone)]
struct Attributes(Vec<Attribute>);

impl From<Attributes> for Vec<Attribute> {
  fn from(attrs: Attributes) -> Self {
    attrs.0
  }
}

impl Attributes {
  /// Consumes the `Attributes` and returns the inner vector of attributes
  pub fn into_inner(self) -> Vec<Attribute> {
    self.0
  }
}

impl FromMeta for Attributes {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let mut attributes = Vec::new();
    for item in items {
      match item {
        darling::ast::NestedMeta::Lit(lit) => {
          return Err(darling::Error::unexpected_lit_type(lit));
        }
        darling::ast::NestedMeta::Meta(meta) => {
          let attr = Attribute::parse_outer
            .parse2(quote! { #[#meta] })
            .map_err(|e| darling::Error::from(e).with_span(meta))?;
          attributes.extend(attr);
        }
      }
    }

    Ok(Self(attributes))
  }
}

fn default_grost_path() -> syn::Path {
  syn::parse_quote!(::grost)
}

#[doc(hidden)]
pub mod __private {
  pub use darling;
  pub use syn;
  pub use quote;
  pub use proc_macro2;

  pub use super::*;
}