use super::{DecodeError, EncodeError, ParseSelectorTagError, Select};

/// The tag for the [`Select`](super::Select).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::IsVariant, derive_more::Display)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[repr(u8)]
pub enum SelectorTag {
  #[display("none")]
  None = 0,
  #[display("all")]
  All = 1,
  #[display("select-one")]
  SelectOne = 2,
  #[display("unselect-one")]
  UnselectOne = 3,
  #[display("select-many")]
  SelectMany = 4,
  #[display("unselect-many")]
  UnselectMany = 5,
}

impl SelectorTag {
  /// Try to create a [`SelectorTag`] from a `u8` value.
  #[inline]
  pub const fn try_from_u8(value: u8) -> Result<Self, ParseSelectorTagError> {
    Ok(match value {
      0 => Self::None,
      1 => Self::All,
      2 => Self::SelectOne,
      3 => Self::UnselectOne,
      4 => Self::SelectMany,
      5 => Self::UnselectMany,
      _ => return Err(ParseSelectorTagError(value)),
    })
  }
}

impl TryFrom<u8> for SelectorTag {
  type Error = ParseSelectorTagError;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    Self::try_from_u8(value)
  }
}

wire_type!(
  enum SelectorWireType<Select> {
    /// The zero-sized type wire format, this wire format is used for encoding [`SelectorTag::None`] and [`SelectorTag::All`].
    "zst" = 0,
    /// The varint wire format, this wire format is used for encoding [`SelectorTag::SelectOne`] and [`SelectorTag::UnselectOne`].
    "varint" = 1,
    /// The length-delimited wire format, this wire format is used for encoding [`SelectorTag::SelectMany`] and [`SelectorTag::UnselectMany`].
    "length-delimited" = 2,
  }
);

/// An identifier for a field in a graph protocol buffer message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[display("({wire_type}, {tag})")]
pub struct SelectorIdentifier {
  wire_type: SelectorWireType,
  tag: SelectorTag,
}

impl SelectorIdentifier {
  /// Creates a new identifier with the given wire type and tag.
  ///
  /// # Panics
  ///
  /// This function will panic if the wire type is not valid for the given tag.
  #[inline]
  pub(super) const fn new(wire_type: SelectorWireType, tag: SelectorTag) -> Self {
    Self { wire_type, tag }
  }

  /// Creates a new identifier for select all.
  #[inline]
  pub const fn all() -> Self {
    Self::new(SelectorWireType::Zst, SelectorTag::All)
  }

  /// Creates a new identifier for select nothing.
  #[inline]
  pub const fn none() -> Self {
    Self::new(SelectorWireType::Zst, SelectorTag::None)
  }

  /// Creates a new identifier for select one.
  #[inline]
  pub const fn select_one() -> Self {
    Self::new(SelectorWireType::Varint, SelectorTag::SelectOne)
  }

  /// Creates a new identifier for unselect one.
  #[inline]
  pub const fn unselect_one() -> Self {
    Self::new(SelectorWireType::Varint, SelectorTag::UnselectOne)
  }

  /// Creates a new identifier for select one.
  #[inline]
  pub const fn select_many() -> Self {
    Self::new(SelectorWireType::LengthDelimited, SelectorTag::SelectMany)
  }

  /// Creates a new identifier for unselect one.
  #[inline]
  pub const fn unselect_many() -> Self {
    Self::new(SelectorWireType::LengthDelimited, SelectorTag::UnselectMany)
  }

  /// Returns the wire type of the identifier.
  #[inline]
  pub const fn wire_type(&self) -> SelectorWireType {
    self.wire_type
  }

  /// Returns the tag of the identifier.
  #[inline]
  pub const fn tag(&self) -> SelectorTag {
    self.tag
  }

  /// Consume the `SelectorIdentifier` and returns the `SelectorWireType` and `SelectorTag`.
  #[inline]
  pub const fn into_components(self) -> (SelectorWireType, SelectorTag) {
    (self.wire_type, self.tag)
  }

  /// Returns the encoded identifier as a `u32`.
  #[inline]
  pub const fn as_u8(&self) -> u8 {
    ((self.tag as u8) << 3) | (self.wire_type as u8)
  }

  /// Creates an identifier from a `u32` value.
  #[inline]
  pub const fn try_from_u8(val: u8) -> Result<Self, DecodeError> {
    let wire_type = val & 0b111; // Get last 3 bits for wire type
    let tag = val >> 3; // Shift right to get the tag
    match (
      SelectorTag::try_from_u8(tag),
      SelectorWireType::try_from_u8(wire_type),
    ) {
      (Ok(tag), Ok(wire_type)) => Ok(Self::new(wire_type, tag)),
      (Err(e), Ok(_)) => Err(DecodeError::unknown_tag_value(e.value())),
      (Ok(_), Err(e)) => Err(DecodeError::unknown_wire_type_value(e.value())),
      (Err(et), Err(ew)) => Err(DecodeError::unknown_identifier_value(
        ew.value(),
        et.value(),
      )),
    }
  }
}

impl crate::identifier::Identifier<super::Select> for SelectorIdentifier {
  fn tag(&self) -> SelectorTag {
    self.tag
  }
  fn wire_type(&self) -> SelectorWireType {
    self.wire_type
  }

  fn encode(&self, dst: &mut [u8]) -> Result<usize, super::EncodeError> {
    if dst.is_empty() {
      return Err(EncodeError::buffer_too_small(1, 0));
    }

    dst[0] = self.as_u8();
    Ok(1)
  }

  fn encoded_len(&self) -> usize {
    1
  }

  fn decode(buf: B) -> Result<(usize, Self), DecodeError>
  where
    B: crate::buffer::BytesBuffer + Sized,
    Self: Sized,
  {
    let src = buf.as_bytes();
    if !src.has_remaining() {
      return Err(DecodeError::buffer_underflow());
    }

    let identifier = Self::try_from_u8(src[0])?;
    Ok((1, identifier))
  }
}
