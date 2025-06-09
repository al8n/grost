use darling::FromMeta;
use syn::{Attribute, Ident};

use crate::utils::Attributes;

#[derive(Debug, Default, Clone, FromMeta)]
pub(in crate::object) struct PartialDecodedObjectFromMeta {
  #[darling(default, rename = "rename")]
  pub(in crate::object) name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  pub(in crate::object) attrs: Vec<Attribute>,
  #[darling(default)]
  pub(in crate::object) copy: bool,
}
