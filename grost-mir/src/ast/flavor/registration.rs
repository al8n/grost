use darling::{FromMeta, ast::NestedMeta};
use indexmap::IndexMap;
use syn::{Ident, Meta};

use super::{DecodeAttribute, EncodeAttribute, IdentifierAttribute};


#[derive(Debug, Clone, FromMeta)]
struct BuiltinFlavorValue {
  #[darling(default)]
  encode: EncodeAttribute,
  #[darling(default)]
  decode: DecodeAttribute,
}

#[derive(Debug, Clone, FromMeta)]
struct RegistrationValueParser {
  #[darling(default)]
  or_else_default: bool,
  #[darling(rename = "type")]
  ty: syn::Type,
  identifier: IdentifierAttribute,
  #[darling(default)]
  encode: EncodeAttribute,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub(super) struct RegistrationValueAttribute {
  pub(super) or_else_default: bool,
  #[cfg_attr(
    feature = "serde",
    serde(rename = "type", with = "super::serde::serde_type")
  )]
  pub(super) ty: syn::Type,
  pub(super) identifier: IdentifierAttribute,
  pub(super) encode: EncodeAttribute,
}

impl TryFrom<&str> for RegistrationValueAttribute {
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
      "specify flavor file is not supported without the `serde` feature",
    ))
  }
}

impl FromMeta for RegistrationValueAttribute {
  fn from_string(value: &str) -> darling::Result<Self> {
    Self::try_from(value)
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let parser = RegistrationValueParser::from_list(items)?;
    Ok(Self {
      or_else_default: parser.or_else_default,
      ty: parser.ty,
      identifier: parser.identifier,
      encode: parser.encode,
    })
  }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(
  feature = "serde",
  serde(
    try_from = "super::serde::RegistrationSerdeHelper",
    into = "super::serde::RegistrationSerdeHelper"
  )
)]
pub(super) struct RegistrationAttribute {
  pub(super) flavors: IndexMap<Ident, RegistrationValueAttribute>,
}

impl FromMeta for RegistrationAttribute {
  fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
    let mut flavors = IndexMap::new();

    for item in items {
      match item {
        NestedMeta::Lit(lit) => return Err(darling::Error::unexpected_lit_type(lit)),
        NestedMeta::Meta(meta) => match meta {
          Meta::Path(_) => return Err(darling::Error::unsupported_format("word")),
          Meta::List(meta_list) => {
            let value = RegistrationValueAttribute::from_list(
              &NestedMeta::parse_meta_list(meta_list.tokens.clone())?[..],
            )?;
            let Some(ident) = meta_list.path.get_ident() else {
              return Err(darling::Error::custom(
                "missing the flavor name of the registration",
              ));
            };
            if flavors.contains_key(ident) {
              return Err(darling::Error::custom(
                "duplicate flavor name in registration",
              ));
            }
            flavors.insert(ident.clone(), value);
          }
          Meta::NameValue(meta_name_value) => {
            let Some(ident) = meta.path().get_ident() else {
              return Err(darling::Error::custom(
                "missing the flavor name of the registration",
              ));
            };

            let value = RegistrationValueAttribute::from_expr(&meta_name_value.value)?;
            if flavors.contains_key(ident) {
              return Err(darling::Error::custom(
                "duplicate flavor name in registration",
              ));
            }

            flavors.insert(ident.clone(), value);
          }
        },
      }
    }

    Ok(Self { flavors })
  }
}
