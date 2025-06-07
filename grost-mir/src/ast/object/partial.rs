use quote::format_ident;
use syn::{Attribute, Ident, TypeParam};

use crate::meta::object::partial::PartialObjectFromMeta;

impl PartialObjectFromMeta {
  pub(super) fn finalize(self, unknown_buffer_generic: TypeParam) -> PartialObjectAttribute {
    PartialObjectAttribute {
      name: self.name,
      attrs: self.attrs,
      unknown_buffer_generic,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialObjectAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  unknown_buffer_generic: TypeParam,
}

impl PartialObjectAttribute {
  /// Returns the name of the partial object
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the partial object
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the unknown buffer generic parameter
  pub const fn unknown_buffer(&self) -> &TypeParam {
    &self.unknown_buffer_generic
  }
}

#[derive(Debug, Clone)]
pub struct PartialObject {
  name: Ident,
  attrs: Vec<Attribute>,
  unknown_buffer_generic: TypeParam,
}

impl PartialObject {
  /// Returns the name of the partial object
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the partial object
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the unknown buffer generic parameter
  pub const fn unknown_buffer(&self) -> &TypeParam {
    &self.unknown_buffer_generic
  }

  pub(super) fn from_attribute(
    name: &Ident,
    attribute: &PartialObjectAttribute,
  ) -> darling::Result<Self> {
    let name = if let Some(name) = &attribute.name {
      name.clone()
    } else {
      format_ident!("Partial{}", name)
    };

    Ok(Self {
      name,
      attrs: attribute.attrs().to_vec(),
      unknown_buffer_generic: attribute.unknown_buffer().clone(),
    })
  }
}
