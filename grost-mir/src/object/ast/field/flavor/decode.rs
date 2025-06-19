use crate::{
  object::meta::FieldDecodeFromMeta,
  utils::{Invokable, MissingOperation},
};

#[derive(Debug, Clone)]
pub struct FieldDecodeAttribute {
  pub(crate) missing_operation: Option<MissingOperation>,
  pub(crate) then: Option<Invokable>,
}

impl FieldDecodeAttribute {
  /// Returns the `or_else` variant if specified, otherwise `None`.
  #[inline]
  pub const fn missing_operation(&self) -> Option<&MissingOperation> {
    self.missing_operation.as_ref()
  }

  /// Returns the path to a function that determines if an error should be raised during decoding.
  #[inline]
  pub const fn then(&self) -> Option<&Invokable> {
    self.then.as_ref()
  }

  pub(crate) fn new(missing_operation: Option<MissingOperation>, then: Option<Invokable>) -> Self {
    Self {
      missing_operation,
      then,
    }
  }
}

impl FieldDecodeFromMeta {
  /// Finalizes the parsing and returns a `FieldDecodeAttribute`.
  pub fn finalize(self) -> darling::Result<FieldDecodeAttribute> {
    Ok(FieldDecodeAttribute {
      missing_operation: self.missing_operation,
      then: self.then,
    })
  }
}
