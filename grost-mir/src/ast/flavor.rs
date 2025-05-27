use darling::{ast::NestedMeta, FromMeta};
pub use encode::*;
pub use decode::*;
pub use identifier::*;

use indexmap::IndexMap;
use quote::format_ident;
use registration::*;
use syn::{parse_quote, spanned::Spanned, Ident, Meta};

use super::{GenericParams, GenericRepr};

mod decode;
mod encode;
mod identifier;
mod registration;

#[cfg(feature = "serde")]
mod serde;

fn default_flavor_attribute() -> bool {
  true
}

#[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
struct DefaultFlavorValueParser {
  #[darling(default)]
  encode: EncodeAttribute,
  #[darling(default)]
  decode: DecodeAttribute,
}

impl TryFrom<&str> for DefaultFlavorValueParser {
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
    try_from = "serde::DefaultFlavorValueSerdeHelper",
    into = "serde::DefaultFlavorValueSerdeHelper"
  )
)]
enum DefaultFlavorRepr {
  Nested(DefaultFlavorValueParser),
  Bool(bool),
}

impl Default for DefaultFlavorRepr {
  fn default() -> Self {
    Self::Bool(true)
  }
}

impl FromMeta for DefaultFlavorRepr {
  fn from_bool(value: bool) -> darling::Result<Self> {
    Ok(Self::Bool(value))
  }

  fn from_string(value: &str) -> darling::Result<Self> {
    DefaultFlavorValueParser::try_from(value)
      .map(Self::Nested)
      .map_err(Into::into)
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    DefaultFlavorValueParser::from_list(items)
      .map(Self::Nested)
      .map_err(Into::into)
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
  identifier: IdentifierAttribute,
  #[darling(default)]
  encode: EncodeAttribute,
  #[darling(default)]
  decode: DecodeAttribute,
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
  ty: syn::Type,
  format: syn::Type,
  identifier: IdentifierAttribute,
  encode: EncodeAttribute,
  decode: DecodeAttribute,
}

impl From<FlavorValueParser> for FlavorValue {
  fn from(FlavorValueParser {
    ty,
    format,
    identifier,
    encode,
    decode,
  }: FlavorValueParser) -> Self {
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

pub(super) struct FlavorParser {
  default: DefaultFlavorRepr,
  flavors: IndexMap<Ident, FlavorValue>,
}

impl FromMeta for FlavorParser {
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
              return Err(darling::Error::custom(
                "missing the name of the flavor",
              ));
            };

            if ident.eq("default") {
              if default.is_some() {
                return Err(darling::Error::custom(
                  "duplicate flavor, default flavor is already defined",
                ));
              }

              let value = DefaultFlavorRepr::from_list(&NestedMeta::parse_meta_list(meta_list.tokens.clone())?[..])?;
              default = Some(value);
              continue;
            }
            
            if flavors.contains_key(ident) {
              return Err(darling::Error::custom(
                format!("duplicate flavor, `{ident}` flavor is already defined"),
              ));
            }

            let value = FlavorValue::from_list(
              &NestedMeta::parse_meta_list(meta_list.tokens.clone())?[..],
            )?;
            flavors.insert(ident.clone(), value);
          }
          Meta::NameValue(meta_name_value) => {
            let Some(ident) = meta.path().get_ident() else {
              return Err(darling::Error::custom(
                "missing the name of the flavor",
              ));
            };

            if ident.eq("default") {
              if default.is_some() {
                return Err(darling::Error::custom(
                  "duplicate flavor, default flavor is already defined",
                ));
              }

              let value = DefaultFlavorRepr::from_expr(&meta_name_value.value)?;
              default = Some(value);
              continue;
            }

            if flavors.contains_key(ident) {
              return Err(darling::Error::custom(
                format!("duplicate flavor, `{ident}` flavor is already defined"),
              ));
            }

            let value = FlavorValue::from_expr(&meta_name_value.value)?;
            flavors.insert(ident.clone(), value);
          }
        },
      }
    }

    Ok(Self { flavors, default: default.unwrap_or_default() })
  }
}

impl FlavorParser {
  pub(crate) fn into_object_flavors(self, path_to_grost: &syn::Path) -> syn::Result<Vec<FlavorAttribute>> {
    let mut flavors = Vec::new();

    match self.default {
      DefaultFlavorRepr::Nested(default_flavor_value_parser) => {
        flavors.push(FlavorAttribute {
          name: format_ident!("default"),
          ty: parse_quote!(#path_to_grost::__private::flavors::Network),
          format: parse_quote!(#path_to_grost::__private::flavors::network::LengthDelimited),
          identifier: IdentifierAttribute::network(path_to_grost),
          encode: default_flavor_value_parser.encode,
          decode: default_flavor_value_parser.decode,
        })
      },
      DefaultFlavorRepr::Bool(val) => {
        if val {
          flavors.push(FlavorAttribute::network_object(path_to_grost)?);
        }
      },
    }

    for (name, value) in self.flavors {
      let ty = value.ty;
      let format = value.format;
      let identifier = value.identifier;
      let encode = value.encode;
      let decode = value.decode;

      flavors.push(FlavorAttribute {
        name,
        ty,
        format,
        identifier,
        encode,
        decode,
      });
    }

    Ok(flavors)
  }
}

#[derive(Debug, Clone)]
pub struct FlavorAttribute {
  name: syn::Ident,
  ty: syn::Type,
  format: syn::Type,
  identifier: IdentifierAttribute,
  encode: EncodeAttribute,
  decode: DecodeAttribute,
}

impl FlavorAttribute {
  fn network_object(path_to_grost: &syn::Path) -> syn::Result<Self> {
    let ty = parse_quote!(#path_to_grost::__private::flavors::Network);
    let format = parse_quote!(#path_to_grost::__private::flavors::network::LengthDelimited);
    let identifier = IdentifierAttribute::network(path_to_grost);
    let encode = EncodeAttribute::network(path_to_grost)?;
    let decode = DecodeAttribute::network(path_to_grost);

    Ok(Self {
      name: format_ident!("network"),
      ty,
      format,
      identifier,
      encode,
      decode,
    })
  }

  /// Returns the name of the flavor.
  pub const fn name(&self) -> &syn::Ident {
    &self.name
  }

  /// Returns the type of the flavor.
  pub const fn ty(&self) -> &syn::Type {
    &self.ty
  }

  /// Returns the wire format for the type of the flavor.
  /// 
  /// e.g. If the macro is used on object, then this will return the wire format for the object.
  pub const fn wire_format(&self) -> &syn::Type {
    &self.format
  }

  /// Returns the identifier attribute of the flavor.
  pub const fn identifier(&self) -> &IdentifierAttribute {
    &self.identifier
  }

  /// Returns the encode attribute of the flavor.
  pub const fn encode(&self) -> &EncodeAttribute {
    &self.encode
  }

  /// Returns the decode attribute of the flavor.
  pub const fn decode(&self) -> &DecodeAttribute {
    &self.decode
  }
}

#[cfg(test)]
mod tests {
  use quote::quote;

  #[test]
  fn test_register_flavor() {
    let _flavor = quote! {
      #[object(
        partial_decoded(),
        generic(
          lifetime = "'a",
          flavor = "F",
          unknown_buffer = "UB",
        ),
        flavor(
          default(
            encode(
              skip_default,
              scalar(
                path = "my_flavor::scalar",
                skip_default = false,
              ),
              bytes = "my_flavor::bytes",
              string = "my_flavor::string",
              object = "my_flavor::object",
              enum = "my_flavor::enumeration",
              interface = "my_flavor::interface",
              union = "my_flavor::union",
            ),
            decode(
              or_else_default,
              scalar(
                or_else_default = false,
              ),
            ),
          ),
          storage(
            type = "my_flavor::MyFlavor",
            format = "my_flavor::ObjectWireFormat",
            identifier(
              new = "new_storage_identifier",
              encode = "StorageIdentifier::encode",
            ),
            encode(
              scalar(
                path = "my_flavor::scalar",
                skip_encode_default = false,
              ),
              bytes = "my_flavor::bytes",
              string = "my_flavor::string",
              object = "my_flavor::object",
              enum = "my_flavor::enumeration",
              interface = "my_flavor::interface",
              union = "my_flavor::union",
            ),
            decode(
              scalar(
                or_else_default = false,
              ),
            )
          ),
        )
      )]
      struct User {
        #[grost(
          tag = 1,
          string,

          optional,
          packed,
          stream,

          flavor(
            default(
              format = "my_flavor::UserFormat",
              partial_decoded(
                type = "&'a str",
                encode(skip_default, skip_if = "path/to/skip_fn", error_if = "path/to/error_fn"),
                decode(
                  or_else_default = "path/to/default_fn",
                  or_else = "path/to/or_else_fn",
                  ok_or_else = "path/to/ok_or_else_fn",
                  error_if = "path/to/error_if_fn",
                ),
              ),
              partial(
                type = "String",
              ),
              convert(
                from_decoded = "path/to/from_decoded_fn",
                try_from_decoded = "path/to/try_from_decoded_fn",
                default_on_missing = "path/to/default_on_missing_fn",
              ),
            ),
            storage(
              
            ),
          ),
        )]
        name: String,
        like: u32,
      }
    };
  }
}
