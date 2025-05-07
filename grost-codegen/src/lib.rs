pub use case::*;
pub use enum_::*;
pub use flavors::*;
use indexmap::IndexMap;
pub use safe_ident::*;
pub use struct_::*;

use quote::{ToTokens, quote};
use smol_str::SmolStr;

mod case;
/// Enum structs
mod enum_;
mod flavors;
mod safe_ident;
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
pub struct DefaultGenerator {
  grost_path: syn::Path,
  derive: bool,
  flavors: IndexMap<SmolStr, Box<dyn FlavorGenerator + 'static>>,
}

impl Default for DefaultGenerator {
  fn default() -> Self {
    let grost_path = syn::parse_str("::grost").unwrap();
    let network = Box::new(network::Network::new(&grost_path)) as Box<dyn FlavorGenerator>;
    let k = core::any::type_name::<network::Network>();
    Self {
      flavors: {
        let mut flavors = IndexMap::new();
        flavors.insert(SmolStr::new(k), network);
        flavors
      },
      grost_path,
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
    for (flavor_name, flavor) in self.flavors.iter_mut() {
      if flavor_name.eq(core::any::type_name::<network::Network>()) {
        let path = &self.grost_path;
        flavor.set_ty(syn::parse_quote!(#path::__private::flavors::Network));
      }
    }
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
    let defination = struct_.struct_defination();

    let selector_defination = struct_.generate_selector_defination(&self.grost_path);
    let selector_iter_defination = struct_.generate_selector_iter_defination();
    let selector_iter_impl = struct_.generate_selector_iter_impl();
    let selector_impl = struct_.generate_selector_impl(&self.grost_path);

    let indexer_defination = struct_.generate_indexer_defination();
    let indexer_impl = struct_.generate_indexer_impl(&self.grost_path);

    let struct_impl = struct_.struct_impl();
    let codec = self
      .flavors
      .iter()
      .map(|(_, f)| f.generate_struct_codec(&self.grost_path, struct_));

    let selection_codecs = self
      .flavors
      .iter()
      .map(|(_, f)| f.generate_selection_codec(&self.grost_path, struct_));

    let reflection = struct_.generate_reflection(&self.grost_path);
    let reflection_impls = self
      .flavors
      .iter()
      .map(|(_, f)| f.generate_struct_reflection_impl(&self.grost_path, struct_));

    Ok(quote! {
      #defination

      #indexer_defination

      #selector_defination
      #selector_iter_defination

      const _: () = {
        #struct_impl

        #selector_iter_impl
        #selector_impl

        #indexer_impl

        #reflection

        #(const _: () = {
          #reflection_impls
        };)*

        #(
          const _: () = {
            #codec
          };
        )*
      };

      #(#selection_codecs)*
    })
  }

  fn generate_enum(&self, enum_: &Enum) -> Result<proc_macro2::TokenStream, Self::Error> {
    let defination = (!self.derive).then(|| enum_.enum_defination());
    let name = enum_.name();
    let as_str = enum_.enum_as_str();
    let from_str = enum_.enum_from_str();
    let is_variant = enum_.enum_is_variant();
    let reflection = enum_.generate_reflection(&self.grost_path);
    #[cfg(feature = "quickcheck")]
    let quickcheck = self
      .flavors
      .iter()
      .map(|(_, f)| enum_.enum_quickcheck(&self.grost_path, f));
    #[cfg(not(feature = "quickcheck"))]
    let quickcheck = core::iter::once(quote! {}).into_iter();
    #[cfg(feature = "arbitrary")]
    let arbitrary = enum_.enum_arbitrary(&self.grost_path);
    #[cfg(not(feature = "arbitrary"))]
    let arbitrary = quote! {};
    let repr_conversion = enum_.enum_repr_conversion(&self.grost_path);
    let codecs = self
      .flavors
      .iter()
      .map(|(_, f)| f.generate_enum_codec(&self.grost_path, enum_));

    Ok(quote! {
      #defination

      impl #name {
        #reflection
      }

      #as_str
      #from_str
      #is_variant
      #repr_conversion

      #(#codecs)*
      #(#quickcheck)*
      #arbitrary
    })
  }
}

// trait WireTypeExt {
//   fn raw(&self) -> &'static str;
//   fn to_tokens(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream;
// }

// impl WireTypeExt for grost_proto::flavors::network::WireType {
//   fn raw(&self) -> &'static str {
//     match self {
//       Self::Zst => "WireType::Zst",
//       Self::Varint => "WireType::Varint",
//       Self::LengthDelimited => "WireType::LengthDelimited",
//       Self::Byte => "WireType::Byte",
//       Self::Fixed16 => "WireType::Fixed16",
//       Self::Fixed32 => "WireType::Fixed32",
//       Self::Fixed64 => "WireType::Fixed64",
//       Self::Fixed128 => "WireType::Fixed128",
//     }
//   }

//   fn to_tokens(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
//     match self {
//       Self::Zst => quote! { #path_to_grost::__private::WireType::Zst },
//       Self::Varint => quote! { #path_to_grost::__private::WireType::Varint },
//       Self::LengthDelimited => quote! { #path_to_grost::__private::WireType::LengthDelimited },
//       Self::Byte => quote! { #path_to_grost::__private::WireType::Byte },
//       Self::Fixed16 => quote! { #path_to_grost::__private::WireType::Fixed16 },
//       Self::Fixed32 => quote! { #path_to_grost::__private::WireType::Fixed32 },
//       Self::Fixed64 => quote! { #path_to_grost::__private::WireType::Fixed64 },
//       Self::Fixed128 => quote! { #path_to_grost::__private::WireType::Fixed128 },
//     }
//   }
// }

#[cfg(test)]
mod tests;
