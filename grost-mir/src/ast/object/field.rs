use core::num::NonZeroU32;
use std::sync::{Arc, OnceLock};

use darling::FromMeta;
use syn::{Attribute, Ident, Type};
use type_spec::TypeHintMeta;

use super::{Attributes, SchemaMeta};

pub use select::{Selection, SelectorFieldMeta};
pub use type_spec::TypeSpecification;

mod select;
mod type_spec;

/// The meta of the partial reference object field
#[derive(Debug, Default, Clone, FromMeta)]
pub struct PartialDecodedFieldMeta {
  #[darling(default)]
  copy: bool,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl PartialDecodedFieldMeta {
  /// Returns the attributes of the partial reference object field
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial reference object field is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }
}

/// The meta of the partial object field
#[derive(Debug, Default, Clone, FromMeta)]
pub struct PartialFieldMeta {
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl PartialFieldMeta {
  /// Returns the attributes of the partial object field
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}

/// The meta of the object field
#[derive(Debug, Clone, FromMeta)]
pub struct FieldMeta {
  #[darling(default)]
  schema: SchemaMeta,
  #[darling(default)]
  default: Option<syn::Path>,
  #[darling(default)]
  tag: Option<NonZeroU32>,
  #[darling(default)]
  wire: Option<Type>,
  #[darling(default)]
  partial: PartialFieldMeta,
  #[darling(default)]
  partial_decoded: PartialDecodedFieldMeta,
  #[darling(default)]
  selector: SelectorFieldMeta,
  #[darling(default)]
  copy: bool,
  #[darling(default)]
  skip: bool,
  #[darling(flatten)]
  meta: TypeHintMeta,
  #[darling(skip)]
  specification: Arc<OnceLock<Option<TypeSpecification>>>,
}

impl FieldMeta {
  /// Returns the tag of the field
  pub const fn tag(&self) -> Option<NonZeroU32> {
    self.tag
  }

  /// Returns the wire format will be used for the field
  pub const fn wire(&self) -> Option<&Type> {
    self.wire.as_ref()
  }

  /// Returns the information about the partial object field
  pub const fn partial(&self) -> &PartialFieldMeta {
    &self.partial
  }

  /// Returns the information about the partial reference object field
  pub const fn partial_decoded(&self) -> &PartialDecodedFieldMeta {
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
  pub const fn selector(&self) -> &SelectorFieldMeta {
    &self.selector
  }

  /// Returns the type specification of the field
  pub fn type_specification(&self) -> Option<&TypeSpecification> {
    self
      .specification
      .get_or_init(|| self.meta.clone().into_specification())
      .as_ref()
  }

  /// Returns the schema information of the field
  pub const fn schema(&self) -> &SchemaMeta {
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

  /// Returns the field meta information
  fn meta(&self) -> &FieldMeta;
}
