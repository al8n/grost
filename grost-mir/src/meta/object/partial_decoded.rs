use darling::FromMeta;
use syn::{Attribute, Ident};

use super::super::Attributes;

#[derive(Debug, Default, Clone, FromMeta)]
pub(crate) struct PartialDecodedObjectFromMeta {
  #[darling(default, rename = "rename")]
  pub(crate) name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  pub(crate) attrs: Vec<Attribute>,
  #[darling(default)]
  pub(crate) copy: bool,
}
