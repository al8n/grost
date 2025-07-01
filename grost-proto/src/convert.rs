pub use partial_transform::*;
pub use transform::*;

mod partial_transform;
mod transform;

/// The generic `S` can be any type representing a state.
pub trait State<S: ?Sized> {
  /// The output state type.
  type Output: ?Sized;
}

/// A state which yields the type itself.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identity;

impl<T> State<Identity> for T {
  type Output = T;
}

/// A state which shows the type in partial state.
///
/// A partial is a state that represents a type that contains partial information.
///
/// e.g. The below example shows a partial of a user type, the partial type may not contains all fields of the user type.
///
/// ```ignore
/// struct User {
///   name: String,
///   email: String,
///   age: u8,
/// }
///
/// struct PartialUser {
///   name: Option<String>,
///   email: Option<String>,
///   age: Option<u8>,
/// }
/// ```
///
/// See also [`PartialRef`] state.
pub struct Partial<F: ?Sized> {
  _flavor: core::marker::PhantomData<F>,
}

impl<F> Clone for Partial<F>
where
  F: ?Sized,
{
  fn clone(&self) -> Self {
    *self
  }
}

impl<F> Copy for Partial<F> where F: ?Sized {}

impl<F, T> State<Partial<F>> for &T
where
  F: ?Sized,
  T: State<Partial<F>>,
{
  type Output = T::Output;
}

impl<F, T> State<Partial<F>> for &mut T
where
  F: ?Sized,
  T: State<Partial<F>>,
{
  type Output = T::Output;
}

/// A state which shows the type in partial reference state.
///
/// A partial reference is a state that represents a type that contains partial information, and the fields in the type may or may not be fully decoded.
///
/// e.g. The below example shows a partial reference of a user type, the partial reference type may not contains all fields of the user type, and the type are partially decoded.
///
/// ```ignore
/// struct User {
///   name: String,
///   email: String,
///   age: u8,
/// }
///
/// struct PartialUserRef<'a> {
///   name: Option<&'a str>,
///   email: Option<&'a str>,
///   age: Option<u8>,
/// }
/// ```
pub struct PartialRef<'a, RB: ?Sized, UB: ?Sized, W: ?Sized, F: ?Sized> {
  _wf: core::marker::PhantomData<&'a W>,
  _flavor: core::marker::PhantomData<&'a F>,
  _read_buf: core::marker::PhantomData<RB>,
  _buffer: core::marker::PhantomData<UB>,
}

impl<'a, RB, UB, W, F> Clone for PartialRef<'a, RB, UB, W, F>
where
  F: ?Sized,
  W: ?Sized,
  UB: ?Sized,
  RB: ?Sized,
{
  fn clone(&self) -> Self {
    *self
  }
}

impl<'a, RB, UB, W, F> Copy for PartialRef<'a, RB, UB, W, F>
where
  F: ?Sized,
  W: ?Sized,
  UB: ?Sized,
  RB: ?Sized,
{
}

impl<'a, RB, UB, W, F, T> State<PartialRef<'a, RB, UB, W, F>> for &'a T
where
  F: ?Sized,
  W: ?Sized,
  T: State<PartialRef<'a, RB, UB, W, F>>,
  RB: ?Sized,
  UB: ?Sized,
{
  type Output = T::Output;
}

/// A sub-state of [`Flattened`] which means get the innermost type for flattening.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Innermost(());

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
pub struct Flattened<S: ?Sized = Option<Innermost>> {
  _state: core::marker::PhantomData<S>,
}

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

#[allow(dead_code)]
macro_rules! wrapper_impl {
  (@partial_state $($ty:ty => $output:ty),+$(,)?) => {
    $(
      impl<'a, F, T> State<Partial<F>> for $ty
      where
        T: State<Partial<F>> + ?Sized,
        F: ?Sized,
      {
        type Output = $output;
      }
    )*
  };
  (@partial_ref_state $($ty:ty),+$(,)?) => {
    $(
      impl<'a, F, W, B, UB, T> State<PartialRef<'a, F, W, B, UB>> for $ty
      where
        T: State<PartialRef<'a, F, W, B, UB>> + ?Sized,
        F: ?Sized,
        W: ?Sized,
        B: ?Sized,
        UB: ?Sized,
      {
        type Output = T::Output;
      }

      wrapper_impl!(@flatten $ty);
    )*
  };
  (@flatten $($ty:ty),+$(,)?) => {
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
  (@flatten(Sized) $($ty:ty),+$(,)?) => {
    $(
      impl<S, T> State<Flattened<S>> for $ty
      where
        T: State<Flattened<S>>,
      {
        type Output = T::Output;
      }
    )*
  };
  (@flatten(Sized, ?Optional) $($ty:ty),+$(,)?) => {
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

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::{boxed::Box, rc::Rc, sync::Arc};

  wrapper_impl!(
    @partial_ref_state
    Box<T>,
    Rc<T>,
    Arc<T>,
  );

  wrapper_impl!(
    @partial_state
    Box<T> => Box<T::Output>,
    Rc<T> => Rc<T::Output>,
    Arc<T> => Arc<T::Output>,
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
    @partial_ref_state
    Arc<T>,
  );
  wrapper_impl!(
    @partial_state
    Arc<T> => Arc<T::Output>,
  );
};
