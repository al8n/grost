use crate::{decode::DecodeOwned, flavors::WireFormat};

use super::{
  buffer::Buffer,
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::Flavor,
};

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
pub struct Decoded<'a, F: ?Sized, W: ?Sized> {
  _wf: core::marker::PhantomData<&'a W>,
  _flavor: core::marker::PhantomData<&'a F>,
}

impl<'a, F, W, T> State<Decoded<'a, F, W>> for &'a T
where
  F: ?Sized,
  W: ?Sized,
  T: State<Decoded<'a, F, W>>,
{
  type Input = T::Input;
  type Output = T::Output;
}

/// A state which shows the type in optional state.
pub struct Optional;

impl<T> State<Optional> for Option<T> {
  type Input = T;
  type Output = Self;
}

impl<T> State<Optional> for &T
where
  T: State<Optional>,
{
  type Input = T::Input;
  type Output = T::Output;
}

/// A state which shows the type in base state.
pub struct Base;

impl<T: ?Sized> State<Base> for T {
  type Input = T;
  type Output = T;
}

/// A state which shows that the type is in its flatten state.
pub struct Flatten<S: ?Sized = Optional> {
  _state: core::marker::PhantomData<S>,
}

impl<S, T> State<Flatten<S>> for &T
where
  S: ?Sized,
  T: State<Flatten<S>>,
{
  type Input = T::Input;
  type Output = T::Output;
}

impl<T> State<Flatten<Base>> for [T]
where
  T: State<Flatten<Base>>,
{
  type Input = T::Input;
  type Output = T::Output;
}

impl<T> State<Flatten> for [T]
where
  T: State<Flatten>,
{
  type Input = Self;
  type Output = Self;
}

impl<T, const N: usize> State<Flatten<Base>> for [T; N]
where
  T: State<Flatten<Base>>,
{
  type Input = T::Input;
  type Output = T::Output;
}

impl<T, const N: usize> State<Flatten> for [T; N]
where
  T: State<Flatten>,
{
  type Input = Self;
  type Output = Self;
}

/// A partial message which may or may not contain all of fields of a [`Message`].
pub trait PartialMessage<F: Flavor + ?Sized, W: WireFormat<F>>: PartialEncode<F, W> {
  type UnknownBuffer<B>: Buffer<F::Unknown<B>>;

  /// A encoded representation of this type with lifetime 'a.
  ///
  /// This type can be converted back to the original type and decoded from raw bytes.
  type Decoded<'a>
  where
    Self: Sized + 'a;

  /// A borrowed view of this type with lifetime 'a.
  ///
  /// This type provides a non-owned view that can be created from a reference
  /// and encoded when needed.
  type Borrowed<'a>
  where
    Self: 'a;

  /// An owned encoded representation of this type.
  type EncodedOwned
  where
    Self: Sized + 'static;
}

/// A message type that can be encoded and decoded.
///
/// This trait defines how output types can be encoded, decoded,
/// borrowed, and converted between different representations.
///
/// * `Decoded<'a>` - A encoded representation with lifetime 'a
/// * `Borrowed<'a>` - A borrowed view with lifetime 'a
/// * `EncodedOwned` - An owned encoded representation
pub trait Message<F: Flavor + ?Sized, W: WireFormat<F>>: Encode<F, W> {
  /// The partial type of this message.
  type Partial: PartialMessage<F, W>;

  /// A encoded representation of this type with lifetime 'a.
  ///
  /// This type can be converted back to the original type and decoded from raw bytes.
  type Decoded<'a>: Copy + TypeRef<F, Self> + Encode<F, W> + Decode<'a, F, W, Self::Decoded<'a>>
  where
    Self: Sized + 'a;

  /// A borrowed view of this type with lifetime 'a.
  ///
  /// This type provides a non-owned view that can be created from a reference
  /// and encoded when needed.
  type Borrowed<'a>: Copy + TypeBorrowed<'a, F, Self> + Encode<F, W>
  where
    Self: 'a;

  /// An owned encoded representation of this type.
  type EncodedOwned: Clone
    + TypeOwned<F, Self>
    + Encode<F, W>
    + DecodeOwned<F, W, Self::EncodedOwned>
  where
    Self: Sized + 'static;
}

/// A trait for consuming `Self` and converting it to `T`.
pub trait IntoTarget<F: Flavor + ?Sized, T> {
  /// Consumes this type and converts it to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn into_target(self) -> Result<T, F::DecodeError>;
}

impl<F, T> IntoTarget<F, T> for T
where
  F: Flavor + ?Sized,
{
  fn into_target(self) -> Result<T, F::DecodeError> {
    Ok(self)
  }
}

/// A trait for types that can be converted to another type.
///
/// This trait enables bidirectional conversion between encoded
/// representations and their corresponding decoded types.
///
/// * `T` - The target type to convert to
pub trait TypeRef<F: Flavor + ?Sized, T>: IntoTarget<F, T> {
  /// Converts a reference of this type to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn to(&self) -> Result<T, F::DecodeError>;
}

impl<F, T> TypeRef<F, Self> for T
where
  F: Flavor + ?Sized,
  T: Clone,
{
  fn to(&self) -> Result<T, <F as Flavor>::DecodeError> {
    Ok(self.clone())
  }
}

/// A trait for types that can be converted to another type.
///
/// This trait enables bidirectional conversion between encoded
/// representations and their corresponding decoded types.
///
/// * `T` - The target type to convert to
pub trait TypeBorrowed<'a, F: Flavor + ?Sized, T: ?Sized> {
  /// Converts a reference of this type to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn from_borrow(val: &'a T) -> Self;
}

impl<'a, F: Flavor, T: ?Sized> TypeBorrowed<'a, F, T> for &'a T {
  fn from_borrow(val: &'a T) -> Self {
    val
  }
}

/// A trait for types that can be converted to another type.
///
/// This trait enables bidirectional conversion between encoded
/// representations and their corresponding decoded types.
///
/// * `T` - The target type to convert to
pub trait TypeOwned<F: Flavor + ?Sized, T>: IntoTarget<F, T> {
  /// Converts a reference of this type to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn to(&self) -> Result<T, F::DecodeError>;
}

impl<F, T> TypeOwned<F, Self> for T
where
  F: Flavor + ?Sized,
  T: Clone,
{
  fn to(&self) -> Result<T, <F as Flavor>::DecodeError> {
    Ok(self.clone())
  }
}

macro_rules! wrapper_impl {
  (@encoded_state $($ty:ty),+$(,)?) => {
    $(
      impl<'a, F, W, T> State<Decoded<'a, F, W>> for $ty
      where
        T: State<Decoded<'a, F, W>> + ?Sized,
        F: ?Sized,
        W: ?Sized,
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

      impl<T> State<Flatten<Base>> for $ty
      where
        T: State<Flatten<Base>>,
      {
        type Input = T::Input;
        type Output = T::Output;
      }
    )*
  };
  (@into_target $($ty:ty:$constructor:ident),+$(,)?) => {
    $(
      impl<F, T> IntoTarget<F, $ty> for T
      where
        T: IntoTarget<F, T>,
        F: Flavor + ?Sized,
      {
        fn into_target(self) -> Result<$ty, F::DecodeError> {
          Ok(<$ty>::$constructor(self))
        }
      }
    )*
  };
  (@type_ref $($ty:ty:$constructor:ident),+$(,)?) => {
    $(
      impl<F, T> TypeRef<F, $ty> for T
      where
        T: TypeRef<F, T> + Clone,
        F: Flavor + ?Sized,
      {
        fn to(&self) -> Result<$ty, F::DecodeError> {
          Ok(<$ty>::$constructor(self.clone()))
        }
      }
    )*
  };
  (@type_owned $($ty:ty:$constructor:ident),+$(,)?) => {
    $(
      impl<F, T> TypeOwned<F, $ty> for T
      where
        T: TypeOwned<F, T> + Clone,
        F: Flavor + ?Sized,
      {
        fn to(&self) -> Result<$ty, F::DecodeError> {
          Ok(<$ty>::$constructor(self.clone()))
        }
      }
    )*
  };
  (@type_borrowed $($ty:ty),+$(,)?) => {
    $(
      impl<'a, F: Flavor, T: ?Sized> TypeBorrowed<'a, F, $ty> for &'a T {
        fn from_borrow(val: &'a $ty) -> Self {
          ::core::ops::Deref::deref(val)
        }
      }
    )*
  };
  (@partial_message $($ty:ty),+$(,)?) => {
    $(
      impl<T, F, W> PartialMessage<F, W> for $ty
      where
        T: PartialMessage<F, W> + Clone + 'static,
        for<'a> T::Decoded<'a>: TypeRef<F, $ty>,
        for<'a> T::Borrowed<'a>: TypeBorrowed<'a, F, $ty>,
        T::EncodedOwned: TypeOwned<F, $ty>,
        F: Flavor + ?Sized,
        W: WireFormat<F>,
      {
        type UnknownBuffer<B> = T::UnknownBuffer<B>;

        type Decoded<'a> = T::Decoded<'a>;
        type Borrowed<'a> = T::Borrowed<'a>;
        type EncodedOwned = T::EncodedOwned;
      }
    )*
  };
  (@message $($ty:ty),+$(,)?) => {
    $(
      impl<T, F, W> Message<F, W> for $ty
      where
        T: Message<F, W> + Clone + 'static,
        for<'a> T::Decoded<'a>: TypeRef<F, $ty>,
        for<'a> T::Borrowed<'a>: TypeBorrowed<'a, F, $ty>,
        T::EncodedOwned: TypeOwned<F, $ty>,
        F: Flavor + ?Sized,
        W: WireFormat<F>,
      {
        type Partial = T::Partial;

        type Decoded<'a> = T::Decoded<'a>;
        type Borrowed<'a> = T::Borrowed<'a>;
        type EncodedOwned = T::EncodedOwned;
      }
    )*
  }
}

wrapper_impl!(@into_target Option<T>:Some);
wrapper_impl!(@type_ref Option<T>:Some);
wrapper_impl!(@type_owned Option<T>:Some);
wrapper_impl!(@partial_message Option<T>);
wrapper_impl!(@message Option<T>);

impl<'a, F, W, T> State<Decoded<'a, F, W>> for Option<T>
where
  T: State<Decoded<'a, F, W>>,
  F: ?Sized,
  W: ?Sized,
{
  type Input = T::Input;
  type Output = T::Output;
}

impl<T> State<Flatten> for Option<T>
where
  T: State<Flatten>,
{
  type Input = T::Input;
  type Output = T::Output;
}

impl<T> State<Flatten<Base>> for Option<T>
where
  T: State<Flatten<Base>>,
{
  type Input = T::Input;
  type Output = T::Output;
}

impl<'a, T, F> TypeBorrowed<'a, F, T> for Option<&'a T>
where
  T: TypeBorrowed<'a, F, T>,
  F: Flavor + ?Sized,
{
  fn from_borrow(val: &'a T) -> Self {
    Some(val)
  }
}

impl<'a, T, F> TypeBorrowed<'a, F, Option<T>> for Option<&'a T>
where
  T: TypeBorrowed<'a, F, T>,
  F: Flavor + ?Sized,
{
  fn from_borrow(val: &'a Option<T>) -> Self {
    val.as_ref()
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::{boxed::Box, rc::Rc, sync::Arc};

  wrapper_impl!(
    @into_target
    Box<T>:new,
    Rc<T>:new,
    Arc<T>:new,
  );
  wrapper_impl!(
    @type_ref
    Box<T>:new,
    Rc<T>:new,
    Arc<T>:new,
  );
  wrapper_impl!(
    @type_owned
    Box<T>:new,
    Rc<T>:new,
    Arc<T>:new,
  );
  wrapper_impl!(
    @type_borrowed
    Box<T>,
    Rc<T>,
    Arc<T>,
  );
  wrapper_impl!(
    @partial_message
    Box<T>,
    Rc<T>,
    Arc<T>,
  );
  wrapper_impl!(
    @message
    Box<T>,
    Rc<T>,
    Arc<T>,
  );
  wrapper_impl!(
    @encoded_state
    Box<T>,
    Rc<T>,
    Arc<T>,
  );

  wrapper_impl!(
    @flatten(Sized, ?Optional)
    std::vec::Vec<T>,
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
    @into_target
    Arc<T>:new,
  );
  wrapper_impl!(
    @type_ref
    Arc<T>:new,
  );
  wrapper_impl!(
    @type_owned
    Arc<T>:new,
  );
  wrapper_impl!(
    @type_borrowed
    Arc<T>,
  );
  wrapper_impl!(
    @partial_message
    Arc<T>,
  );
  wrapper_impl!(
    @message
    Arc<T>,
  );
  wrapper_impl!(
    @encoded_state
    Arc<T>,
  );
};
