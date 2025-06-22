use darling::FromMeta;
use quote::quote;
use syn::Path;

use crate::utils::Invokable;

#[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
pub(super) struct IdentifierParser {
  constructor: Invokable,
  encode: Invokable,
}

impl From<IdentifierParser> for IdentifierFromMeta {
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

impl From<IdentifierFromMeta> for IdentifierParser {
  fn from(
    IdentifierFromMeta {
      constructor,
      encode,
    }: IdentifierFromMeta,
  ) -> Self {
    Self {
      constructor,
      encode,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[cfg_attr(
  feature = "serde",
  serde(try_from = "super::serde::IdentifierSerdeHelper",)
)]
pub struct IdentifierFromMeta {
  pub(crate) constructor: Invokable,
  pub(crate) encode: Invokable,
}

impl IdentifierFromMeta {
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

  pub(crate) fn network(path_to_grost: &Path) -> darling::Result<Self> {
    let constructor =
      syn::parse2::<Path>(quote! { #path_to_grost::__private::flavors::network::Identifier::new })?;
    let encode = syn::parse2::<Path>(
      quote! { #path_to_grost::__private::flavors::network::Identifier::encode },
    )?;

    Ok(Self {
      constructor: constructor.into(),
      encode: encode.into(),
    })
  }
}

impl TryFrom<&str> for IdentifierFromMeta {
  type Error = syn::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let module: syn::Path = syn::parse_str(value)?;
    let constructor = syn::parse2::<Path>(quote! { #module::constructor })?;
    let encode = syn::parse2::<Path>(quote! { #module::encode })?;

    Ok(Self {
      constructor: constructor.into(),
      encode: encode.into(),
    })
  }
}

impl FromMeta for IdentifierFromMeta {
  fn from_string(value: &str) -> darling::Result<Self> {
    Self::try_from(value).map_err(Into::into)
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    IdentifierParser::from_list(items).map(Into::into)
  }
}
