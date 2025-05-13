use std::num::NonZeroU32;

use darling::{FromDeriveInput, FromMeta, ast::Data, util::Ignored};
use field::TypeHint;
use syn::{Attribute, DeriveInput, Ident, Visibility};

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
  generics: syn::Generics,
  attrs: Vec<Attribute>,
  #[darling(rename = "crate", default)]
  path_to_grost: Option<syn::Path>,
  #[darling(default)]
  default: Option<syn::Path>,
  #[darling(default)]
  schema: SchemaMeta,
  #[darling(default)]
  partial: PartialObjectMeta,
  #[darling(default)]
  partial_ref: PartialRefObjectMeta,
  data: Data<Ignored, ObjectFieldDeriveInput>,
}

pub struct Selector {}

pub struct PartialRefField {
  name: Ident,
  ty: syn::Type,
  vis: Visibility,
  tag: NonZeroU32,
  wire: Option<syn::Type>,
  attrs: Vec<Attribute>,
  copy: bool,
}

pub struct PartialField {
  name: Ident,
  ty: syn::Type,
  vis: Visibility,
  tag: NonZeroU32,
  hint: TypeHint,
  attrs: Vec<Attribute>,
  copy: bool,
}

pub struct Field {
  name: Ident,
  ty: syn::Type,
  vis: Visibility,
  schema: SchemaMeta,
  default: Option<syn::Path>,
  tag: NonZeroU32,
  wire: Option<syn::Type>,
  hint: TypeHint,
  attrs: Vec<Attribute>,
  copy: bool,
}

pub struct PartialObject {
  name: Ident,
  schema: SchemaMeta,
  vis: Visibility,
  generics: syn::Generics,
  fields: Vec<PartialField>,
  attrs: Vec<Attribute>,
  copy: bool,
}

pub struct PartialRefObject {
  name: Ident,
  vis: Visibility,
  generics: syn::Generics,
  fields: Vec<PartialRefField>,
  attrs: Vec<Attribute>,
  copy: bool,
}

pub struct Object {
  name: Ident,
  path_to_grost: syn::Path,
  schema: SchemaMeta,
  vis: Visibility,
  generics: syn::Generics,
  fields: Vec<Field>,
  partial: PartialObject,
  partial_ref: PartialRefObject,
  selector: Selector,
}

impl Object {
  pub fn from_input(input: ObjectDeriveInput) -> darling::Result<Self> {
    todo!()
  }
}
