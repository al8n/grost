use crate::{flavors::Groto, selection::Selector};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MapSelector<K, V> {
  key: K,
  value: V,
}

impl<K, V> MapSelector<K, V> {
  #[inline]
  pub const fn new(key: K, value: V) -> Self {
    Self { key, value }
  }

  #[inline]
  pub const fn key(&self) -> &K {
    &self.key
  }

  #[inline]
  pub const fn value(&self) -> &V {
    &self.value
  }
}

impl<K, V> Selector<Groto> for MapSelector<K, V>
where
  K: Selector<Groto>,
  V: Selector<Groto>,
{
  const ALL: Self = Self {
    key: K::ALL,
    value: V::ALL,
  };

  const NONE: Self = Self {
    key: K::NONE,
    value: V::NONE,
  };

  const DEFAULT: Self = Self {
    key: K::DEFAULT,
    value: V::DEFAULT,
  };

  fn selected(&self) -> usize {
    let mut count = 0;
    if self.key.ne(&K::NONE) {
      count += 1;
    }
    if self.value.ne(&V::NONE) {
      count += 1;
    }
    count
  }

  fn unselected(&self) -> usize {
    let mut count = 0;
    if self.key.eq(&K::NONE) {
      count += 1;
    }
    if self.value.eq(&V::NONE) {
      count += 1;
    }
    count
  }

  fn flip(&mut self) -> &mut Self {
    self.key.flip();
    self.value.flip();
    self
  }

  fn merge(&mut self, other: Self) -> &mut Self {
    self.key.merge(other.key);
    self.value.merge(other.value);
    self
  }
}
