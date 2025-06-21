use darling::FromMeta;
use indexmap::IndexMap;
use syn::{Ident, Meta};

pub(in crate::object) use decode::*;
pub(in crate::object) use encode::*;

use crate::{
  flavor::{complex_flavor_ident_error, duplicate_flavor_error},
  object::meta::ConvertFromMeta,
  utils::NestedMeta,
};

mod decode;
mod encode;

#[derive(Debug, Clone)]
pub(in crate::object) struct FieldFlavorValueFromMeta {
  pub(in crate::object) ty: Option<syn::Type>,
  pub(in crate::object) format: Option<syn::Type>,
  pub(in crate::object) encode: EncodeFromMeta,
  pub(in crate::object) decode: DecodeFromMeta,
  pub(in crate::object) convert: ConvertFromMeta,
}

impl FromMeta for FieldFlavorValueFromMeta {
  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (match *item {
      Meta::Path(_) => Self::from_word(),
      Meta::List(ref value) => {
        /// The meta of the partial reference object field
        #[derive(Debug, Default, Clone, FromMeta)]
        struct Helper {
          #[darling(rename = "type", default)]
          ty: Option<syn::Type>,
          #[darling(default)]
          format: Option<syn::Type>,
          #[darling(default)]
          encode: EncodeFromMeta,
          #[darling(default)]
          decode: DecodeFromMeta,
          #[darling(default)]
          convert: ConvertFromMeta,
        }

        let Helper {
          format,
          encode,
          decode,
          convert,
          ty,
        } = Helper::from_list(&NestedMeta::parse_meta_list(value.tokens.clone())?)?;

        Ok(Self {
          ty,
          format,
          encode,
          decode,
          convert,
        })
      }

      Meta::NameValue(ref value) => Self::from_expr(&value.value),
    })
    .map_err(|e| e.with_span(item))
  }
}

#[derive(Debug, Default, Clone)]
pub struct FieldFlavorFromMeta {
  pub(in crate::object) flavors: IndexMap<Ident, FieldFlavorValueFromMeta>,
}

impl FromMeta for FieldFlavorFromMeta {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let mut flavors = IndexMap::new();

    for item in items {
      match item {
        darling::ast::NestedMeta::Lit(_) => {
          return Err(darling::Error::unsupported_format("literal"));
        }
        darling::ast::NestedMeta::Meta(meta) => {
          let ident = meta
            .path()
            .get_ident()
            .ok_or_else(complex_flavor_ident_error)?;

          if ident.eq("network") {
            return Err(darling::Error::custom(
              "The `network` flavor is reserved and equivalent to the default flavor. Use `default` to configure the default behavior instead.",
            ));
          }

          match meta {
            Meta::Path(_) => {
              return Err(darling::Error::unsupported_format("word"));
            }
            Meta::NameValue(name_value) => {
              if flavors.contains_key(ident) {
                return Err(duplicate_flavor_error(ident));
              }

              match &name_value.value {
                syn::Expr::Lit(lit) => match &lit.lit {
                  syn::Lit::Str(lit_str) => {
                    let format = syn::parse_str(&lit_str.value()).map_err(darling::Error::from)?;

                    flavors.insert(
                      ident.clone(),
                      FieldFlavorValueFromMeta {
                        ty: None,
                        format: Some(format),
                        encode: EncodeFromMeta::default(),
                        decode: DecodeFromMeta::default(),
                        convert: ConvertFromMeta::default(),
                      },
                    );
                  }
                  lit => return Err(darling::Error::unexpected_lit_type(lit)),
                },
                expr => {
                  return Err(darling::Error::unexpected_expr_type(expr));
                }
              }
            }
            Meta::List(_) => {
              // let nested_meta = NestedMeta::parse_meta_list(meta_list.tokens.clone())?;
              let value = FieldFlavorValueFromMeta::from_meta(meta)?;
              if flavors.contains_key(ident) {
                return Err(duplicate_flavor_error(ident));
              }

              flavors.insert(
                ident.clone(),
                FieldFlavorValueFromMeta {
                  ty: value.ty,
                  format: value.format,
                  encode: value.encode,
                  decode: value.decode,
                  convert: value.convert,
                },
              );
            }
          }
        }
      }
    }

    Ok(FieldFlavorFromMeta { flavors })
  }
}
