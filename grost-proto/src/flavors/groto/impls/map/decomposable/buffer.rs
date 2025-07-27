use core::marker::PhantomData;

use crate::{
  buffer::{Buffer, DefaultBuffer},
  flavors::Groto,
  selection::Selectable,
  state::Partial,
  state::State,
};

mod packed;
mod repeated;

use super::PartialDecomposableMapEntry;

/// The default partial set buffer type.
pub type DefaultDecomposablePartialMapBuffer<K, V> = DecomposablePartialMapBuffer<K, V>;

/// A buffer for partial map entries.
pub struct DecomposablePartialMapBuffer<K, V, B = DefaultBuffer<PartialDecomposableMapEntry<K, V>>>
{
  buffer: B,
  _k: PhantomData<K>,
  _v: PhantomData<V>,
}

impl<K, V, B> DecomposablePartialMapBuffer<K, V, B> {
  #[inline]
  pub fn into_inner(self) -> B {
    self.buffer
  }
}

impl<K, V, B> crate::encode::Length for DecomposablePartialMapBuffer<K, V, B>
where
  B: Buffer<Item = PartialDecomposableMapEntry<K, V>>,
{
  fn length(&self) -> usize {
    self.buffer.len()
  }
}

impl<K, V, B> Buffer for DecomposablePartialMapBuffer<K, V, B>
where
  B: Buffer<Item = PartialDecomposableMapEntry<K, V>>,
{
  type Item = PartialDecomposableMapEntry<K, V>;

  #[inline]
  fn new() -> Self {
    Self {
      buffer: B::new(),
      _k: PhantomData,
      _v: PhantomData,
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
      _v: PhantomData,
    })
  }

  #[inline]
  fn try_reserve(&mut self, additional: usize) -> bool {
    self.buffer.try_reserve(additional)
  }

  #[inline]
  fn try_reserve_exact(&mut self, additional: usize) -> bool {
    self.buffer.try_reserve_exact(additional)
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

impl<K, V, B> Selectable<Groto> for DecomposablePartialMapBuffer<K, V, B>
where
  K: Selectable<Groto>,
  V: Selectable<Groto>,
{
  type Selector = super::DecomposableMapSelector<K::Selector, V::Selector>;
}

impl<K, V, B> State<Partial<Groto>> for DecomposablePartialMapBuffer<K, V, B> {
  type Output = Self;
}
