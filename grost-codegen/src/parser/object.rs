use darling::{ast::Data, util::{Ignored, SpannedValue}, FromDeriveInput, FromMeta};
use syn::{Attribute, Generics, Ident, Visibility};

use super::*;

pub use field::{ObjectFieldDeriveInput, PartialFieldMeta, PartialRefFieldMeta, Selection};

mod field;

#[derive(Default, Debug, FromMeta)]
pub struct SchemaMeta {
  #[darling(default)]
  name: Option<Ident>,
  #[darling(default)]
  description: Option<String>,
}

#[derive(Debug, Default, FromMeta)]
pub struct PartialObjectMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into")]
  attrs: Attributes,
}

#[derive(Debug, Default, FromMeta)]
pub struct PartialRefObjectMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
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
