use core::marker::PhantomData;

use crate::{
  Referenceable,
  flavors::{
    Network,
    network::{Fixed16, Fixed32, Fixed64, Fixed128, LengthDelimited, Repeated, Varint},
  },
};

macro_rules! impl_referenceable {
  ($($wf:ty),+$(,)?) => {
    $(
      impl<V> Referenceable<Network, Repeated<$wf>> for [V] {
        type Ref<'a> = RepeatedDecoder<'a, V>
        where
          Self: 'a;
      }
    )*
  };
}

/// Decoder for repeated fields.
pub struct RepeatedDecoder<'a, V: ?Sized> {
  src: &'a [u8],
  offset: usize,
  _m: PhantomData<V>,
}

impl<V: ?Sized> Clone for RepeatedDecoder<'_, V> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<V: ?Sized> Copy for RepeatedDecoder<'_, V> {}

impl<'a, V: ?Sized> RepeatedDecoder<'a, V> {
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

impl_referenceable!(Fixed16, Fixed32, Fixed64, Fixed128, Varint, LengthDelimited,);
