use darling::FromMeta;
use quote::quote;
use syn::{Attribute, parse::Parser};

pub use grost_darling_derive::*;

/// The meta for Grost schema types
pub mod meta;

/// The Mid-level Intermediate Representation (MIR) for Grost schema types.
pub mod mir;

/// Returns a `'__grost_lifetime__` lifetime, which is the lifetime name used
/// in the generated code. This is used to avoid conflicts with other
/// lifetimes in the code.
pub fn grost_lifetime() -> syn::Lifetime {
  syn::parse_quote!('__grost_lifetime__)
}

/// Returns a generic parameter `__GROST_FLAVOR__`, which is used to represent
/// the a flavor generic parameter in the generated code. This is used to avoid
/// conflicts with other generic parameters in the code.
pub fn grost_flavor_generic() -> syn::Ident {
  quote::format_ident!("__GROST_FLAVOR__")
}

/// Returns a generic parameter `__GROST_UNKNOWN_BUFFER__`, which is used to represent
/// the unknown buffer generic parameter in the generated code, which is used to store unknown data.
/// This is used to avoid conflicts with other generic parameters in the code.
pub fn grost_unknown_buffer_generic() -> syn::Ident {
  quote::format_ident!("__GROST_UNKNOWN_BUFFER__")
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

#[doc(hidden)]
pub mod __private {
  pub use darling;
  pub use proc_macro2;
  pub use quote;
  pub use syn;

  pub use super::*;

  pub fn default_grost_path() -> syn::Path {
    syn::parse_quote!(::grost)
  }
}
