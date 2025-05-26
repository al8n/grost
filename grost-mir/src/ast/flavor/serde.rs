use super::*;

use indexmap::IndexMap;
use quote::ToTokens;

use ::serde::{Deserialize, Serialize, de::Deserializer, ser::Serializer};
use syn::{Ident, Path};

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub(super) enum EncodeSerdeHelper {
  Config {
    #[cfg_attr(feature = "serde", serde(default, with = "serde_optional_path"))]
    scalar: Option<Path>,
    #[cfg_attr(feature = "serde", serde(default, with = "serde_optional_path"))]
    bytes: Option<Path>,
    #[cfg_attr(feature = "serde", serde(default, with = "serde_optional_path"))]
    string: Option<Path>,
    #[cfg_attr(feature = "serde", serde(default, with = "serde_optional_path"))]
    object: Option<Path>,
    #[cfg_attr(feature = "serde", serde(default, with = "serde_optional_path"))]
    enumeration: Option<Path>,
    #[cfg_attr(feature = "serde", serde(default, with = "serde_optional_path"))]
    interface: Option<Path>,
    #[cfg_attr(feature = "serde", serde(default, with = "serde_optional_path"))]
    union: Option<Path>,
  },
  Module(String),
}

impl TryFrom<EncodeSerdeHelper> for EncodeAttribute {
  type Error = syn::Error;

  fn try_from(value: EncodeSerdeHelper) -> Result<Self, Self::Error> {
    match value {
      EncodeSerdeHelper::Module(s) => EncodeAttribute::try_from(s.as_str()),
      EncodeSerdeHelper::Config {
        scalar,
        bytes,
        string,
        object,
        enumeration,
        interface,
        union,
      } => Ok(Self {
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
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub(super) enum RegistrationValueSerdeHelper {
  File(String),
  Config {
    or_else_default: bool,
    #[cfg_attr(feature = "serde", serde(rename = "type", with = "serde_type"))]
    ty: syn::Type,
    identifier: IdentifierAttribute,
    encode: EncodeAttribute,
  },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub(super) struct RegistrationSerdeHelper(IndexMap<String, RegistrationValueSerdeHelper>);

impl From<RegistrationAttribute> for RegistrationSerdeHelper {
  fn from(value: RegistrationAttribute) -> Self {
    Self(
      value
        .flavors
        .into_iter()
        .map(|(k, v)| {
          (
            k.to_string(),
            RegistrationValueSerdeHelper::Config {
              or_else_default: v.or_else_default,
              ty: v.ty,
              identifier: v.identifier,
              encode: v.encode,
            },
          )
        })
        .collect(),
    )
  }
}

impl TryFrom<RegistrationSerdeHelper> for RegistrationAttribute {
  type Error = syn::Error;

  fn try_from(value: RegistrationSerdeHelper) -> Result<Self, Self::Error> {
    value
      .0
      .into_iter()
      .map(|(k, v)| {
        let ident = syn::parse_str(k.as_str())?;
        let value = match v {
          RegistrationValueSerdeHelper::File(path_buf) => {
            RegistrationValueAttribute::try_from(path_buf.as_str()).map_err(syn::Error::from)?
          }
          RegistrationValueSerdeHelper::Config {
            or_else_default,
            ty,
            identifier,
            encode,
          } => RegistrationValueAttribute {
            or_else_default,
            ty,
            identifier,
            encode,
          },
        };
        Ok((ident, value))
      })
      .collect::<Result<IndexMap<Ident, RegistrationValueAttribute>, syn::Error>>()
      .map(|flavors| Self { flavors })
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
        .scalar
        .as_ref()
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
        .scalar
        .as_ref()
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
        .scalar
        .as_ref()
        .unwrap()
        .to_token_stream()
        .to_string()
        .replace(" ", ""),
      "grost::flavors::network::Encode::scalar"
    );

    assert_eq!(
      t.encode
        .bytes
        .as_ref()
        .unwrap()
        .to_token_stream()
        .to_string()
        .replace(" ", ""),
      "grost::flavors::network::Encode::bytes"
    );
    assert!(t.encode.string.is_none());
    assert!(t.encode.object.is_none());
    assert!(t.encode.enumeration.is_none());
    assert!(t.encode.interface.is_none());
    assert!(t.encode.union.is_none());
  }
}
