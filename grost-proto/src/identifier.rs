use super::{
  buffer::{Chunk, ChunkMut, ChunkWriter},
  error::{DecodeError, EncodeError},
  flavors::Flavor,
};

/// The identifier
pub trait Identifier<F: Flavor + ?Sized>: Copy + core::fmt::Debug + core::fmt::Display {
  /// Returns the wire type of the identifier.
  fn wire_type(&self) -> F::WireType;

  /// Returns the tag of the identifier.
  fn tag(&self) -> F::Tag;

  /// Encode the identifier into a buffer.
  fn encode<B>(&self, dst: impl Into<ChunkWriter<B>>) -> Result<usize, EncodeError<F>>
  where
    B: ChunkMut;

  /// Return the length of the encoded identifier.
  fn encoded_len(&self) -> usize;

  /// Decode the identifier from a buffer.
  fn decode<'de, B>(buf: B) -> Result<(usize, Self), DecodeError<F>>
  where
    B: Chunk + Sized + 'de,
    Self: Sized;
}
