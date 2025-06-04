use crate::ast::BoolOption;

use super::{decode::DecodeValue, *};

use quote::ToTokens;

use ::serde::{Deserialize, Serialize, de::Deserializer, ser::Serializer};
use syn::Path;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum IdentifierSerdeHelper {
  Module(String),
  Config {
    #[cfg_attr(feature = "serde", serde(with = "super::serde::serde_path"))]
    constructor: syn::Path,
    #[cfg_attr(feature = "serde", serde(with = "super::serde::serde_path"))]
    encode: syn::Path,
  },
}

impl TryFrom<IdentifierSerdeHelper> for IdentifierAttribute {
  type Error = syn::Error;

  fn try_from(value: IdentifierSerdeHelper) -> Result<Self, Self::Error> {
    match value {
      IdentifierSerdeHelper::Module(s) => IdentifierAttribute::try_from(s.as_str()),
      IdentifierSerdeHelper::Config {
        constructor,
        encode,
      } => Ok(Self {
        constructor,
        encode,
      }),
    }
  }
}

impl From<IdentifierAttribute> for IdentifierSerdeHelper {
  fn from(value: IdentifierAttribute) -> Self {
    IdentifierSerdeHelper::Config {
      constructor: value.constructor,
      encode: value.encode,
    }
  }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum EncodeValueSerdeHelper {
  Path(String),
  Config(EncodeValueParser),
}

impl TryFrom<EncodeValueSerdeHelper> for EncodeValue {
  type Error = syn::Error;

  fn try_from(value: EncodeValueSerdeHelper) -> Result<Self, Self::Error> {
    match value {
      EncodeValueSerdeHelper::Path(s) => syn::parse_str::<Path>(s.as_str()).map(Into::into),
      EncodeValueSerdeHelper::Config(parser) => Ok(parser.into()),
    }
  }
}

impl From<EncodeValue> for EncodeValueSerdeHelper {
  fn from(value: EncodeValue) -> Self {
    Self::Config(value.into())
  }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub(super) enum EncodeSerdeHelper {
  Config {
    skip_default: BoolOption,
    scalar: EncodeValue,
    bytes: EncodeValue,
    string: EncodeValue,
    object: EncodeValue,
    #[serde(rename = "enum")]
    enumeration: EncodeValue,
    interface: EncodeValue,
    union: EncodeValue,
  },
  Module(String),
}

impl TryFrom<EncodeSerdeHelper> for EncodeAttribute {
  type Error = syn::Error;

  fn try_from(value: EncodeSerdeHelper) -> Result<Self, Self::Error> {
    match value {
      EncodeSerdeHelper::Module(s) => EncodeAttribute::try_from(s.as_str()),
      EncodeSerdeHelper::Config {
        skip_default,
        scalar,
        bytes,
        string,
        object,
        enumeration,
        interface,
        union,
      } => Ok(Self {
        skip_default,
        scalar,
        bytes,
        string,
        object,
        enumeration,
        interface,
        union,
      }),
    }
  }
}

impl From<EncodeAttribute> for EncodeSerdeHelper {
  fn from(value: EncodeAttribute) -> Self {
    EncodeSerdeHelper::Config {
      scalar: value.scalar,
      bytes: value.bytes,
      string: value.string,
      object: value.object,
      enumeration: value.enumeration,
      interface: value.interface,
      union: value.union,
      skip_default: value.skip_default,
    }
  }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum DecodeValueSerdeHelper {
  Config(DecodeValueParser),
}

impl TryFrom<DecodeValueSerdeHelper> for DecodeValue {
  type Error = syn::Error;

  fn try_from(value: DecodeValueSerdeHelper) -> Result<Self, Self::Error> {
    match value {
      DecodeValueSerdeHelper::Config(parser) => Ok(parser.into()),
    }
  }
}

impl From<DecodeValue> for DecodeValueSerdeHelper {
  fn from(value: DecodeValue) -> Self {
    Self::Config(value.into())
  }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub(super) enum DecodeSerdeHelper {
  Config {
    or_else_default: BoolOption,
    scalar: DecodeValue,
    bytes: DecodeValue,
    string: DecodeValue,
    object: DecodeValue,
    #[serde(rename = "enum")]
    enumeration: DecodeValue,
    interface: DecodeValue,
    union: DecodeValue,
  },
}

impl TryFrom<DecodeSerdeHelper> for DecodeAttribute {
  type Error = syn::Error;

  fn try_from(value: DecodeSerdeHelper) -> Result<Self, Self::Error> {
    match value {
      DecodeSerdeHelper::Config {
        or_else_default,
        scalar,
        bytes,
        string,
        object,
        enumeration,
        interface,
        union,
      } => Ok(Self {
        or_else_default,
        scalar,
        bytes,
        string,
        object,
        enumeration,
        interface,
        union,
      }),
    }
  }
}

impl From<DecodeAttribute> for DecodeSerdeHelper {
  fn from(value: DecodeAttribute) -> Self {
    DecodeSerdeHelper::Config {
      scalar: value.scalar,
      bytes: value.bytes,
      string: value.string,
      object: value.object,
      enumeration: value.enumeration,
      interface: value.interface,
      union: value.union,
      or_else_default: value.or_else_default,
    }
  }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub(super) enum BuiltinFlavorValueSerdeHelper {
  File(String),
  Config {
    encode: EncodeAttribute,
    decode: DecodeAttribute,
  },
  Bool(bool),
}

impl TryFrom<BuiltinFlavorValueSerdeHelper> for BuiltinFlavorRepr {
  type Error = darling::Error;

  fn try_from(value: BuiltinFlavorValueSerdeHelper) -> Result<Self, Self::Error> {
    match value {
      BuiltinFlavorValueSerdeHelper::File(path_buf) => {
        BuiltinFlavorValueParser::try_from(path_buf.as_str()).map(BuiltinFlavorRepr::Nested)
      }
      BuiltinFlavorValueSerdeHelper::Config { encode, decode } => {
        Ok(BuiltinFlavorRepr::Nested(BuiltinFlavorValueParser {
          encode,
          decode,
        }))
      }
      BuiltinFlavorValueSerdeHelper::Bool(b) => Ok(BuiltinFlavorRepr::Bool(b)),
    }
  }
}

impl From<BuiltinFlavorRepr> for BuiltinFlavorValueSerdeHelper {
  fn from(value: BuiltinFlavorRepr) -> Self {
    match value {
      BuiltinFlavorRepr::Nested(parser) => Self::Config {
        encode: parser.encode,
        decode: parser.decode,
      },
      BuiltinFlavorRepr::Bool(b) => Self::Bool(b),
    }
  }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub(super) enum FlavorValueSerdeHelper {
  File(String),
  Config {
    #[serde(with = "serde_type")]
    ty: syn::Type,
    #[serde(with = "serde_type")]
    format: syn::Type,
    identifier: IdentifierAttribute,
    encode: EncodeAttribute,
    decode: DecodeAttribute,
  },
}

impl TryFrom<FlavorValueSerdeHelper> for FlavorValue {
  type Error = darling::Error;

  fn try_from(value: FlavorValueSerdeHelper) -> Result<Self, Self::Error> {
    match value {
      FlavorValueSerdeHelper::File(path_buf) => {
        FlavorValueParser::from_path(path_buf.as_str()).map(Into::into)
      }
      FlavorValueSerdeHelper::Config {
        ty,
        format,
        identifier,
        encode,
        decode,
      } => Ok(Self {
        ty,
        format,
        identifier,
        encode,
        decode,
      }),
    }
  }
}

impl From<FlavorValue> for FlavorValueSerdeHelper {
  fn from(value: FlavorValue) -> Self {
    Self::Config {
      ty: value.ty,
      format: value.format,
      identifier: value.identifier,
      encode: value.encode,
      decode: value.decode,
    }
  }
}

pub(super) mod serde_type {
  use super::*;

  pub fn serialize<S>(val: &syn::Type, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let val = val
      .to_token_stream()
      .to_string()
      .split_whitespace()
      .collect::<String>();
    serializer.serialize_str(val.as_str())
  }

  pub fn deserialize<'de, S>(deserializer: S) -> Result<syn::Type, S::Error>
  where
    S: Deserializer<'de>,
  {
    <&str as Deserialize<'_>>::deserialize(deserializer)
      .and_then(|val| syn::parse_str(val).map_err(<S::Error as ::serde::de::Error>::custom))
  }
}

pub(super) mod serde_path {
  use super::*;

  pub fn serialize<S>(val: &syn::Path, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let val = val
      .to_token_stream()
      .to_string()
      .split_whitespace()
      .collect::<String>();
    serializer.serialize_str(val.as_str())
  }

  pub fn deserialize<'de, S>(deserializer: S) -> Result<syn::Path, S::Error>
  where
    S: Deserializer<'de>,
  {
    <&str as Deserialize<'_>>::deserialize(deserializer)
      .and_then(|val| syn::parse_str(val).map_err(<S::Error as ::serde::de::Error>::custom))
  }
}

pub(super) mod serde_optional_path {
  use super::*;

  pub fn serialize<S>(val: &Option<syn::Path>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match val {
      Some(val) => super::serde_path::serialize(val, serializer),
      None => serializer.serialize_unit(),
    }
  }

  pub fn deserialize<'de, S>(deserializer: S) -> Result<Option<syn::Path>, S::Error>
  where
    S: Deserializer<'de>,
  {
    <&str as Deserialize<'_>>::deserialize(deserializer)
      .and_then(|val| syn::parse_str(val).map_err(<S::Error as ::serde::de::Error>::custom))
      .map(Some)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_identifier_serde() {
    #[derive(Serialize, Deserialize)]
    struct T {
      identifier: IdentifierAttribute,
    }

    let module = r###"
    {
      "identifier": "my_identifier"
    }
    "###;
    let t = serde_json::from_str::<'_, T>(module).unwrap();
    assert_eq!(
      t.identifier
        .constructor()
        .to_token_stream()
        .to_string()
        .replace(" ", ""),
      "my_identifier::constructor"
    );
    assert_eq!(
      t.identifier
        .encode()
        .to_token_stream()
        .to_string()
        .replace(" ", ""),
      "my_identifier::encode"
    );

    let config = r###"
    {
      "identifier": {
        "constructor": "grost::flavors::network::Identifier::constructor",
        "encode": "grost::flavors::network::Identifier::encode"
      }
    }
    "###;
    let t = serde_json::from_str::<'_, T>(config).unwrap();
    assert_eq!(
      t.identifier
        .constructor()
        .to_token_stream()
        .to_string()
        .replace(" ", ""),
      "grost::flavors::network::Identifier::constructor"
    );
    assert_eq!(
      t.identifier
        .encode()
        .to_token_stream()
        .to_string()
        .replace(" ", ""),
      "grost::flavors::network::Identifier::encode"
    );
  }

  #[test]
  fn test_encode_serde() {
    #[derive(Serialize, Deserialize)]
    struct T {
      encode: EncodeAttribute,
    }

    let module = r###"
    {
      "encode": "my_encode"
    }
    "###;
    let t = serde_json::from_str::<'_, T>(module).unwrap();
    assert_eq!(
      t.encode
        .scalar()
        .unwrap()
        .to_token_stream()
        .to_string()
        .replace(" ", ""),
      "my_encode::scalar"
    );

    let config = r###"
    {
      "encode": {
        "scalar": "grost::flavors::network::Encode::scalar",
        "bytes": "grost::flavors::network::Encode::bytes",
        "string": "grost::flavors::network::Encode::string",
        "object": "grost::flavors::network::Encode::object",
        "enumeration": "grost::flavors::network::Encode::enumeration",
        "interface": "grost::flavors::network::Encode::interface",
        "union": "grost::flavors::network::Encode::union"
      }
    }
    "###;
    let t = serde_json::from_str::<T>(config).unwrap();
    assert_eq!(
      t.encode
        .scalar()
        .unwrap()
        .to_token_stream()
        .to_string()
        .replace(" ", ""),
      "grost::flavors::network::Encode::scalar"
    );

    let config_partial = r###"
    {
      "encode": {
        "scalar": "grost::flavors::network::Encode::scalar",
        "bytes": "grost::flavors::network::Encode::bytes"
      }
    }
    "###;

    let t = serde_json::from_str::<T>(config_partial).unwrap();
    assert_eq!(
      t.encode
        .scalar()
        .unwrap()
        .to_token_stream()
        .to_string()
        .replace(" ", ""),
      "grost::flavors::network::Encode::scalar"
    );

    assert_eq!(
      t.encode
        .bytes()
        .unwrap()
        .to_token_stream()
        .to_string()
        .replace(" ", ""),
      "grost::flavors::network::Encode::bytes"
    );
    assert!(t.encode.string().is_none());
    assert!(t.encode.object().is_none());
    assert!(t.encode.enumeration().is_none());
    assert!(t.encode.interface().is_none());
    assert!(t.encode.union().is_none());
  }
}
