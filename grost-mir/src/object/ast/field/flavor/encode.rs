use syn::Path;

use crate::object::meta::FieldEncodeFromMeta;

#[derive(Debug, Clone)]
pub struct FieldEncodeAttribute {
  pub(in crate::object) skip_default: Option<bool>,
  pub(in crate::object) skip_if: Option<Path>,
  pub(in crate::object) error_if: Option<Path>,
}

impl FieldEncodeAttribute {
  /// Returns `true` if the field should skip encoding the default value.
  #[inline]
  pub const fn skip_default(&self) -> Option<bool> {
    self.skip_default
  }

  /// Returns the path to a function that determines if the field should be skipped.
  #[inline]
  pub const fn skip_if(&self) -> Option<&Path> {
    self.skip_if.as_ref()
  }

  /// Returns the path to a function that determines if an error on such value should be raised during encoding.
  #[inline]
  pub const fn error_if(&self) -> Option<&Path> {
    self.error_if.as_ref()
  }

  pub(crate) fn new(
    skip_default: Option<bool>,
    skip_if: Option<syn::Path>,
    error_if: Option<syn::Path>,
  ) -> Self {
    Self {
      skip_default,
      skip_if,
      error_if,
    }
  }
}

impl FieldEncodeFromMeta {
  pub fn finalize(self) -> darling::Result<FieldEncodeAttribute> {
    Ok(FieldEncodeAttribute {
      skip_default: self.skip_default.into(),
      skip_if: self.skip_if,
      error_if: self.error_if,
    })
  }
}
