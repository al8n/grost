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
}

impl DecodeOptions {
  #[inline]
  pub const fn or_default_by_label(&self, label: &Label) -> bool {
    match label {
      Label::Scalar => self.scalar.or_default.is_yes(),
      Label::Bytes => self.bytes.or_default.is_yes(),
      Label::String => self.string.or_default.is_yes(),
      Label::Object => self.object.or_default.is_yes(),
      Label::Enum => self.enumeration.or_default.is_yes(),
      Label::Interface => self.interface.or_default.is_yes(),
      Label::Union => self.union.or_default.is_yes(),
      Label::Map { .. } => self.map.or_default.is_yes(),
      Label::Set(_) => self.set.or_default.is_yes(),
      Label::List(_) => self.list.or_default.is_yes(),
      Label::Optional(_) => false,
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
