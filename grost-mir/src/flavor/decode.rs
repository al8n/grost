use crate::utils::BoolOption;

use super::meta::{DecodeFromMeta, DecodeValue};

impl From<DecodeFromMeta> for DecodeAttribute {
  fn from(value: DecodeFromMeta) -> Self {
    Self {
      or_else_default: value.or_else_default,
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
pub struct DecodeAttribute {
  pub(super) or_else_default: BoolOption,
  pub(super) scalar: DecodeValue,
  pub(super) bytes: DecodeValue,
  pub(super) string: DecodeValue,
  pub(super) object: DecodeValue,
  pub(super) enumeration: DecodeValue,
  pub(super) interface: DecodeValue,
  pub(super) union: DecodeValue,
  pub(super) map: DecodeValue,
  pub(super) set: DecodeValue,
  pub(super) list: DecodeValue,
}

impl DecodeAttribute {
  pub(super) fn network(_: &syn::Path) -> Self {
    let scalar = DecodeValue {
      or_else_default: BoolOption::default(),
    };
    let bytes = DecodeValue {
      or_else_default: BoolOption::yes(),
    };
    let string = DecodeValue {
      or_else_default: BoolOption::yes(),
    };
    let object = DecodeValue {
      or_else_default: BoolOption::default(),
    };
    let enumeration = DecodeValue {
      or_else_default: BoolOption::default(),
    };
    let interface = DecodeValue {
      or_else_default: BoolOption::default(),
    };
    let union = DecodeValue {
      or_else_default: BoolOption::default(),
    };
    let map = DecodeValue {
      or_else_default: BoolOption::default(),
    };
    let set = DecodeValue {
      or_else_default: BoolOption::default(),
    };
    let list = DecodeValue {
      or_else_default: BoolOption::default(),
    };

    Self {
      or_else_default: BoolOption::default(),
      scalar,
      bytes,
      string,
      object,
      enumeration,
      interface,
      union,
      map,
      set,
      list,
    }
  }

  /// Returns `true` if the encoding should skip default values
  #[inline]
  pub const fn or_else_default_scalar(&self) -> bool {
    if self.scalar.or_else_default.is_yes() {
      true
    } else {
      self.or_else_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for bytes
  #[inline]
  pub const fn or_else_default_bytes(&self) -> bool {
    if self.bytes.or_else_default.is_yes() {
      true
    } else {
      self.or_else_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for strings
  #[inline]
  pub const fn or_else_default_string(&self) -> bool {
    if self.string.or_else_default.is_yes() {
      true
    } else {
      self.or_else_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for objects
  #[inline]
  pub const fn or_else_default_object(&self) -> bool {
    if self.object.or_else_default.is_yes() {
      true
    } else {
      self.or_else_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for enumerations
  #[inline]
  pub const fn or_else_default_enumeration(&self) -> bool {
    if self.enumeration.or_else_default.is_yes() {
      true
    } else {
      self.or_else_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for interfaces
  #[inline]
  pub const fn or_else_default_interface(&self) -> bool {
    if self.interface.or_else_default.is_yes() {
      true
    } else {
      self.or_else_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for unions
  #[inline]
  pub const fn or_else_default_union(&self) -> bool {
    if self.union.or_else_default.is_yes() {
      true
    } else {
      self.or_else_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for maps
  #[inline]
  pub const fn or_else_default_map(&self) -> bool {
    if self.map.or_else_default.is_yes() {
      true
    } else {
      self.or_else_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for sets
  #[inline]
  pub const fn or_else_default_set(&self) -> bool {
    if self.set.or_else_default.is_yes() {
      true
    } else {
      self.or_else_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for lists
  #[inline]
  pub const fn or_else_default_list(&self) -> bool {
    if self.list.or_else_default.is_yes() {
      true
    } else {
      self.or_else_default.is_yes()
    }
  }
}
