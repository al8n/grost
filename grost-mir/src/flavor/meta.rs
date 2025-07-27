use darling::{FromMeta, ast::NestedMeta};
pub use identifier::*;
pub use tag::*;

use indexmap::IndexMap;
use syn::{Ident, Meta, MetaList, Type};

use crate::utils::OrDefault;

mod identifier;
mod tag;

pub(crate) fn duplicate_flavor_error(ident: &Ident) -> darling::Error {
  darling::Error::custom(format!(
    "Duplicate flavor `{ident}` found. Each flavor can only be defined once within the same attribute."
  ))
}

pub(crate) fn complex_flavor_ident_error() -> darling::Error {
  darling::Error::custom(
    "Flavor names must be simple identifiers. Complex paths are not supported.",
  )
}

pub(super) const RESERVED_FLAVOR_NAMES: &[(&str, &str)] = &[
  (
    "groto",
    "The `groto` flavor is reserved and equivalent to the default flavor. Use `default` to configure the default behavior instead.",
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
pub(crate) struct BuiltinFlavorValueParser {
  #[darling(default)]
  pub(crate) decode: OrDefault,
}

#[derive(Debug, Clone)]
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

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    BuiltinFlavorValueParser::from_list(items).map(Self::Nested)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
struct FlavorValueParser {
  #[darling(rename = "type")]
  ty: syn::Type,
  format: syn::Type,
  identifier: IdentifierFromMeta,
  tag: TagFromMeta,
  #[darling(default)]
  decode: OrDefault,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FlavorValue {
  pub(crate) ty: Type,
  pub(crate) format: Type,
  pub(crate) identifier: IdentifierFromMeta,
  pub(crate) tag: TagFromMeta,
  pub(crate) decode: OrDefault,
}

impl From<FlavorValueParser> for FlavorValue {
  fn from(
    FlavorValueParser {
      ty,
      format,
      identifier,
      tag,
      decode,
    }: FlavorValueParser,
  ) -> Self {
    Self {
      ty,
      format,
      identifier,
      tag,
      decode,
    }
  }
}

impl FromMeta for FlavorValue {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    FlavorValueParser::from_list(items).map(Into::into)
  }
}

impl FlavorValue {
  fn parse_meta_list(list: &MetaList) -> darling::Result<Self> {
    #[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
    struct Helper {
      #[darling(rename = "type")]
      ty: Type,
      format: syn::Type,
      identifier: IdentifierFromMeta,
      tag: TagFromMeta,
      #[darling(default)]
      decode: OrDefault,
    }

    let helper = Helper::from_list(&NestedMeta::parse_meta_list(list.tokens.clone())?)?;
    Ok(Self {
      ty: helper.ty,
      format: helper.format,
      identifier: helper.identifier,
      tag: helper.tag,
      decode: helper.decode,
    })
  }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct GenericFlavorFromMeta {
  pub(crate) default: BuiltinFlavorRepr,
  pub(crate) flavors: IndexMap<Ident, FlavorValue>,
}

impl FromMeta for GenericFlavorFromMeta {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let mut flavors = IndexMap::new();
    let mut default = None;

    for item in items {
      match item {
        darling::ast::NestedMeta::Lit(lit) => return Err(darling::Error::unexpected_lit_type(lit)),
        darling::ast::NestedMeta::Meta(meta) => match meta {
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

#[derive(Debug, Clone)]
pub(crate) enum FlavorFromMeta {
  Builtin(BuiltinFlavorRepr),
  Custom { name: Ident, value: FlavorValue },
}

impl FromMeta for FlavorFromMeta {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    match items.len() {
      0 => Ok(Self::Builtin(BuiltinFlavorRepr::default())),
      1 => {
        let item = &items[0];

        match item {
          darling::ast::NestedMeta::Lit(lit) => Err(darling::Error::unexpected_lit_type(lit)),
          darling::ast::NestedMeta::Meta(meta) => match meta {
            Meta::Path(_) => Err(darling::Error::unsupported_format("word")),
            Meta::List(meta_list) => {
              let Some(ident) = meta_list.path.get_ident() else {
                return Err(complex_flavor_ident_error());
              };

              if ident.eq("default") {
                let value = BuiltinFlavorRepr::from_list(
                  &NestedMeta::parse_meta_list(meta_list.tokens.clone())?[..],
                )?;
                return Ok(Self::Builtin(value));
              }

              if let Some(reason) = RESERVED_FLAVOR_NAMES
                .iter()
                .find_map(|(name, reason)| if ident.eq(name) { Some(*reason) } else { None })
              {
                return Err(darling::Error::custom(reason));
              }

              FlavorValue::parse_meta_list(meta_list).map(|value| Self::Custom {
                name: ident.clone(),
                value,
              })
            }
            Meta::NameValue(meta_name_value) => {
              let Some(ident) = meta.path().get_ident() else {
                return Err(complex_flavor_ident_error());
              };

              if ident.eq("default") {
                return BuiltinFlavorRepr::from_expr(&meta_name_value.value).map(Self::Builtin);
              }

              if let Some(reason) = RESERVED_FLAVOR_NAMES
                .iter()
                .find_map(|(name, reason)| if ident.eq(name) { Some(*reason) } else { None })
              {
                return Err(darling::Error::custom(reason));
              }

              FlavorValue::from_expr(&meta_name_value.value).map(|value| Self::Custom {
                name: ident.clone(),
                value,
              })
            }
          },
        }
      }
      _ => Err(darling::Error::custom("Only one flavor can be configured")),
    }
  }
}
