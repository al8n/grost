use darling::{FromMeta, ast::NestedMeta};
pub use decode::*;
pub use encode::*;
pub use identifier::*;

use indexmap::IndexMap;
use syn::{Ident, Meta, MetaList, Type, parse::Parser};

use crate::meta::NestedMetaWithTypeField;

mod decode;
mod encode;
mod identifier;

pub(super) fn duplicate_flavor_error(ident: &Ident) -> darling::Error {
  darling::Error::custom(format!(
    "Duplicate flavor `{}` found. Each flavor can only be defined once within the same attribute.",
    ident
  ))
}

pub(super) fn complex_flavor_ident_error() -> darling::Error {
  darling::Error::custom(
    "Flavor names must be simple identifiers. Complex paths are not supported.",
  )
}

#[cfg(feature = "serde")]
mod serde;

pub(super) const RESERVED_FLAVOR_NAMES: &[(&str, &str)] = &[
  (
    "network",
    "The `network` flavor is reserved and equivalent to the default flavor. Use `default` to configure the default behavior instead.",
  ),
  (
    "storage",
    "The `storage` flavor is a reserved flavor name, please use a different name",
  ),
  (
    "proto2",
    "The `proto2` flavor is a reserved flavor name, please use a different name",
  ),
  (
    "proto3",
    "The `proto3` flavor is a reserved flavor name, please use a different name",
  ),
  (
    "zerocopy",
    "The `zerocopy` flavor is a reserved flavor name, please use a different name",
  ),
  (
    "json",
    "The `json` flavor is a reserved flavor name, please use a different name",
  ),
  (
    "graphql",
    "The `graphql` flavor is a reserved flavor name, please use a different name",
  ),
  (
    "postgres",
    "The `postgres` flavor is a reserved flavor name, please use a different name",
  ),
  (
    "mysql",
    "The `mysql` flavor is a reserved flavor name, please use a different name",
  ),
  (
    "sqlite",
    "The `sqlite` flavor is a reserved flavor name, please use a different name",
  ),
  (
    "mongodb",
    "The `mongodb` flavor is a reserved flavor name, please use a different name",
  ),
];

#[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub(crate) struct BuiltinFlavorValueParser {
  #[darling(default)]
  pub(crate) encode: EncodeFromMeta,
  #[darling(default)]
  pub(crate) decode: DecodeFromMeta,
}

impl TryFrom<&str> for BuiltinFlavorValueParser {
  type Error = darling::Error;

  fn try_from(_value: &str) -> Result<Self, Self::Error> {
    #[cfg(feature = "serde")]
    {
      use config::{Config, File};

      Config::builder()
        .add_source(File::with_name(_value))
        .build()
        .and_then(Config::try_deserialize)
        .map_err(darling::Error::custom)
    }

    #[cfg(not(feature = "serde"))]
    Err(darling::Error::custom(
      "specify a flavor config file is not supported without the `serde` feature",
    ))
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(
  feature = "serde",
  serde(
    try_from = "serde::BuiltinFlavorValueSerdeHelper",
    into = "serde::BuiltinFlavorValueSerdeHelper"
  )
)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum BuiltinFlavorRepr {
  Nested(BuiltinFlavorValueParser),
  Bool(bool),
}

impl Default for BuiltinFlavorRepr {
  fn default() -> Self {
    Self::Bool(true)
  }
}

impl FromMeta for BuiltinFlavorRepr {
  fn from_bool(value: bool) -> darling::Result<Self> {
    Ok(Self::Bool(value))
  }

  fn from_string(value: &str) -> darling::Result<Self> {
    BuiltinFlavorValueParser::try_from(value).map(Self::Nested)
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    BuiltinFlavorValueParser::from_list(items).map(Self::Nested)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
struct FlavorValueParser {
  #[darling(rename = "type")]
  #[cfg_attr(feature = "serde", serde(with = "serde::serde_type"))]
  ty: syn::Type,
  #[cfg_attr(feature = "serde", serde(with = "serde::serde_type"))]
  format: syn::Type,
  identifier: IdentifierFromMeta,
  #[darling(default)]
  encode: EncodeFromMeta,
  #[darling(default)]
  decode: DecodeFromMeta,
}

impl FlavorValueParser {
  fn from_path(_value: &str) -> darling::Result<Self> {
    #[cfg(feature = "serde")]
    {
      use config::{Config, File};

      Config::builder()
        .add_source(File::with_name(_value))
        .build()
        .and_then(Config::try_deserialize)
        .map_err(darling::Error::custom)
    }

    #[cfg(not(feature = "serde"))]
    Err(darling::Error::custom(
      "specify a flavor config file is not supported without the `serde` feature",
    ))
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(
  feature = "serde",
  serde(
    try_from = "serde::FlavorValueSerdeHelper",
    into = "serde::FlavorValueSerdeHelper"
  )
)]
struct FlavorValue {
  pub(crate) ty: Type,
  pub(crate) format: Type,
  pub(crate) identifier: IdentifierFromMeta,
  pub(crate) encode: EncodeFromMeta,
  pub(crate) decode: DecodeFromMeta,
}

impl From<FlavorValueParser> for FlavorValue {
  fn from(
    FlavorValueParser {
      ty,
      format,
      identifier,
      encode,
      decode,
    }: FlavorValueParser,
  ) -> Self {
    Self {
      ty,
      format,
      identifier,
      encode,
      decode,
    }
  }
}

impl FromMeta for FlavorValue {
  fn from_string(value: &str) -> darling::Result<Self> {
    FlavorValueParser::from_path(value).map(Into::into)
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    FlavorValueParser::from_list(items).map(Into::into)
  }
}

impl FlavorValue {
  fn parse_meta_list(list: &MetaList) -> darling::Result<Self> {
    let punctuated =
      syn::punctuated::Punctuated::<NestedMetaWithTypeField, syn::Token![,]>::parse_terminated
        .parse2(list.tokens.clone())?;

    let mut nested_meta = Vec::new();
    let mut ty = None;
    for item in punctuated {
      match item {
        NestedMetaWithTypeField::Type(t) => {
          if ty.is_some() {
            return Err(darling::Error::duplicate_field("type"));
          }
          ty = Some(t);
        }
        NestedMetaWithTypeField::Nested(value) => {
          nested_meta.push(value);
        }
      }
    }

    #[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
    struct Helper {
      format: syn::Type,
      identifier: IdentifierFromMeta,
      #[darling(default)]
      encode: EncodeFromMeta,
      #[darling(default)]
      decode: DecodeFromMeta,
    }

    let helper = Helper::from_list(&nested_meta)?;

    Ok(Self {
      ty: ty.ok_or_else(|| darling::Error::missing_field("type"))?,
      format: helper.format,
      identifier: helper.identifier,
      encode: helper.encode,
      decode: helper.decode,
    })
  }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FlavorFromMeta {
  pub(crate) default: BuiltinFlavorRepr,
  pub(crate) flavors: IndexMap<Ident, FlavorValue>,
}

impl FromMeta for FlavorFromMeta {
  fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
    let mut flavors = IndexMap::new();
    let mut default = None;

    for item in items {
      match item {
        NestedMeta::Lit(lit) => return Err(darling::Error::unexpected_lit_type(lit)),
        NestedMeta::Meta(meta) => match meta {
          Meta::Path(_) => return Err(darling::Error::unsupported_format("word")),
          Meta::List(meta_list) => {
            let Some(ident) = meta_list.path.get_ident() else {
              return Err(complex_flavor_ident_error());
            };

            if ident.eq("default") {
              if default.is_some() {
                return Err(duplicate_flavor_error(ident));
              }

              let value = BuiltinFlavorRepr::from_list(
                &NestedMeta::parse_meta_list(meta_list.tokens.clone())?[..],
              )?;
              default = Some(value);
              continue;
            }

            if let Some(reason) = RESERVED_FLAVOR_NAMES
              .iter()
              .find_map(|(name, reason)| if ident.eq(name) { Some(*reason) } else { None })
            {
              return Err(darling::Error::custom(reason));
            }

            if flavors.contains_key(ident) {
              return Err(duplicate_flavor_error(ident));
            }

            let value = FlavorValue::parse_meta_list(meta_list)?;
            flavors.insert(ident.clone(), value);
          }
          Meta::NameValue(meta_name_value) => {
            let Some(ident) = meta.path().get_ident() else {
              return Err(complex_flavor_ident_error());
            };

            if ident.eq("default") {
              if default.is_some() {
                return Err(duplicate_flavor_error(ident));
              }

              let value = BuiltinFlavorRepr::from_expr(&meta_name_value.value)?;
              default = Some(value);
              continue;
            }

            if let Some(reason) = RESERVED_FLAVOR_NAMES
              .iter()
              .find_map(|(name, reason)| if ident.eq(name) { Some(*reason) } else { None })
            {
              return Err(darling::Error::custom(reason));
            }

            if flavors.contains_key(ident) {
              return Err(duplicate_flavor_error(ident));
            }

            let value = FlavorValue::from_expr(&meta_name_value.value)?;
            flavors.insert(ident.clone(), value);
          }
        },
      }
    }

    Ok(Self {
      flavors,
      default: default.unwrap_or_default(),
    })
  }
}
