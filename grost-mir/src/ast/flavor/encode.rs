use darling::FromMeta;
use quote::quote;
use syn::{Path, parse2};

#[derive(Debug, FromMeta)]
pub(super) struct EncodeParser {
  #[darling(default)]
  scalar: Option<Path>,
  #[darling(default)]
  bytes: Option<Path>,
  #[darling(default)]
  string: Option<Path>,
  #[darling(default)]
  object: Option<Path>,
  #[darling(default, rename = "enum")]
  enumeration: Option<Path>,
  #[darling(default)]
  interface: Option<Path>,
  #[darling(default)]
  union: Option<Path>,
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
  pub(super) scalar: Option<Path>,
  pub(super) bytes: Option<Path>,
  pub(super) string: Option<Path>,
  pub(super) object: Option<Path>,
  pub(super) enumeration: Option<Path>,
  pub(super) interface: Option<Path>,
  pub(super) union: Option<Path>,
}

impl From<EncodeParser> for EncodeAttribute {
  fn from(
    EncodeParser {
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

    let scalar = quote! { #module::scalar };
    let bytes = quote! { #module::bytes };
    let string = quote! { #module::string };
    let object = quote! { #module::object };
    let enum_ = quote! { #module::enumeration };
    let union = quote! { #module::union };
    let interface = quote! { #module::interface };

    Ok(Self {
      scalar: Some(parse2(scalar)?),
      bytes: Some(parse2(bytes)?),
      string: Some(parse2(string)?),
      object: Some(parse2(object)?),
      enumeration: Some(parse2(enum_)?),
      interface: Some(parse2(interface)?),
      union: Some(parse2(union)?),
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
  /// Returns the path to the encoding module of scalar types
  #[inline]
  pub const fn scalar(&self) -> Option<&Path> {
    self.scalar.as_ref()
  }

  /// Returns the path to the encoding module of bytes types
  #[inline]
  pub const fn bytes(&self) -> Option<&Path> {
    self.bytes.as_ref()
  }

  /// Returns the path to the encoding module of string types
  #[inline]
  pub const fn string(&self) -> Option<&Path> {
    self.string.as_ref()
  }

  /// Returns the path to the encoding module of object types
  #[inline]
  pub const fn object(&self) -> Option<&Path> {
    self.object.as_ref()
  }

  /// Returns the path to the encoding module of enumeration types
  #[inline]
  pub const fn enumeration(&self) -> Option<&Path> {
    self.enumeration.as_ref()
  }

  /// Returns the path to the encoding module of interface types
  #[inline]
  pub const fn interface(&self) -> Option<&Path> {
    self.interface.as_ref()
  }

  /// Returns the path to the encoding module of union types
  #[inline]
  pub const fn union(&self) -> Option<&Path> {
    self.union.as_ref()
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
