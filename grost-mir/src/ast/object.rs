use darling::FromMeta;
use quote::format_ident;
use syn::{Attribute, Ident};

use super::{Attributes, FlavorAttribute, FlavorFromMeta, GenericAttribute, GenericFromMeta, SchemaFromMeta};

pub use field::*;

mod field;

#[derive(Debug, Default, Clone, FromMeta)]
pub struct PartialObjectFromMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl PartialObjectFromMeta {
  /// Returns the name of the partial object
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the partial object
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct PartialDecodedObjectFromMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
  #[darling(default)]
  copy: bool,
}

impl PartialDecodedObjectFromMeta {
  /// Returns the name of the partial reference object
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the partial reference object
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial reference object is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct SelectorIterFromMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl SelectorIterFromMeta {
  /// Returns the name of the selector iterator
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the selector iterator
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct SelectorFromMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl SelectorFromMeta {
  /// Returns the name of the selector
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the selector
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct IndexerFromMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl IndexerFromMeta {
  /// Returns the name of the indexer
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the indexer
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct ObjectFromMeta {
  #[darling(default)]
  default: Option<syn::Path>,
  #[darling(default)]
  generic: GenericFromMeta,
  #[darling(default)]
  schema: SchemaFromMeta,
  #[darling(default)]
  partial: PartialObjectFromMeta,
  #[darling(default)]
  partial_decoded: PartialDecodedObjectFromMeta,
  #[darling(default)]
  selector: SelectorFromMeta,
  #[darling(default)]
  selector_iter: SelectorIterFromMeta,
  #[darling(default)]
  indexer: IndexerFromMeta,
  #[darling(default)]
  flavor: FlavorFromMeta,
  #[darling(default)]
  copy: bool,
}

impl ObjectFromMeta {
  /// Returns the path to the fn that returns the default value of the object
  pub const fn default(&self) -> Option<&syn::Path> {
    self.default.as_ref()
  }

  /// Returns the schema information
  pub const fn schema(&self) -> &SchemaFromMeta {
    &self.schema
  }

  /// Returns the partial object information
  pub const fn partial(&self) -> &PartialObjectFromMeta {
    &self.partial
  }

  /// Returns the partial decoded object information
  pub const fn partial_decoded(&self) -> &PartialDecodedObjectFromMeta {
    &self.partial_decoded
  }

  /// Returns the selector information
  pub const fn selector(&self) -> &SelectorFromMeta {
    &self.selector
  }

  /// Returns the selector iterator information
  pub const fn selector_iter(&self) -> &SelectorIterFromMeta {
    &self.selector_iter
  }

  /// Returns the indexer information
  pub const fn indexer(&self) -> &IndexerFromMeta {
    &self.indexer
  }

  /// Returns whether the object is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }
}

pub struct ObjectAttribute {
  generic: Option<GenericAttribute>,
  flavors: Vec<FlavorAttribute>,
  default: Option<syn::Path>,
  schema: SchemaFromMeta,
  partial: PartialObjectFromMeta,
  partial_decoded: PartialDecodedObjectFromMeta,
  selector: SelectorFromMeta,
  selector_iter: SelectorIterFromMeta,
  indexer: IndexerFromMeta,
  copy: bool,
}

/// The trait for the object derive input
pub trait Object: Clone {
  /// The type of the field
  type Field: Field;

  /// Returns the path to the `grost` crate
  fn path(&self) -> &syn::Path;

  /// Returns the name of the object
  fn name(&self) -> &Ident;

  /// Returns the visibility of the object
  fn vis(&self) -> &syn::Visibility;

  /// Returns the generics of the object
  fn generics(&self) -> &syn::Generics;

  /// Returns the attributes of the object
  fn attrs(&self) -> &[Attribute];

  /// Returns the fields of the object
  fn fields(&self) -> Vec<&Self::Field>;

  /// Returns the meta of the object
  fn meta(&self) -> &ObjectFromMeta;
}

/// The extension trait for the object
pub trait ObjectExt: Object {
  #[inline]
  fn partial_decoded_name(&self) -> Ident {
    self
      .meta()
      .partial_decoded()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("PartialDecoded{}", self.name()))
  }

  #[inline]
  fn partial_name(&self) -> Ident {
    self
      .meta()
      .partial()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("Partial{}", self.name()))
  }

  #[inline]
  fn selector_name(&self) -> Ident {
    self
      .meta()
      .selector()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Selector", self.name()))
  }

  #[inline]
  fn selector_iter_name(&self) -> Ident {
    self
      .meta()
      .selector_iter()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Iter", self.selector_name()))
  }

  #[inline]
  fn indexer_name(&self) -> Ident {
    self
      .meta()
      .indexer()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Index", self.name()))
  }
}

impl<T: Object> ObjectExt for T {}
