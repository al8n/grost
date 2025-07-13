use darling::FromMeta;
use quote::quote;
use syn::{Attribute, Ident, LifetimeParam, Type, TypeParam};

use crate::{
  flavor::{IdentifierFromMeta, TagFromMeta},
  utils::{
    Attributes, BoolOption, Invokable, NoopFromMeta, OrDefault, SchemaFromMeta, grost_buffer_param,
    grost_lifetime, grost_read_buffer_param, grost_write_buffer_param,
  },
};

use super::{IndexerFromMeta, SelectorFromMeta, SelectorIterFromMeta};

pub use field::*;
mod field;

fn string_to_lifetime(s: String) -> darling::Result<LifetimeParam> {
  syn::parse_str(&s).map_err(Into::into)
}

#[derive(Debug, Clone, FromMeta)]
pub(in crate::object) struct Generic {
  #[darling(default = grost_lifetime, and_then = "string_to_lifetime")]
  pub(in crate::object) lifetime: LifetimeParam,
  #[darling(rename = "unknown", default = grost_buffer_param)]
  pub(in crate::object) buffer: TypeParam,
  #[darling(rename = "read", default = grost_read_buffer_param)]
  pub(in crate::object) read_buffer: TypeParam,
  #[darling(rename = "write", default = grost_write_buffer_param)]
  pub(in crate::object) write_buffer: TypeParam,
}

impl Default for Generic {
  fn default() -> Self {
    Self {
      lifetime: grost_lifetime(),
      buffer: grost_buffer_param(),
      read_buffer: grost_read_buffer_param(),
      write_buffer: grost_write_buffer_param(),
    }
  }
}

#[derive(Debug, Clone, FromMeta)]
pub(in crate::object) struct ObjectFlavorFromMeta {
  #[darling(rename = "type")]
  pub(in crate::object) ty: Type,
  pub(in crate::object) wire_format: Type,
  pub(in crate::object) tag: TagFromMeta,
  pub(in crate::object) identifier: IdentifierFromMeta,
}

impl ObjectFlavorFromMeta {
  pub(in crate::object) fn groto(path_to_grost: &syn::Path) -> darling::Result<Self> {
    let ty = syn::parse2(quote!(#path_to_grost::__private::flavors::Groto))?;
    let wire_format =
      syn::parse2(quote!(#path_to_grost::__private::flavors::groto::LengthDelimited))?;
    let identifier = IdentifierFromMeta::groto(path_to_grost)?;
    let tag = TagFromMeta::groto(path_to_grost)?;

    Ok(Self {
      ty,
      wire_format,
      identifier,
      tag,
    })
  }
}

//
// flavor(
//   type = "Groto",
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
  #[darling(default, flatten)]
  pub(in crate::object) or_default: OrDefault,
}

#[derive(Debug, Default, Clone, FromMeta)]
pub(in crate::object) struct PartialObjectFromMeta {
  #[darling(default, rename = "rename")]
  pub(in crate::object) name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  pub(in crate::object) attrs: Vec<Attribute>,
  #[darling(default)]
  pub(in crate::object) copy: bool,
  #[darling(default, flatten)]
  pub(in crate::object) or_default: OrDefault,
}

#[derive(Debug, Default, Clone, FromMeta)]
pub(in crate::object) struct RefObjectFromMeta {
  #[darling(default, rename = "rename")]
  pub(in crate::object) name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  pub(in crate::object) attrs: Vec<Attribute>,
  #[darling(default)]
  pub(in crate::object) copy: bool,
  #[darling(default, flatten)]
  pub(in crate::object) or_default: OrDefault,
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct ObjectFromMeta<E = NoopFromMeta> {
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
  #[darling(default, rename = "ref")]
  pub(in crate::object) ref_: RefObjectFromMeta,
  #[darling(default)]
  pub(in crate::object) selector: SelectorFromMeta,
  #[darling(default)]
  pub(in crate::object) selector_iter: SelectorIterFromMeta,
  #[darling(default)]
  pub(in crate::object) indexer: IndexerFromMeta,
  #[darling(default)]
  pub(in crate::object) flavor: Option<ObjectFlavorFromMeta>,
  #[darling(default)]
  pub(crate) or_default: BoolOption,
  #[darling(default)]
  pub(crate) or_default_scalar: BoolOption,
  #[darling(default)]
  pub(crate) or_default_bytes: BoolOption,
  #[darling(default)]
  pub(crate) or_default_string: BoolOption,
  #[darling(default)]
  pub(crate) or_default_object: BoolOption,
  #[darling(default)]
  pub(crate) or_default_enum: BoolOption,
  #[darling(default)]
  pub(crate) or_default_interface: BoolOption,
  #[darling(default)]
  pub(crate) or_default_union: BoolOption,
  #[darling(default)]
  pub(crate) or_default_map: BoolOption,
  #[darling(default)]
  pub(crate) or_default_set: BoolOption,
  #[darling(default)]
  pub(crate) or_default_list: BoolOption,
  #[darling(default)]
  pub(crate) or_default_generic: BoolOption,
  #[darling(default)]
  pub(in crate::object) copy: bool,
  #[darling(flatten)]
  pub(in crate::object) extra: E,
}
