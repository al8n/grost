/// The generic `S` can be any type representing a state.
pub trait State<S: ?Sized> {
  /// The output state type.
  type Output: ?Sized;
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
pub struct Partial<F: ?Sized, W: ?Sized> {
  _wf: core::marker::PhantomData<W>,
  _flavor: core::marker::PhantomData<F>,
}

impl<F, W> Clone for Partial<F, W>
where
  F: ?Sized,
  W: ?Sized,
{
  fn clone(&self) -> Self {
    *self
  }
}

impl<F, W> Copy for Partial<F, W>
where
  F: ?Sized,
  W: ?Sized,
{
}

impl<F, W, T> State<Partial<F, W>> for &T
where
  F: ?Sized,
  W: ?Sized,
  T: State<Partial<F, W>>,
{
  type Output = T::Output;
}

impl<F, W, T> State<Partial<F, W>> for &mut T
where
  F: ?Sized,
  W: ?Sized,
  T: State<Partial<F, W>>,
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
pub struct PartialRef<'a, F: ?Sized, W: ?Sized, B: ?Sized, UB: ?Sized> {
  _wf: core::marker::PhantomData<&'a W>,
  _flavor: core::marker::PhantomData<&'a F>,
  _read_buf: core::marker::PhantomData<B>,
  _unknown_buffer: core::marker::PhantomData<UB>,
}

impl<'a, F, W, B, UB> Clone for PartialRef<'a, F, W, B, UB>
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

impl<'a, F, W, B, UB> Copy for PartialRef<'a, F, W, B, UB>
where
  F: ?Sized,
  W: ?Sized,
  UB: ?Sized,
  B: ?Sized,
{
}

impl<'a, F, W, T, B, UB> State<PartialRef<'a, F, W, B, UB>> for &'a T
where
  F: ?Sized,
  W: ?Sized,
  T: State<PartialRef<'a, F, W, B, UB>>,
  B: ?Sized,
  UB: ?Sized,
{
  type Output = T::Output;
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

/// A sub-state of [`Flatten`] which means get the innermost type for flattening.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Innermost(());

impl<T: ?Sized> State<Innermost> for T {
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
  type Output = T::Output;
}

impl<S, T> State<Flatten<S>> for &mut T
where
  S: ?Sized,
  T: State<Flatten<S>> + ?Sized,
{
  type Output = T::Output;
}

macro_rules! wrapper_impl {
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
      impl<S, T> State<Flatten<S>> for $ty
      where
        T: State<Flatten<S>> + ?Sized,
        S: ?Sized,
      {
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
        type Output = Self;
      }

      impl<T> State<Flatten<Innermost>> for $ty
      where
        T: State<Flatten<Innermost>>,
      {
        type Output = T::Output;
      }
    )*
  };
}

impl<'a, F, W, B, UB, T> State<PartialRef<'a, F, W, B, UB>> for Option<T>
where
  T: State<PartialRef<'a, F, W, B, UB>>,
  F: ?Sized,
  W: ?Sized,
  B: ?Sized,
  UB: ?Sized,
{
  type Output = T::Output;
}

impl<T> State<Flatten> for Option<T>
where
  T: State<Flatten>,
{
  type Output = T::Output;
}

impl<T> State<Flatten<Innermost>> for Option<T>
where
  T: State<Flatten<Innermost>>,
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
};
