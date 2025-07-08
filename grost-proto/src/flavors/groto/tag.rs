use crate::{error::ParseTagError, selectable};

use super::{Error, Groto};



/// Tag in a graph protocol buffer message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
#[display("{_0}")]
pub struct Tag(pub(super) u32);

selectable!(@scalar Groto: Tag);

impl Tag {
  /// The maximum Tag value is `2^29 - 1` (536870911).
  pub const MAX: Self = Self((1u32 << 29) - 1);
  /// The minimum Tag value is `1`.
  pub const MIN: Self = Self(1);

  /// Try to create a new tag with the given value,
  /// returning an error if the value is not in range `1..=536870911`.
  #[inline]
  pub const fn try_new(tag: u32) -> Result<Self, ParseTagError> {
    const MAX: u32 = (1u32 << 29) - 1;

    if tag > MAX || tag == 0 {
      Err(ParseTagError(tag))
    } else {
      Ok(Self(tag))
    }
  }

  /// Create a new tag.
  ///
  /// ## Panics
  ///
  /// This function will panic if the value is not in range `1..=536870911`.
  #[inline]
  pub const fn new(tag: u32) -> Self {
    const MAX: u32 = (1u32 << 29) - 1;

    if tag > MAX || tag == 0 {
      panic!("the tag value must in range 1..=536870911");
    } else {
      Self(tag)
    }
  }

  /// Returns the tag as a `u32`.
  #[inline]
  pub const fn get(&self) -> u32 {
    self.0
  }

  /// Encodes the tag to bytes slice.
  #[inline]
  pub const fn encode(&self) -> varing::utils::Buffer<6> {
    varing::encode_u32_varint(self.0)
  }

  /// Returns the encoded length of the tag.
  #[inline]
  pub const fn encoded_len(&self) -> usize {
    varing::encoded_u32_varint_len(self.0)
  }

  /// Decodes the tag from bytes slice.
  #[inline]
  pub const fn decode(bytes: &[u8]) -> Result<(usize, Self), Error> {
    match varing::decode_u32_varint(bytes) {
      Ok((len, tag)) => match Self::try_new(tag) {
        Ok(tag) => Ok((len, tag)),
        Err(e) => Err(Error::parse_tag_error(e)),
      },
      Err(e) => Err(Error::from_varint_decode_error(e)),
    }
  }
}

impl From<Tag> for u32 {
  fn from(tag: Tag) -> u32 {
    tag.0
  }
}

impl TryFrom<u32> for Tag {
  type Error = ParseTagError;

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    Self::try_new(value)
  }
}
