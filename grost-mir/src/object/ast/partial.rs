use quote::format_ident;
use syn::{Attribute, Ident};

use crate::object::meta::PartialObjectFromMeta;

impl PartialObjectFromMeta {
  pub(super) fn finalize(self) -> PartialObjectAttribute {
    PartialObjectAttribute {
      name: self.name,
      attrs: self.attrs,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialObjectAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
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
}

#[derive(Debug, Clone)]
pub struct PartialObject {
  name: Ident,
  attrs: Vec<Attribute>,
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

  pub(super) fn from_attribute(
    name: &Ident,
    attribute: &PartialObjectAttribute,
  ) -> darling::Result<Self> {
    let name = if let Some(name) = attribute.name() {
      name.clone()
    } else {
      format_ident!("Partial{}", name)
    };

    Ok(Self {
      name,
      attrs: attribute.attrs().to_vec(),
    })
  }
}
