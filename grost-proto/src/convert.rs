/// A trait for types that can be transformed from the input type to the output type.
///
/// The generic `S` can be any type representing a state.
pub trait State<S: ?Sized> {
  /// The input state type.
  type Input: ?Sized;
  /// The output state type.
  type Output: ?Sized;
}

/// A state which shows the type in encoded state.
pub struct Decoded<'a, F: ?Sized, W: ?Sized, B: ?Sized, UB: ?Sized> {
  _wf: core::marker::PhantomData<&'a W>,
  _flavor: core::marker::PhantomData<&'a F>,
  _read_buf: core::marker::PhantomData<B>,
  _unknown_buffer: core::marker::PhantomData<UB>,
}

impl<'a, F, W, B, UB> Clone for Decoded<'a, F, W, B, UB>
where
  F: ?Sized,
  W: ?Sized,
  UB: ?Sized,
  B: ?Sized,
{
  fn clone(&self) -> Self {
    *self
  }
}

impl<'a, F, W, B, UB> Copy for Decoded<'a, F, W, B, UB>
where
  F: ?Sized,
  W: ?Sized,
  UB: ?Sized,
  B: ?Sized,
{
}

impl<'a, F, W, T, B, UB> State<Decoded<'a, F, W, B, UB>> for &'a T
where
  F: ?Sized,
  W: ?Sized,
  T: State<Decoded<'a, F, W, B, UB>>,
  B: ?Sized,
  UB: ?Sized,
{
  type Input = T::Input;
  type Output = T::Output;
}

impl<T> State<Option<Innermost>> for Option<T> {
  type Input = T;
  type Output = Self;
}

impl<T> State<Option<Innermost>> for &T
where
  T: State<Option<Innermost>>,
{
  type Input = T::Input;
  type Output = T::Output;
}

/// A sub-state of [`Flatten`] which means get the innermost type for flattening.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Innermost(());

impl<T: ?Sized> State<Innermost> for T {
  type Input = T;
  type Output = T;
}

/// A state which shows that the type is in its flatten state.
pub struct Flatten<S: ?Sized = Option<Innermost>> {
  _state: core::marker::PhantomData<S>,
}

impl<S, T> State<Flatten<S>> for &T
where
  S: ?Sized,
  T: State<Flatten<S>> + ?Sized,
{
  type Input = T::Input;
  type Output = T::Output;
}

macro_rules! wrapper_impl {
  (@decoded_state $($ty:ty),+$(,)?) => {
    $(
      impl<'a, F, W, B, UB, T> State<Decoded<'a, F, W, B, UB>> for $ty
      where
        T: State<Decoded<'a, F, W, B, UB>> + ?Sized,
        F: ?Sized,
        W: ?Sized,
        B: ?Sized,
        UB: ?Sized,
      {
        type Input = T::Input;
        type Output = T::Output;
      }

      wrapper_impl!(@flatten $ty);
    )*
  };
  (@flatten $($ty:ty),+$(,)?) => {
    $(
      impl<S, T> State<Flatten<S>> for $ty
      where
        T: State<Flatten<S>> + ?Sized,
        S: ?Sized,
      {
        type Input = T::Input;
        type Output = T::Output;
      }
    )*
  };
  (@flatten(Sized) $($ty:ty),+$(,)?) => {
    $(
      impl<S, T> State<Flatten<S>> for $ty
      where
        T: State<Flatten<S>>,
      {
        type Input = T::Input;
        type Output = T::Output;
      }
    )*
  };
  (@flatten(Sized, ?Optional) $($ty:ty),+$(,)?) => {
    $(
      impl<T> State<Flatten> for $ty
      where
        T: State<Flatten>,
      {
        type Input = Self;
        type Output = Self;
      }

      impl<T> State<Flatten<Innermost>> for $ty
      where
        T: State<Flatten<Innermost>>,
      {
        type Input = T::Input;
        type Output = T::Output;
      }
    )*
  };
}

impl<'a, F, W, B, UB, T> State<Decoded<'a, F, W, B, UB>> for Option<T>
where
  T: State<Decoded<'a, F, W, B, UB>>,
  F: ?Sized,
  W: ?Sized,
  B: ?Sized,
  UB: ?Sized,
{
  type Input = T::Input;
  type Output = T::Output;
}

impl<T> State<Flatten> for Option<T>
where
  T: State<Flatten>,
{
  type Input = Self;
  type Output = T::Output;
}

impl<T> State<Flatten<Innermost>> for Option<T>
where
  T: State<Flatten<Innermost>>,
{
  type Input = T::Input;
  type Output = T::Output;
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::{boxed::Box, rc::Rc, sync::Arc};

  wrapper_impl!(
    @decoded_state
    Box<T>,
    Rc<T>,
    Arc<T>,
  );

  wrapper_impl!(
    @flatten(Sized, ?Optional)
    std::collections::VecDeque<T>,
    std::collections::LinkedList<T>,
    std::collections::BTreeSet<T>,
  );
};

#[cfg(feature = "std")]
const _: () = {
  use std::collections::HashSet;

  wrapper_impl!(
    @flatten(Sized)
    HashSet<T>,
  );
};

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  use triomphe_0_1::Arc;

  wrapper_impl!(
    @decoded_state
    Arc<T>,
  );
};
