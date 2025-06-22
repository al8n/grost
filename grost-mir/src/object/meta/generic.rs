use darling::FromMeta;
use syn::{Attribute, Ident, LifetimeParam, TypeParam};

use crate::{flavor::GenericFlavorFromMeta, utils::{
  grost_flavor_param, grost_lifetime, grost_read_buffer_param, grost_unknown_buffer_param, grost_wire_format_param, grost_write_buffer_param, Invokable, SchemaFromMeta, Attributes,
}};

use super::{
  IndexerFromMeta, PartialObjectFromMeta,
  SelectorFromMeta, SelectorIterFromMeta,
};

pub use field::*;
mod field;

fn string_to_lifetime(s: String) -> darling::Result<LifetimeParam> {
  syn::parse_str(&s).map_err(Into::into)
}

#[derive(Debug, Default, Clone, FromMeta)]
pub(in crate::object) struct Generic {
  #[darling(default = grost_lifetime, and_then = "string_to_lifetime")]
  pub(in crate::object) lifetime: LifetimeParam,
  #[darling(default = grost_flavor_param)]
  pub(in crate::object) flavor: TypeParam,
  #[darling(default = grost_unknown_buffer_param)]
  pub(in crate::object) unknown_buffer: TypeParam,
  #[darling(default = grost_read_buffer_param)]
  pub(in crate::object) read_buffer: TypeParam,
  #[darling(default = grost_write_buffer_param)]
  pub(in crate::object) write_buffer: TypeParam,
  #[darling(default = grost_wire_format_param)]
  pub(in crate::object) wire_format: TypeParam,
}

#[derive(Debug, Default, Clone, FromMeta)]
pub(in crate::object) struct GenericPartialRefObjectFromMeta {
  #[darling(default, rename = "rename")]
  pub(in crate::object) name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  pub(in crate::object) attrs: Vec<Attribute>,
  #[darling(default)]
  pub(in crate::object) copy: bool,
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct GenericObjectFromMeta<E> {
  #[darling(default)]
  pub(in crate::object) default: Option<Invokable>,
  #[darling(default)]
  pub(in crate::object) generic: Generic,
  #[darling(default)]
  pub(in crate::object) schema: SchemaFromMeta,
  #[darling(default)]
  pub(in crate::object) partial: PartialObjectFromMeta,
  #[darling(default)]
  pub(in crate::object) partial_ref: GenericPartialRefObjectFromMeta,
  #[darling(default)]
  pub(in crate::object) selector: SelectorFromMeta,
  #[darling(default)]
  pub(in crate::object) selector_iter: SelectorIterFromMeta,
  #[darling(default)]
  pub(in crate::object) indexer: IndexerFromMeta,
  #[darling(default)]
  pub(in crate::object) flavor: GenericFlavorFromMeta,
  #[darling(default)]
  pub(in crate::object) copy: bool,
  #[darling(flatten)]
  pub(in crate::object) extra: E,
}
