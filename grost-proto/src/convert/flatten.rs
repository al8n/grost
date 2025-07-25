use ghost::phantom;

use super::State;

/// A sub-state of [`Extracted`] which means get the key type for flattening.
///
/// e.g.
/// - `HashMap<K, V>`, the key type is `K`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapKey;

/// A sub-state of [`Extracted`] which means get the value type for flattening.
///
/// e.g.
/// - `HashMap<K, V>`, the key type is `V`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapValue;

/// A sub-state of [`Extracted`] which means get the inner type for flattening.
///
/// e.g.
/// - `Option<T>`, the inner type is `T`.
/// - `Vec<T>`, the inner type is `T`.
/// - `HashMap<K, V>`, the inner type is `(K, V)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Inner;

/// A sub-state of [`Extracted`] which means get the innermost type for flattening.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Innermost;

impl<T: ?Sized> State<Innermost> for T {
  type Output = T;
}

impl<T> State<Option<Innermost>> for Option<T> {
  type Output = Self;
}

impl<T> State<Option<Innermost>> for &T
where
  T: State<Option<Innermost>>,
{
  type Output = T::Output;
}

impl<T> State<Option<Innermost>> for &mut T
where
  T: State<Option<Innermost>>,
{
  type Output = T::Output;
}

/// A state which shows that the type is in its flatten state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[phantom]
pub struct Extracted<S: ?Sized = Option<Innermost>>;

impl<S, T> State<Extracted<S>> for &T
where
  S: ?Sized,
  T: State<Extracted<S>> + ?Sized,
{
  type Output = T::Output;
}

impl<S, T> State<Extracted<S>> for &mut T
where
  S: ?Sized,
  T: State<Extracted<S>> + ?Sized,
{
  type Output = T::Output;
}

impl<T> State<Extracted> for Option<T>
where
  T: State<Extracted>,
{
  type Output = T::Output;
}

impl<T> State<Extracted<Innermost>> for Option<T>
where
  T: State<Extracted<Innermost>>,
{
  type Output = T::Output;
}

impl<T> State<Extracted<Inner>> for Option<T> {
  type Output = T;
}
