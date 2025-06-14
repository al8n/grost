use quote::format_ident;
use syn::{Attribute, Ident};

use crate::object::meta::{SelectorFromMeta, SelectorIterFromMeta};

impl SelectorIterFromMeta {
  pub(super) fn finalize(self) -> SelectorIterAttribute {
    SelectorIterAttribute {
      name: self.name,
      attrs: self.attrs,
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct SelectorIterAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
}

impl SelectorIterAttribute {
  /// Returns the name of the selector iterator
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the selector iterator
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}

impl SelectorFromMeta {
  pub(super) fn finalize(self) -> SelectorAttribute {
    SelectorAttribute {
      name: self.name,
      attrs: self.attrs,
    }
  }
}

#[derive(Debug, Clone)]
pub struct SelectorAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
}

impl SelectorAttribute {
  /// Returns the name of the selector
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the selector
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}

#[derive(Debug, Clone)]
pub struct ConcreteSelectorIter {
  name: Ident,
  attrs: Vec<Attribute>,
}

impl ConcreteSelectorIter {
  /// Returns the name of the concrete selector iterator
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the concrete selector iterator
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  pub(super) fn from_attribute(
    selector_name: &Ident,
    attribute: &SelectorIterAttribute,
  ) -> darling::Result<Self> {
    let name = if let Some(name) = attribute.name() {
      name.clone()
    } else {
      format_ident!("{selector_name}Iter")
    };

    Ok(Self {
      name,
      attrs: attribute.attrs().to_vec(),
    })
  }
}

#[derive(Debug, Clone)]
pub struct ConcreteSelector {
  name: Ident,
  attrs: Vec<Attribute>,
}

impl ConcreteSelector {
  /// Returns the name of the concrete selector
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the concrete selector
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  pub(super) fn from_attribute(
    name: &Ident,
    attribute: &SelectorAttribute,
  ) -> darling::Result<Self> {
    let name = if let Some(name) = &attribute.name {
      name.clone()
    } else {
      format_ident!("{name}Selector")
    };

    Ok(Self {
      name,
      attrs: attribute.attrs().to_vec(),
    })
  }
}

#[derive(Debug, Clone)]
pub struct GenericSelector {
  name: Ident,
  attrs: Vec<Attribute>,
}

impl GenericSelector {
  /// Returns the name of the generic selector
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the generic selector
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  pub(super) fn from_attribute(
    name: &Ident,
    attribute: &SelectorAttribute,
  ) -> darling::Result<Self> {
    let name = if let Some(name) = attribute.name() {
      name.clone()
    } else {
      format_ident!("{name}Selector")
    };

    Ok(Self {
      name,
      attrs: attribute.attrs().to_vec(),
    })
  }
}

#[derive(Debug, Clone)]
pub struct GenericSelectorIter {
  name: Ident,
  attrs: Vec<Attribute>,
}

impl GenericSelectorIter {
  /// Returns the name of the generic selector iterator
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the generic selector iterator
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  pub(super) fn from_attribute(
    selector_name: &Ident,
    attribute: &SelectorIterAttribute,
  ) -> darling::Result<Self> {
    let name = if let Some(name) = attribute.name() {
      name.clone()
    } else {
      format_ident!("{selector_name}Iter")
    };

    Ok(Self {
      name,
      attrs: attribute.attrs().to_vec(),
    })
  }
}
