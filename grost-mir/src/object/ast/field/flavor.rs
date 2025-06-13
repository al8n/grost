use syn::{Ident, Type};

pub use decode::FieldDecodeAttribute;
pub use encode::FieldEncodeAttribute;

use crate::object::meta::FieldFlavorFromMeta;

mod decode;
mod encode;

#[derive(Debug, Clone)]
pub struct FieldFlavorAttribute {
  pub(super) name: Ident,
  pub(super) ty: Option<Type>,
  pub(super) format: Option<Type>,
  pub(super) encode: FieldEncodeAttribute,
  pub(super) decode: FieldDecodeAttribute,
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
    }
  }
}

impl FieldFlavorFromMeta {
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
        })
      })
      .collect()
  }
}
