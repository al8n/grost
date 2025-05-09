/// A trait for type-based field selection within types implementing [`Selectable`].
///
/// `Selector` provides mechanisms to include or exclude specific fields or components
/// within a composite type. This is useful for operations like serialization, validation,
/// or any process where you need to specify which parts of a data structure to process.
pub trait Selector<F: ?Sized>: Clone + core::fmt::Debug + Eq {
  /// Creates a selector that includes all possible fields.
  ///
  /// This constant provides a convenient starting point when you want to select
  /// everything and then potentially exclude specific fields.
  const ALL: Self;

  /// Creates a selector that includes no fields.
  ///
  /// This constant provides a convenient starting point when you want to build
  /// a selector by gradually adding only the specific fields you need.
  const NONE: Self;

  /// Returns the number of selected fields.
  fn selected(&self) -> usize;

  /// Returns the number of unselected fields.
  fn unselected(&self) -> usize;

  /// Inverts the current selection.
  ///
  /// This method flips the selection state of all fields: previously selected fields
  /// become unselected, and previously unselected fields become selected.
  ///
  /// Returns a mutable reference to self for method chaining.
  fn flip(&mut self) -> &mut Self;

  /// Combines this selector with another, typically using a union operation.
  ///
  /// This method incorporates the fields from `other` into the current selector.
  /// The exact behavior depends on the implementing type, but generally results
  /// in a union of the selected fields from both selectors.
  ///
  /// Returns a mutable reference to self for method chaining.
  fn merge(&mut self, other: Self) -> &mut Self;

  /// Creates a new selector by combining this selector with another.
  ///
  /// This is a convenience method that calls `merge` but returns a new selector
  /// rather than modifying the original. This is useful when you want to preserve
  /// the original selector.
  fn merge_into(mut self, other: Self) -> Self {
    self.merge(other);
    self
  }

  /// Returns `true` if this selector is empty.
  fn is_empty(&self) -> bool {
    self == &Self::NONE
  }

  /// Returns `true` if this selector is all.
  fn is_all(&self) -> bool {
    self == &Self::ALL
  }
}

impl<F: ?Sized> Selector<F> for () {
  const ALL: Self = ();
  const NONE: Self = ();

  fn selected(&self) -> usize {
    1
  }

  fn unselected(&self) -> usize {
    0
  }

  fn flip(&mut self) -> &mut Self {
    self
  }

  fn merge(&mut self, _: Self) -> &mut Self {
    self
  }
}

impl<F: ?Sized> Selector<F> for bool {
  const ALL: Self = true;
  const NONE: Self = false;

  #[inline]
  fn selected(&self) -> usize {
    if *self { 1 } else { 0 }
  }

  #[inline]
  fn unselected(&self) -> usize {
    if *self { 0 } else { 1 }
  }

  #[inline]
  fn flip(&mut self) -> &mut Self {
    *self = !*self;
    self
  }

  #[inline]
  fn merge(&mut self, other: Self) -> &mut Self {
    if other {
      *self = true;
    }
    self
  }
}

/// A trait for types that can be selected.
pub trait Selectable<F: ?Sized, W: ?Sized> {
  /// The corresponding selector for this type.
  type Selector: Selector<F>;
}

impl<T, F, W> Selectable<F, W> for &T
where
  T: Selectable<F, W> + ?Sized,
  F: ?Sized,
  W: ?Sized,
{
  type Selector = T::Selector;
}

impl<T, F, W> Selectable<F, W> for Option<T>
where
  T: Selectable<F, W>,
  F: ?Sized,
  W: ?Sized,
{
  type Selector = T::Selector;
}
