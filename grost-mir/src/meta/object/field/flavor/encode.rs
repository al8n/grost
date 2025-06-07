use darling::FromMeta;

use crate::meta::BoolOption;

#[derive(Debug, Default, Clone, PartialEq, Eq, FromMeta)]
pub(crate) struct EncodeFromMeta {
  #[darling(default)]
  pub(crate) skip_default: BoolOption,
  #[darling(default)]
  pub(crate) skip_if: Option<syn::Path>,
  #[darling(default)]
  pub(crate) error_if: Option<syn::Path>,
}
