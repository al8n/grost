use super::{
  buffer::{ReadBuf, WriteBuf},
  flavors::Flavor,
};

/// The identifier
pub trait Identifier<F: Flavor + ?Sized>: Copy + core::fmt::Debug + core::fmt::Display {
  /// Returns the wire type of the identifier.
  fn wire_type(&self) -> F::WireType;

  /// Returns the tag of the identifier.
  fn tag(&self) -> F::Tag;

  /// Encode the identifier into a buffer.
  fn encode<B>(&self, dst: &mut B) -> Result<usize, F::Error>
  where
    B: WriteBuf + ?Sized;

  /// Return the length of the encoded identifier.
  fn encoded_len(&self) -> usize;

  /// Decode the identifier from a buffer.
  fn decode<'de, B>(buf: B) -> Result<(usize, Self), F::Error>
  where
    B: ReadBuf + Sized + 'de,
    Self: Sized;
}
