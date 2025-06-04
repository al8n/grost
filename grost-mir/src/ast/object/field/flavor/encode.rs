use darling::FromMeta;

use crate::ast::BoolOption;

#[derive(Debug, Default, Clone, PartialEq, Eq, FromMeta)]
pub(super) struct EncodeFromMeta {
  #[darling(default)]
  skip_default: BoolOption,
  #[darling(default)]
  skip_if: Option<syn::Path>,
  #[darling(default)]
  error_if: Option<syn::Path>,
}

impl EncodeFromMeta {
  pub fn finalize(self) -> darling::Result<EncodeAttribute> {
    Ok(EncodeAttribute {
      skip_default: self.skip_default.into(),
      skip_if: self.skip_if,
      error_if: self.error_if,
    })
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EncodeAttribute {
  skip_default: Option<bool>,
  skip_if: Option<syn::Path>,
  error_if: Option<syn::Path>,
}

impl EncodeAttribute {
  /// Returns `true` if the field should skip encoding the default value.
  #[inline]
  pub const fn skip_default(&self) -> Option<bool> {
    self.skip_default
  }

  /// Returns the path to a function that determines if the field should be skipped.
  #[inline]
  pub const fn skip_if(&self) -> Option<&syn::Path> {
    self.skip_if.as_ref()
  }

  /// Returns the path to a function that determines if an error on such value should be raised during encoding.
  #[inline]
  pub const fn error_if(&self) -> Option<&syn::Path> {
    self.error_if.as_ref()
  }
}
