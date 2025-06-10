use darling::{FromMeta, ast::NestedMeta};
use indexmap::IndexMap;
use syn::{Ident, Meta};

pub(crate) use decode::*;
pub(crate) use encode::*;

use crate::flavor::{complex_flavor_ident_error, duplicate_flavor_error};

mod decode;
mod encode;

#[derive(Debug, Clone, FromMeta)]
pub struct FieldFlavorValueFromMeta {
  #[darling(default)]
  pub(in crate::object) format: Option<syn::Type>,
  #[darling(default)]
  pub(in crate::object) encode: EncodeFromMeta,
  #[darling(default)]
  pub(in crate::object) decode: DecodeFromMeta,
}

#[derive(Debug, Default, Clone)]
pub struct FieldFlavorFromMeta {
  pub(in crate::object) flavors: IndexMap<Ident, FieldFlavorValueFromMeta>,
}

impl FromMeta for FieldFlavorFromMeta {
  fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
    let mut flavors = IndexMap::new();

    for item in items {
      match item {
        NestedMeta::Lit(_) => return Err(darling::Error::unsupported_format("literal")),
        NestedMeta::Meta(meta) => {
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
                        format: Some(format),
                        encode: EncodeFromMeta::default(),
                        decode: DecodeFromMeta::default(),
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
            Meta::List(meta_list) => {
              let nested_meta = NestedMeta::parse_meta_list(meta_list.tokens.clone())?;
              let value = FieldFlavorValueFromMeta::from_list(&nested_meta)?;
              if flavors.contains_key(ident) {
                return Err(duplicate_flavor_error(ident));
              }

              flavors.insert(
                ident.clone(),
                FieldFlavorValueFromMeta {
                  format: value.format,
                  encode: value.encode,
                  decode: value.decode,
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
