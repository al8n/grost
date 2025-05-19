use core::marker::PhantomData;

pub use schema::*;
pub use wire_format::WireFormatReflection;

macro_rules! phantom {
  ($(
    $(#[$meta:meta])*
    $name:ident
  ),+$(,)?) => {
    paste::paste! {
      $(
        $(#[$meta])*
        #[repr(transparent)]
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

#[allow(unused_macros)]
macro_rules! zst {
  ($(
    $(#[$meta:meta])*
    $name:ident
  ),+$(,)?) => {
    paste::paste! {
      $(
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name;
      )*
    }
  };
}

mod decoded;
mod identifier;
mod schema;
mod tag;
mod wire_format;
mod wire_type;

phantom!(
  /// Reflection to the identifier of a field
  IdentifierReflection,
  /// Reflection to the tag of a field
  TagReflection,
  /// Reflection to the wire type of a field
  WireTypeReflection,
  /// Reflection to length related
  Len,
  /// Reflection to an encode fn.
  EncodeReflection,
  /// Reflection to an partial encode fn.
  PartialEncodeReflection,
);

/// Reflectable.
pub trait Reflectable<F: ?Sized> {
  type Reflection: ?Sized + 'static;

  /// The reflection of this type
  const REFLECTION: &Self::Reflection;
}

macro_rules! blank_type_reflection_impl {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<T: ?Sized, F: ?Sized> Reflectable<T> for Reflection<$ty, Type, F>
      where
        Reflection<T, Type, F>: Reflectable<F, Reflection = Type>,
      {
        type Reflection = Type;

        const REFLECTION: &'static Self::Reflection =
          <Reflection<T, Type, F> as Reflectable<F>>::REFLECTION;
      }
    )*
  };
}

blank_type_reflection_impl!(&T, ::core::marker::PhantomData<T>);

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  blank_type_reflection_impl!(std::boxed::Box<T>, std::rc::Rc<T>, std::sync::Arc<T>);
};

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  blank_type_reflection_impl!(triomphe_0_1::Arc<T>);
};

/// A phantom relection type which can be dereferenced to [`Reflectable::REFLECTION`].
#[repr(transparent)]
pub struct Reflection<T: ?Sized, R: ?Sized, F: ?Sized> {
  _r: PhantomData<R>,
  _f: PhantomData<F>,
  _t: PhantomData<T>,
}

impl<T, R, F> Default for Reflection<T, R, F>
where
  T: ?Sized,
  R: ?Sized,
  F: ?Sized,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<T, R, F> Reflection<T, R, F>
where
  T: ?Sized,
  R: ?Sized,
  F: ?Sized,
{
  /// Creates a new [`Reflection`].
  #[inline]
  pub const fn new() -> Self {
    Self {
      _r: PhantomData,
      _f: PhantomData,
      _t: PhantomData,
    }
  }
}

impl<T, R, F> core::fmt::Debug for Reflection<T, R, F>
where
  T: ?Sized,
  R: ?Sized,
  F: ?Sized,
  Self: Reflectable<T>,
  <Self as Reflectable<T>>::Reflection: core::fmt::Debug,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::ops::Deref::deref(self).fmt(f)
  }
}

impl<T, R, F> core::fmt::Display for Reflection<T, R, F>
where
  T: ?Sized,
  R: ?Sized,
  F: ?Sized,
  Self: Reflectable<T>,
  <Self as Reflectable<T>>::Reflection: core::fmt::Display,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::ops::Deref::deref(self).fmt(f)
  }
}

impl<T, R, F> core::ops::Deref for Reflection<T, R, F>
where
  T: ?Sized,
  R: ?Sized,
  F: ?Sized,
  Self: Reflectable<T>,
{
  type Target = <Self as Reflectable<T>>::Reflection;

  fn deref(&self) -> &Self::Target {
    Self::REFLECTION
  }
}

// #[allow(clippy::type_complexity)]
// impl<T, F, const TAG: u32> Reflection<T, Identified<ObjectField, TAG>, F>
// where
//   T: ?Sized,
//   F: ?Sized + Flavor,
//   Self: Reflectable<T, Reflection = ObjectField>,
// {
//   /// Returns the reflection to the encoded identifier of the field.
//   #[inline]
//   pub const fn encoded_identifier(
//     &self,
//   ) -> Reflection<T, Identified<EncodeReflection<IdentifierReflection<F::Identifier>>, TAG>, F>
//   where
//     Reflection<T, Identified<EncodeReflection<IdentifierReflection<F::Identifier>>, TAG>, F>:
//       Reflectable<T, Reflection = [u8]>,
//   {
//     Reflection::new()
//   }

//   /// Returns the reflection to the length of the encoded identifier of the field.
//   #[inline]
//   pub const fn encoded_identifier_len(
//     &self,
//   ) -> Reflection<T, Identified<EncodeReflection<Len<IdentifierReflection<F::Identifier>>>, TAG>, F>
//   where
//     Reflection<T, Identified<EncodeReflection<Len<IdentifierReflection<F::Identifier>>>, TAG>, F>:
//       Reflectable<T, Reflection = usize>,
//   {
//     Reflection::new()
//   }

//   /// Returns the reflection to the encoded tag of the field.
//   #[inline]
//   pub const fn encoded_tag(
//     &self,
//   ) -> Reflection<T, Identified<EncodeReflection<TagReflection<F::Tag>>, TAG>, F>
//   where
//     Reflection<T, Identified<EncodeReflection<TagReflection<F::Tag>>, TAG>, F>:
//       Reflectable<T, Reflection = [u8]>,
//   {
//     Reflection::new()
//   }

//   /// Returns the reflection to the length of the encoded tag of the field.
//   #[inline]
//   pub const fn encoded_tag_len(
//     &self,
//   ) -> Reflection<T, Identified<EncodeReflection<Len<TagReflection<F::Tag>>>, TAG>, F>
//   where
//     Reflection<T, Identified<EncodeReflection<Len<TagReflection<F::Tag>>>, TAG>, F>:
//       Reflectable<T, Reflection = usize>,
//   {
//     Reflection::new()
//   }

//   /// Returns the reflection to the corresponding field in decoded struct.
//   #[inline]
//   pub const fn decoded(
//     &self,
//   ) -> decoded::DecodedReflection<T, Identified<ObjectField, TAG>, F>
//   {
//     decoded::DecodedReflection::new()
//   }

//   /// Returns the reflection to the encode fn of the field.
//   #[inline]
//   pub const fn encode(
//     &self,
//   ) -> Reflection<
//     T,
//     Identified<
//       EncodeReflection<
//         <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//       >,
//       TAG,
//     >,
//     F,
//   >
//   where
//     Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
//     Reflection<
//       T,
//       Identified<
//         EncodeReflection<
//           <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//         >,
//         TAG,
//       >,
//       F,
//     >: Reflectable<T>,
//   {
//     Reflection::new()
//   }

//   /// Returns the reflection to fn which will give the length of the encoded data.
//   #[inline]
//   pub const fn encoded_len(
//     &self,
//   ) -> Reflection<
//     T,
//     Identified<
//       EncodeReflection<
//         Len<
//           <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//         >,
//       >,
//       TAG,
//     >,
//     F,
//   >
//   where
//     Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
//     Reflection<
//       T,
//       Identified<
//         EncodeReflection<
//           Len<
//             <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//           >,
//         >,
//         TAG,
//       >,
//       F,
//     >: Reflectable<T, Reflection = usize>,
//   {
//     Reflection::new()
//   }

//   /// Returns the reflection to the partial encode fn for the field.
//   #[inline]
//   pub const fn partial_encode(
//     &self,
//   ) -> Reflection<
//     T,
//     Identified<
//       PartialEncodeReflection<
//         <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//       >,
//       TAG,
//     >,
//     F,
//   >
//   where
//     Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
//     Reflection<
//       T,
//       Identified<
//         PartialEncodeReflection<
//           <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//         >,
//         TAG,
//       >,
//       F,
//     >: Reflectable<T>,
//   {
//     Reflection::new()
//   }

//   /// Returns the reflection to fn which will give the length of the encoded data.
//   #[inline]
//   pub const fn partial_encoded_len(
//     &self,
//   ) -> Reflection<
//     T,
//     Identified<
//       PartialEncodeReflection<
//         Len<
//           <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//         >,
//       >,
//       TAG,
//     >,
//     F,
//   >
//   where
//     Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
//     Reflection<
//       T,
//       Identified<
//         PartialEncodeReflection<
//           Len<
//             <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//           >,
//         >,
//         TAG,
//       >,
//       F,
//     >: Reflectable<T, Reflection = usize>,
//   {
//     Reflection::new()
//   }

//     /// Returns the reflection to the encode fn to encode decoded field.
//   #[inline]
//   pub const fn encode_decoded<'a>(
//     &self,
//   ) -> Reflection<
//     T,
//     Identified<
//       EncodeReflection<
//         super::Decoded<
//           'a,
//           F,
//           <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//         >,
//       >,
//       TAG,
//     >,
//     F,
//   >
//   where
//     Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
//     Reflection<
//       T,
//       Identified<
//         EncodeReflection<
//           super::Decoded<
//             'a,
//             F,
//             <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//           >,
//         >,
//         TAG,
//       >,
//       F,
//     >: Reflectable<T>,
//   {
//     Reflection::new()
//   }

//   /// Returns the reflection to fn which will give the length of the encoded data.
//   #[inline]
//   pub const fn encoded_decoded_len<'a>(
//     &self,
//   ) -> Reflection<T, Identified<EncodeReflection<Len<super::Decoded<
//     'a,
//     F,
//     <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//   >>>, TAG>, F>
//   where
//     Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
//     Reflection<T, Identified<EncodeReflection<Len<super::Decoded<
//       'a,
//       F,
//       <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//     >>>, TAG>, F>: Reflectable<T, Reflection = usize>,
//   {
//     Reflection::new()
//   }

//   /// Returns the reflection to the partial encode fn which can encode the decoded field.
//   #[inline]
//   pub const fn partial_encode_decoded<'a>(
//     &self,
//   ) -> Reflection<
//     T,
//     Identified<
//       PartialEncodeReflection<
//         super::Decoded<
//           'a,
//           F,
//           <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//         >,
//       >,
//       TAG,
//     >,
//     F,
//   >
//   where
//     Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
//     Reflection<
//       T,
//       Identified<
//         PartialEncodeReflection<
//           super::Decoded<
//             'a,
//             F,
//             <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//           >,
//         >,
//         TAG,
//       >,
//       F,
//     >: Reflectable<T>,
//   {
//     Reflection::new()
//   }

//   /// Returns the reflection to fn which will give the length of the encoded data.
//   #[inline]
//   pub const fn partial_encoded_decoded_len<'a>(
//     &self,
//   ) -> Reflection<
//     T,
//     Identified<
//       PartialEncodeReflection<
//         Len<
//           super::Decoded<
//             'a,
//             F,
//             <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//           >,
//         >,
//       >,
//       TAG,
//     >,
//     F,
//   >
//   where
//     Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
//     Reflection<
//       T,
//       Identified<
//         PartialEncodeReflection<
//           Len<
//             super::Decoded<
//               'a,
//               F,
//               <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//             >,
//           >,
//         >,
//         TAG,
//       >,
//       F,
//     >: Reflectable<T, Reflection = usize>,
//   {
//     Reflection::new()
//   }
// }
