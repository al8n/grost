use darling::FromMeta;

use crate::{object::Label, utils::BoolOption};

#[derive(Debug, Default, Clone, PartialEq, Eq, FromMeta)]
pub struct OrDefault {
  #[darling(default)]
  pub(crate) or_default: BoolOption,
  #[darling(default)]
  pub(crate) or_default_scalar: BoolOption,
  #[darling(default)]
  pub(crate) or_default_bytes: BoolOption,
  #[darling(default)]
  pub(crate) or_default_string: BoolOption,
  #[darling(default)]
  pub(crate) or_default_object: BoolOption,
  #[darling(default)]
  pub(crate) or_default_enum: BoolOption,
  #[darling(default)]
  pub(crate) or_default_interface: BoolOption,
  #[darling(default)]
  pub(crate) or_default_union: BoolOption,
  #[darling(default)]
  pub(crate) or_default_map: BoolOption,
  #[darling(default)]
  pub(crate) or_default_set: BoolOption,
  #[darling(default)]
  pub(crate) or_default_list: BoolOption,
  #[darling(default)]
  pub(crate) or_default_generic: BoolOption,
}

impl OrDefault {
  /// Returns `true` if the encoding should skip default values
  #[inline]
  pub const fn or_default_scalar(&self) -> bool {
    if self.or_default_scalar.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for or_default_bytes
  #[inline]
  pub const fn or_default_bytes(&self) -> bool {
    if self.or_default_bytes.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for or_default_strings
  #[inline]
  pub const fn or_default_string(&self) -> bool {
    if self.or_default_string.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for or_default_objects
  #[inline]
  pub const fn or_default_object(&self) -> bool {
    if self.or_default_object.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for or_default_enums
  #[inline]
  pub const fn or_default_enum(&self) -> bool {
    if self.or_default_enum.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for or_default_interfaces
  #[inline]
  pub const fn or_default_interface(&self) -> bool {
    if self.or_default_interface.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for or_default_unions
  #[inline]
  pub const fn or_default_union(&self) -> bool {
    if self.or_default_union.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for or_default_maps
  #[inline]
  pub const fn or_default_map(&self) -> bool {
    if self.or_default_map.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for or_default_sets
  #[inline]
  pub const fn or_default_set(&self) -> bool {
    if self.or_default_set.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  /// Returns `true` if the encoding should skip default values for or_default_lists
  #[inline]
  pub const fn or_default_list(&self) -> bool {
    if self.or_default_list.is_yes() {
      true
    } else {
      self.or_default.is_yes()
    }
  }

  #[inline]
  pub const fn or_default_by_label(&self, label: &Label) -> bool {
    match label {
      Label::Scalar(_) => self.or_default_scalar(),
      Label::Bytes(_) => self.or_default_bytes(),
      Label::String(_) => self.or_default_string(),
      Label::Object(_) => self.or_default_object(),
      Label::Enum(_) => self.or_default_enum(),
      Label::Interface(_) => self.or_default_interface(),
      Label::Union(_) => self.or_default_union(),
      Label::Map { .. } => self.or_default_map(),
      Label::Set(_) => self.or_default_set(),
      Label::List(_) => self.or_default_list(),
      _ => true,
    }
  }
}
