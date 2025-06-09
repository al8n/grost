pub use grost_darling::*;

pub(crate) mod generic;

/// The flavor of the `grost` schema.
pub mod flavor;

/// The object of the `grost` schema.
pub mod object;

/// Utilities types
pub mod utils;

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
