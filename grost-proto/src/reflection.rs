use core::marker::PhantomData;

use super::flavors::Flavor;

pub use enum_::*;
pub use struct_::*;

macro_rules! phantom {
  ($(
    $(#[$meta:meta])*
    $name:ident
  ),+$(,)?) => {
    paste::paste! {
      $(
        $(#[$meta])*
        pub struct $name<R: ?::core::marker::Sized>(::core::marker::PhantomData<R>);

        impl<R: ?::core::marker::Sized> ::core::clone::Clone for $name<R> {
          fn clone(&self) -> Self {
            *self
          }
        }

        impl<R: ?::core::marker::Sized> ::core::marker::Copy for $name<R> {}

        impl<R: ?::core::marker::Sized> ::core::default::Default for $name<R> {
          fn default() -> Self {
            Self::new()
          }
        }

        impl<R: ?::core::marker::Sized> $name<R> {
          #[doc = "creates a new `" $name "`"]
          #[inline]
          pub const fn new() -> Self {
            Self(::core::marker::PhantomData)
          }
        }
      )*
    }
  };
}

/// Reflection for encoding.
pub mod encode;

mod enum_;
mod struct_;

/// The type in the Graph protocol schema
#[derive(Debug)]
pub enum Type<F: Flavor + ?Sized> {
  Primitive {
    name: &'static str,
    description: &'static str,
  },
  List(&'static Type<F>),
  Map {
    key: &'static Type<F>,
    value: &'static Type<F>,
  },
  Optional(&'static Type<F>),
  Struct(&'static StructReflection<F>),
  UintEnum(EnumReflection),
  Union(),
  Interface(),
}

impl<F: Flavor + ?Sized> Clone for Type<F> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<F: Flavor + ?Sized> Copy for Type<F> {}

phantom!(
  /// Reflection to the identifier of a field
  IdentifierReflection,
  /// Reflection to the encoded identifier of a field
  EncodedIdentifierReflection,
  /// Reflection to the tag of a field
  TagReflection,
  /// Reflection to the encoded tag of a field
  EncodedTagReflection,
  /// Reflection to the wire type of a field
  WireTypeReflection,
  /// Reflection to length related
  Len
);

/// Reflectable.
pub trait Reflectable<F: Flavor + ?Sized> {
  type Reflection: ?Sized + 'static;

  /// The reflection of this type
  const REFLECTION: &Self::Reflection;
}

/// A phantom relection type which can be dereferenced to [`Reflectable::REFLECTION`].
#[repr(transparent)]
pub struct Reflection<T: ?Sized, R: ?Sized, F: ?Sized> {
  _r: PhantomData<R>,
  _f: PhantomData<F>,
  t: T,
}

impl<T, R, F> Reflection<T, R, F>
where
  R: ?Sized + 'static,
  F: Flavor + ?Sized,
{
  /// Creates a new [`Reflection`].
  #[inline]
  pub const fn new(t: T) -> Self {
    Self {
      _r: PhantomData,
      _f: PhantomData,
      t,
    }
  }
}

// impl<T, R, F> Clone for Reflection<T, R, F>
// where
//   T: Reflectable<R, F> + Clone,
//   R: ?Sized + 'static,
//   F: Flavor + ?Sized,
// {
//   fn clone(&self) -> Self {
//     Self {
//       _r: PhantomData,
//       _f: PhantomData,
//       t: self.t.clone(),
//     }
//   }
// }

// impl<T, R, F> Copy for Reflection<T, R, F>
// where
//   T: Reflectable<R, F> + Copy,
//   R: ?Sized + 'static,
//   F: Flavor + ?Sized,
// {
// }

// impl<T, R, F> core::ops::Deref for Reflection<T, R, F>
// where
//   T: Reflectable<R, F>,
//   R: ?Sized + 'static,
//   F: Flavor + ?Sized,
// {
//   type Target = R;

//   fn deref(&self) -> &Self::Target {
//     T::REFLECTION
//   }
// }
