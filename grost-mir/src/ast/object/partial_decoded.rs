use quote::format_ident;
use syn::{Attribute, Ident, LifetimeParam, Type, TypeParam};

use crate::meta::object::partial_decoded::PartialDecodedObjectFromMeta;

impl PartialDecodedObjectFromMeta {
  pub(super) fn finalize(
    self,
    flavor_param: Option<syn::TypeParam>,
    unknown_buffer_param: syn::TypeParam,
    lifetime_param: syn::LifetimeParam,
  ) -> PartialDecodedObjectAttribute {
    PartialDecodedObjectAttribute {
      name: self.name,
      attrs: self.attrs,
      copy: self.copy,
      flavor_param,
      unknown_buffer_param,
      lifetime_param,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialDecodedObjectAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  copy: bool,
  flavor_param: Option<syn::TypeParam>,
  unknown_buffer_param: syn::TypeParam,
  lifetime_param: syn::LifetimeParam,
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

  /// Returns the flavor generic parameter if it exists
  pub const fn flavor(&self) -> Option<&syn::TypeParam> {
    self.flavor_param.as_ref()
  }

  /// Returns the unknown buffer generic parameter
  pub const fn unknown_buffer(&self) -> &syn::TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns the lifetime generic parameter
  pub const fn lifetime(&self) -> &syn::LifetimeParam {
    &self.lifetime_param
  }
}

#[derive(Debug, Clone)]
pub struct ConcretePartialDecodedObject {
  name: Ident,
  attrs: Vec<Attribute>,
  copy: bool,
  flavor_type: Type,
  unknown_buffer_param: TypeParam,
  lifetime_param: LifetimeParam,
}

impl ConcretePartialDecodedObject {
  pub(super) fn from_attribute(
    object_name: &Ident,
    flavor_type: Type,
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
      flavor_type,
      unknown_buffer_param: attribute.unknown_buffer().clone(),
      lifetime_param: attribute.lifetime().clone(),
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

  /// Returns the flavor type of the concrete partial decoded object
  pub const fn flavor_type(&self) -> &Type {
    &self.flavor_type
  }

  /// Returns the unknown buffer type generic parameter of the concrete partial decoded object
  pub const fn unknown_buffer(&self) -> &TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns the lifetime generic parameter of the concrete partial decoded object
  pub const fn lifetime(&self) -> &LifetimeParam {
    &self.lifetime_param
  }
}

#[derive(Debug, Clone)]
pub struct GenericPartialDecodedObject {
  name: Ident,
  attrs: Vec<Attribute>,
  copy: bool,
  flavor_param: TypeParam,
  unknown_buffer_param: TypeParam,
  lifetime_param: LifetimeParam,
}

impl GenericPartialDecodedObject {
  pub(super) fn from_attribute(
    object_name: &Ident,
    flavor_param: &TypeParam,
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
      flavor_param: flavor_param.clone(),
      unknown_buffer_param: attribute.unknown_buffer().clone(),
      lifetime_param: attribute.lifetime().clone(),
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

  /// Returns the flavor type parameter of the generic partial decoded object
  pub const fn flavor(&self) -> &TypeParam {
    &self.flavor_param
  }

  /// Returns the unknown buffer type parameter of the generic partial decoded object
  pub const fn unknown_buffer(&self) -> &TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns the lifetime parameter of the generic partial decoded object
  pub const fn lifetime(&self) -> &LifetimeParam {
    &self.lifetime_param
  }
}
