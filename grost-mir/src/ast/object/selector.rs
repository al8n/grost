use quote::format_ident;
use syn::{Attribute, Ident, Type, TypeParam};

use crate::meta::object::selector::{SelectorFromMeta, SelectorIterFromMeta};

impl SelectorIterFromMeta {
  pub(super) fn finalize(self, flavor_param: Option<TypeParam>) -> SelectorIterAttribute {
    SelectorIterAttribute {
      name: self.name,
      attrs: self.attrs,
      flavor_param,
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct SelectorIterAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  flavor_param: Option<TypeParam>,
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

  /// Returns the flavor generic parameter if it exists
  pub const fn flavor(&self) -> Option<&TypeParam> {
    self.flavor_param.as_ref()
  }
}

impl SelectorFromMeta {
  pub(super) fn finalize(
    self,
    flavor_param: Option<TypeParam>,
    wire_format: TypeParam,
  ) -> SelectorAttribute {
    SelectorAttribute {
      name: self.name,
      attrs: self.attrs,
      flavor_param,
      wire_format_param: wire_format,
    }
  }
}

#[derive(Debug, Clone)]
pub struct SelectorAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  flavor_param: Option<TypeParam>,
  wire_format_param: TypeParam,
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

  /// Returns the flavor generic parameter if it exists
  pub const fn flavor(&self) -> Option<&TypeParam> {
    self.flavor_param.as_ref()
  }

  /// Returns the wire format generic parameter
  pub const fn wire_format(&self) -> &TypeParam {
    &self.wire_format_param
  }
}

#[derive(Debug, Clone)]
pub struct ConcreteSelectorIter {
  name: Ident,
  attrs: Vec<Attribute>,
  flavor_type: Type,
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

  /// Returns the flavor type of the concrete selector iterator
  pub const fn flavor_type(&self) -> &Type {
    &self.flavor_type
  }

  pub(super) fn from_attribute(
    selector_name: &Ident,
    flavor_type: &Type,
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
      flavor_type: flavor_type.clone(),
    })
  }
}

#[derive(Debug, Clone)]
pub struct ConcreteSelector {
  name: Ident,
  attrs: Vec<Attribute>,
  flavor_type: Type,
  wire_format_param: TypeParam,
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

  /// Returns the flavor type of the concrete selector
  pub const fn flavor_type(&self) -> &Type {
    &self.flavor_type
  }

  /// Returns the wire format generic parameter
  pub const fn wire_format(&self) -> &TypeParam {
    &self.wire_format_param
  }

  pub(super) fn from_attribute(
    name: &Ident,
    flavor_type: &Type,
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
      flavor_type: flavor_type.clone(),
      wire_format_param: attribute.wire_format().clone(),
    })
  }
}

#[derive(Debug, Clone)]
pub struct GenericSelector {
  name: Ident,
  attrs: Vec<Attribute>,
  flavor_param: TypeParam,
  wire_format: TypeParam,
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

  /// Returns the generic flavor parameter of the generic selector
  pub const fn flavor(&self) -> &TypeParam {
    &self.flavor_param
  }

  /// Returns the wire format generic parameter
  pub const fn wire_format(&self) -> &TypeParam {
    &self.wire_format
  }

  pub(super) fn from_attribute(
    name: &Ident,
    flavor_param: &TypeParam,
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
      flavor_param: flavor_param.clone(),
      wire_format: attribute.wire_format().clone(),
    })
  }
}

#[derive(Debug, Clone)]
pub struct GenericSelectorIter {
  name: Ident,
  attrs: Vec<Attribute>,
  flavor_param: TypeParam,
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

  /// Returns the generic flavor parameter of the generic selector iterator
  pub const fn flavor(&self) -> &TypeParam {
    &self.flavor_param
  }

  pub(super) fn from_attribute(
    name: &Ident,
    flavor_param: &TypeParam,
    attribute: &SelectorIterAttribute,
  ) -> darling::Result<Self> {
    let name = if let Some(name) = attribute.name() {
      name.clone()
    } else {
      format_ident!("{name}SelectorIter")
    };

    Ok(Self {
      name,
      attrs: attribute.attrs().to_vec(),
      flavor_param: flavor_param.clone(),
    })
  }
}
