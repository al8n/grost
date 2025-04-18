use grost_types::WireType;
use quote::{ToTokens, quote};

pub use case::*;
pub use enum_::*;
pub use safe_ident::*;
pub use struct_::*;

mod case;
mod safe_ident;

/// Enum structs
mod enum_;
/// structs
mod struct_;

/// Types for the generator
pub mod ty;

pub trait Generator {
  type Error;

  fn generate_struct(&self, struct_: &Struct) -> Result<proc_macro2::TokenStream, Self::Error>;
  fn generate_enum(&self, enum_: &Enum) -> Result<proc_macro2::TokenStream, Self::Error>;
}

/// The default generator
#[derive(Clone)]
pub struct DefaultGenerator {
  grost_path: syn::Path,
  derive: bool,
}

impl Default for DefaultGenerator {
  fn default() -> Self {
    Self {
      grost_path: syn::parse_str("::grost").unwrap(),
      derive: false,
    }
  }
}

impl DefaultGenerator {
  /// Returns a new `DefaultGenerator`
  pub fn new() -> Self {
    Self::default()
  }

  /// Returns a new `DefaultGenerator` with the given `grost_path`
  pub fn with_grost_path(mut self, grost_path: syn::Path) -> Self {
    self.grost_path = grost_path;
    self
  }

  /// Sets the generator to adapt derive macros
  pub fn with_derive(mut self) -> Self {
    self.derive = true;
    self
  }
}

impl core::fmt::Debug for DefaultGenerator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("DefaultGenerator")
      .field("grost_path", &self.grost_path.to_token_stream().to_string())
      .field("derive", &self.derive)
      .finish()
  }
}

impl Generator for DefaultGenerator {
  type Error = ();

  fn generate_struct(&self, struct_: &Struct) -> Result<proc_macro2::TokenStream, Self::Error> {
    let defination = (!self.derive).then(|| struct_.struct_defination());
    let basic = struct_.struct_basic(&self.grost_path);
    let selection = struct_.generate_selection(&self.grost_path);

    Ok(quote! {
      #defination
      #basic
      #selection
    })
  }

  fn generate_enum(&self, enum_: &Enum) -> Result<proc_macro2::TokenStream, Self::Error> {
    let defination = (!self.derive).then(|| enum_.enum_defination());
    let name = enum_.name();
    let as_str = enum_.enum_as_str();
    let from_str = enum_.enum_from_str();
    let is_variant = enum_.enum_is_variant();
    let info = enum_.generate_info(&self.grost_path);
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

      impl #name {
        #info
      }

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

trait WireTypeExt {
  fn raw(&self) -> &'static str;
  fn to_tokens(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream;
}

impl WireTypeExt for WireType {
  fn raw(&self) -> &'static str {
    match self {
      Self::Zst => "WireType::Zst",
      Self::Varint => "WireType::Varint",
      Self::LengthDelimited => "WireType::LengthDelimited",
      Self::Byte => "WireType::Byte",
      Self::Fixed16 => "WireType::Fixed16",
      Self::Fixed32 => "WireType::Fixed32",
      Self::Fixed64 => "WireType::Fixed64",
      Self::Fixed128 => "WireType::Fixed128",
    }
  }

  fn to_tokens(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    match self {
      Self::Zst => quote! { #path_to_grost::__private::WireType::Zst },
      Self::Varint => quote! { #path_to_grost::__private::WireType::Varint },
      Self::LengthDelimited => quote! { #path_to_grost::__private::WireType::LengthDelimited },
      Self::Byte => quote! { #path_to_grost::__private::WireType::Byte },
      Self::Fixed16 => quote! { #path_to_grost::__private::WireType::Fixed16 },
      Self::Fixed32 => quote! { #path_to_grost::__private::WireType::Fixed32 },
      Self::Fixed64 => quote! { #path_to_grost::__private::WireType::Fixed64 },
      Self::Fixed128 => quote! { #path_to_grost::__private::WireType::Fixed128 },
    }
  }
}

#[cfg(test)]
mod tests;
