use super::{
  super::{Reflectable, SchemaTypeReflection},
  Enum, Object, Scalar,
};

macro_rules! impl_type_reflection {
  (@map $(@<$g:ident>)? $ty:ty) => {
    impl <K, V, $($g)?> $crate::__private::reflection::Reflectable<$ty> for $crate::__private::reflection::SchemaTypeReflection<$ty>
    where
      $crate::__private::reflection::SchemaTypeReflection<K>: $crate::__private::reflection::Reflectable<K, Reflection = $crate::__private::reflection::SchemaType>,
      $crate::__private::reflection::SchemaTypeReflection<V>: $crate::__private::reflection::Reflectable<V, Reflection = $crate::__private::reflection::SchemaType>,
    {
      type Reflection = $crate::__private::reflection::SchemaType;

      const REFLECTION: &'static Self::Reflection =
        &SchemaType::Map {
          key: <$crate::__private::reflection::SchemaTypeReflection<K> as $crate::__private::reflection::Reflectable<K>>::REFLECTION,
          value: <$crate::__private::reflection::SchemaTypeReflection<V> as $crate::__private::reflection::Reflectable<V>>::REFLECTION,
        };
    }
  };
  (@set $(@<$g:ident>)? $ty:ty) => {
    impl<T, $($g)?> $crate::__private::reflection::Reflectable<$ty> for $crate::__private::reflection::SchemaTypeReflection<$ty>
    where
      $crate::__private::reflection::SchemaTypeReflection<T>: $crate::__private::reflection::Reflectable<T, Reflection = $crate::__private::reflection::SchemaType>,
    {
      type Reflection = $crate::__private::reflection::SchemaType;

      const REFLECTION: &'static Self::Reflection = {
        &$crate::__private::reflection::SchemaType::Set(
          <$crate::__private::reflection::SchemaTypeReflection<T> as $crate::__private::reflection::Reflectable<T>>::REFLECTION
        )
      };
    }
  };
  (@wrapper $($ty:ty $([ ?$sized:path ])?),+$(,)?) => {
    $(
      impl<T> $crate::reflection::Reflectable<$ty> for $crate::reflection::SchemaTypeReflection<$ty>
      where
        $crate::reflection::SchemaTypeReflection<T>: $crate::reflection::Reflectable<T, Reflection = $crate::reflection::SchemaType>,
        $(T: ?$sized)?
      {
        type Reflection = $crate::reflection::SchemaType;

        const REFLECTION: &'static Self::Reflection = <$crate::reflection::SchemaTypeReflection<T> as $crate::reflection::Reflectable<T>>::REFLECTION;
      }

      impl<T> $crate::reflection::Reflectable<$ty> for $crate::reflection::SchemaTypeReflection<T>
      where
        $crate::reflection::SchemaTypeReflection<T>: $crate::reflection::Reflectable<T, Reflection = $crate::reflection::SchemaType>,
        $(T: ?$sized)?
      {
        type Reflection = $crate::reflection::SchemaType;

        const REFLECTION: &'static Self::Reflection = <$crate::reflection::SchemaTypeReflection<T> as $crate::reflection::Reflectable<T>>::REFLECTION;
      }

      impl<T> $crate::reflection::Reflectable<T> for $crate::reflection::SchemaTypeReflection<$ty>
      where
        $crate::reflection::SchemaTypeReflection<T>: $crate::reflection::Reflectable<T, Reflection = $crate::reflection::SchemaType>,
        $(T: ?$sized)?
      {
        type Reflection = $crate::reflection::SchemaType;

        const REFLECTION: &'static Self::Reflection = <$crate::reflection::SchemaTypeReflection<T> as $crate::reflection::Reflectable<T>>::REFLECTION;
      }
    )*
  };
  (@list $($ty:ty),+$(,)?) => {
    $(
      impl<T> $crate::__private::reflection::Reflectable<$ty> for $crate::__private::reflection::SchemaTypeReflection<$ty>
      where
        $crate::__private::reflection::SchemaTypeReflection<T>: $crate::__private::reflection::Reflectable<T, Reflection = $crate::__private::reflection::SchemaType>,
      {
        type Reflection = $crate::__private::reflection::SchemaType;

        const REFLECTION: &'static Self::Reflection = {
          if <$crate::__private::reflection::SchemaTypeReflection<T> as $crate::__private::reflection::Reflectable<T>>::REFLECTION.is_byte() {
            &$crate::__private::reflection::SchemaType::bytes()
          } else {
            &$crate::__private::reflection::SchemaType::List(
              <$crate::__private::reflection::SchemaTypeReflection<T> as $crate::__private::reflection::Reflectable<T>>::REFLECTION
            )
          }
        };
      }
    )*
  };
  (@array $($ty:ty),+$(,)?) => {
    $(
      impl<T, const N: usize> $crate::__private::reflection::Reflectable<$ty> for $crate::__private::reflection::SchemaTypeReflection<$ty>
      where
        $crate::__private::reflection::SchemaTypeReflection<T>: $crate::__private::reflection::Reflectable<T, Reflection = $crate::__private::reflection::SchemaType>,
      {
        type Reflection = $crate::__private::reflection::SchemaType;

        const REFLECTION: &'static Self::Reflection = {
          if <$crate::__private::reflection::SchemaTypeReflection<T> as $crate::__private::reflection::Reflectable<T>>::REFLECTION.is_byte() {
            &$crate::__private::reflection::SchemaType::bytes()
          } else {
            &$crate::__private::reflection::SchemaType::List(
              <$crate::__private::reflection::SchemaTypeReflection<T> as $crate::__private::reflection::Reflectable<T>>::REFLECTION
            )
          }
        };
      }
    )*
  };
  (@tinyvec $($ty:ty),+$(,)?) => {
    $(
      impl<T, A> $crate::__private::reflection::Reflectable<$ty> for $crate::__private::reflection::SchemaTypeReflection<$ty>
      where
        A: tinyvec_1::Array<Item = T>,
        $crate::__private::reflection::SchemaTypeReflection<T>: $crate::__private::reflection::Reflectable<T, Reflection = $crate::__private::reflection::SchemaType>,
      {
        type Reflection = $crate::__private::reflection::SchemaType;

        const REFLECTION: &'static Self::Reflection = {
          if <$crate::__private::reflection::SchemaTypeReflection<T> as $crate::__private::reflection::Reflectable<T>>::REFLECTION.is_byte() {
            &$crate::__private::reflection::SchemaType::bytes()
          } else {
            &$crate::__private::reflection::SchemaType::List(
              <$crate::__private::reflection::SchemaTypeReflection<T> as $crate::__private::reflection::Reflectable<T>>::REFLECTION
            )
          }
        };
      }
    )*
  };
}

impl_type_reflection!(@list [T]);
impl_type_reflection!(@array [T; N]);

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
    sync::Arc,
    vec::Vec,
  };

  impl_type_reflection!(@map BTreeMap<K, V>);
  impl_type_reflection!(@set BTreeSet<T>);
  impl_type_reflection!(@list Vec<T>);
  impl_type_reflection!(@wrapper
    Box<T> [?Sized],
    Rc<T> [?Sized],
    Arc<T> [?Sized],
  );
};

#[cfg(feature = "std")]
const _: () = {
  impl_type_reflection!(@map @<S> std::collections::HashMap<K, V, S>);
  impl_type_reflection!(@set @<S> std::collections::HashSet<T, S>);
};

#[cfg(feature = "hashbrown_0_15")]
const _: () = {
  impl_type_reflection!(@map @<S> hashbrown_0_15::HashMap<K, V, S>);
  impl_type_reflection!(@set @<S> hashbrown_0_15::HashSet<T, S>);
};

#[cfg(feature = "indexmap_2")]
const _: () = {
  impl_type_reflection!(@map @<S> indexmap_2::IndexMap<K, V, S>);
  impl_type_reflection!(@set @<S> indexmap_2::IndexSet<T, S>);
};

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  impl_type_reflection!(@wrapper
    triomphe_0_1::Arc<T> [?Sized],
  );
};

#[cfg(feature = "tinyvec_1")]
const _: () = {
  impl_type_reflection!(@tinyvec tinyvec_1::ArrayVec<A>);

  #[cfg(any(feature = "alloc", feature = "std"))]
  impl_type_reflection!(@tinyvec tinyvec_1::TinyVec<A>);
};

#[cfg(feature = "arrayvec_0_7")]
const _: () = {
  impl_type_reflection!(@array arrayvec_0_7::ArrayVec<T, N>);
};

#[cfg(feature = "smallvec_1")]
const _: () = {
  impl_type_reflection!(@array smallvec_1::SmallVec<[T; N]>);
};

impl<T> Reflectable<Option<T>> for SchemaTypeReflection<Option<T>>
where
  SchemaTypeReflection<T>: Reflectable<T, Reflection = SchemaType>,
{
  type Reflection = SchemaType;

  const REFLECTION: &'static Self::Reflection =
    &SchemaType::Optional(<SchemaTypeReflection<T> as Reflectable<T>>::REFLECTION);
}

impl<T> Reflectable<T> for SchemaTypeReflection<Option<T>>
where
  SchemaTypeReflection<T>: Reflectable<T, Reflection = SchemaType>,
{
  type Reflection = SchemaType;

  const REFLECTION: &'static Self::Reflection =
    &SchemaType::Optional(<SchemaTypeReflection<T> as Reflectable<T>>::REFLECTION);
}

impl<T> Reflectable<Option<T>> for SchemaTypeReflection<T>
where
  SchemaTypeReflection<T>: Reflectable<T, Reflection = SchemaType>,
{
  type Reflection = SchemaType;

  const REFLECTION: &'static Self::Reflection =
    &SchemaType::Optional(<SchemaTypeReflection<T> as Reflectable<T>>::REFLECTION);
}

impl<T> Reflectable<&T> for SchemaTypeReflection<&T>
where
  SchemaTypeReflection<T>: Reflectable<T, Reflection = SchemaType>,
{
  type Reflection = SchemaType;

  const REFLECTION: &'static Self::Reflection =
    <SchemaTypeReflection<T> as Reflectable<T>>::REFLECTION;
}

impl<T> Reflectable<T> for SchemaTypeReflection<&T>
where
  SchemaTypeReflection<T>: Reflectable<T, Reflection = SchemaType>,
{
  type Reflection = SchemaType;

  const REFLECTION: &'static Self::Reflection =
    <SchemaTypeReflection<T> as Reflectable<T>>::REFLECTION;
}

impl<T> Reflectable<&T> for SchemaTypeReflection<T>
where
  SchemaTypeReflection<T>: Reflectable<T, Reflection = SchemaType>,
{
  type Reflection = SchemaType;

  const REFLECTION: &'static Self::Reflection =
    <SchemaTypeReflection<T> as Reflectable<T>>::REFLECTION;
}

/// The type in the Graph protocol schema
#[derive(Debug)]
pub enum SchemaType {
  Scalar(Scalar),
  List(&'static SchemaType),
  Set(&'static SchemaType),
  Map {
    key: &'static SchemaType,
    value: &'static SchemaType,
  },
  Optional(&'static SchemaType),
  Object(&'static Object),
  Enum(&'static Enum),
  Union(),
  Interface(),
}

impl Clone for SchemaType {
  fn clone(&self) -> Self {
    *self
  }
}

impl Copy for SchemaType {}

impl SchemaType {
  /// Construct a
  pub const fn custom_scalar(name: &'static str, description: &'static str) -> Self {
    Self::Scalar(Scalar::custom(name, description))
  }

  /// Creates a string schema type
  pub const fn string() -> Self {
    Self::Scalar(Scalar::String)
  }

  /// Creates a bytes schema type
  pub const fn bytes() -> Self {
    Self::Scalar(Scalar::Bytes)
  }

  /// Creates a boolean schema type
  pub const fn duration() -> Self {
    Self::Scalar(Scalar::Duration)
  }

  /// Creates a UTC schema type
  pub const fn utc() -> Self {
    Self::Scalar(Scalar::Utc)
  }

  /// Returns `true` if this type is `byte` or `u8`
  pub const fn is_byte(self) -> bool {
    match self {
      Self::Scalar(scalar) => scalar.is_byte(),
      _ => false,
    }
  }
}
