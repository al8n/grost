use syn::Path;

use crate::meta::{MissingOperation, object::field::FieldDecodeFromMeta};

#[derive(Debug, Clone)]
pub struct FieldDecodeAttribute {
  pub(crate) missing_operation: Option<MissingOperation>,
  pub(crate) error_if: Option<Path>,
}

impl FieldDecodeAttribute {
  /// Returns the `or_else` variant if specified, otherwise `None`.
  #[inline]
  pub const fn missing_operation(&self) -> Option<&MissingOperation> {
    self.missing_operation.as_ref()
  }

  /// Returns the path to a function that determines if an error should be raised during decoding.
  #[inline]
  pub const fn error_if(&self) -> Option<&Path> {
    self.error_if.as_ref()
  }

  pub(crate) fn new(
    missing_operation: Option<MissingOperation>,
    error_if: Option<syn::Path>,
  ) -> Self {
    Self {
      missing_operation,
      error_if,
    }
  }
}

impl FieldDecodeFromMeta {
  /// Finalizes the parsing and returns a `FieldDecodeAttribute`.
  pub fn finalize(self) -> darling::Result<FieldDecodeAttribute> {
    Ok(FieldDecodeAttribute {
      missing_operation: self.missing_operation,
      error_if: self.error_if,
    })
  }
}
