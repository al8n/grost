use core::mem;

use crate::{
  flavors::Flavor,
  selection::{Selectable, Selector},
};

/// The selector for a map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecomposableMapSelector<KS, VS> {
  /// Only selects the keys of the map.
  Key(KS),
  /// Only selects the values of the map.
  Value(VS),
  /// Selects both the keys and values of the map.
  Entry {
    /// The selector of the key.
    key: KS,
    /// The selector of the value.
    value: VS,
  },
}

impl<K, V, F> Selector<F> for DecomposableMapSelector<K, V>
where
  K: Selector<F>,
  V: Selector<F>,
  F: Flavor + ?Sized,
{
  const ALL: Self = DecomposableMapSelector::Entry {
    key: K::ALL,
    value: V::ALL,
  };

  const NONE: Self = DecomposableMapSelector::Entry {
    key: K::NONE,
    value: V::NONE,
  };

  const DEFAULT: Self = Self::ALL;

  fn selected(&self) -> usize {
    match self {
      DecomposableMapSelector::Key(k) => k.selected(),
      DecomposableMapSelector::Value(v) => v.selected(),
      DecomposableMapSelector::Entry { key, value } => key.selected() + value.selected(),
    }
  }

  fn unselected(&self) -> usize {
    match self {
      DecomposableMapSelector::Key(k) => k.unselected(),
      DecomposableMapSelector::Value(v) => v.unselected(),
      DecomposableMapSelector::Entry { key, value } => key.unselected() + value.unselected(),
    }
  }

  fn flip(&mut self) -> &mut Self {
    match self {
      DecomposableMapSelector::Key(k) => {
        k.flip();
      }
      DecomposableMapSelector::Value(v) => {
        v.flip();
      }
      DecomposableMapSelector::Entry { key, value } => {
        key.flip();
        value.flip();
      }
    }
    self
  }

  fn merge(&mut self, other: Self) -> &mut Self {
    match (&mut *self, other) {
      (DecomposableMapSelector::Key(k1), DecomposableMapSelector::Key(k2)) => {
        k1.merge(k2);
        self
      }
      (DecomposableMapSelector::Value(v1), DecomposableMapSelector::Value(v2)) => {
        v1.merge(v2);
        self
      }
      (
        DecomposableMapSelector::Entry { key: k1, value: v1 },
        DecomposableMapSelector::Entry { key: k2, value: v2 },
      ) => {
        k1.merge(k2);
        v1.merge(v2);
        self
      }
      (DecomposableMapSelector::Key(k), DecomposableMapSelector::Entry { key: k2, value: v2 }) => {
        k.merge(k2);
        let k = mem::replace(k, K::NONE);
        *self = DecomposableMapSelector::Entry { key: k, value: v2 };
        self
      }
      (DecomposableMapSelector::Key(k), DecomposableMapSelector::Value(v)) => {
        let k = mem::replace(k, K::NONE);
        *self = DecomposableMapSelector::Entry { key: k, value: v };
        self
      }
      (DecomposableMapSelector::Value(v), DecomposableMapSelector::Key(k)) => {
        let v = mem::replace(v, V::NONE);
        *self = DecomposableMapSelector::Entry { key: k, value: v };
        self
      }
      (DecomposableMapSelector::Value(v), DecomposableMapSelector::Entry { key, value }) => {
        v.merge(value);
        let v = mem::replace(v, V::NONE);
        *self = DecomposableMapSelector::Entry { key, value: v };
        self
      }
      (DecomposableMapSelector::Entry { key, .. }, DecomposableMapSelector::Key(k)) => {
        key.merge(k);
        self
      }
      (DecomposableMapSelector::Entry { value, .. }, DecomposableMapSelector::Value(v)) => {
        value.merge(v);
        self
      }
    }
  }
}

/// A map entry that contains a key and a value.
pub struct MapEntry<K, V> {
  key: K,
  value: V,
}

impl<K, V> From<(K, V)> for MapEntry<K, V> {
  fn from((key, value): (K, V)) -> Self {
    Self { key, value }
  }
}

impl<K, V> From<MapEntry<K, V>> for (K, V) {
  fn from(MapEntry { key, value }: MapEntry<K, V>) -> Self {
    (key, value)
  }
}

impl<K, V> MapEntry<K, V> {
  /// Creates a new map entry with the given key and value.
  #[inline]
  pub const fn new(key: K, value: V) -> Self {
    Self { key, value }
  }

  /// Returns a reference to the key of the map entry.
  #[inline]
  pub const fn key(&self) -> &K {
    &self.key
  }

  /// Returns a reference to the value of the map entry.
  #[inline]
  pub const fn value(&self) -> &V {
    &self.value
  }

  /// Returns a reference to the key and value of the map entry.
  #[inline]
  pub const fn key_value(&self) -> (&K, &V) {
    (&self.key, &self.value)
  }

  /// Consumes the map entry and returns the key.
  #[inline]
  pub fn into_key(self) -> K {
    self.key
  }

  /// Consumes the map entry and returns the value.
  #[inline]
  pub fn into_value(self) -> V {
    self.value
  }

  /// Consumes the map entry and returns the key and value.
  #[inline]
  pub fn into_key_value(self) -> (K, V) {
    (self.key, self.value)
  }
}

impl<'k, 'v, K: 'k + ?Sized, V: 'v + ?Sized> MapEntry<&'k K, &'v V> {
  /// Creates a new map entry with the given refernece key and value.
  #[inline]
  pub const fn from_ref(key: &'k K, value: &'v V) -> Self {
    Self { key, value }
  }
}

impl<K, V, F> Selectable<F> for MapEntry<K, V>
where
  V: Selectable<F>,
  F: Flavor + ?Sized,
{
  type Selector = V::Selector;
}
