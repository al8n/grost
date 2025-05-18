use core::marker::PhantomData;

use super::flavors::Flavor;

use encode::EncodeField;
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
pub enum Type {
  Scalar {
    name: &'static str,
    description: &'static str,
  },
  List(&'static Type),
  Map {
    key: &'static Type,
    value: &'static Type,
  },
  Optional(&'static Type),
  Object(&'static ObjectReflection),
  UintEnum(EnumReflection),
  Union(),
  Interface(),
}

impl Clone for Type {
  fn clone(&self) -> Self {
    *self
  }
}

impl Copy for Type {}

impl Type {
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

pub struct Identified<T: ?Sized, const TAG: u32>(PhantomData<T>);

impl<T: ?Sized, F: ?Sized> Reflectable<T> for Reflection<&T, Type, F>
where
  Reflection<T, Type, F>: Reflectable<F, Reflection = Type>,
{
  type Reflection = Type;

  const REFLECTION: &'static Self::Reflection =
    <Reflection<T, Type, F> as Reflectable<F>>::REFLECTION;
}

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

impl<T, R, F> core::ops::Deref for Reflection<T, R, F>
where
  Self: Reflectable<T>,
{
  type Target = <Self as Reflectable<T>>::Reflection;

  fn deref(&self) -> &Self::Target {
    Self::REFLECTION
  }
}

#[allow(clippy::type_complexity)]
impl<T: ?Sized, F: ?Sized, const TAG: u32> Reflection<T, Identified<ObjectFieldReflection, TAG>, F>
where
  Self: Reflectable<T>,
{
  /// Returns the relection to the wire format of the field.
  #[inline]
  pub const fn wire_format(&self) -> Reflection<T, Identified<WireFormatReflection, TAG>, F>
  where
    Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to the tag of the field.
  #[inline]
  pub const fn tag(&self) -> Reflection<T, Identified<TagReflection<F::Tag>, TAG>, F>
  where
    F: Flavor,
    Reflection<T, Identified<TagReflection<F::Tag>, TAG>, F>: Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to the wire type of the field.
  #[inline]
  pub const fn wire_type(
    &self,
  ) -> Reflection<T, Identified<WireTypeReflection<F::WireType>, TAG>, F>
  where
    F: Flavor,
    Reflection<T, Identified<WireTypeReflection<F::WireType>, TAG>, F>: Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to the identifier of the field.
  #[inline]
  pub const fn identifier(
    &self,
  ) -> Reflection<T, Identified<IdentifierReflection<F::Identifier>, TAG>, F>
  where
    F: Flavor,
    Reflection<T, Identified<IdentifierReflection<F::Identifier>, TAG>, F>: Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to the encoded identifier of the field.
  #[inline]
  pub const fn encoded_identifier(
    &self,
  ) -> Reflection<T, Identified<EncodedIdentifierReflection<F::Identifier>, TAG>, F>
  where
    F: Flavor,
    Reflection<T, Identified<EncodedIdentifierReflection<F::Identifier>, TAG>, F>: Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to the length of the encoded identifier of the field.
  #[inline]
  pub const fn encoded_identifier_len(
    &self,
  ) -> Reflection<T, Identified<Len<EncodedIdentifierReflection<F::Identifier>>, TAG>, F>
  where
    F: Flavor,
    Reflection<T, Identified<Len<EncodedIdentifierReflection<F::Identifier>>, TAG>, F>:
      Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to the encoded tag of the field.
  #[inline]
  pub const fn encoded_tag(&self) -> Reflection<T, Identified<EncodedTagReflection<F::Tag>, TAG>, F>
  where
    F: Flavor,
    Reflection<T, Identified<EncodedTagReflection<F::Tag>, TAG>, F>: Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to the length of the encoded tag of the field.
  #[inline]
  pub const fn encoded_tag_len(
    &self,
  ) -> Reflection<T, Identified<Len<EncodedTagReflection<F::Tag>>, TAG>, F>
  where
    F: Flavor,
    Reflection<T, Identified<Len<EncodedTagReflection<F::Tag>>, TAG>, F>: Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to the encode fn of the field.
  #[inline]
  pub const fn encode(
    &self,
  ) -> Reflection<
    T,
    Identified<
      encode::EncodeReflection<
        <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
      >,
      TAG,
    >,
    F,
  >
  where
    F: Flavor,
    Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
    Reflection<
      T,
      Identified<
        encode::EncodeReflection<
          <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
        >,
        TAG,
      >,
      F,
    >: Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to fn which will give the length of the encoded data.
  #[inline]
  pub const fn encoded_len(
    &self,
  ) -> Reflection<
    T,
    Identified<
      encode::EncodeReflection<
        Len<
          <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
        >,
      >,
      TAG,
    >,
    F,
  >
  where
    F: Flavor,
    Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
    Reflection<
      T,
      Identified<
        encode::EncodeReflection<
          Len<
            <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
          >,
        >,
        TAG,
      >,
      F,
    >: Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to the encode fn to encode decoded field.
  #[inline]
  pub const fn encode_decoded<'a>(
    &self,
  ) -> Reflection<
    T,
    Identified<
      encode::EncodeReflection<
        super::Decoded<
          'a,
          F,
          <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
        >,
      >,
      TAG,
    >,
    F,
  >
  where
    F: Flavor,
    Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
    Reflection<
      T,
      Identified<
        encode::EncodeReflection<
          super::Decoded<
            'a,
            F,
            <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
          >,
        >,
        TAG,
      >,
      F,
    >: Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to fn which will give the length of the encoded data.
  #[inline]
  pub const fn encoded_decoded_len<'a>(
    &self,
  ) -> Reflection<T, Identified<encode::EncodeReflection<Len<super::Decoded<
    'a,
    F,
    <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
  >>>, TAG>, F>
  where
    F: Flavor,
    Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
    Reflection<T, Identified<encode::EncodeReflection<Len<super::Decoded<
      'a,
      F,
      <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
    >>>, TAG>, F>: Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to the partial encode fn for the field.
  #[inline]
  pub const fn partial_encode(
    &self,
  ) -> Reflection<
    T,
    Identified<
      encode::PartialEncodeReflection<
        <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
      >,
      TAG,
    >,
    F,
  >
  where
    F: Flavor,
    Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
    Reflection<
      T,
      Identified<
        encode::PartialEncodeReflection<
          <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
        >,
        TAG,
      >,
      F,
    >: Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to fn which will give the length of the encoded data.
  #[inline]
  pub const fn partial_encoded_len(
    &self,
  ) -> Reflection<
    T,
    Identified<
      encode::PartialEncodeReflection<
        Len<
          <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
        >,
      >,
      TAG,
    >,
    F,
  >
  where
    F: Flavor,
    Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
    Reflection<
      T,
      Identified<
        encode::PartialEncodeReflection<
          Len<
            <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
          >,
        >,
        TAG,
      >,
      F,
    >: Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to the partial encode fn which can encode the decoded field.
  #[inline]
  pub const fn partial_encode_decoded<'a>(
    &self,
  ) -> Reflection<
    T,
    Identified<
      encode::PartialEncodeReflection<
        super::Decoded<
          'a,
          F,
          <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
        >,
      >,
      TAG,
    >,
    F,
  >
  where
    F: Flavor,
    Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
    Reflection<
      T,
      Identified<
        encode::PartialEncodeReflection<
          super::Decoded<
            'a,
            F,
            <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
          >,
        >,
        TAG,
      >,
      F,
    >: Reflectable<T>,
  {
    Reflection::new()
  }

  /// Returns the reflection to fn which will give the length of the encoded data.
  #[inline]
  pub const fn partial_encoded_decoded_len<'a>(
    &self,
  ) -> Reflection<
    T,
    Identified<
      encode::PartialEncodeReflection<
        Len<
          super::Decoded<
            'a,
            F,
            <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
          >,
        >,
      >,
      TAG,
    >,
    F,
  >
  where
    F: Flavor,
    Reflection<T, Identified<WireFormatReflection, TAG>, F>: Reflectable<T>,
    Reflection<
      T,
      Identified<
        encode::PartialEncodeReflection<
          Len<
            super::Decoded<
              'a,
              F,
              <Reflection<T, Identified<WireFormatReflection, TAG>, F> as Reflectable<T>>::Reflection,
            >,
          >,
        >,
        TAG,
      >,
      F,
    >: Reflectable<T>,
  {
    Reflection::new()
  }
}
