use crate::{object::Label, utils::BoolOption};

use super::meta::{DecodeFromMeta, DecodeValue};

impl From<DecodeFromMeta> for DecodeOptions {
  fn from(value: DecodeFromMeta) -> Self {
    Self {
      or_default: value.or_default,
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
      generic: value.generic,
    }
  }
}

#[derive(Debug, Clone)]
pub struct DecodeOptions {
  pub(super) or_default: BoolOption,
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
  pub(super) generic: DecodeValue,
}

impl DecodeOptions {
  #[inline]
  pub const fn or_default_by_label(&self, label: &Label) -> bool {
    match label {
      Label::Scalar(_) => self.scalar.or_default.is_yes(),
      Label::Bytes(_) => self.bytes.or_default.is_yes(),
      Label::String(_) => self.string.or_default.is_yes(),
      Label::Object(_) => self.object.or_default.is_yes(),
      Label::Enum(_) => self.enumeration.or_default.is_yes(),
      Label::Interface(_) => self.interface.or_default.is_yes(),
      Label::Union(_) => self.union.or_default.is_yes(),
      Label::Map { .. } => self.map.or_default.is_yes(),
      Label::Set(_) => self.set.or_default.is_yes(),
      Label::List(_) => self.list.or_default.is_yes(),
      Label::Generic(_) => self.generic.or_default.is_yes(),
      Label::Nullable(_) => false,
    }
  }

  /// Returns `true` if there is a global `or_default` option set
  #[inline]
  pub const fn or_default(&self) -> bool {
    self.or_default.is_yes()
  }

  /// Returns `true` if the encoding should skip default values
  #[inline]
  pub const fn or_default_scalar(&self) -> bool {
    if self.scalar.or_default.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for bytes
  #[inline]
  pub const fn or_default_bytes(&self) -> bool {
    if self.bytes.or_default.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for strings
  #[inline]
  pub const fn or_default_string(&self) -> bool {
    if self.string.or_default.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for generics
  #[inline]
  pub const fn or_default_generic(&self) -> bool {
    if self.generic.or_default.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for objects
  #[inline]
  pub const fn or_default_object(&self) -> bool {
    if self.object.or_default.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for enumerations
  #[inline]
  pub const fn or_default_enumeration(&self) -> bool {
    if self.enumeration.or_default.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for interfaces
  #[inline]
  pub const fn or_default_interface(&self) -> bool {
    if self.interface.or_default.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for unions
  #[inline]
  pub const fn or_default_union(&self) -> bool {
    if self.union.or_default.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for maps
  #[inline]
  pub const fn or_default_map(&self) -> bool {
    if self.map.or_default.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for sets
  #[inline]
  pub const fn or_default_set(&self) -> bool {
    if self.set.or_default.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for lists
  #[inline]
  pub const fn or_default_list(&self) -> bool {
    if self.list.or_default.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }
}
