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
pub trait Reflectable<R: ?Sized + 'static, F: Flavor + ?Sized> {
  /// The reflection of this type
  const REFLECTION: &R;
}

pub struct CommentField<F: Flavor + ?Sized, const TAG: u32>(core::marker::PhantomData<F>);

impl<F: Flavor + ?Sized> Reflectable<u128, F> for CommentField<F, 1> {
  const REFLECTION: &u128 = &1;
}

impl<F: Flavor + ?Sized> Reflectable<u128, F> for CommentField<F, 2> {
  const REFLECTION: &u128 = &2;
}
