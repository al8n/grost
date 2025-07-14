pub trait ListLike<T>: IntoIterator<Item = T>
where
  for<'a> &'a Self: IntoIterator<Item = &'a T>,
{
  /// Returns the length of the list.
  fn len(&self) -> usize;

  /// Returns true if the list is empty.
  fn is_empty(&self) -> bool {
    self.len() == 0
  }
}
