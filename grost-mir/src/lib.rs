use quote::quote;

pub use grost_darling::*;
pub use mir::*;

/// Utility types for working with the AST.
pub mod ast;

/// The Mid-level Intermediate Representation (MIR) for Grost schema types.
mod mir;

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
