use darling::FromMeta;
use syn::{Attribute, Ident, LifetimeParam, Meta, Type, TypeParam};

use crate::{flavor::{DecodeFromMeta, IdentifierFromMeta, TagFromMeta}, utils::{
  grost_lifetime, grost_read_buffer_param, grost_unknown_buffer_param, grost_wire_format_param, grost_write_buffer_param, Invokable, NestedMeta, SchemaFromMeta, Attributes
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
  #[darling(default = grost_unknown_buffer_param)]
  pub(in crate::object) unknown_buffer: TypeParam,
  #[darling(default = grost_read_buffer_param)]
  pub(in crate::object) read_buffer: TypeParam,
  #[darling(default = grost_write_buffer_param)]
  pub(in crate::object) write_buffer: TypeParam,
  #[darling(default = grost_wire_format_param)]
  pub(in crate::object) wire_format: TypeParam,
}

#[derive(Debug, Clone)]
pub struct ObjectFlavorFromMeta {
  ty: Type,
  wire_format: Type,
  tag: TagFromMeta,
  identifier: IdentifierFromMeta,
}

impl FromMeta for ObjectFlavorFromMeta {
  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (match *item {
      Meta::Path(_) => Self::from_word(),
      Meta::NameValue(ref value) => Self::from_expr(&value.value),
      Meta::List(ref value) => {
        #[derive(Debug, Clone, FromMeta)]
        struct Helper {
          #[darling(default, rename = "type")]
          ty: Type,
          #[darling(default)]
          wire_format: Type,
          #[darling(default)]
          tag: TagFromMeta,
          #[darling(default)]
          identifier: IdentifierFromMeta,
        }
        let Helper {
          ty,
          wire_format,
          tag,
          identifier,
        } = Helper::from_list(&NestedMeta::parse_meta_list(value.tokens.clone())?[..])?;

        Ok(Self {
          ty,
          wire_format,
          tag,
          identifier,
        })
      }
    })
    .map_err(|e| e.with_span(item))
  }
}

//
// flavor(
//   type = "Network",
//   wire_format = "LengthDelimited",
//   tag(),
//   identifier(),
// ),
// partial_ref(
//   decode()
// ),
// 


#[derive(Debug, Default, Clone, FromMeta)]
pub(in crate::object) struct PartialRefObjectFromMeta {
  #[darling(default, rename = "rename")]
  pub(in crate::object) name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  pub(in crate::object) attrs: Vec<Attribute>,
  #[darling(default)]
  pub(in crate::object) copy: bool,
  #[darling(default)]
  pub(in crate::object) decode: DecodeFromMeta,
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct ObjectFromMeta<E> {
  #[darling(default)]
  pub(in crate::object) default: Option<Invokable>,
  #[darling(default)]
  pub(in crate::object) generic: Generic,
  #[darling(default)]
  pub(in crate::object) schema: SchemaFromMeta,
  #[darling(default)]
  pub(in crate::object) partial: PartialObjectFromMeta,
  #[darling(default)]
  pub(in crate::object) partial_ref: PartialRefObjectFromMeta,
  #[darling(default)]
  pub(in crate::object) selector: SelectorFromMeta,
  #[darling(default)]
  pub(in crate::object) selector_iter: SelectorIterFromMeta,
  #[darling(default)]
  pub(in crate::object) indexer: IndexerFromMeta,
  #[darling(default)]
  pub(in crate::object) flavor: Option<ObjectFlavorFromMeta>,
  #[darling(default)]
  pub(in crate::object) copy: bool,
  #[darling(flatten)]
  pub(in crate::object) extra: E,
}
