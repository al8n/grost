use ghost::phantom;

use super::State;

/// A sub-state of [`Flattened`] which means get the key type for flattening.
///
/// e.g.
/// - `HashMap<K, V>`, the key type is `K`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapKey;

/// A sub-state of [`Flattened`] which means get the value type for flattening.
///
/// e.g.
/// - `HashMap<K, V>`, the key type is `V`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapValue;

/// A sub-state of [`Flattened`] which means get the inner type for flattening.
///
/// e.g.
/// - `Option<T>`, the inner type is `T`.
/// - `Vec<T>`, the inner type is `T`.
/// - `HashMap<K, V>`, the inner type is `(K, V)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Inner;

/// A sub-state of [`Flattened`] which means get the innermost type for flattening.
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
pub struct Flattened<S: ?Sized = Option<Innermost>>;

impl<S, T> State<Flattened<S>> for &T
where
  S: ?Sized,
  T: State<Flattened<S>> + ?Sized,
{
  type Output = T::Output;
}

impl<S, T> State<Flattened<S>> for &mut T
where
  S: ?Sized,
  T: State<Flattened<S>> + ?Sized,
{
  type Output = T::Output;
}

impl<T> State<Flattened> for Option<T>
where
  T: State<Flattened>,
{
  type Output = T::Output;
}

impl<T> State<Flattened<Innermost>> for Option<T>
where
  T: State<Flattened<Innermost>>,
{
  type Output = T::Output;
}

impl<T> State<Flattened<Inner>> for Option<T> {
  type Output = T;
}

#[allow(dead_code)]
macro_rules! flatten_impl {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<S, T> State<Flattened<S>> for $ty
      where
        T: State<Flattened<S>> + ?Sized,
        S: ?Sized,
      {
        type Output = T::Output;
      }
    )*
  };
  (@Sized $($ty:ty),+$(,)?) => {
    $(
      impl<S, T> State<Flattened<S>> for $ty
      where
        T: State<Flattened<S>>,
      {
        type Output = T::Output;
      }
    )*
  };
  (@(Sized & ?Optional) $($ty:ty),+$(,)?) => {
    $(
      impl<T> State<Flattened> for $ty
      where
        T: State<Flattened>,
      {
        type Output = Self;
      }

      impl<T> State<Flattened<Innermost>> for $ty
      where
        T: State<Flattened<Innermost>>,
      {
        type Output = T::Output;
      }
    )*
  };
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::collections::*;

  flatten_impl!(
    @(Sized & ?Optional)
    VecDeque<T>,
    LinkedList<T>,
    BTreeSet<T>,
  );
};

#[cfg(feature = "std")]
const _: () = {
  use std::collections::HashSet;

  flatten_impl!(
    @Sized
    HashSet<T>,
  );
};
