use darling::FromMeta;
use quote::quote;
use syn::{Path, parse2};

#[cfg(feature = "serde")]
fn is_false(value: &bool) -> bool {
  !*value
}

#[derive(Debug, Default, Clone, PartialEq, Eq, darling::FromMeta)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub(super) struct EncodeValueParser {
  #[darling(default)]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "super::serde::serde_optional_path"))]
  path: Option<Path>,
  #[darling(default)]
  #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_false"))]
  skip_default: bool,
}

impl From<EncodeValueParser> for EncodeValue {
  fn from(EncodeValueParser { path, skip_default }: EncodeValueParser) -> Self {
    Self {
      path,
      skip_default,
    }
  }
}

impl From<EncodeValue> for EncodeValueParser {
  fn from(EncodeValue { path, skip_default }: EncodeValue) -> Self {
    Self {
      path,
      skip_default,
    }
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
pub(super) struct EncodeValue {
  path: Option<Path>,
  skip_default: bool,
}

impl FromMeta for EncodeValue {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    EncodeValueParser::from_list(items).map(Into::into)
  }

  fn from_string(value: &str) -> darling::Result<Self> {
    let path = syn::parse_str::<Path>(value)?;
    Ok(Self {
      path: Some(path),
      skip_default: false,
    })
  }
}

impl From<Path> for EncodeValue {
  fn from(value: Path) -> Self {
    Self {
      path: Some(value),
      skip_default: false,
    }
  }
}

#[derive(Debug, FromMeta)]
pub(super) struct EncodeParser {
  #[darling(default)]
  skip_default: bool,
  #[darling(default)]
  scalar: EncodeValue,
  #[darling(default)]
  bytes: EncodeValue,
  #[darling(default)]
  string: EncodeValue,
  #[darling(default)]
  object: EncodeValue,
  #[darling(default, rename = "enum")]
  enumeration: EncodeValue,
  #[darling(default)]
  interface: EncodeValue,
  #[darling(default)]
  union: EncodeValue,
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
pub struct EncodeAttribute {
  pub(super) skip_default: bool,
  pub(super) scalar: EncodeValue,
  pub(super) bytes: EncodeValue,
  pub(super) string: EncodeValue,
  pub(super) object: EncodeValue,
  pub(super) enumeration: EncodeValue,
  pub(super) interface: EncodeValue,
  pub(super) union: EncodeValue,
}

impl From<EncodeParser> for EncodeAttribute {
  fn from(
    EncodeParser {
      skip_default,
      scalar,
      bytes,
      string,
      object,
      enumeration,
      interface,
      union,
    }: EncodeParser,
  ) -> Self {
    Self {
      skip_default,
      scalar,
      bytes,
      string,
      object,
      enumeration,
      interface,
      union,
    }
  }
}

impl TryFrom<&str> for EncodeAttribute {
  type Error = syn::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let module = syn::parse_str::<Path>(value)?;
    Self::try_from(&module)
  }
}

impl TryFrom<&syn::Path> for EncodeAttribute {
  type Error = syn::Error;

  fn try_from(module: &syn::Path) -> Result<Self, Self::Error> {
    let scalar = quote! { #module::scalar };
    let bytes = quote! { #module::bytes };
    let string = quote! { #module::string };
    let object = quote! { #module::object };
    let enum_ = quote! { #module::enumeration };
    let union = quote! { #module::union };
    let interface = quote! { #module::interface };

    Ok(Self {
      skip_default: false,
      scalar: EncodeValue::from(parse2::<Path>(scalar)?),
      bytes: EncodeValue::from(parse2::<Path>(bytes)?),
      string: EncodeValue::from(parse2::<Path>(string)?),
      object: EncodeValue::from(parse2::<Path>(object)?),
      enumeration: EncodeValue::from(parse2::<Path>(enum_)?),
      interface: EncodeValue::from(parse2::<Path>(interface)?),
      union: EncodeValue::from(parse2::<Path>(union)?),
    })
  }
}

impl FromMeta for EncodeAttribute {
  fn from_string(value: &str) -> darling::Result<Self> {
    Self::try_from(value).map_err(Into::into)
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    EncodeParser::from_list(items).map(Into::into)
  }
}

impl EncodeAttribute {
  pub(super) fn network(path_to_grost: &Path) -> syn::Result<Self> {
    Self::try_from(path_to_grost).map(|mut s| {
      s.bytes.skip_default = true;
      s.string.skip_default = true;
      s
    })
  }

  /// Returns the path to the encoding module of scalar types
  #[inline]
  pub const fn scalar(&self) -> Option<&Path> {
    self.scalar.path.as_ref()
  }

  /// Returns `true` if the encoding should skip default values
  #[inline]
  pub const fn skip_default_scalar(&self) -> bool {
    if self.scalar.skip_default {
      true
    } else {
      self.skip_default
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
    if self.bytes.skip_default {
      true
    } else {
      self.skip_default
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
    if self.string.skip_default {
      true
    } else {
      self.skip_default
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
    if self.object.skip_default {
      true
    } else {
      self.skip_default
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
    if self.enumeration.skip_default {
      true
    } else {
      self.skip_default
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
    if self.interface.skip_default {
      true
    } else {
      self.skip_default
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
    if self.union.skip_default {
      true
    } else {
      self.skip_default
    }
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

    let decoded: EncodeAttribute =
      EncodeAttribute::from_meta(&syn::parse2(module).unwrap()).unwrap();
    let want = EncodeAttribute::try_from("my_flavor::encode").unwrap();
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

    let decoded: EncodeAttribute =
      EncodeAttribute::from_meta(&syn::parse2(nested_all).unwrap()).unwrap();
    let want = EncodeAttribute::try_from("my_flavor::encode").unwrap();
    assert_eq!(decoded, want);

    let nested_partial = quote! {
      encode(
        scalar = "my_flavor::encode::scalar",
        bytes = "my_flavor::encode::bytes",
      )
    };

    let decoded: EncodeAttribute =
      EncodeAttribute::from_meta(&syn::parse2(nested_partial).unwrap()).unwrap();
    assert_eq!(decoded.scalar().unwrap(), want.scalar().unwrap(),);
    assert_eq!(decoded.bytes().unwrap(), want.bytes().unwrap(),);
  }
}
