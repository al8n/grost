use quote::format_ident;
use syn::{Attribute, Ident};

use crate::object::meta::PartialDecodedObjectFromMeta;

impl PartialDecodedObjectFromMeta {
  pub(super) fn finalize(self) -> PartialDecodedObjectAttribute {
    PartialDecodedObjectAttribute {
      name: self.name,
      attrs: self.attrs,
      copy: self.copy,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialDecodedObjectAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  copy: bool,
}

impl PartialDecodedObjectAttribute {
  /// Returns the name of the partial reference object
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the partial reference object
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial reference object is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }
}

#[derive(Debug, Clone)]
pub(in crate::object) struct ConcretePartialDecodedObject {
  name: Ident,
  attrs: Vec<Attribute>,
  copy: bool,
}

impl ConcretePartialDecodedObject {
  pub(super) fn from_attribute(
    object_name: &Ident,
    attribute: &PartialDecodedObjectAttribute,
  ) -> darling::Result<Self> {
    let name = if let Some(name) = attribute.name() {
      name.clone()
    } else {
      format_ident!("PartialDecoded{}", object_name)
    };

    Ok(Self {
      name,
      attrs: attribute.attrs().to_vec(),
      copy: attribute.copy(),
    })
  }

  /// Returns the name of the concrete partial decoded object
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the concrete partial decoded object
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the concrete partial decoded object is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }
}

#[derive(Debug, Clone)]
pub struct GenericPartialDecodedObject {
  name: Ident,
  attrs: Vec<Attribute>,
  copy: bool,
}

impl GenericPartialDecodedObject {
  pub(super) fn from_attribute(
    object_name: &Ident,
    attribute: &PartialDecodedObjectAttribute,
  ) -> darling::Result<Self> {
    let name = if let Some(name) = attribute.name() {
      name.clone()
    } else {
      format_ident!("PartialDecoded{}", object_name)
    };

    Ok(Self {
      name,
      attrs: attribute.attrs().to_vec(),
      copy: attribute.copy(),
    })
  }

  /// Returns the name of the generic partial decoded object
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the generic partial decoded object
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the generic partial decoded object is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }
}
