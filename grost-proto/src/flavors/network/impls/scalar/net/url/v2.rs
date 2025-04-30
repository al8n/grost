use url_2::Url;

use crate::{
  IntoTarget, TypeRef, encode_bridge,
  flavors::{Network, network::DecodeError},
  try_decode_bridge,
};

encode_bridge!(Network: str {
  Url {
    convert: Url::as_str;
  }
});

try_decode_bridge!(@without_decode_owned Network: &'de str {
  Url {
    convert: |s: &str| Url::parse(s).map_err(|_| DecodeError::custom("invalid URL"));
  }
});

impl IntoTarget<Network, Url> for &str {
  fn into_target(self) -> Result<Url, DecodeError> {
    Url::parse(self).map_err(|_| DecodeError::custom("invalid URL"))
  }
}

impl TypeRef<Network, Url> for &str {
  fn to(&self) -> Result<Url, DecodeError> {
    self.into_target()
  }
}

#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::{Message, PartialMessage, TypeOwned};
  use bytes_1::Bytes;

  impl IntoTarget<Network, Url> for Bytes {
    fn into_target(self) -> Result<Url, DecodeError> {
      self.to()
    }
  }

  impl TypeOwned<Network, Url> for Bytes {
    fn to(&self) -> Result<Url, <Network as crate::flavors::Flavor>::DecodeError> {
      crate::utils::from_utf8(self.as_ref())
        .map_err(|_| DecodeError::custom("invalid UTF-8"))
        .and_then(|s| Url::parse(s).map_err(|_| DecodeError::custom("invalid URL")))
    }
  }

  impl PartialMessage<Network> for Url {
    type UnknownBuffer<B> = ();

    type Encoded<'a>
      = &'a str
    where
      Self: Sized + 'a;

    type Borrowed<'a>
      = &'a Self
    where
      Self: 'a;

    type EncodedOwned
      = Bytes
    where
      Self: Sized + 'static;
  }

  impl Message<Network> for Url {
    type Partial = Self;

    type Encoded<'a>
      = &'a str
    where
      Self: Sized + 'a;

    type Borrowed<'a>
      = &'a Self
    where
      Self: 'a;

    type EncodedOwned
      = Bytes
    where
      Self: Sized + 'static;
  }
};
