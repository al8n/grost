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

macro_rules! zst {
  ($(
    $(#[$meta:meta])*
    $name:ident
  ),+$(,)?) => {
    paste::paste! {
      $(
        $(#[$meta])*
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name;
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
  Scalar {
    name: &'static str,
    description: &'static str,
  },
  List(&'static Type<F>),
  Map {
    key: &'static Type<F>,
    value: &'static Type<F>,
  },
  Optional(&'static Type<F>),
  Object(&'static ObjectReflection<F>),
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

impl<F: Flavor + ?Sized> Type<F> {
  /// Construct a scalar type
  pub const fn scalar(name: &'static str, description: &'static str) -> Self {
    Self::Scalar { name, description }
  }

  /// Creates a string schema type
  pub const fn string() -> Self {
    Self::scalar("string", "A string")
  }

  /// Creates a bytes schema type
  pub const fn bytes() -> Self {
    Self::scalar("bytes", "A bytes")
  }

  /// Creates a boolean schema type
  pub const fn duration() -> Self {
    Self::scalar("Duration", "A duration")
  }

  /// Creates a UTC schema type
  pub const fn utc() -> Self {
    Self::scalar("Utc", "A UTC")
  }

  /// Returns `true` if this type is `byte` or `u8`
  pub const fn is_byte(self) -> bool {
    match self {
      Type::Scalar { name, .. } => matches!(name.as_bytes(), b"byte" | b"u8"),
      _ => false,
    }
  }
}

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
  Len,
  /// Reflection to the schema type
  SchemaTypeReflection,
);

zst!(
  /// Reflection to the wire format of a field
  WireFormatReflection
);

/// Reflectable.
pub trait Reflectable<F: ?Sized> {
  type Reflection: ?Sized + 'static;

  /// The reflection of this type
  const REFLECTION: &Self::Reflection;
}

impl<T, F: Flavor + ?Sized> Reflectable<F> for SchemaTypeReflection<&T>
where
  SchemaTypeReflection<T>: Reflectable<F, Reflection = Type<F>>,
{
  type Reflection = <SchemaTypeReflection<T> as Reflectable<F>>::Reflection;

  const REFLECTION: &'static Self::Reflection =
    <SchemaTypeReflection<T> as Reflectable<F>>::REFLECTION;
}

impl<T, F: Flavor + ?Sized> Reflectable<F> for SchemaTypeReflection<Option<T>>
where
  SchemaTypeReflection<T>: Reflectable<F, Reflection = Type<F>>,
{
  type Reflection = <SchemaTypeReflection<T> as Reflectable<F>>::Reflection;

  const REFLECTION: &'static Self::Reflection =
    &Type::Optional(<SchemaTypeReflection<T> as Reflectable<F>>::REFLECTION);
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
