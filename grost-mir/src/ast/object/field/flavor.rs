use darling::{FromMeta, ast::NestedMeta};
pub use decode::*;
pub use encode::*;
use indexmap::IndexMap;
use syn::{Ident, Meta};

use crate::ast::{complex_flavor_ident_error, duplicate_flavor_error};

mod decode;
mod encode;

#[derive(Debug, Clone, FromMeta)]
struct FieldFlavorValueFromMeta {
  #[darling(default)]
  format: Option<syn::Type>,
  #[darling(default)]
  encode: EncodeFromMeta,
  #[darling(default)]
  decode: DecodeFromMeta,
}

#[derive(Debug, Default, Clone)]
pub(super) struct FieldFlavorFromMeta {
  flavors: IndexMap<Ident, FieldFlavorValueFromMeta>,
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

impl FieldFlavorFromMeta {
  pub fn finalize(self) -> darling::Result<Vec<FieldFlavorAttribute>> {
    self
      .flavors
      .into_iter()
      .map(|(name, value)| {
        let format = value.format;
        let encode = value.encode.finalize()?;
        let decode = value.decode.finalize()?;

        Ok(FieldFlavorAttribute {
          name,
          format,
          encode,
          decode,
        })
      })
      .collect()
  }
}

#[derive(Debug, Clone)]
pub struct FieldFlavorAttribute {
  name: Ident,
  format: Option<syn::Type>,
  encode: EncodeAttribute,
  decode: DecodeAttribute,
}

impl FieldFlavorAttribute {
  /// Returns the name of the flavor.
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the wire format type for the field of this flavor, if specified.
  pub const fn format(&self) -> Option<&syn::Type> {
    self.format.as_ref()
  }

  /// Returns the encode attribute for this flavor.
  pub const fn encode(&self) -> &EncodeAttribute {
    &self.encode
  }

  /// Returns the decode attribute for this flavor.
  pub const fn decode(&self) -> &DecodeAttribute {
    &self.decode
  }

  pub(crate) fn new(
    name: Ident,
    format: Option<syn::Type>,
    encode: EncodeAttribute,
    decode: DecodeAttribute,
  ) -> Self {
    Self {
      name,
      format: None,
      encode: EncodeAttribute::default(),
      decode: DecodeAttribute::default(),
    }
  }
}
