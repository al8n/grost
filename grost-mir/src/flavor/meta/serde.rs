use crate::utils::BoolOption;

use super::{decode::DecodeValue, *};

use quote::ToTokens;

use ::serde::{Deserialize, Serialize, de::Deserializer, ser::Serializer};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum TagSerdeHelper {
  Module(String),
  Config {
    #[cfg_attr(feature = "serde", serde(with = "super::serde::serde_path"))]
    constructor: syn::Path,
    #[cfg_attr(feature = "serde", serde(with = "super::serde::serde_path"))]
    encode: syn::Path,
  },
}

impl TryFrom<TagSerdeHelper> for TagFromMeta {
  type Error = syn::Error;

  fn try_from(value: TagSerdeHelper) -> Result<Self, Self::Error> {
    match value {
      TagSerdeHelper::Module(s) => TagFromMeta::try_from(s.as_str()),
      TagSerdeHelper::Config {
        constructor,
        encode,
      } => Ok(Self {
        constructor: constructor.into(),
        encode: encode.into(),
      }),
    }
  }
}

impl TryFrom<TagFromMeta> for TagSerdeHelper {
  type Error = darling::Error;

  fn try_from(value: TagFromMeta) -> Result<Self, Self::Error> {
    Ok(TagSerdeHelper::Config {
      constructor: value.constructor.try_into()?,
      encode: value.encode.try_into()?,
    })
  }
}

impl Serialize for TagFromMeta {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let helper: TagSerdeHelper = self
      .clone()
      .try_into()
      .map_err(<S::Error as ::serde::ser::Error>::custom)?;
    helper.serialize(serializer)
  }
}

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

impl TryFrom<IdentifierSerdeHelper> for IdentifierFromMeta {
  type Error = syn::Error;

  fn try_from(value: IdentifierSerdeHelper) -> Result<Self, Self::Error> {
    match value {
      IdentifierSerdeHelper::Module(s) => IdentifierFromMeta::try_from(s.as_str()),
      IdentifierSerdeHelper::Config {
        constructor,
        encode,
      } => Ok(Self {
        constructor: constructor.into(),
        encode: encode.into(),
      }),
    }
  }
}

impl TryFrom<IdentifierFromMeta> for IdentifierSerdeHelper {
  type Error = darling::Error;

  fn try_from(value: IdentifierFromMeta) -> Result<Self, Self::Error> {
    Ok(IdentifierSerdeHelper::Config {
      constructor: value.constructor.try_into()?,
      encode: value.encode.try_into()?,
    })
  }
}

impl Serialize for IdentifierFromMeta {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let helper: IdentifierSerdeHelper = self
      .clone()
      .try_into()
      .map_err(<S::Error as ::serde::ser::Error>::custom)?;
    helper.serialize(serializer)
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
    or_default: BoolOption,
    scalar: DecodeValue,
    bytes: DecodeValue,
    string: DecodeValue,
    object: DecodeValue,
    #[serde(rename = "enum")]
    enumeration: DecodeValue,
    interface: DecodeValue,
    union: DecodeValue,
    map: DecodeValue,
    set: DecodeValue,
    list: DecodeValue,
  },
}

impl TryFrom<DecodeSerdeHelper> for DecodeFromMeta {
  type Error = syn::Error;

  fn try_from(value: DecodeSerdeHelper) -> Result<Self, Self::Error> {
    match value {
      DecodeSerdeHelper::Config {
        or_default,
        scalar,
        bytes,
        string,
        object,
        enumeration,
        interface,
        union,
        map,
        set,
        list,
      } => Ok(Self {
        or_default,
        scalar,
        bytes,
        string,
        object,
        enumeration,
        interface,
        union,
        map,
        set,
        list,
      }),
    }
  }
}

impl From<DecodeFromMeta> for DecodeSerdeHelper {
  fn from(value: DecodeFromMeta) -> Self {
    DecodeSerdeHelper::Config {
      scalar: value.scalar,
      bytes: value.bytes,
      string: value.string,
      object: value.object,
      enumeration: value.enumeration,
      interface: value.interface,
      union: value.union,
      or_default: value.or_default,
      map: value.map,
      set: value.set,
      list: value.list,
    }
  }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub(super) enum BuiltinFlavorValueSerdeHelper {
  File(String),
  Config {
    decode: DecodeFromMeta,
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
      BuiltinFlavorValueSerdeHelper::Config { decode } => {
        Ok(BuiltinFlavorRepr::Nested(BuiltinFlavorValueParser {
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
    identifier: IdentifierFromMeta,
    tag: TagFromMeta,
    decode: DecodeFromMeta,
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
        tag,
        decode,
      } => Ok(Self {
        ty,
        format,
        identifier,
        tag,
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
      tag: value.tag,
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
      identifier: TagFromMeta,
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
        "constructor": "grost::flavors::network::Tag::constructor",
        "encode": "grost::flavors::network::Tag::encode"
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
      "grost::flavors::network::Tag::constructor"
    );
    assert_eq!(
      t.identifier
        .encode()
        .to_token_stream()
        .to_string()
        .replace(" ", ""),
      "grost::flavors::network::Tag::encode"
    );
  }
}
