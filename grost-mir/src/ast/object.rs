use darling::FromMeta;
use quote::format_ident;
use syn::{Attribute, Ident};

use super::{
  Attributes, FlavorAttribute, FlavorFromMeta, GenericAttribute, SchemaAttribute, SchemaFromMeta,
  grost_flavor_param,
};

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
  fn finalize(self, unknown_buffer_generic: syn::TypeParam) -> PartialObjectAttribute {
    PartialObjectAttribute {
      name: self.name,
      attrs: self.attrs,
      unknown_buffer_generic,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialObjectAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  unknown_buffer_generic: syn::TypeParam,
}

impl PartialObjectAttribute {
  /// Returns the name of the partial object
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the partial object
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the unknown buffer generic parameter
  pub const fn unknown_buffer(&self) -> &syn::TypeParam {
    &self.unknown_buffer_generic
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
struct PartialDecodedObjectFromMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
  #[darling(default)]
  copy: bool,
}

impl PartialDecodedObjectFromMeta {
  fn finalize(
    self,
    flavor_param: Option<syn::TypeParam>,
    unknown_buffer_param: syn::TypeParam,
    lifetime_param: syn::LifetimeParam,
  ) -> PartialDecodedObjectAttribute {
    PartialDecodedObjectAttribute {
      name: self.name,
      attrs: self.attrs,
      copy: self.copy,
      flavor_param,
      unknown_buffer_param,
      lifetime_param,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialDecodedObjectAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  copy: bool,
  flavor_param: Option<syn::TypeParam>,
  unknown_buffer_param: syn::TypeParam,
  lifetime_param: syn::LifetimeParam,
}

impl PartialDecodedObjectAttribute {
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

  /// Returns the flavor generic parameter if it exists
  pub const fn flavor(&self) -> Option<&syn::TypeParam> {
    self.flavor_param.as_ref()
  }

  /// Returns the unknown buffer generic parameter
  pub const fn unknown_buffer(&self) -> &syn::TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns the lifetime generic parameter
  pub const fn lifetime(&self) -> &syn::LifetimeParam {
    &self.lifetime_param
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
struct SelectorIterFromMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl SelectorIterFromMeta {
  fn finalize(self, flavor_param: Option<syn::TypeParam>) -> SelectorIterAttribute {
    SelectorIterAttribute {
      name: self.name,
      attrs: self.attrs,
      flavor_param,
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct SelectorIterAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  flavor_param: Option<syn::TypeParam>,
}

impl SelectorIterAttribute {
  /// Returns the name of the selector iterator
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the selector iterator
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the flavor generic parameter if it exists
  pub const fn flavor(&self) -> Option<&syn::TypeParam> {
    self.flavor_param.as_ref()
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
  fn finalize(
    self,
    flavor_param: Option<syn::TypeParam>,
    wire_format: syn::TypeParam,
  ) -> SelectorAttribute {
    SelectorAttribute {
      name: self.name,
      attrs: self.attrs,
      flavor_param,
      wire_format_param: wire_format,
    }
  }
}

#[derive(Debug, Clone)]
pub struct SelectorAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  flavor_param: Option<syn::TypeParam>,
  wire_format_param: syn::TypeParam,
}

impl SelectorAttribute {
  /// Returns the name of the selector
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the selector
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the flavor generic parameter if it exists
  pub const fn flavor(&self) -> Option<&syn::TypeParam> {
    self.flavor_param.as_ref()
  }

  /// Returns the wire format generic parameter
  pub const fn wire_format(&self) -> &syn::TypeParam {
    &self.wire_format_param
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
struct IndexerFromMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl From<IndexerFromMeta> for IndexerAttribute {
  fn from(meta: IndexerFromMeta) -> Self {
    Self {
      name: meta.name,
      attrs: meta.attrs,
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct IndexerAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
}

impl IndexerAttribute {
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
  generic: GenericAttribute,
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
  pub fn finalize(self, path_to_grost: syn::Path) -> syn::Result<ObjectAttribute> {
    let flavors = self.flavor.into_object_flavors(&path_to_grost)?;
    let mut flavor_generic = self.generic.flavor().cloned();
    if flavors.len() > 1 {
      flavor_generic.get_or_insert_with(grost_flavor_param);
    }

    Ok(ObjectAttribute {
      path_to_grost,
      flavors,
      default: self.default,
      schema: self.schema.into(),
      partial: self.partial.finalize(self.generic.unknown_buffer().clone()),
      partial_decoded: self.partial_decoded.finalize(
        flavor_generic.clone(),
        self.generic.unknown_buffer().clone(),
        self.generic.lifetime().clone(),
      ),
      selector: self
        .selector
        .finalize(flavor_generic.clone(), self.generic.wire_format().clone()),
      selector_iter: self.selector_iter.finalize(flavor_generic),
      indexer: self.indexer.into(),
      copy: self.copy,
    })
  }
}

#[derive(Debug, Clone)]
pub struct ObjectAttribute {
  path_to_grost: syn::Path,
  flavors: Vec<FlavorAttribute>,
  default: Option<syn::Path>,
  schema: SchemaAttribute,
  partial: PartialObjectAttribute,
  partial_decoded: PartialDecodedObjectAttribute,
  selector: SelectorAttribute,
  selector_iter: SelectorIterAttribute,
  indexer: IndexerAttribute,
  copy: bool,
}

impl ObjectAttribute {
  /// Returns the path to the `grost` crate
  pub const fn path_to_grost(&self) -> &syn::Path {
    &self.path_to_grost
  }

  /// Returns the path to the fn that returns the default value of the object
  pub const fn default(&self) -> Option<&syn::Path> {
    self.default.as_ref()
  }

  /// Returns the schema information
  pub const fn schema(&self) -> &SchemaAttribute {
    &self.schema
  }

  /// Returns the partial object information
  pub const fn partial(&self) -> &PartialObjectAttribute {
    &self.partial
  }

  /// Returns the partial decoded object information
  pub const fn partial_decoded(&self) -> &PartialDecodedObjectAttribute {
    &self.partial_decoded
  }

  /// Returns the selector information
  pub const fn selector(&self) -> &SelectorAttribute {
    &self.selector
  }

  /// Returns the selector iterator information
  pub const fn selector_iter(&self) -> &SelectorIterAttribute {
    &self.selector_iter
  }

  /// Returns the indexer information
  pub const fn indexer(&self) -> &IndexerAttribute {
    &self.indexer
  }

  /// Returns whether the object is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the flavors of the object
  pub const fn flavors(&self) -> &[FlavorAttribute] {
    self.flavors.as_slice()
  }
}

/// The trait for the object derive input
pub trait Object: Clone {
  /// The type of the field
  type Field: Field;

  /// Returns the name of the object
  fn name(&self) -> &Ident;

  /// Returns the visibility of the object
  fn vis(&self) -> &syn::Visibility;

  /// Returns the generics in the object defination.
  fn generics(&self) -> &syn::Generics;

  /// Returns the attributes in the object defination.
  fn attrs(&self) -> &[Attribute];

  /// Returns the fields of the object
  fn fields(&self) -> Vec<&Self::Field>;

  /// Returns the path to the `grost` crate
  fn path_to_grost(&self) -> &syn::Path;

  /// Returns the path to the fn that returns the default value of the object
  fn default(&self) -> Option<&syn::Path>;

  /// Returns the schema information
  fn schema(&self) -> &SchemaAttribute;

  /// Returns the partial object information
  fn partial(&self) -> &PartialObjectAttribute;

  /// Returns the partial decoded object information
  fn partial_decoded(&self) -> &PartialDecodedObjectAttribute;

  /// Returns the selector information
  fn selector(&self) -> &SelectorAttribute;

  /// Returns the selector iterator information
  fn selector_iter(&self) -> &SelectorIterAttribute;

  /// Returns the indexer information
  fn indexer(&self) -> &IndexerAttribute;

  /// Returns whether the object is copyable
  fn copy(&self) -> bool;

  /// Returns the flavors of the object
  fn flavors(&self) -> &[FlavorAttribute];
}

/// The extension trait for the object
pub trait ObjectExt: Object {
  #[inline]
  fn partial_decoded_name(&self) -> Ident {
    self
      .partial_decoded()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("PartialDecoded{}", self.name()))
  }

  #[inline]
  fn partial_name(&self) -> Ident {
    self
      .partial()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("Partial{}", self.name()))
  }

  #[inline]
  fn selector_name(&self) -> Ident {
    self
      .selector()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Selector", self.name()))
  }

  #[inline]
  fn selector_iter_name(&self) -> Ident {
    self
      .selector_iter()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Iter", self.selector_name()))
  }

  #[inline]
  fn indexer_name(&self) -> Ident {
    self
      .indexer()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Index", self.name()))
  }
}

impl<T: Object> ObjectExt for T {}
