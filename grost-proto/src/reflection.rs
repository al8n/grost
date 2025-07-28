use ghost::phantom;

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
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        #[phantom]
        pub struct $name<R: ?::core::marker::Sized>;

        impl<R: ?::core::marker::Sized> ::core::default::Default for $name<R> {
          #[inline]
          fn default() -> Self {
            Self::new()
          }
        }

        impl<R: ?::core::marker::Sized> $name<R> {
          #[doc = "creates a new `" $name "`"]
          #[inline]
          pub const fn new() -> Self {
            $name
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
  WireSchemaTypeReflection,
  /// Reflection to length related
  Len,
  /// Reflection to an encode fn.
  EncodeReflection,
  /// Reflection to an partial encode fn.
  PartialEncodeReflection,
  /// Reflection to the schema [`Type`].
  SchemaTypeReflection,
);

/// Reflectable.
pub trait Reflectable<F: ?Sized> {
  type Reflection: ?Sized + 'static;

  /// The reflection of this type
  const REFLECTION: &Self::Reflection;
}

/// A phantom relection type which can be dereferenced to [`Reflectable::REFLECTION`].

#[phantom]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Reflection<T: ?Sized, R: ?Sized, F: ?Sized>;

impl<T, R, F> Reflection<T, R, F>
where
  T: ?Sized,
  R: ?Sized,
  F: ?Sized,
{
  /// Creates a new [`Reflection`].
  #[inline]
  pub const fn new() -> Self {
    Reflection
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
//   ) -> decoded::PartialRefReflection<T, Identified<ObjectField, TAG>, F>
//   {
//     decoded::PartialRefReflection::new()
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
//   pub const fn partial_encode<WB>(
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
//         super::PartialRef<
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
//           super::PartialRef<
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
//   ) -> Reflection<T, Identified<EncodeReflection<Len<super::PartialRef<
//     'a,
//     F,
//     <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
//   >>>, TAG>, F>
//   where
//     Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
//     Reflection<T, Identified<EncodeReflection<Len<super::PartialRef<
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
//         super::PartialRef<
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
//           super::PartialRef<
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
//           super::PartialRef<
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
//             super::PartialRef<
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
