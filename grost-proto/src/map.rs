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
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::collections::BTreeMap;

  use crate::flavors::Selectable;

  impl<K, V> Selectable for BTreeMap<K, V>
  where
    K: Selectable,
    V: Selectable,
  {
    type Selector = MapSelector<K::Selector, V::Selector>;
  }
};
