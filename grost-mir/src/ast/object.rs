use darling::FromMeta;
use quote::format_ident;
use syn::{Attribute, Ident};

use super::{Attributes, SchemaMeta};

pub use field::*;

mod field;

#[derive(Debug, Default, Clone, FromMeta)]
pub struct PartialObjectMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl PartialObjectMeta {
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
pub struct PartialRefObjectMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
  #[darling(default)]
  copy: bool,
}

impl PartialRefObjectMeta {
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
pub struct SelectorIterMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl SelectorIterMeta {
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
pub struct SelectorMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl SelectorMeta {
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
pub struct ReflectionMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
}

impl ReflectionMeta {
  /// Returns the name of the reflection
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct IndexerMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl IndexerMeta {
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
pub struct ObjectMeta {
  #[darling(default)]
  default: Option<syn::Path>,
  #[darling(default)]
  schema: SchemaMeta,
  #[darling(default)]
  partial: PartialObjectMeta,
  #[darling(default)]
  partial_ref: PartialRefObjectMeta,
  #[darling(default)]
  selector: SelectorMeta,
  #[darling(default)]
  reflection: ReflectionMeta,
  #[darling(default)]
  selector_iter: SelectorIterMeta,
  #[darling(default)]
  indexer: IndexerMeta,
  #[darling(default)]
  copy: bool,
}

impl ObjectMeta {
  /// Returns the path to the fn that returns the default value of the object
  pub const fn default(&self) -> Option<&syn::Path> {
    self.default.as_ref()
  }

  /// Returns the schema information
  pub const fn schema(&self) -> &SchemaMeta {
    &self.schema
  }

  /// Returns the partial object information
  pub const fn partial(&self) -> &PartialObjectMeta {
    &self.partial
  }

  /// Returns the partial reference object information
  pub const fn partial_ref(&self) -> &PartialRefObjectMeta {
    &self.partial_ref
  }

  /// Returns the selector information
  pub const fn selector(&self) -> &SelectorMeta {
    &self.selector
  }

  /// Returns the selector iterator information
  pub const fn selector_iter(&self) -> &SelectorIterMeta {
    &self.selector_iter
  }

  /// Returns the reflection information
  pub const fn reflection(&self) -> &ReflectionMeta {
    &self.reflection
  }

  /// Returns the indexer information
  pub const fn indexer(&self) -> &IndexerMeta {
    &self.indexer
  }

  /// Returns whether the object is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }
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
  fn meta(&self) -> &ObjectMeta;
}

/// The extension trait for the object
pub trait ObjectExt: Object {
  #[inline]
  fn reflection_name(&self) -> Ident {
    self
      .meta()
      .reflection()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Reflection", self.name()))
  }

  #[inline]
  fn partial_ref_name(&self) -> Ident {
    self
      .meta()
      .partial_ref()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("PartialRef{}", self.name()))
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
