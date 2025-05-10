pub use case::*;
pub use enum_::*;
pub use derive::*;
use indexmap::{IndexMap, IndexSet};
pub use safe_ident::*;
pub use struct_::*;

use quote::{ToTokens, quote};
use smol_str::SmolStr;

mod case;
/// Enum structs
mod enum_;
mod derive;
mod safe_ident;
/// structs
mod struct_;

/// Types for the generator
pub mod ty;

pub struct SchemaGeneratorBuilder {
  grost_path: syn::Path,
  structs: IndexSet<Struct>,
  enums: IndexSet<Enum>,
}

impl Default for SchemaGeneratorBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl SchemaGeneratorBuilder {
  /// Returns a new `SchemaGeneratorBuilder`
  pub fn new() -> Self {
    Self {
      grost_path: syn::parse_str("::grost").unwrap(),
      structs: IndexSet::new(),
      enums: IndexSet::new(),
    }
  }

  pub fn build(self) -> SchemaGenerator {
    SchemaGenerator {
      grost_path: self.grost_path,
      structs: self.structs,
      enums: self.enums,
    }
  }

  /// Returns the path to the grost crate
  #[inline]
  pub const fn grost_path(&self) -> &syn::Path {
    &self.grost_path
  }

  /// Returns a new `SchemaGenerator` with the given `grost_path`
  pub fn with_grost_path(mut self, grost_path: syn::Path) -> Self {
    self.grost_path = grost_path;
    self
  }

  /// Sets the path to the grost crate
  pub fn set_grost_path(&mut self, grost_path: syn::Path) -> &mut Self {
    self.grost_path = grost_path;
    self
  }

  /// Returns the objects in the schema builder
  pub fn objects_mut(&mut self) -> &mut IndexSet<Struct> {
    &mut self.structs
  }

  /// Returns the enums in the schema builder
  pub fn enums_mut(&mut self) -> &mut IndexSet<Enum> {
    &mut self.enums
  }

  /// Returns the objects in the schema builder
  pub fn objects(&mut self) -> &IndexSet<Struct> {
    &mut self.structs
  }

  /// Returns the enums in the schema builder
  pub fn enums(&mut self) -> &IndexSet<Enum> {
    &mut self.enums
  }
}

/// The schema generator is used to generate the objects, enums, unions and
/// interfaces in a grost schema to the corresponding Rust definitions.
pub struct SchemaGenerator {
  grost_path: syn::Path,
  structs: IndexSet<Struct>,
  enums: IndexSet<Enum>,
}

impl SchemaGenerator {
  /// Returns the path to the grost crate
  #[inline]
  pub const fn grost_path(&self) -> &syn::Path {
    &self.grost_path
  }

  /// Returns a new `SchemaGenerator` with the given `grost_path`
  #[inline]
  pub const fn structs(&self) -> &IndexSet<Struct> {
    &self.structs
  }

  /// Returns a new `SchemaGenerator` with the given `grost_path`
  #[inline]
  pub const fn enums(&self) -> &IndexSet<Enum> {
    &self.enums
  }
}

impl core::fmt::Debug for SchemaGenerator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SchemaGenerator")
      .field("grost_path", &self.grost_path.to_token_stream().to_string())
      .finish()
  }
}

impl SchemaGenerator {
  /// Derives the implementations for the schema types
  pub fn derive<G>(&self, derive: &G) -> Result<proc_macro2::TokenStream, G::Error>
  where
    G: DeriveGenerator,
  {
    let structs = self.structs.iter().map(|s| {
      derive.derive_object(&self.grost_path, s)
    }).collect::<Result<Vec<_>, G::Error>>()?;

    let enums = self.enums.iter().map(|e| {
      #[cfg(feature = "quickcheck")]
      let quickcheck = e.enum_quickcheck(&self.grost_path, derive);
      #[cfg(not(feature = "quickcheck"))]
      let quickcheck = core::iter::once(quote! {}).into_iter();

      let impls = self.generate_enum_impl(e);

      derive.derive_enum(&self.grost_path, e)
        .map(|code| quote! {
          const _: () = {
            #impls

            const _: () = {
              #code
            };
          };

          #quickcheck
        })
    }).collect::<Result<Vec<_>, G::Error>>()?;

    Ok(quote! {
      #(#structs)*
      #(#enums)*
    })
  }

  /// Generates the definations for the schema types
  pub fn generate(&self) -> proc_macro2::TokenStream {
    let enums = self.enums.iter().map(|e| {
      e.enum_defination()
    });

    quote! {
      #(#enums)*
    }
  }

  // pub fn generate_struct(&self, struct_: &Struct) -> Result<proc_macro2::TokenStream, Box<dyn core::error::Error + Send + Sync + 'static>> {
  //   let defination = struct_.struct_defination();
  //   let partial_defination = struct_.partial_struct_defination();
  //   let partial_impl = struct_.partial_struct_impl(&self.grost_path);

  //   let indexer_defination = struct_.generate_indexer_defination();
  //   let indexer_impl = struct_.generate_indexer_impl(&self.grost_path);

  //   let struct_impl = struct_.struct_impl();
  //   let codec = self.flavors.iter().map(|(_, f)| {
  //     let codec = f.generate_struct_codec(&self.grost_path, struct_);
  //     let partial_ref_defination = struct_.partial_ref_struct_defination(&self.grost_path, f);
  //     quote! {
  //       #partial_ref_defination

  //       #codec
  //     }
  //   });

  //   let selector_definations = self.flavors.iter().map(|(_, f)| {
  //     struct_.generate_selector_defination(&self.grost_path, f)
  //   });
  //   let selector_iter_defination = struct_.generate_selector_iter_defination();
  //   let selector_iter_impl = struct_.generate_selector_iter_impl();
  //   let selector_impl = struct_.generate_selector_impl(&self.grost_path);

  //   let selector_codecs = self
  //     .flavors
  //     .iter()
  //     .map(|(_, f)| f.generate_selection_codec(&self.grost_path, struct_));

  //   let reflection = struct_.generate_reflection(&self.grost_path);
  //   let reflection_impls = self
  //     .flavors
  //     .iter()
  //     .map(|(_, f)| f.generate_struct_reflection_impl(&self.grost_path, struct_));

  //   Ok(quote! {
  //     #defination

  //     #partial_defination

  //     #indexer_defination

  //     // #selector_iter_defination

  //     const _: () = {
  //       #struct_impl

  //       #partial_impl

  //       #selector_iter_impl
  //       #selector_impl

  //       #indexer_impl

  //       #reflection

  //       #(const _: () = {
  //         #reflection_impls
  //       };)*

  //       #(
  //         const _: () = {
  //           #codec
  //         };
  //       )*
  //     };

  //     #(#selector_codecs)*
  //   })
  // }

  fn generate_enum_impl(&self, enum_: &Enum) -> proc_macro2::TokenStream {
    let name = enum_.name();
    let as_str = enum_.enum_as_str();
    let from_str = enum_.enum_from_str();
    let is_variant = enum_.enum_is_variant();
    let reflection = enum_.generate_reflection(&self.grost_path);
    let repr_conversion = enum_.enum_repr_conversion(&self.grost_path);
    #[cfg(feature = "arbitrary")]
    let arbitrary = enum_.enum_arbitrary(&self.grost_path);
    #[cfg(not(feature = "arbitrary"))]
    let arbitrary = quote! {};

    quote! {
      // impl #name {
      //   #reflection
      // }

      #reflection

      #as_str
      #from_str
      #is_variant
      #repr_conversion

      #arbitrary
    }
  }
}

#[cfg(test)]
mod tests;
