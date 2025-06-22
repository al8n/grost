use crate::{
  object::meta::{
    FieldConvertFromMeta, FieldDecodeFromMeta, FieldEncodeFromMeta, FieldSkipEncodeOperation,
  },
  utils::{ConvertOperation, Invokable, MissingOperation},
};

impl FieldConvertFromMeta {
  /// Finalizes the parsing and returns a `FieldConvertOptions`.
  pub fn finalize(self) -> darling::Result<FieldConvertOptions> {
    Ok(FieldConvertOptions {
      missing_operation: self.missing_operation,
      convert_operation: self.convert_operation,
    })
  }
}

impl FieldEncodeFromMeta {
  /// Finalizes the parsing and returns a `FieldEncodeOptions`.
  pub fn finalize(self) -> darling::Result<FieldEncodeOptions> {
    Ok(FieldEncodeOptions {
      skip_operation: self.skip_operation,
      error_if: self.error_if,
    })
  }
}

impl FieldDecodeFromMeta {
  /// Finalizes the parsing and returns a `FieldDecodeOptions`.
  pub fn finalize(self) -> darling::Result<FieldDecodeOptions> {
    Ok(FieldDecodeOptions {
      missing_operation: self.missing_operation,
      then: self.then,
      func: self.func,
    })
  }
}

/// Represents the options for converting a field during converting.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FieldConvertOptions {
  missing_operation: Option<MissingOperation>,
  convert_operation: Option<ConvertOperation>,
}

impl FieldConvertOptions {
  /// Returns the missing operation that should be performed if the field is missing during converting.
  pub const fn missing_operation(&self) -> Option<&MissingOperation> {
    self.missing_operation.as_ref()
  }

  /// Returns the path to a function that determines if an error should be raised during converting.
  pub const fn convert_operation(&self) -> Option<&ConvertOperation> {
    self.convert_operation.as_ref()
  }
}

/// Represents the options for encoding a field.
#[derive(Debug, Default, Clone)]
pub struct FieldEncodeOptions {
  skip_operation: Option<FieldSkipEncodeOperation>,
  error_if: Option<Invokable>,
}

impl FieldEncodeOptions {
  /// Returns the skip operation that should be performed if the field is skipped during encoding.
  pub const fn skip_operation(&self) -> Option<&FieldSkipEncodeOperation> {
    self.skip_operation.as_ref()
  }

  /// Returns the path to a function that determines if an error should be raised during encoding.
  pub const fn error_if(&self) -> Option<&Invokable> {
    self.error_if.as_ref()
  }
}

/// Represents the options for decoding a field.
#[derive(Debug, Default, Clone)]
pub struct FieldDecodeOptions {
  missing_operation: Option<MissingOperation>,
  then: Option<Invokable>,
  func: Option<Invokable>,
}

impl FieldDecodeOptions {
  /// Returns the missing operation that should be performed if the field is missing during decoding.
  pub const fn missing_operation(&self) -> Option<&MissingOperation> {
    self.missing_operation.as_ref()
  }

  /// Returns an invokable that should be called after decoding successfully.
  pub const fn then(&self) -> Option<&Invokable> {
    self.then.as_ref()
  }

  /// Returns an invokable that should be called for decoding.
  pub const fn func(&self) -> Option<&Invokable> {
    self.func.as_ref()
  }
}
