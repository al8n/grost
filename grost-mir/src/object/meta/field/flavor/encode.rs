use darling::FromMeta;

use crate::utils::BoolOption;

#[derive(Debug, Default, Clone, PartialEq, Eq, FromMeta)]
pub(in crate::object) struct EncodeFromMeta {
  #[darling(default)]
  pub(in crate::object) skip_default: BoolOption,
  #[darling(default)]
  pub(in crate::object) skip_if: Option<syn::Path>,
  #[darling(default)]
  pub(in crate::object) error_if: Option<syn::Path>,
}
