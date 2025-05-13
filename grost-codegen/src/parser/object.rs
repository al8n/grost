use darling::{FromDeriveInput, FromMeta, ast::Data, util::Ignored};
use syn::{Attribute, Generics, Ident, Visibility};

use super::*;

pub use field::{ObjectFieldDeriveInput, PartialFieldMeta, PartialRefFieldMeta, Selection};

mod field;

#[derive(Default, Debug, FromMeta)]
pub struct SchemaMeta {
  name: Option<String>,
  description: Option<String>,
}

#[derive(Debug, Default, FromMeta)]
pub struct PartialObjectMeta {
  #[darling(default, rename = "rename")]
  name: Option<String>,
  #[darling(default, map = "Attributes::into")]
  attrs: Attributes,
}

#[derive(Debug, Default, FromMeta)]
pub struct PartialRefObjectMeta {
  #[darling(default, rename = "rename")]
  name: Option<String>,
  #[darling(default, map = "Attributes::into")]
  attrs: Attributes,
  #[darling(default)]
  copy: bool,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(grost), supports(struct_named), forward_attrs)]
pub struct ObjectDeriveInput {
  ident: Ident,
  vis: Visibility,
  generics: Generics,
  attrs: Vec<Attribute>,
  #[darling(default, and_then = "from_optional_string")]
  default: Option<syn::Path>,
  #[darling(default)]
  schema: SchemaMeta,
  #[darling(default)]
  partial: PartialObjectMeta,
  #[darling(default)]
  partial_ref: PartialRefObjectMeta,
  data: Data<Ignored, ObjectFieldDeriveInput>,
}
