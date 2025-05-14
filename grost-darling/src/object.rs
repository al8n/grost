use darling::FromMeta;
use syn::Attribute;

use super::{SchemaMeta, Attributes, Ident};

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
  pub const fn name(&self) -> Option<&Ident> {
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
  pub const fn name(&self) -> Option<&Ident> {
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
pub struct SelectorMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl SelectorMeta {
  /// Returns the name of the selector
  pub const fn name(&self) -> Option<&Ident> {
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
  pub const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct FieldReflectionMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
}

impl FieldReflectionMeta {
  /// Returns the name of the field reflection
  pub const fn name(&self) -> Option<&Ident> {
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
  pub const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the indexer
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}


#[derive(Debug, Clone, FromMeta)]
pub struct ObjectMeta {
  #[darling(rename = "crate", default = "super::default_grost_path")]
  path_to_crate: syn::Path,
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
  field_reflection: FieldReflectionMeta,
  #[darling(default)]
  indexer: IndexerMeta,
}

impl Default for ObjectMeta {
  fn default() -> Self {
    Self {
      path_to_crate: super::default_grost_path(),
      default: None,
      schema: SchemaMeta::default(),
      partial: PartialObjectMeta::default(),
      partial_ref: PartialRefObjectMeta::default(),
      selector: SelectorMeta::default(),
      reflection: ReflectionMeta::default(),
      field_reflection: FieldReflectionMeta::default(),
      indexer: IndexerMeta::default(),
    }
  }
}

impl ObjectMeta {
  /// Returns the path to the crate
  pub const fn path_to_crate(&self) -> &syn::Path {
    &self.path_to_crate
  }

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

  /// Returns the reflection information
  pub const fn reflection(&self) -> &ReflectionMeta {
    &self.reflection
  }

  /// Returns the field reflection information
  pub const fn field_reflection(&self) -> &FieldReflectionMeta {
    &self.field_reflection
  }

  /// Returns the indexer information
  pub const fn indexer(&self) -> &IndexerMeta {
    &self.indexer
  }
}

/// The trait for the object derive input
pub trait Object {
  /// The type of the field
  type Field: Field;

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