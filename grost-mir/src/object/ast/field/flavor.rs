use syn::{Ident, Path, Type};

pub use decode::FieldDecodeAttribute;
pub use encode::FieldEncodeAttribute;

use crate::{
  object::{ConvertAttribute, meta::GenericFieldFlavorFromMeta},
  utils::{Invokable, MissingOperation},
};

mod decode;
mod encode;

#[derive(Debug, Clone)]
pub struct FieldFlavorAttribute {
  pub(in crate::object) name: Ident,
  pub(in crate::object) ty: Option<Type>,
  pub(in crate::object) format: Option<Type>,
  pub(in crate::object) encode: FieldEncodeAttribute,
  pub(in crate::object) decode: FieldDecodeAttribute,
  pub(in crate::object) convert: ConvertAttribute,
}

impl FieldFlavorAttribute {
  /// Returns the name of the flavor.
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the wire format type for the field of this flavor, if specified.
  pub const fn format(&self) -> Option<&syn::Type> {
    self.format.as_ref()
  }

  /// Returns the type of the partial decoded field for this flavor, if specified.
  pub const fn ty(&self) -> Option<&syn::Type> {
    self.ty.as_ref()
  }

  /// Returns the encode attribute for this flavor.
  pub const fn encode(&self) -> &FieldEncodeAttribute {
    &self.encode
  }

  /// Returns the decode attribute for this flavor.
  pub const fn decode(&self) -> &FieldDecodeAttribute {
    &self.decode
  }

  pub(crate) fn new(
    name: Ident,
    ty: Option<syn::Type>,
    format: Option<syn::Type>,
    encode: FieldEncodeAttribute,
    decode: FieldDecodeAttribute,
  ) -> Self {
    Self {
      name,
      ty,
      format,
      encode,
      decode,
      convert: ConvertAttribute::default(),
    }
  }
}

impl GenericFieldFlavorFromMeta {
  pub fn finalize(self) -> darling::Result<Vec<FieldFlavorAttribute>> {
    self
      .flavors
      .into_iter()
      .map(|(name, value)| {
        let ty = value.ty;
        let format = value.format;
        let encode = value.encode.finalize()?;
        let decode = value.decode.finalize()?;

        Ok(FieldFlavorAttribute {
          name,
          ty,
          format,
          encode,
          decode,
          convert: value.convert.finalize()?,
        })
      })
      .collect()
  }
}

#[derive(Debug, Clone)]
pub struct FieldEncodeFlavor {
  pub(in crate::object) skip_default: bool,
  pub(in crate::object) skip_if: Option<Path>,
  pub(in crate::object) error_if: Option<Path>,
}

impl FieldEncodeFlavor {
  /// Returns whether the field should be skipped if it has a default value
  #[inline]
  pub const fn skip_default(&self) -> bool {
    self.skip_default
  }

  /// Returns the path to the condition that determines if the field should be skipped
  #[inline]
  pub const fn skip_if(&self) -> Option<&Path> {
    self.skip_if.as_ref()
  }

  /// Returns the path to the condition that determines if an error should be raised
  #[inline]
  pub const fn error_if(&self) -> Option<&Path> {
    self.error_if.as_ref()
  }
}

#[derive(Debug, Clone)]
pub struct FieldDecodeFlavor {
  pub(in crate::object) missing_operation: Option<MissingOperation>,
  pub(in crate::object) then: Option<Invokable>,
}

impl FieldDecodeFlavor {
  /// Returns the operation to perform if the field is missing
  #[inline]
  pub const fn missing_operation(&self) -> Option<&MissingOperation> {
    self.missing_operation.as_ref()
  }

  /// Returns the path to the condition that determines if an error should be raised
  #[inline]
  pub const fn then(&self) -> Option<&Invokable> {
    self.then.as_ref()
  }
}

#[derive(Debug, Clone)]
pub struct FieldFlavor {
  pub(in crate::object) ty: Option<Type>,
  pub(in crate::object) format: Option<Type>,
  pub(in crate::object) flavor_type: Type,
  pub(in crate::object) encode: FieldEncodeFlavor,
  pub(in crate::object) decode: FieldDecodeFlavor,
  pub(in crate::object) convert: ConvertAttribute,
}

impl FieldFlavor {
  /// Returns the wire format type for the field of this flavor, if specified
  #[inline]
  pub const fn format(&self) -> Option<&Type> {
    self.format.as_ref()
  }

  /// Returns the type of the partial decoded field for this flavor, if specified
  #[inline]
  pub const fn ty(&self) -> Option<&Type> {
    self.ty.as_ref()
  }

  /// Returns the type of the flavor
  #[inline]
  pub const fn flavor_type(&self) -> &Type {
    &self.flavor_type
  }

  /// Returns the encode attribute for this flavor
  #[inline]
  pub const fn encode(&self) -> &FieldEncodeFlavor {
    &self.encode
  }

  /// Returns the decode attribute for this flavor
  #[inline]
  pub const fn decode(&self) -> &FieldDecodeFlavor {
    &self.decode
  }
}
