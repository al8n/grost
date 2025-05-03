use crate::flavors::Selector;

/// A selector for maps.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MapSelector<K, V> {
  key: Option<K>,
  value: Option<V>,
}

impl<K, V> MapSelector<K, V> {
  /// Creates a new map selector.
  #[inline]
  pub const fn new(key: Option<K>, value: Option<V>) -> Self {
    Self { key, value }
  }

  /// Returns the key selector.
  #[inline]
  pub const fn key(&self) -> Option<&K> {
    self.key.as_ref()
  }

  /// Returns the value selector.
  #[inline]
  pub const fn value(&self) -> Option<&V> {
    self.value.as_ref()
  }

  /// Returns the key selector.
  #[inline]
  pub const fn key_mut(&mut self) -> Option<&mut K> {
    self.key.as_mut()
  }

  /// Returns the value selector.
  #[inline]
  pub const fn value_mut(&mut self) -> Option<&mut V> {
    self.value.as_mut()
  }

  /// Sets the key selector.
  #[inline]
  pub fn set_key(&mut self, key: Option<K>) -> &mut Self {
    self.key = key;
    self
  }

  /// Sets the key selector.
  #[inline]
  pub fn with_key(mut self, key: Option<K>) -> Self
  where
    K: Copy,
  {
    self.key = key;
    self
  }

  /// Sets the value selector.
  #[inline]
  pub fn with_value(mut self, value: Option<V>) -> Self
  where
    V: Copy,
  {
    self.value = value;
    self
  }

  /// Sets the value selector.
  #[inline]
  pub fn set_value(&mut self, value: Option<V>) -> &mut Self {
    self.value = value;
    self
  }
}

impl<K, V> Selector for MapSelector<K, V>
where
  K: Selector,
  V: Selector,
{
  const ALL: Self = MapSelector::new(Some(K::ALL), Some(V::ALL));
  const NONE: Self = MapSelector::new(None, None);

  fn flip(&mut self) -> &mut Self {
    if let Some(key) = self.key_mut() {
      key.flip();
    }

    if let Some(value) = self.value_mut() {
      value.flip();
    }
    self
  }

  fn merge(&mut self, other: Self) -> &mut Self {
    if let Some(key) = other.key {
      self.set_key(Some(key));
    }

    if let Some(value) = other.value {
      self.set_value(Some(value));
    }

    self
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::collections::BTreeMap;

  use crate::flavors::{Flavor, Selectable};

  impl<K, V, F> Selectable<F> for BTreeMap<K, V>
  where
    K: Selectable<F>,
    V: Selectable<F>,
    F: Flavor + ?Sized,
  {
    type Selector = MapSelector<K::Selector, V::Selector>;
  }
};
