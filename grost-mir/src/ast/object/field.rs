use core::num::NonZeroU32;

use darling::FromMeta;
use either::Either;
use label::FieldLabelFromMeta;
use syn::{Attribute, Ident, Type};

use crate::ast::SchemaAttribute;

use super::{Attributes, SchemaFromMeta};
use select::SelectorFieldFromMeta;

pub use label::Label;
pub use select::{Selection, SelectorFieldAttribute};

mod label;
mod select;
mod wire;

/// The meta of the partial reference object field
#[derive(Debug, Default, Clone, FromMeta)]
struct PartialDecodedFieldFromMeta {
  #[darling(default)]
  copy: bool,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
  #[darling(rename = "type", default)]
  ty: Option<Type>,
  #[darling(default)]
  owned: bool,
}

impl PartialDecodedFieldFromMeta {
  /// Finalizes the partial decoded field meta and returns the attribute
  pub fn finalize(self) -> PartialDecodedFieldAttribute {
    PartialDecodedFieldAttribute {
      copy: self.copy,
      attrs: self.attrs,
      ty: self.ty,
      owned: self.owned,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialDecodedFieldAttribute {
  copy: bool,
  attrs: Vec<Attribute>,
  ty: Option<Type>,
  owned: bool,
}

impl PartialDecodedFieldAttribute {
  /// Returns the attributes of the partial reference object field
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial reference object field is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the type of the partial decoded object field
  pub const fn ty(&self) -> Option<&Type> {
    self.ty.as_ref()
  }

  /// Returns `true` if the partial decoded object field is owned
  pub const fn owned(&self) -> bool {
    self.owned
  }
}

/// The meta of the partial object field
#[derive(Debug, Default, Clone, FromMeta)]
struct PartialFieldFromMeta {
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl PartialFieldFromMeta {
  /// Finalizes the partial field meta and returns the attribute
  pub fn finalize(self) -> PartialFieldAttribute {
    PartialFieldAttribute { attrs: self.attrs }
  }
}

#[derive(Debug, Clone)]
pub struct PartialFieldAttribute {
  attrs: Vec<Attribute>,
}

impl PartialFieldAttribute {
  /// Returns the attributes of the partial object field
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}

/// The meta of the object field
#[derive(Debug, Clone, FromMeta)]
pub struct FieldFromMeta {
  #[darling(default)]
  schema: SchemaFromMeta,
  #[darling(default)]
  default: Option<syn::Path>,
  #[darling(default)]
  tag: Option<NonZeroU32>,
  #[darling(default)]
  wire: Option<Type>,
  #[darling(default)]
  partial: PartialFieldFromMeta,
  #[darling(default)]
  partial_decoded: PartialDecodedFieldFromMeta,
  #[darling(default)]
  selector: SelectorFieldFromMeta,
  #[darling(default)]
  copy: bool,
  #[darling(default)]
  skip: bool,
  #[darling(flatten)]
  label: FieldLabelFromMeta,
}

impl FieldFromMeta {
  pub fn finalize(self) -> darling::Result<FieldAttribute> {
    Ok(FieldAttribute {
      default: self.default,
      schema: self.schema.into(),
      tag: self.tag,
      skip: self.skip,
      wire: self.wire,
      partial: self.partial.finalize(),
      partial_decoded: self.partial_decoded.finalize(),
      selector: self.selector.finalize(),
      copy: self.copy,
      label: self.label.into_label(),
    })
  }
}

#[derive(Debug, Clone)]
pub struct SkippedFieldAttribute {}

impl SkippedFieldAttribute {}

#[derive(Debug, Clone)]
pub struct FieldAttribute {
  default: Option<syn::Path>,
  schema: SchemaAttribute,
  tag: Option<NonZeroU32>,
  wire: Option<Type>,
  partial: PartialFieldAttribute,
  partial_decoded: PartialDecodedFieldAttribute,
  selector: SelectorFieldAttribute,
  copy: bool,
  skip: bool,
  label: Option<Label>,
}

impl FieldAttribute {
  /// Returns the tag of the field
  pub const fn tag(&self) -> Option<NonZeroU32> {
    self.tag
  }

  /// Returns the wire format will be used for the field
  pub const fn wire(&self) -> Option<&Type> {
    self.wire.as_ref()
  }

  /// Returns the information about the partial object field
  pub const fn partial(&self) -> &PartialFieldAttribute {
    &self.partial
  }

  /// Returns the information about the partial reference object field
  pub const fn partial_decoded(&self) -> &PartialDecodedFieldAttribute {
    &self.partial_decoded
  }

  /// Returns whether the field type is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns whether the field should be skipped
  pub const fn skip(&self) -> bool {
    self.skip
  }

  /// Returns the default selection for the field
  pub const fn selector(&self) -> &SelectorFieldAttribute {
    &self.selector
  }

  /// Returns the type specification of the field
  pub fn label(&self) -> Option<&Label> {
    self.label.as_ref()
  }

  /// Returns the schema information of the field
  pub const fn schema(&self) -> &SchemaAttribute {
    &self.schema
  }

  /// Returns the fn which will be used to generate the default value for the field
  pub const fn default(&self) -> Option<&syn::Path> {
    self.default.as_ref()
  }
}

/// The trait for the field derive input
pub trait Field: Clone {
  /// Returns the name of the field
  fn name(&self) -> &Ident;

  /// Returns the type of the field
  fn ty(&self) -> &Type;

  /// Returns the visibility of the field
  fn vis(&self) -> &syn::Visibility;

  /// Returns the attributes of the field
  fn attrs(&self) -> &[Attribute];

  /// Returns the tag of the field
  fn tag(&self) -> Option<NonZeroU32>;

  /// Returns the wire format will be used for the field
  fn wire(&self) -> Option<&Type>;

  /// Returns the information about the partial object field
  fn partial(&self) -> &PartialFieldAttribute;

  /// Returns the information about the partial reference object field
  fn partial_decoded(&self) -> &PartialDecodedFieldAttribute;

  /// Returns whether the field type is copyable
  fn copy(&self) -> bool;

  /// Returns whether the field should be skipped
  fn skip(&self) -> bool;

  /// Returns the default selection for the field
  fn selector(&self) -> &SelectorFieldAttribute;

  /// Returns the type specification of the field
  fn label(&self) -> Option<&Label>;

  /// Returns the schema information of the field
  fn schema(&self) -> &SchemaAttribute;

  /// Returns the fn which will be used to generate the default value for the field
  fn default(&self) -> Option<&syn::Path>;
}
