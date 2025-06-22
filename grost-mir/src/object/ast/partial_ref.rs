use quote::format_ident;
use syn::{Attribute, Ident};

use crate::object::meta::PartialRefObjectFromMeta;

impl PartialRefObjectFromMeta {
  pub(super) fn finalize(self) -> PartialRefObjectAttribute {
    PartialRefObjectAttribute {
      name: self.name,
      attrs: self.attrs,
      copy: self.copy,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialRefObjectAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  copy: bool,
}

impl PartialRefObjectAttribute {
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
pub(in crate::object) struct ConcretePartialRefObject {
  name: Ident,
  attrs: Vec<Attribute>,
  copy: bool,
}

impl ConcretePartialRefObject {
  pub(super) fn from_attribute(
    object_name: &Ident,
    attribute: &PartialRefObjectAttribute,
  ) -> darling::Result<Self> {
    let name = if let Some(name) = attribute.name() {
      name.clone()
    } else {
      format_ident!("PartialRef{}", object_name)
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
pub struct GenericPartialRefObject {
  name: Ident,
  attrs: Vec<Attribute>,
  copy: bool,
}

impl GenericPartialRefObject {
  pub(super) fn from_attribute(
    object_name: &Ident,
    attribute: &PartialRefObjectAttribute,
  ) -> darling::Result<Self> {
    let name = if let Some(name) = attribute.name() {
      name.clone()
    } else {
      format_ident!("PartialRef{}", object_name)
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
