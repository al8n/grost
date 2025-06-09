use crate::{
  object::meta::ConvertFromMeta,
  utils::{MissingOperation, TransformOperation},
};

impl ConvertFromMeta {
  /// Finalizes the parsing and returns a `ConvertAttribute`.
  pub fn finalize(self) -> darling::Result<ConvertAttribute> {
    Ok(ConvertAttribute {
      missing_operation: self.missing_operation,
      transform_operation: self.transform_operation,
    })
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvertAttribute {
  missing_operation: Option<MissingOperation>,
  transform_operation: Option<TransformOperation>,
}

impl ConvertAttribute {
  /// Returns the `or_else` variant if specified, otherwise `None`.
  pub const fn missing_operation(&self) -> Option<&MissingOperation> {
    self.missing_operation.as_ref()
  }

  /// Returns the path to a function that determines if an error should be raised during decoding.
  pub const fn transform_operation(&self) -> Option<&TransformOperation> {
    self.transform_operation.as_ref()
  }
}
