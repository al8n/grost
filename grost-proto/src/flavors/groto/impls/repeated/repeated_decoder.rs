use core::marker::PhantomData;

/// Decoder for repeated fields.
pub struct PackedDecoder<'a, V: ?Sized> {
  src: &'a [u8],
  offset: usize,
  _m: PhantomData<V>,
}

impl<V: ?Sized> Clone for PackedDecoder<'_, V> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<V: ?Sized> Copy for PackedDecoder<'_, V> {}

impl<'a, V: ?Sized> PackedDecoder<'a, V> {
  /// Creates a new decoder for repeated fields.
  #[inline]
  pub fn new(src: &'a [u8], offset: usize) -> Self {
    Self {
      src,
      offset,
      _m: PhantomData,
    }
  }

  /// Returns the source buffer.
  #[inline]
  pub const fn data(&self) -> &'a [u8] {
    self.src
  }

  /// Returns the remaining bytes which are not be decoded.
  #[inline]
  pub const fn remaining(&self) -> &'a [u8] {
    if self.src.is_empty() {
      return &[];
    }

    self.src.split_at(self.offset).1
  }

  /// Returns the current position of the decoder to the original buffer.
  #[inline]
  pub const fn position(&self) -> usize {
    self.offset
  }
}
