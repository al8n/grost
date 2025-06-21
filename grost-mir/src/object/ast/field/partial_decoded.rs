use syn::{Attribute, Type};

use crate::object::{
  FieldDecodeAttribute, FieldEncodeAttribute, meta::PartialDecodedFieldFromMeta,
};

impl PartialDecodedFieldFromMeta {
  /// Finalizes the partial decoded field meta and returns the attribute
  pub fn finalize(self) -> darling::Result<PartialDecodedFieldOptions> {
    Ok(PartialDecodedFieldOptions {
      copy: self.copy,
      attrs: self.attrs,
      ty: self.ty,
      encode: self.encode.finalize()?,
      decode: self.decode.finalize()?,
    })
  }
}

#[derive(Debug, Clone)]
pub struct PartialDecodedFieldOptions {
  pub(crate) copy: bool,
  pub(crate) attrs: Vec<Attribute>,
  pub(crate) ty: Option<Type>,
  pub(crate) encode: FieldEncodeAttribute,
  pub(crate) decode: FieldDecodeAttribute,
}

impl PartialDecodedFieldOptions {
  /// Returns the attributes of the partial reference object field
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial reference object field is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the type of the partial decoded object field
  pub const fn ty(&self) -> Option<&Type> {
    self.ty.as_ref()
  }

  /// Returns the encode attribute of the partial decoded object field
  pub const fn encode(&self) -> &FieldEncodeAttribute {
    &self.encode
  }

  /// Returns the decode attribute of the partial decoded object field
  pub const fn decode(&self) -> &FieldDecodeAttribute {
    &self.decode
  }
}
