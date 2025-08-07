use varing::Varint;

use super::{Tag, WireType};

use crate::{
  buffer::{Buf, BufExt, BufMut, WriteBuf},
  error::{DecodeTagError, ParseTagError},
};

/// An identifier for a field in a graph protocol buffer message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[display("({wire_type}, {tag})")]
pub struct Identifier {
  wire_type: WireType,
  tag: Tag,
  merged: u32,
  encoded: varing::utils::Buffer<6>,
}

impl Identifier {
  /// Creates a new identifier with the given wire type and tag.
  #[inline]
  pub const fn new(wire_type: WireType, tag: Tag) -> Self {
    let merged = merge_to_u32(tag, wire_type);
    Self {
      wire_type,
      tag,
      merged,
      encoded: varing::encode_u32_varint(merged),
    }
  }

  /// Returns the wire type of the identifier.
  #[inline]
  pub const fn wire_type(&self) -> WireType {
    self.wire_type
  }

  /// Returns the tag of the identifier.
  #[inline]
  pub const fn tag(&self) -> Tag {
    self.tag
  }

  /// Consume the `Identifier` and returns the `WireType` and `Tag`.
  #[inline]
  pub const fn into_components(self) -> (WireType, Tag) {
    (self.wire_type, self.tag)
  }

  /// Returns the encoded identifier as a `u32`.
  #[inline]
  pub const fn to_u32(&self) -> u32 {
    self.merged
  }

  /// Creates an identifier from a `u32` value.
  ///
  /// # Panics
  /// This function will panic if the tag is not in range `1..=536870911`
  #[inline]
  pub const fn from_u32(val: u32) -> Self {
    let wire_type = val & 0b111; // Get last 3 bits for wire type
    let tag = val >> 3; // Shift right to get the tag
    // Using from_u8_unchecked since we know wire_type is within 0-7
    Self::new(WireType::from_u8_unchecked(wire_type as u8), Tag::new(tag))
  }

  /// Creates an identifier from a `u32` value.
  ///
  /// # Panics
  /// This function will panic if the tag is not in range `1..=536870911`
  #[inline]
  pub const fn try_from_u32(val: u32) -> Result<Self, ParseTagError> {
    let wire_type = val & 0b111; // Get last 3 bits for wire type
    let tag = val >> 3; // Shift right to get the tag
    // Using from_u8_unchecked since we know wire_type is within 0-7
    // Self::new(WireType::from_u8_unchecked(wire_type as u8), Tag::new(tag))
    match Tag::try_new(tag) {
      Ok(tag) => Ok(Self::new(WireType::from_u8_unchecked(wire_type as u8), tag)),
      Err(e) => Err(e),
    }
  }

  /// Encodes the identifier.
  #[inline]
  pub const fn encode(&self) -> varing::utils::Buffer<6> {
    self.encoded
  }

  /// Returns the encoded length of the identifier.
  #[inline]
  pub const fn encoded_len(&self) -> usize {
    self.encoded.len()
  }

  /// Encodes the identifier.
  #[inline]
  pub const fn encode_to(&self, dst: &mut [u8]) -> Result<usize, super::EncodeError> {
    match varing::encode_u32_varint_to(self.merged, dst) {
      Ok(len) => Ok(len),
      Err(e) => Err(super::EncodeError::from_varint_error(e)),
    }
  }

  /// Decodes an identifier from a buffer.
  #[inline]
  pub const fn decode(buf: &[u8]) -> Result<(usize, Self), super::DecodeError> {
    match Self::decode_inner(buf) {
      Ok((bytes_read, value)) => Ok((bytes_read, value)),
      Err(e) => Err(super::DecodeError::from_decode_tag_error(e)),
    }
  }

  pub(super) const fn decode_inner(buf: &[u8]) -> Result<(usize, Self), DecodeTagError> {
    match varing::decode_u32_varint(buf) {
      Ok((bytes_read, value)) => match Self::try_from_u32(value) {
        Ok(identifier) => Ok((bytes_read, identifier)),
        Err(e) => Err(DecodeTagError::Parse(e)),
      },
      Err(e) => Err(DecodeTagError::Read(e)),
    }
  }
}

impl crate::identifier::Identifier<super::Groto> for Identifier {
  fn tag(&self) -> Tag {
    self.tag
  }

  fn wire_type(&self) -> WireType {
    self.wire_type
  }

  fn encode<B>(&self, dst: impl Into<WriteBuf<B>>) -> Result<usize, super::EncodeError>
  where
    B: BufMut,
  {
    let mut dst: WriteBuf<B> = dst.into();
    dst
      .try_write_slice(self.encoded.as_slice())
      .map_err(Into::into)
  }

  fn encoded_len(&self) -> usize {
    self.encoded_len()
  }

  fn decode<'de, B>(mut buf: B) -> Result<(usize, Self), super::DecodeError>
  where
    B: Buf + Sized + 'de,
    Self: Sized,
  {
    match buf.read_varint::<u32>() {
      Ok((read, val)) => match Self::try_from_u32(val) {
        Ok(id) => Ok((read, id)),
        Err(e) => Err(e.into()),
      },
      Err(e) => Err(e.into()),
    }
  }
}

#[inline(always)]
const fn merge_to_u32(tag: Tag, wire_type: WireType) -> u32 {
  (tag.get() << 3) | (wire_type as u8 as u32)
}
