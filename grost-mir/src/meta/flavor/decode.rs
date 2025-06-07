use darling::FromMeta;

use crate::meta::BoolOption;

#[derive(Debug, Default, Clone, PartialEq, Eq, darling::FromMeta)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub(super) struct DecodeValueParser {
  #[darling(default)]
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "BoolOption::is_none")
  )]
  or_else_default: BoolOption,
}

impl From<DecodeValueParser> for DecodeValue {
  fn from(DecodeValueParser { or_else_default }: DecodeValueParser) -> Self {
    Self { or_else_default }
  }
}

impl From<DecodeValue> for DecodeValueParser {
  fn from(DecodeValue { or_else_default }: DecodeValue) -> Self {
    Self { or_else_default }
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
  pub(crate) or_else_default: BoolOption,
}

impl FromMeta for DecodeValue {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    DecodeValueParser::from_list(items).map(Into::into)
  }
}

#[derive(Debug, FromMeta)]
pub(super) struct DecodeParser {
  #[darling(default)]
  or_else_default: BoolOption,
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
  pub(crate) or_else_default: BoolOption,
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
}

impl From<DecodeParser> for DecodeFromMeta {
  fn from(
    DecodeParser {
      or_else_default,
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
    }: DecodeParser,
  ) -> Self {
    Self {
      or_else_default,
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
}

impl FromMeta for DecodeFromMeta {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    DecodeParser::from_list(items).map(Into::into)
  }
}

impl DecodeFromMeta {
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

#[cfg(test)]
mod tests {
  use super::*;
  use quote::quote;

  #[test]
  fn test_from_meta() {
    let module = quote! {
      decode(
        or_else_default,
      )
    };

    let decoded: DecodeFromMeta = DecodeFromMeta::from_meta(&syn::parse2(module).unwrap()).unwrap();
    let want = DecodeFromMeta::default();
    assert_eq!(decoded, want);

    let nested_all = quote! {
      decode(
        or_else_default,
        scalar(
          or_else_default = false,
        ),
        bytes(
          or_else_default = false,
        ),
        string(
          or_else_default = false,
        ),
        object(
          or_else_default = false,
        ),
        enum(
          or_else_default = false,
        ),
        interface(
          or_else_default = false,
        ),
        union(
          or_else_default = false,
        ),
      )
    };

    let decoded: DecodeFromMeta =
      DecodeFromMeta::from_meta(&syn::parse2(nested_all).unwrap()).unwrap();
    assert!(!decoded.or_else_default_bytes());
    assert!(!decoded.or_else_default_scalar());
    assert!(!decoded.or_else_default_string());
    assert!(!decoded.or_else_default_object());
    assert!(!decoded.or_else_default_enumeration());
    assert!(!decoded.or_else_default_interface());
    assert!(!decoded.or_else_default_union());

    let nested_partial = quote! {
      decode(
        scalar(
          or_else_default = true,
        ),
        bytes(
          or_else_default = true,
        ),
      )
    };

    let decoded: DecodeFromMeta =
      DecodeFromMeta::from_meta(&syn::parse2(nested_partial).unwrap()).unwrap();
    assert!(decoded.or_else_default_scalar());
    assert!(decoded.or_else_default_bytes());
  }
}
