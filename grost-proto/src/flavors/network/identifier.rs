use varing::Varint;

use super::{Tag, WireType, tag::ParseTagError};

/// An identifier for a field in a graph protocol buffer message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[display("({wire_type}, {tag})")]
pub struct Identifier {
  wire_type: WireType,
  tag: Tag,
}

impl Identifier {
  /// Creates a new identifier with the given wire type and tag.
  ///
  /// # Panics
  ///
  /// This function will panic if the wire type is not valid for the given tag.
  #[inline]
  pub const fn new(wire_type: WireType, tag: Tag) -> Self {
    Self { wire_type, tag }
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
    (self.tag.get() << 3) | (self.wire_type as u8 as u32)
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
    varing::encode_u32_varint(self.to_u32())
  }

  /// Returns the encoded length of the identifier.
  #[inline]
  pub const fn encoded_len(&self) -> usize {
    varing::encoded_u32_varint_len(self.to_u32())
  }

  /// Encodes the identifier.
  #[inline]
  pub const fn encode_to(&self, dst: &mut [u8]) -> Result<usize, super::EncodeError> {
    match self.encode_to_inner(dst) {
      Ok(bytes_written) => Ok(bytes_written),
      Err(e) => Err(super::EncodeError::from_varint_error(e)),
    }
  }

  /// Decodes an identifier from a buffer.
  #[inline]
  pub const fn decode(buf: &[u8]) -> Result<(usize, Self), super::DecodeError> {
    match Self::decode_inner(buf) {
      Ok((bytes_read, value)) => Ok((bytes_read, value)),
      Err(e) => Err(super::DecodeError::from_varint_error(e)),
    }
  }

  const fn encode_to_inner(&self, dst: &mut [u8]) -> Result<usize, varing::EncodeError> {
    varing::encode_u32_varint_to(self.to_u32(), dst)
  }

  pub(super) const fn decode_inner(buf: &[u8]) -> Result<(usize, Self), varing::DecodeError> {
    match varing::decode_u32_varint(buf) {
      Ok((bytes_read, value)) => Ok((bytes_read, Self::from_u32(value))),
      Err(e) => Err(e),
    }
  }
}

impl Varint for Identifier {
  const MIN_ENCODED_LEN: usize = 1;
  const MAX_ENCODED_LEN: usize = u32::MAX_ENCODED_LEN;

  #[inline]
  fn encoded_len(&self) -> usize {
    self.encoded_len()
  }

  #[inline]
  fn encode(&self, buf: &mut [u8]) -> Result<usize, varing::EncodeError> {
    self.encode_to_inner(buf)
  }

  #[inline]
  fn decode(buf: &[u8]) -> Result<(usize, Self), varing::DecodeError>
  where
    Self: Sized,
  {
    Self::decode_inner(buf)
  }
}

impl crate::flavors::Identifier<super::Network> for Identifier {
  fn tag(&self) -> Tag {
    self.tag
  }

  fn wire_type(&self) -> WireType {
    self.wire_type
  }

  fn encode(&self, dst: &mut [u8]) -> Result<usize, super::EncodeError> {
    self.encode_to(dst)
  }

  fn encoded_len(&self) -> usize {
    self.encoded_len()
  }

  fn decode<B>(buf: B) -> Result<(usize, Self), super::DecodeError>
  where
    B: crate::buffer::BytesBuffer + Sized,
    Self: Sized,
  {
    Self::decode(buf.as_bytes())
  }
}
