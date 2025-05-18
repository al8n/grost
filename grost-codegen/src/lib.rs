use std::collections::HashMap;

pub use case::*;
pub use flavors::*;
// pub use enum_::*;
pub use object::*;
pub use safe_ident::*;

use indexmap::{IndexMap, IndexSet};
use quote::{ToTokens, quote};

mod case;
mod flavors;
// /// Enum structs
// mod enum_;
mod object;
mod safe_ident;

/// Types for the generator
pub mod ty;

pub struct SchemaGeneratorBuilder {
  structs: IndexSet<Object>,
  // enums: IndexSet<Enum>,
  flavors: IndexMap<&'static str, Box<dyn FlavorGenerator + 'static>>,
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
      flavors: IndexMap::new(),
      structs: IndexSet::new(),
      // enums: IndexSet::new(),
    }
  }

  pub fn build(self) -> SchemaGenerator {
    SchemaGenerator {
      structs: self.structs,
      flavors: self.flavors,
      // enums: self.enums,
    }
  }

  /// Adds a flavor derive generator to the schema builder
  pub fn add_flavor<F>(&mut self, name: &'static str, flavor: F) -> Result<&mut Self, F>
  where
    F: FlavorGenerator + 'static,
  {
    if self.flavors.contains_key(flavor.name()) {
      return Err(flavor);
    }

    self.flavors.insert(name, Box::new(flavor));
    Ok(self)
  }

  /// Returns the objects in the schema builder
  pub fn objects_mut(&mut self) -> &mut IndexSet<Object> {
    &mut self.structs
  }

  // /// Returns the enums in the schema builder
  // pub fn enums_mut(&mut self) -> &mut IndexSet<Enum> {
  //   &mut self.enums
  // }

  // /// Returns the enums in the schema builder
  // pub fn enums(&mut self) -> &IndexSet<Enum> {
  //   &mut self.enums
  // }
}

/// The schema generator is used to generate the objects, enums, unions and
/// interfaces in a grost schema to the corresponding Rust definitions.
pub struct SchemaGenerator {
  structs: IndexSet<Object>,
  // enums: IndexSet<Enum>,
  flavors: IndexMap<&'static str, Box<dyn FlavorGenerator + 'static>>,
}

impl SchemaGenerator {
  /// Returns a new `SchemaGenerator` with the given `grost_path`
  #[inline]
  pub const fn structs(&self) -> &IndexSet<Object> {
    &self.structs
  }

  /// Returns the flavor generators of the schema generator
  #[inline]
  pub const fn flavors(&self) -> &IndexMap<&'static str, Box<dyn FlavorGenerator + 'static>> {
    &self.flavors
  }

  pub fn derive(&self) -> syn::Result<proc_macro2::TokenStream> {
    let objects = self
      .structs
      .iter()
      .flat_map(|object| {
        object.derive().map(|basic| {
          self
            .flavors
            .values()
            .map(|flavor| flavor.derive_object(object))
            .collect::<syn::Result<Vec<_>>>()
            .map(|stream| {
              quote! {
                #basic

                #(#stream)*
              }
            })
        })
      })
      .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! {
      #(#objects)*
    })
  }

  // /// Returns a new `SchemaGenerator` with the given `grost_path`
  // #[inline]
  // pub const fn enums(&self) -> &IndexSet<Enum> {
  //   &self.enums
  // }
}

#[cfg(test)]
mod tests;
