use ghost::phantom;

pub use flatten::*;
pub use partial_transform::*;
pub use transform::*;

mod flatten;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[phantom]
pub struct Partial<F: ?Sized>;

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
#[phantom]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PartialRef<'a, RB: ?Sized, UB: ?Sized, W: ?Sized, F: ?Sized>;

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

/// A state which shows the type in reference state.
///
/// A reference is a state that represents a type that contains the full information, but the fields in the type may or may not be owned.
///
/// e.g. The below example shows a reference of a user type, the reference type contains all fields of the user type, and the type are may or may not owned.
///
/// ```ignore
/// struct User {
///   name: String,
///   email: String,
///   age: u8,
/// }
///
/// struct UserRef<'a> {
///   name: &'a str,
///   email: &'a str,
///   age: u8,
/// }
/// ```
#[phantom]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ref<'a, RB: ?Sized, UB: ?Sized, W: ?Sized, F: ?Sized>;

impl<'a, RB, UB, W, F, T> State<Ref<'a, RB, UB, W, F>> for &'a T
where
  F: ?Sized,
  W: ?Sized,
  T: State<Ref<'a, RB, UB, W, F>>,
  RB: ?Sized,
  UB: ?Sized,
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

      // wrapper_impl!(@flatten $ty);
    )*
  };
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
