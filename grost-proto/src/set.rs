
pub trait Set: IntoIterator<Item = Self::Key> {
  /// The type of the items in the set.
  type Key;

  /// Returns the length of the list.
  fn len(&self) -> usize;

  /// Returns true if the list is empty.
  fn is_empty(&self) -> bool {
    self.len() == 0
  }
}

impl<'a, T: Set> Set for &'a T
where
  &'a T: IntoIterator<Item = &'a T::Key>,
{
  type Key = &'a T::Key;

  fn len(&self) -> usize {
    T::len(self)
  }

  fn is_empty(&self) -> bool {
    T::is_empty(self)
  }
}

pub trait SetMut: Set

{
  /// Inserts an item into the set.
  /// 
  /// Returns `true` if the item was inserted, `false` if it was already present.
  fn insert(&mut self, item: Self::Key) -> bool;
}