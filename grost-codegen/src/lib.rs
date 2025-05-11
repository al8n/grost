pub use case::*;
pub use derive::*;
pub use enum_::*;
pub use object::*;
pub use safe_ident::*;

use indexmap::IndexSet;
use quote::{ToTokens, quote};

mod case;
mod derive;
/// Enum structs
mod enum_;
/// structs
mod object;
mod safe_ident;

/// Types for the generator
pub mod ty;

pub struct SchemaGeneratorBuilder {
  grost_path: syn::Path,
  structs: IndexSet<Object>,
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
  pub fn objects_mut(&mut self) -> &mut IndexSet<Object> {
    &mut self.structs
  }

  /// Returns the enums in the schema builder
  pub fn enums_mut(&mut self) -> &mut IndexSet<Enum> {
    &mut self.enums
  }

  /// Returns the objects in the schema builder
  pub fn objects(&mut self) -> &IndexSet<Object> {
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
  structs: IndexSet<Object>,
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
  pub const fn structs(&self) -> &IndexSet<Object> {
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
    G: FlavorGenerator,
  {
    let structs = self
      .structs
      .iter()
      .map(|s| {
        let flavor_impl = derive.derive_object(&self.grost_path, s)?;

        let object_impl = s.derive_object();
        let partial_object_impl = s.derive_partial_object(&self.grost_path);
        let partial_ref_object_impl = s.derive_partial_ref_object(&self.grost_path);

        let indexer_impl = s.derive_indexer(&self.grost_path);
        let selector_impl = s.derive_selector(&self.grost_path);
        let selector_iter_impl = s.derive_selector_iter(&self.grost_path);
        let reflection_impl = s.derive_reflection(&self.grost_path);

        Ok(quote! {
          const _: () = {
            #object_impl
          };

          const _: () = {
            #partial_object_impl
          };

          const _: () = {
            #partial_ref_object_impl
          };

          const _: () = {
            #reflection_impl
          };

          const _: () = {
            #indexer_impl
          };

          const _: () = {
            #selector_impl
          };
          
          const _: () = {
            #selector_iter_impl            
          };

          const _: () = {
            #flavor_impl
          };
        })
      })
      .collect::<Result<Vec<_>, G::Error>>()?;

    let enums = self
      .enums
      .iter()
      .map(|e| {
        #[cfg(feature = "quickcheck")]
        let quickcheck = e.enum_quickcheck(&self.grost_path, derive);
        #[cfg(not(feature = "quickcheck"))]
        let quickcheck = core::iter::once(quote! {}).into_iter();

        let impls = self.generate_enum_impl(e);

        derive.derive_enum(&self.grost_path, e).map(|code| {
          quote! {
            const _: () = {
              #impls
            };

            const _: () = {
              #code
            };

            #quickcheck
          }
        })
      })
      .collect::<Result<Vec<_>, G::Error>>()?;

    Ok(quote! {
      #(#structs)*
      #(#enums)*
    })
  }

  /// Generates the definations for the schema types
  pub fn generate(&self) -> proc_macro2::TokenStream {
    let enums = self.enums.iter().map(|e| e.enum_definations());
    let objects = self.structs.iter().map(|s| {
      let defination = s.generate_object();
      let partial_defination = s.generate_partial_object();
      let partial_ref_generate_object = s.generate_partial_ref_object(&self.grost_path);
      let indexer = s.generate_indexer();
      let reflection_defination = s.generate_reflection();
      let selector = s.generate_selector(&self.grost_path);
      let selector_iter = s.generate_selector_iter(&self.grost_path);

      quote! {
        #defination

        #partial_defination

        #partial_ref_generate_object

        #indexer

        #reflection_defination

        #selector

        #selector_iter
      }
    });

    quote! {
      #(#enums)*

      #(#objects)*
    }
  }

  // pub fn generate_struct(&self, struct_: &Object) -> Result<proc_macro2::TokenStream, Box<dyn core::error::Error + Send + Sync + 'static>> {
  //   let codec = self.flavors.iter().map(|(_, f)| {
  //     let codec = f.generate_struct_codec(&self.grost_path, struct_);
  //     let partial_ref_defination = struct_.partial_ref_generate_object(&self.grost_path, f);
  //     quote! {
  //       #partial_ref_defination
  //       #codec
  //     }
  //   });
  //   let generate_selectors = self.flavors.iter().map(|(_, f)| {
  //     struct_.generate_generate_selector(&self.grost_path, f)
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
      #reflection

      #as_str
      #from_str
      #is_variant
      #repr_conversion

      #arbitrary
    }
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
pub fn grost_flavor_generic() -> syn::Ident {
  quote::format_ident!("__GROST_FLAVOR__")
}

#[cfg(test)]
mod tests;
