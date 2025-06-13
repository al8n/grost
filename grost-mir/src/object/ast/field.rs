use std::num::NonZeroU32;
use syn::{Attribute, Ident, Type, Visibility};

pub use convert::ConvertAttribute;
pub use flavor::{FieldDecodeAttribute, FieldEncodeAttribute, FieldFlavorAttribute};
pub use partial::PartialFieldAttribute;
pub use partial_decoded::PartialDecodedFieldAttribute;
pub use selector::SelectorFieldAttribute;

use crate::{
  object::meta::{FieldFromMeta, Label},
  utils::{Invokable, SchemaAttribute},
};

mod convert;
mod flavor;
mod partial;
mod partial_decoded;
mod selector;

#[derive(Debug, Clone)]
pub struct FieldAttribute {
  convert: ConvertAttribute,
  default: Option<Invokable>,
  schema: SchemaAttribute,
  tag: Option<NonZeroU32>,
  label: Option<Label>,
  flavor: Vec<FieldFlavorAttribute>,
  partial: PartialFieldAttribute,
  partial_decoded: PartialDecodedFieldAttribute,
  selector: SelectorFieldAttribute,
  copy: bool,
  skip: bool,
}

impl FieldAttribute {
  /// Returns the tag of the field
  pub const fn tag(&self) -> Option<NonZeroU32> {
    self.tag
  }

  /// Returns the flavor specified settings for the field
  pub const fn flavors(&self) -> &[FieldFlavorAttribute] {
    self.flavor.as_slice()
  }

  /// Returns the convert attribute for the field
  pub const fn convert(&self) -> &ConvertAttribute {
    &self.convert
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

  /// Returns the type label of the field
  pub const fn label(&self) -> Option<&Label> {
    self.label.as_ref()
  }

  /// Returns the schema information of the field
  pub const fn schema(&self) -> &SchemaAttribute {
    &self.schema
  }

  /// Returns the fn which will be used to generate the default value for the field
  pub const fn default(&self) -> Option<&Invokable> {
    self.default.as_ref()
  }
}

impl FieldFromMeta {
  pub fn finalize(self) -> darling::Result<FieldAttribute> {
    Ok(FieldAttribute {
      default: self.default,
      schema: self.schema.into(),
      tag: self.tag,
      label: self.label,
      skip: self.skip,
      convert: self.convert.finalize()?,
      flavor: self.flavor.finalize()?,
      partial: self.partial.finalize(),
      partial_decoded: self.partial_decoded.finalize(),
      selector: self.selector.finalize(),
      copy: self.copy,
    })
  }
}

/// The trait for the field derive input
pub trait RawField: Clone {
  /// The custom metadata type for the field
  type Meta: Clone;

  /// Returns the name of the field
  fn name(&self) -> &Ident;

  /// Returns the type of the field
  fn ty(&self) -> &Type;

  /// Returns the visibility of the field
  fn vis(&self) -> &Visibility;

  /// Returns the attributes of the field
  fn attrs(&self) -> &[Attribute];

  /// Returns the tag of the field
  fn tag(&self) -> Option<NonZeroU32>;

  /// Returns the convert attribute for the field
  fn convert(&self) -> &ConvertAttribute;

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

  /// Returns the type label of the field
  fn label(&self) -> Option<&Label>;

  /// Returns the schema information of the field
  fn schema(&self) -> &SchemaAttribute;

  /// Returns the fn which will be used to generate the default value for the field
  fn default(&self) -> Option<&Invokable>;

  /// Returns the field flavor attribute for the field
  fn flavors(&self) -> &[FieldFlavorAttribute];

  /// Returns the custom metadata of the field
  fn meta(&self) -> &Self::Meta;
}
