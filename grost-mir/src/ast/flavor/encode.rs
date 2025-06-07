use quote::quote;
use syn::{Path, parse2};

use crate::meta::{
  BoolOption,
  flavor::{EncodeFromMeta, EncodeValue},
};

impl From<EncodeFromMeta> for EncodeAttribute {
  fn from(value: EncodeFromMeta) -> Self {
    Self {
      skip_default: value.skip_default,
      scalar: value.scalar,
      bytes: value.bytes,
      string: value.string,
      object: value.object,
      enumeration: value.enumeration,
      interface: value.interface,
      union: value.union,
      map: value.map,
      set: value.set,
      list: value.list,
    }
  }
}

#[derive(Debug, Clone)]
pub struct EncodeAttribute {
  pub(super) skip_default: BoolOption,
  pub(super) scalar: EncodeValue,
  pub(super) bytes: EncodeValue,
  pub(super) string: EncodeValue,
  pub(super) object: EncodeValue,
  pub(super) enumeration: EncodeValue,
  pub(super) interface: EncodeValue,
  pub(super) union: EncodeValue,
  pub(super) map: EncodeValue,
  pub(super) set: EncodeValue,
  pub(super) list: EncodeValue,
}

impl TryFrom<&Path> for EncodeAttribute {
  type Error = syn::Error;

  fn try_from(module: &Path) -> Result<Self, Self::Error> {
    let scalar = quote! { #module::scalar };
    let bytes = quote! { #module::bytes };
    let string = quote! { #module::string };
    let object = quote! { #module::object };
    let enum_ = quote! { #module::enumeration };
    let union = quote! { #module::union };
    let interface = quote! { #module::interface };
    let map = quote! { #module::map };
    let set = quote! { #module::set };
    let list = quote! { #module::list };

    Ok(Self {
      skip_default: BoolOption::default(),
      scalar: EncodeValue::from(parse2::<Path>(scalar)?),
      bytes: EncodeValue::from(parse2::<Path>(bytes)?),
      string: EncodeValue::from(parse2::<Path>(string)?),
      object: EncodeValue::from(parse2::<Path>(object)?),
      enumeration: EncodeValue::from(parse2::<Path>(enum_)?),
      interface: EncodeValue::from(parse2::<Path>(interface)?),
      union: EncodeValue::from(parse2::<Path>(union)?),
      map: EncodeValue::from(parse2::<Path>(map)?),
      set: EncodeValue::from(parse2::<Path>(set)?),
      list: EncodeValue::from(parse2::<Path>(list)?),
    })
  }
}

impl EncodeAttribute {
  pub(super) fn network(path_to_grost: &Path) -> syn::Result<Self> {
    Self::try_from(path_to_grost)
  }

  /// Returns the path to the encoding module of scalar types
  #[inline]
  pub const fn scalar(&self) -> Option<&Path> {
    self.scalar.path.as_ref()
  }

  /// Returns `true` if the encoding should skip default values
  #[inline]
  pub const fn skip_default_scalar(&self) -> bool {
    if self.scalar.skip_default.is_yes() {
      true
    } else {
      self.skip_default.is_yes()
    }
  }

  /// Returns the path to the encoding module of bytes types
  #[inline]
  pub const fn bytes(&self) -> Option<&Path> {
    self.bytes.path.as_ref()
  }

  /// Returns `true` if the encoding should skip default values for bytes
  #[inline]
  pub const fn skip_default_bytes(&self) -> bool {
    if self.bytes.skip_default.is_yes() {
      true
    } else {
      self.skip_default.is_yes()
    }
  }

  /// Returns the path to the encoding module of string types
  #[inline]
  pub const fn string(&self) -> Option<&Path> {
    self.string.path.as_ref()
  }

  /// Returns `true` if the encoding should skip default values for strings
  #[inline]
  pub const fn skip_default_string(&self) -> bool {
    if self.string.skip_default.is_yes() {
      true
    } else {
      self.skip_default.is_yes()
    }
  }

  /// Returns the path to the encoding module of object types
  #[inline]
  pub const fn object(&self) -> Option<&Path> {
    self.object.path.as_ref()
  }

  /// Returns `true` if the encoding should skip default values for objects
  #[inline]
  pub const fn skip_default_object(&self) -> bool {
    if self.object.skip_default.is_yes() {
      true
    } else {
      self.skip_default.is_yes()
    }
  }

  /// Returns the path to the encoding module of enumeration types
  #[inline]
  pub const fn enumeration(&self) -> Option<&Path> {
    self.enumeration.path.as_ref()
  }

  /// Returns `true` if the encoding should skip default values for enumerations
  #[inline]
  pub const fn skip_default_enumeration(&self) -> bool {
    if self.enumeration.skip_default.is_yes() {
      true
    } else {
      self.skip_default.is_yes()
    }
  }

  /// Returns the path to the encoding module of interface types
  #[inline]
  pub const fn interface(&self) -> Option<&Path> {
    self.interface.path.as_ref()
  }

  /// Returns `true` if the encoding should skip default values for interfaces
  #[inline]
  pub const fn skip_default_interface(&self) -> bool {
    if self.interface.skip_default.is_yes() {
      true
    } else {
      self.skip_default.is_yes()
    }
  }

  /// Returns the path to the encoding module of union types
  #[inline]
  pub const fn union(&self) -> Option<&Path> {
    self.union.path.as_ref()
  }

  /// Returns `true` if the encoding should skip default values for unions
  #[inline]
  pub const fn skip_default_union(&self) -> bool {
    if self.union.skip_default.is_yes() {
      true
    } else {
      self.skip_default.is_yes()
    }
  }

  /// Returns the path to the encoding module of map types
  #[inline]
  pub const fn map(&self) -> Option<&Path> {
    self.map.path.as_ref()
  }

  /// Returns `true` if the encoding should skip default values for maps
  #[inline]
  pub const fn skip_default_map(&self) -> bool {
    if self.map.skip_default.is_yes() {
      true
    } else {
      self.skip_default.is_yes()
    }
  }

  /// Returns the path to the encoding module of set types
  #[inline]
  pub const fn set(&self) -> Option<&Path> {
    self.set.path.as_ref()
  }

  /// Returns `true` if the encoding should skip default values for sets
  #[inline]
  pub const fn skip_default_set(&self) -> bool {
    if self.set.skip_default.is_yes() {
      true
    } else {
      self.skip_default.is_yes()
    }
  }

  /// Returns the path to the encoding module of list types
  #[inline]
  pub const fn list(&self) -> Option<&Path> {
    self.list.path.as_ref()
  }

  /// Returns `true` if the encoding should skip default values for lists
  #[inline]
  pub const fn skip_default_list(&self) -> bool {
    if self.list.skip_default.is_yes() {
      true
    } else {
      self.skip_default.is_yes()
    }
  }
}
