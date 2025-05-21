use std::path::PathBuf;

use darling::FromMeta;
use quote::quote;
use syn::{Attribute, parse::Parser};

/// The meta for the object
pub mod object;

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

/// Configures the output of the generated code, this is useful when you want to
/// debug the generated code.
#[derive(Debug, Default, Clone)]
pub struct Output {
  path: PathBuf,
  format: bool,
}

impl Output {
  /// Returns the path to the output file
  pub const fn path(&self) -> &PathBuf {
    &self.path
  }

  /// Returns `true` if the output should be formatted
  pub const fn format(&self) -> bool {
    self.format
  }
}

impl FromMeta for Output {
  fn from_string(value: &str) -> darling::Result<Self> {
    Ok(Self {
      path: PathBuf::from(value),
      format: false,
    })
  }

  // fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
  //   match item {
  //     syn::Meta::Path(path) => todo!(),
  //     syn::Meta::List(meta_list) => todo!(),
  //     syn::Meta::NameValue(meta_name_value) => todo!(),
  //   }
  // }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let mut path = None;
    let mut format = false;
    println!("items: {:?}", items);
    for item in items {
      match item {
        darling::ast::NestedMeta::Lit(lit) => {
          return Err(darling::Error::unexpected_lit_type(lit));
        }
        darling::ast::NestedMeta::Meta(meta) => {
          if meta.path().is_ident("path") {
            path = Some(<PathBuf as FromMeta>::from_meta(meta)?);
          } else if meta.path().is_ident("format") {
            format = <bool as FromMeta>::from_meta(meta)?;
          }
        }
      }
    }

    Ok(Self {
      path: path.ok_or_else(|| darling::Error::missing_field("path"))?,
      format,
    })
  }
}

/// Returns a `'__grost_lifetime__` lifetime, which is the lifetime name used
/// in the generated code. This is used to avoid conflicts with other
/// lifetimes in the code.
pub fn grost_lifetime() -> syn::Lifetime {
  syn::parse_quote!('__grost_lifetime__)
}

/// Returns a generic parameter `__GROST_FLAVOR__`, which is used to represent
/// the a flavor generic parameter in the generated code. This is used to avoid
/// conflicts with other generic parameters in the code.
pub fn grost_flavor_param() -> syn::TypeParam {
  syn::parse_quote!(__GROST_FLAVOR__: ?::core::marker::Sized)
}

/// Returns a generic parameter `__GROST_UNKNOWN_BUFFER__`, which is used to represent
/// the unknown buffer generic parameter in the generated code, which is used to store unknown data.
/// This is used to avoid conflicts with other generic parameters in the code.
pub fn grost_unknown_buffer_param() -> syn::TypeParam {
  quote::format_ident!("__GROST_UNKNOWN_BUFFER__").into()
}
