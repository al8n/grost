use darling::FromMeta;

use crate::utils::BoolOption;

#[derive(Debug, Default, Clone, PartialEq, Eq, darling::FromMeta)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub(super) struct DecodeValueParser {
  #[darling(default)]
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "BoolOption::is_none")
  )]
  or_default: BoolOption,
}

impl From<DecodeValueParser> for DecodeValue {
  fn from(DecodeValueParser { or_default }: DecodeValueParser) -> Self {
    Self { or_default }
  }
}

impl From<DecodeValue> for DecodeValueParser {
  fn from(DecodeValue { or_default }: DecodeValue) -> Self {
    Self { or_default }
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(
  feature = "serde",
  serde(
    try_from = "super::serde::DecodeValueSerdeHelper",
    into = "super::serde::DecodeValueSerdeHelper"
  )
)]
pub(crate) struct DecodeValue {
  pub(crate) or_default: BoolOption,
}

impl FromMeta for DecodeValue {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    DecodeValueParser::from_list(items).map(Into::into)
  }
}

#[derive(Debug, FromMeta)]
pub(super) struct DecodeParser {
  #[darling(default)]
  or_default: BoolOption,
  #[darling(default)]
  scalar: DecodeValue,
  #[darling(default)]
  bytes: DecodeValue,
  #[darling(default)]
  string: DecodeValue,
  #[darling(default)]
  object: DecodeValue,
  #[darling(default, rename = "enum")]
  enumeration: DecodeValue,
  #[darling(default)]
  interface: DecodeValue,
  #[darling(default)]
  union: DecodeValue,
  #[darling(default)]
  map: DecodeValue,
  #[darling(default)]
  set: DecodeValue,
  #[darling(default)]
  list: DecodeValue,
  #[darling(default)]
  generic: DecodeValue,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(
  feature = "serde",
  serde(
    try_from = "super::serde::DecodeSerdeHelper",
    into = "super::serde::DecodeSerdeHelper"
  )
)]
pub struct DecodeFromMeta {
  pub(crate) or_default: BoolOption,
  pub(crate) scalar: DecodeValue,
  pub(crate) bytes: DecodeValue,
  pub(crate) string: DecodeValue,
  pub(crate) object: DecodeValue,
  pub(crate) enumeration: DecodeValue,
  pub(crate) interface: DecodeValue,
  pub(crate) union: DecodeValue,
  pub(crate) map: DecodeValue,
  pub(crate) set: DecodeValue,
  pub(crate) list: DecodeValue,
  pub(crate) generic: DecodeValue,
}

impl From<DecodeParser> for DecodeFromMeta {
  fn from(
    DecodeParser {
      or_default,
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
      generic,
    }: DecodeParser,
  ) -> Self {
    Self {
      or_default,
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
      generic,
    }
  }
}

impl FromMeta for DecodeFromMeta {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    DecodeParser::from_list(items).map(Into::into)
  }
}

impl DecodeFromMeta {
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

#[cfg(test)]
mod tests {
  use super::*;
  use quote::quote;

  #[test]
  fn test_from_meta() {
    let module = quote! {
      decode(
        or_default,
      )
    };

    let decoded: DecodeFromMeta = DecodeFromMeta::from_meta(&syn::parse2(module).unwrap()).unwrap();
    let want = DecodeFromMeta::default();
    assert_eq!(decoded, want);

    let nested_all = quote! {
      decode(
        or_default,
        scalar(
          or_default = false,
        ),
        bytes(
          or_default = false,
        ),
        string(
          or_default = false,
        ),
        object(
          or_default = false,
        ),
        enum(
          or_default = false,
        ),
        interface(
          or_default = false,
        ),
        union(
          or_default = false,
        ),
      )
    };

    let decoded: DecodeFromMeta =
      DecodeFromMeta::from_meta(&syn::parse2(nested_all).unwrap()).unwrap();
    assert!(!decoded.or_default_bytes());
    assert!(!decoded.or_default_scalar());
    assert!(!decoded.or_default_string());
    assert!(!decoded.or_default_object());
    assert!(!decoded.or_default_enumeration());
    assert!(!decoded.or_default_interface());
    assert!(!decoded.or_default_union());

    let nested_partial = quote! {
      decode(
        scalar(
          or_default = true,
        ),
        bytes(
          or_default = true,
        ),
      )
    };

    let decoded: DecodeFromMeta =
      DecodeFromMeta::from_meta(&syn::parse2(nested_partial).unwrap()).unwrap();
    assert!(decoded.or_default_scalar());
    assert!(decoded.or_default_bytes());
  }
}
