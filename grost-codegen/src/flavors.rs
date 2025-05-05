use heck::ToShoutySnakeCase;
use quote::{format_ident, quote};
use syn::Ident;

use crate::Field;

use super::{Enum, Struct};

/// The network flavor code generator
pub mod network;

/// The flavor
pub trait FlavorGenerator {
  /// Returns the full qualify path of the flavor type.
  fn ty(&self) -> &syn::Type;

  /// Sets the type of the flavor
  fn set_ty(&mut self, ty: syn::Type);

  /// Returns the name of the flavor this generator generates code for
  fn name(&self) -> &'static str;

  /// Generates the field identifier
  fn generate_field_identifier(
    &self,
    path_to_grost: &syn::Path,
    field: &Field,
  ) -> proc_macro2::TokenStream;

  /// Generates the codec for the struct type
  fn generate_struct_codec(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream;

  /// Generates the codec for the selection type
  fn generate_selection_codec(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream;

  /// Generates the codec for the unit enum
  fn generate_enum_codec(
    &self,
    path_to_grost: &syn::Path,
    enum_: &Enum,
  ) -> proc_macro2::TokenStream;
}

impl<F: FlavorGenerator + ?Sized> FlavorGenerator for Box<F> {
  fn ty(&self) -> &syn::Type {
    self.as_ref().ty()
  }

  fn name(&self) -> &'static str {
    self.as_ref().name()
  }

  fn set_ty(&mut self, ty: syn::Type) {
    self.as_mut().set_ty(ty)
  }

  fn generate_field_identifier(
    &self,
    path_to_grost: &syn::Path,
    field: &Field,
  ) -> proc_macro2::TokenStream {
    self
      .as_ref()
      .generate_field_identifier(path_to_grost, field)
  }

  fn generate_struct_codec(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
    self.as_ref().generate_struct_codec(path_to_grost, struct_)
  }

  fn generate_enum_codec(
    &self,
    path_to_grost: &syn::Path,
    enum_: &Enum,
  ) -> proc_macro2::TokenStream {
    self.as_ref().generate_enum_codec(path_to_grost, enum_)
  }

  fn generate_selection_codec(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
    self
      .as_ref()
      .generate_selection_codec(path_to_grost, struct_)
  }
}

pub trait FlavorGeneratorExt: FlavorGenerator {
  fn field_reflection_name(&self, field_name: &str) -> Ident {
    let flavor_name_ssc = self.name().to_shouty_snake_case();
    format_ident!(
      "{flavor_name_ssc}_FLAVOR_{}_REFLECTION",
      field_name.to_shouty_snake_case()
    )
  }

  fn optional_field_reflection_name(&self, field_name: &str) -> Ident {
    let flavor_name_ssc = self.name().to_shouty_snake_case();
    format_ident!(
      "__{flavor_name_ssc}_FLAVOR_{}_REFLECTION_OPTIONAL__",
      field_name.to_shouty_snake_case()
    )
  }

  fn struct_reflection_name(&self) -> Ident {
    let flavor_name_ssc = self.name().to_shouty_snake_case();
    format_ident!("{flavor_name_ssc}_FLAVOR_REFLECTION")
  }
}

impl<F> FlavorGeneratorExt for F where F: FlavorGenerator + ?Sized {}
