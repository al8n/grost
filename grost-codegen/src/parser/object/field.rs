use core::num::NonZeroU32;

use darling::{FromField, FromMeta};
use hint::TypeHintMeta;
use syn::{Attribute, Ident, Type, Visibility};

use super::{Attributes, SchemaMeta, from_optional_string};

pub use hint::TypeHint;
pub use select::Selection;

mod hint;
mod select;

#[derive(Debug, Default, FromMeta)]
pub struct PartialRefFieldMeta {
  #[darling(default)]
  copy: bool,
  #[darling(default, map = "Attributes::into")]
  attrs: Attributes,
}

#[derive(Debug, Default, FromMeta)]
pub struct PartialFieldMeta {
  #[darling(default, map = "Attributes::into")]
  attrs: Attributes,
}

#[derive(Debug, FromField)]
#[darling(attributes(grost), forward_attrs)]
pub struct ObjectFieldDeriveInput {
  ident: Option<Ident>,
  ty: Type,
  vis: Visibility,
  attrs: Vec<Attribute>,
  #[darling(default)]
  schema: SchemaMeta,
  #[darling(default, and_then = "from_optional_string")]
  default: Option<syn::Path>,
  tag: NonZeroU32,
  #[darling(default)]
  wire: Option<Type>,
  #[darling(default)]
  partial: PartialFieldMeta,
  #[darling(default)]
  partial_ref: PartialRefFieldMeta,
  #[darling(default)]
  select: Selection,
  #[darling(default)]
  copy: bool,
  #[darling(flatten)]
  hint: TypeHintMeta,
}
