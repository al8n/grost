use core::marker::PhantomData;

use crate::{
  buffer::{Buffer, DefaultBuffer},
  convert::Partial,
  flavors::Groto,
  selection::Selectable,
  state::State,
};

mod packed;
mod repeated;

/// The default partial set buffer type.
pub type DefaultPartialSetBuffer<K> = PartialSetBuffer<K>;

/// A buffer for partial map entries.
pub struct PartialSetBuffer<K, B = DefaultBuffer<K>> {
  buffer: B,
  _k: PhantomData<K>,
}

impl<K, B> PartialSetBuffer<K, B> {
  #[inline]
  pub fn into_inner(self) -> B {
    self.buffer
  }
}

impl<K, B> Buffer for PartialSetBuffer<K, B>
where
  B: Buffer<Item = K>,
{
  type Item = K;

  #[inline]
  fn new() -> Self {
    Self {
      buffer: B::new(),
      _k: PhantomData,
    }
  }

  #[inline]
  fn with_capacity(capacity: usize) -> Option<Self>
  where
    Self: Sized,
  {
    Some(Self {
      buffer: B::with_capacity(capacity)?,
      _k: PhantomData,
    })
  }

  #[inline]
  fn push(&mut self, value: Self::Item) -> Option<Self::Item> {
    self.buffer.push(value)
  }

  #[inline]
  fn capacity(&self) -> usize {
    self.buffer.capacity()
  }

  #[inline]
  fn as_slice(&self) -> &[Self::Item] {
    self.buffer.as_slice()
  }

  #[inline]
  fn as_mut_slice(&mut self) -> &mut [Self::Item] {
    self.buffer.as_mut_slice()
  }

  #[inline]
  fn len(&self) -> usize {
    self.buffer.len()
  }

  #[inline]
  fn is_empty(&self) -> bool {
    self.buffer.is_empty()
  }

  fn into_iter(self) -> impl Iterator<Item = Self::Item> {
    self.buffer.into_iter()
  }
}

impl<K, B> Selectable<Groto> for PartialSetBuffer<K, B>
where
  K: Selectable<Groto>,
{
  type Selector = K::Selector;
}

impl<K, B> State<Partial<Groto>> for PartialSetBuffer<K, B> {
  type Output = Self;
}
