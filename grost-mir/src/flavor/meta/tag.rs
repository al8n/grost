use darling::FromMeta;
use quote::quote;
use syn::Path;

use crate::utils::Invokable;

#[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
pub(super) struct TagParser {
  constructor: Invokable,
  encode: Invokable,
}

impl From<TagParser> for TagFromMeta {
  fn from(
    TagParser {
      constructor,
      encode,
    }: TagParser,
  ) -> Self {
    Self {
      constructor,
      encode,
    }
  }
}

impl From<TagFromMeta> for TagParser {
  fn from(
    TagFromMeta {
      constructor,
      encode,
    }: TagFromMeta,
  ) -> Self {
    Self {
      constructor,
      encode,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "super::serde::TagSerdeHelper",))]
pub struct TagFromMeta {
  pub(crate) constructor: Invokable,
  pub(crate) encode: Invokable,
}

impl TagFromMeta {
  /// Returns the path to the constructor fn
  #[inline]
  pub const fn constructor(&self) -> &Invokable {
    &self.constructor
  }

  /// Returns the path to the encode fn
  #[inline]
  pub const fn encode(&self) -> &Invokable {
    &self.encode
  }
}

impl TryFrom<&str> for TagFromMeta {
  type Error = syn::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let module: syn::Path = syn::parse_str(value)?;
    let constructor = syn::parse2::<Path>(quote! { #module::constructor }).map(Into::into)?;
    let encode = syn::parse2::<Path>(quote! { #module::encode }).map(Into::into)?;

    Ok(Self {
      constructor,
      encode,
    })
  }
}

impl FromMeta for TagFromMeta {
  fn from_string(value: &str) -> darling::Result<Self> {
    Self::try_from(value).map_err(Into::into)
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    TagParser::from_list(items).map(Into::into)
  }
}
