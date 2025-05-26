use darling::FromMeta;
use quote::quote;

#[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
pub(super) struct IdentifierParser {
  constructor: syn::Path,
  encode: syn::Path,
}

impl From<IdentifierParser> for IdentifierAttribute {
  fn from(
    IdentifierParser {
      constructor,
      encode,
    }: IdentifierParser,
  ) -> Self {
    Self {
      constructor,
      encode,
    }
  }
}

impl From<IdentifierAttribute> for IdentifierParser {
  fn from(
    IdentifierAttribute {
      constructor,
      encode,
    }: IdentifierAttribute,
  ) -> Self {
    Self {
      constructor,
      encode,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(
  feature = "serde",
  serde(
    try_from = "super::serde::IdentifierSerdeHelper",
    into = "super::serde::IdentifierSerdeHelper",
  )
)]
pub struct IdentifierAttribute {
  pub(super) constructor: syn::Path,
  pub(super) encode: syn::Path,
}

impl IdentifierAttribute {
  /// Returns the path to the constructor fn
  #[inline]
  pub const fn constructor(&self) -> &syn::Path {
    &self.constructor
  }

  /// Returns the path to the encode fn
  #[inline]
  pub const fn encode(&self) -> &syn::Path {
    &self.encode
  }
}

impl TryFrom<&str> for IdentifierAttribute {
  type Error = syn::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let module: syn::Path = syn::parse_str(value)?;
    let constructor = syn::parse2(quote! { #module::constructor })?;
    let encode = syn::parse2(quote! { #module::encode })?;

    Ok(Self {
      constructor,
      encode,
    })
  }
}

impl FromMeta for IdentifierAttribute {
  fn from_string(value: &str) -> darling::Result<Self> {
    Self::try_from(value).map_err(Into::into)
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    IdentifierParser::from_list(items).map(Into::into)
  }
}
