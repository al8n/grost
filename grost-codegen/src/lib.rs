use quote::{ToTokens, quote};

pub use case::*;
pub use enum_::*;
pub use safe_ident::*;

mod case;
mod safe_ident;

/// Enum structs
mod enum_;

pub trait Generator {
  type Error;

  fn generate_enum(&self, enum_: &Enum) -> Result<proc_macro2::TokenStream, Self::Error>;
}

/// The default generator
#[derive(Clone)]
pub struct DefaultGenerator {
  grost_path: syn::Path,
}

impl Default for DefaultGenerator {
  fn default() -> Self {
    Self {
      grost_path: syn::parse_str("::grost").unwrap(),
    }
  }
}

impl DefaultGenerator {
  /// Returns a new `DefaultGenerator`
  pub fn new() -> Self {
    Self::default()
  }

  /// Returns a new `DefaultGenerator` with the given `grost_path`
  pub fn with_grost_path(grost_path: syn::Path) -> Self {
    Self { grost_path }
  }
}

impl core::fmt::Debug for DefaultGenerator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("DefaultGenerator")
      .field("grost_path", &self.grost_path.to_token_stream().to_string())
      .finish()
  }
}

impl Generator for DefaultGenerator {
  type Error = ();

  fn generate_enum(&self, enum_: &Enum) -> Result<proc_macro2::TokenStream, Self::Error> {
    let defination = enum_.enum_defination();
    let as_str = enum_.enum_as_str();
    let from_str = enum_.enum_from_str();
    let is_variant = enum_.enum_is_variant();
    #[cfg(feature = "quickcheck")]
    let quickcheck = enum_.enum_quickcheck(&self.grost_path);
    #[cfg(not(feature = "quickcheck"))]
    let quickcheck = quote! {};
    #[cfg(feature = "arbitrary")]
    let arbitrary = enum_.enum_arbitrary(&self.grost_path);
    #[cfg(not(feature = "arbitrary"))]
    let arbitrary = quote! {};
    let repr_conversion = enum_.enum_repr_conversion(&self.grost_path);
    let codec = enum_.enum_codec(&self.grost_path);

    Ok(quote! {
      #defination
      #as_str
      #from_str
      #repr_conversion
      #codec
      #is_variant
      #quickcheck
      #arbitrary
    })
  }
}

#[cfg(test)]
mod tests;
