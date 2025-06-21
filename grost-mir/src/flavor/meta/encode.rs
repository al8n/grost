use darling::FromMeta;
use quote::quote;
use syn::{Meta, Path, parse2};

use crate::utils::{BoolOption, NestedMeta};

#[derive(Debug, Default, Clone, PartialEq, Eq, darling::FromMeta)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub(super) struct EncodeValueParser {
  #[darling(default)]
  #[cfg_attr(
    feature = "serde",
    serde(
      default,
      skip_serializing_if = "Option::is_none",
      with = "super::serde::serde_optional_path"
    )
  )]
  path: Option<Path>,
  #[darling(default)]
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "BoolOption::is_none")
  )]
  skip_default: BoolOption,
}

impl From<EncodeValueParser> for EncodeValue {
  fn from(EncodeValueParser { path, skip_default }: EncodeValueParser) -> Self {
    Self { path, skip_default }
  }
}

impl From<EncodeValue> for EncodeValueParser {
  fn from(EncodeValue { path, skip_default }: EncodeValue) -> Self {
    Self { path, skip_default }
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(
  feature = "serde",
  serde(
    try_from = "super::serde::EncodeValueSerdeHelper",
    into = "super::serde::EncodeValueSerdeHelper"
  )
)]
pub(crate) struct EncodeValue {
  pub(crate) path: Option<Path>,
  pub(crate) skip_default: BoolOption,
}

impl TryFrom<&str> for EncodeValue {
  type Error = syn::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let path = syn::parse_str::<Path>(value)?;
    Ok(Self {
      skip_default: BoolOption::default(),
      path: Some(path),
    })
  }
}

impl FromMeta for EncodeValue {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    EncodeValueParser::from_list(items).map(Into::into)
  }

  fn from_string(value: &str) -> darling::Result<Self> {
    Self::try_from(value).map_err(Into::into)
  }
}

impl From<Path> for EncodeValue {
  fn from(value: Path) -> Self {
    Self {
      path: Some(value),
      skip_default: BoolOption::default(),
    }
  }
}

#[derive(Debug, FromMeta)]
struct Helper {
  #[darling(default)]
  skip_default: BoolOption,
  #[darling(default)]
  scalar: EncodeValue,
  #[darling(default)]
  bytes: EncodeValue,
  #[darling(default)]
  string: EncodeValue,
  #[darling(default)]
  object: EncodeValue,
  #[darling(default)]
  interface: EncodeValue,
  #[darling(default)]
  union: EncodeValue,
  #[darling(default)]
  map: EncodeValue,
  #[darling(default)]
  set: EncodeValue,
  #[darling(default)]
  list: EncodeValue,
  #[darling(default, rename = "enum")]
  enumeration: EncodeValue,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(
  feature = "serde",
  serde(
    try_from = "super::serde::EncodeSerdeHelper",
    into = "super::serde::EncodeSerdeHelper"
  )
)]
pub struct EncodeFromMeta {
  pub(crate) skip_default: BoolOption,
  pub(crate) scalar: EncodeValue,
  pub(crate) bytes: EncodeValue,
  pub(crate) string: EncodeValue,
  pub(crate) object: EncodeValue,
  pub(crate) enumeration: EncodeValue,
  pub(crate) interface: EncodeValue,
  pub(crate) union: EncodeValue,
  pub(crate) map: EncodeValue,
  pub(crate) set: EncodeValue,
  pub(crate) list: EncodeValue,
}

impl EncodeFromMeta {
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

impl TryFrom<&str> for EncodeFromMeta {
  type Error = syn::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let module = syn::parse_str::<Path>(value)?;
    Self::try_from(&module)
  }
}

impl TryFrom<&syn::Path> for EncodeFromMeta {
  type Error = syn::Error;

  fn try_from(module: &syn::Path) -> Result<Self, Self::Error> {
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

impl FromMeta for EncodeFromMeta {
  fn from_string(value: &str) -> darling::Result<Self> {
    Self::try_from(value).map_err(Into::into)
  }

  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (match *item {
      Meta::Path(_) => Self::from_word(),
      Meta::List(ref value) => {
        let Helper {
          enumeration,
          skip_default,
          scalar,
          bytes,
          string,
          object,
          interface,
          union,
          map,
          set,
          list,
        } = Helper::from_list(&NestedMeta::parse_meta_list(value.tokens.clone())?[..])?;
        Ok(Self {
          skip_default,
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
        })
      }
      Meta::NameValue(ref value) => Self::from_expr(&value.value),
    })
    .map_err(|e| e.with_span(item))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from_meta() {
    let module = quote! {
      encode = "my_flavor::encode"
    };

    let decoded: EncodeFromMeta = EncodeFromMeta::from_meta(&syn::parse2(module).unwrap()).unwrap();
    let want = EncodeFromMeta::try_from("my_flavor::encode").unwrap();
    assert_eq!(decoded, want);

    let nested_all = quote! {
      encode(
        scalar = "my_flavor::encode::scalar",
        bytes = "my_flavor::encode::bytes",
        string = "my_flavor::encode::string",
        object = "my_flavor::encode::object",
        enum = "my_flavor::encode::enumeration",
        interface = "my_flavor::encode::interface",
        union = "my_flavor::encode::union",
      )
    };

    let decoded: EncodeFromMeta =
      EncodeFromMeta::from_meta(&syn::parse2(nested_all).unwrap()).unwrap();
    let want = EncodeFromMeta::try_from("my_flavor::encode").unwrap();
    assert_eq!(decoded, want);

    let nested_partial = quote! {
      encode(
        scalar = "my_flavor::encode::scalar",
        bytes = "my_flavor::encode::bytes",
      )
    };

    let decoded: EncodeFromMeta =
      EncodeFromMeta::from_meta(&syn::parse2(nested_partial).unwrap()).unwrap();
    assert_eq!(decoded.scalar().unwrap(), want.scalar().unwrap(),);
    assert_eq!(decoded.bytes().unwrap(), want.bytes().unwrap(),);
  }
}
