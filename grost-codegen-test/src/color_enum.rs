#![no_implicit_prelude]

#[derive(
  ::core::clone::Clone,
  ::core::fmt::Debug,
  ::core::cmp::PartialEq,
  ::core::cmp::Eq,
  ::core::hash::Hash,
)]
///The error type returned when parsing [`Color`]
pub struct ParseColorError {
  _priv: ::std::string::String,
}
#[derive(
  ::core::marker::Copy,
  ::core::clone::Clone,
  ::core::fmt::Debug,
  ::core::default::Default,
  ::core::cmp::PartialEq,
  ::core::cmp::Eq,
  ::core::hash::Hash,
)]
#[repr(u32)]
#[non_exhaustive]
pub enum Color {
  #[default]
  Red,
  Green,
  Blue,
  /// An unknown variant of the enum, used for backwards and forwards compatibility.
  Unknown(::core::primitive::u32),
}
const _: () = {
  /// The variant relection of the [`Color`] enum
  pub struct ColorVariantReflection<
    R: ?::core::marker::Sized,
    F: ?::core::marker::Sized,
    const VALUE: ::core::primitive::u32,
  > {
    _reflect: ::core::marker::PhantomData<R>,
    _flavor: ::core::marker::PhantomData<F>,
  }
  #[automatically_derived]
  impl<R, F, const VALUE: ::core::primitive::u32> ::core::ops::Deref
    for ColorVariantReflection<R, F, VALUE>
  where
    R: ?::core::marker::Sized,
    F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    Self: ::grost::__private::reflection::Reflectable<F>,
  {
    type Target = <Self as ::grost::__private::reflection::Reflectable<F>>::Reflection;
    fn deref(&self) -> &Self::Target {
      <Self as ::grost::__private::reflection::Reflectable<F>>::REFLECTION
    }
  }
  #[automatically_derived]
  impl<R, F, const VALUE: ::core::primitive::u32>
    ::core::convert::AsRef<<Self as ::core::ops::Deref>::Target>
    for ColorVariantReflection<R, F, VALUE>
  where
    R: ?::core::marker::Sized,
    F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    Self: ::core::ops::Deref,
  {
    fn as_ref(&self) -> &<Self as ::core::ops::Deref>::Target {
      self
    }
  }
  #[automatically_derived]
  impl<R, F, const VALUE: ::core::primitive::u32> ::core::fmt::Debug
    for ColorVariantReflection<R, F, VALUE>
  where
    R: ?::core::marker::Sized,
    F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    Self: ::grost::__private::reflection::Reflectable<F>,
    <Self as ::grost::__private::reflection::Reflectable<F>>::Reflection: ::core::fmt::Debug,
  {
    fn fmt(
      &self,
      f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::result::Result<(), ::core::fmt::Error> {
      ::core::fmt::Debug::fmt(::core::ops::Deref::deref(self), f)
    }
  }
  #[automatically_derived]
  impl<R, F, const VALUE: ::core::primitive::u32> ::core::fmt::Display
    for ColorVariantReflection<R, F, VALUE>
  where
    R: ?::core::marker::Sized,
    F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    Self: ::grost::__private::reflection::Reflectable<F>,
    <Self as ::grost::__private::reflection::Reflectable<F>>::Reflection: ::core::fmt::Display,
  {
    fn fmt(
      &self,
      f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::result::Result<(), ::core::fmt::Error> {
      ::core::fmt::Display::fmt(::core::ops::Deref::deref(self), f)
    }
  }
  #[automatically_derived]
  impl<R, F, const VALUE: ::core::primitive::u32> ColorVariantReflection<R, F, VALUE>
  where
    R: ?::core::marker::Sized,
    F: ?::core::marker::Sized,
  {
    const fn new_in() -> Self {
      Self {
        _reflect: ::core::marker::PhantomData,
        _flavor: ::core::marker::PhantomData,
      }
    }
  }
  #[automatically_derived]
  #[allow(clippy::type_complexity)]
  impl<F, const VALUE: ::core::primitive::u32>
    ColorVariantReflection<::grost::__private::reflection::EnumVariantReflection, F, VALUE>
  where
    F: ?::core::marker::Sized,
  {
    /// Returns the reflection of the field.
    #[inline]
    const fn new() -> Self {
      Self::new_in()
    }
    /// Returns the relection to the encoded value in varint format of the variant.
    #[inline]
    pub const fn encoded_varint(
      &self,
    ) -> ColorVariantReflection<
      ::grost::__private::reflection::encode::EncodeReflection<
        ::grost::__private::reflection::EnumVariantReflection,
      >,
      F,
      VALUE,
    > {
      ColorVariantReflection::new_in()
    }
    /// Returns the relection to the encoded value in varint format of the variant.
    #[inline]
    pub const fn encoded_varint_len(
      &self,
    ) -> ColorVariantReflection<
      ::grost::__private::reflection::Len<
        ::grost::__private::reflection::encode::EncodeReflection<
          ::grost::__private::reflection::EnumVariantReflection,
        >,
      >,
      F,
      VALUE,
    > {
      ColorVariantReflection::new_in()
    }
  }
  #[automatically_derived]
  impl<R, F, const VALUE: ::core::primitive::u32> ::core::clone::Clone
    for ColorVariantReflection<R, F, VALUE>
  where
    R: ?::core::marker::Sized,
    F: ?::core::marker::Sized,
  {
    fn clone(&self) -> Self {
      *self
    }
  }
  #[automatically_derived]
  impl<R, F, const VALUE: ::core::primitive::u32> ::core::marker::Copy
    for ColorVariantReflection<R, F, VALUE>
  where
    R: ?::core::marker::Sized,
    F: ?::core::marker::Sized,
  {
  }
  /// The relection of the [`Color`] enum
  pub struct ColorReflection<R: ?::core::marker::Sized, F: ?::core::marker::Sized> {
    _reflect: ::core::marker::PhantomData<R>,
    _flavor: ::core::marker::PhantomData<F>,
  }
  #[automatically_derived]
  impl<R, F> ::core::ops::Deref for ColorReflection<R, F>
  where
    R: ?::core::marker::Sized,
    F: ?::core::marker::Sized,
    Self: ::grost::__private::reflection::Reflectable<F>,
  {
    type Target = <Self as ::grost::__private::reflection::Reflectable<F>>::Reflection;
    fn deref(&self) -> &Self::Target {
      <Self as ::grost::__private::reflection::Reflectable<F>>::REFLECTION
    }
  }
  #[automatically_derived]
  impl<R, F> ::core::convert::AsRef<<Self as ::core::ops::Deref>::Target> for ColorReflection<R, F>
  where
    R: ?::core::marker::Sized,
    F: ?::core::marker::Sized,
    Self: ::core::ops::Deref,
  {
    fn as_ref(&self) -> &<Self as ::core::ops::Deref>::Target {
      self
    }
  }
  #[automatically_derived]
  impl<R, F> ::core::fmt::Debug for ColorReflection<R, F>
  where
    R: ?::core::marker::Sized,
    F: ?::core::marker::Sized,
    Self: ::grost::__private::reflection::Reflectable<F>,
    <Self as ::grost::__private::reflection::Reflectable<F>>::Reflection: ::core::fmt::Debug,
  {
    fn fmt(
      &self,
      f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::result::Result<(), ::core::fmt::Error> {
      ::core::fmt::Debug::fmt(::core::ops::Deref::deref(self), f)
    }
  }
  #[automatically_derived]
  impl<R, F> ::core::fmt::Display for ColorReflection<R, F>
  where
    R: ?::core::marker::Sized,
    F: ?::core::marker::Sized,
    Self: ::grost::__private::reflection::Reflectable<F>,
    <Self as ::grost::__private::reflection::Reflectable<F>>::Reflection: ::core::fmt::Display,
  {
    fn fmt(
      &self,
      f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::result::Result<(), ::core::fmt::Error> {
      ::core::fmt::Display::fmt(::core::ops::Deref::deref(self), f)
    }
  }
  #[automatically_derived]
  impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::clone::Clone
    for ColorReflection<R, F>
  {
    fn clone(&self) -> Self {
      *self
    }
  }
  #[automatically_derived]
  impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::marker::Copy
    for ColorReflection<R, F>
  {
  }
  #[automatically_derived]
  impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> ColorReflection<R, F> {
    const fn new_in() -> Self {
      Self {
        _reflect: ::core::marker::PhantomData,
        _flavor: ::core::marker::PhantomData,
      }
    }
  }
  #[automatically_derived]
  impl<F> ColorReflection<::grost::__private::reflection::Enum, F>
  where
    F: ?::core::marker::Sized,
  {
    /// Returns the reflection of the enum.
    #[inline]
    const fn new() -> Self {
      Self::new_in()
    }
    /// Returns the variant reflection of the field [`Color::Red`].
    #[inline]
    pub const fn red(
      &self,
    ) -> ColorVariantReflection<::grost::__private::reflection::EnumVariantReflection, F, 1u32>
    {
      ColorVariantReflection::new()
    }
    /// Returns the variant reflection of the field [`Color::Green`].
    #[inline]
    pub const fn green(
      &self,
    ) -> ColorVariantReflection<::grost::__private::reflection::EnumVariantReflection, F, 2u32>
    {
      ColorVariantReflection::new()
    }
    /// Returns the variant reflection of the field [`Color::Blue`].
    #[inline]
    pub const fn blue(
      &self,
    ) -> ColorVariantReflection<::grost::__private::reflection::EnumVariantReflection, F, 3u32>
    {
      ColorVariantReflection::new()
    }
  }
  #[automatically_derived]
  impl Color {
    /// Returns the reflection.
    #[allow(non_camel_case_types)]
    #[inline]
    pub const fn reflection<__GROST_FLAVOR__>()
    -> ColorReflection<::grost::__private::reflection::Enum, __GROST_FLAVOR__>
    where
      __GROST_FLAVOR__: ?::core::marker::Sized,
    {
      ColorReflection::new()
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<__GROST_FLAVOR__: ?::core::marker::Sized>
    ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__> for Color
  {
    type Reflection = ::grost::__private::reflection::Enum;
    const REFLECTION: &Self::Reflection = &{
      ::grost::__private::reflection::EnumBuilder {
        name: "Color",
        schema_name: "Color",
        description: "",
        variants: &[
          <ColorVariantReflection<
            ::grost::__private::reflection::EnumVariantReflection,
            __GROST_FLAVOR__,
            1u32,
          > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::REFLECTION,
          <ColorVariantReflection<
            ::grost::__private::reflection::EnumVariantReflection,
            __GROST_FLAVOR__,
            2u32,
          > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::REFLECTION,
          <ColorVariantReflection<
            ::grost::__private::reflection::EnumVariantReflection,
            __GROST_FLAVOR__,
            3u32,
          > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::REFLECTION,
        ],
        repr: ::grost::__private::reflection::EnumRepr::U32,
      }
      .build()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<__GROST_FLAVOR__: ?::core::marker::Sized>
    ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>
    for ColorReflection<::grost::__private::reflection::Enum, __GROST_FLAVOR__>
  {
    type Reflection = ::grost::__private::reflection::Enum;
    const REFLECTION: &Self::Reflection =
      <Color as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::REFLECTION;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<__GROST_FLAVOR__: ?::core::marker::Sized>
    ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>
    for ColorVariantReflection<
      ::grost::__private::reflection::EnumVariantReflection,
      __GROST_FLAVOR__,
      1u32,
    >
  {
    type Reflection = ::grost::__private::reflection::EnumVariantReflection;
    const REFLECTION: &Self::Reflection = &{
      ::grost::__private::reflection::EnumVariantReflectionBuilder {
        name: "Red",
        schema_name: "Red",
        description: "",
        value: ::grost::__private::reflection::EnumVariantValue::U32(
          ::core::num::NonZeroU32::new(1u32).unwrap(),
        ),
      }
      .build()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<__GROST_FLAVOR__: ?::core::marker::Sized>
    ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>
    for ColorVariantReflection<
      ::grost::__private::reflection::encode::EncodeReflection<
        ::grost::__private::reflection::EnumVariantReflection,
      >,
      __GROST_FLAVOR__,
      1u32,
    >
  {
    type Reflection = ::grost::__private::varing::utils::Buffer<{ 5usize + 1 }>;
    const REFLECTION: &Self::Reflection = &::grost::__private::varing::encode_u32_varint(1u32);
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<__GROST_FLAVOR__: ?::core::marker::Sized>
    ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>
    for ColorVariantReflection<
      ::grost::__private::reflection::Len<
        ::grost::__private::reflection::encode::EncodeReflection<
          ::grost::__private::reflection::EnumVariantReflection,
        >,
      >,
      __GROST_FLAVOR__,
      1u32,
    >
  {
    type Reflection = ::core::primitive::usize;
    const REFLECTION: &Self::Reflection = &{
      <ColorVariantReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
          ::grost::__private::reflection::EnumVariantReflection,
        >,
        __GROST_FLAVOR__,
        1u32,
      > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::REFLECTION
        .len()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<__GROST_FLAVOR__: ?::core::marker::Sized>
    ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>
    for ColorVariantReflection<
      ::grost::__private::reflection::EnumVariantReflection,
      __GROST_FLAVOR__,
      2u32,
    >
  {
    type Reflection = ::grost::__private::reflection::EnumVariantReflection;
    const REFLECTION: &Self::Reflection = &{
      ::grost::__private::reflection::EnumVariantReflectionBuilder {
        name: "Green",
        schema_name: "Green",
        description: "",
        value: ::grost::__private::reflection::EnumVariantValue::U32(
          ::core::num::NonZeroU32::new(2u32).unwrap(),
        ),
      }
      .build()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<__GROST_FLAVOR__: ?::core::marker::Sized>
    ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>
    for ColorVariantReflection<
      ::grost::__private::reflection::encode::EncodeReflection<
        ::grost::__private::reflection::EnumVariantReflection,
      >,
      __GROST_FLAVOR__,
      2u32,
    >
  {
    type Reflection = ::grost::__private::varing::utils::Buffer<{ 5usize + 1 }>;
    const REFLECTION: &Self::Reflection = &::grost::__private::varing::encode_u32_varint(2u32);
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<__GROST_FLAVOR__: ?::core::marker::Sized>
    ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>
    for ColorVariantReflection<
      ::grost::__private::reflection::Len<
        ::grost::__private::reflection::encode::EncodeReflection<
          ::grost::__private::reflection::EnumVariantReflection,
        >,
      >,
      __GROST_FLAVOR__,
      2u32,
    >
  {
    type Reflection = ::core::primitive::usize;
    const REFLECTION: &Self::Reflection = &{
      <ColorVariantReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
          ::grost::__private::reflection::EnumVariantReflection,
        >,
        __GROST_FLAVOR__,
        2u32,
      > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::REFLECTION
        .len()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<__GROST_FLAVOR__: ?::core::marker::Sized>
    ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>
    for ColorVariantReflection<
      ::grost::__private::reflection::EnumVariantReflection,
      __GROST_FLAVOR__,
      3u32,
    >
  {
    type Reflection = ::grost::__private::reflection::EnumVariantReflection;
    const REFLECTION: &Self::Reflection = &{
      ::grost::__private::reflection::EnumVariantReflectionBuilder {
        name: "Blue",
        schema_name: "Blue",
        description: "",
        value: ::grost::__private::reflection::EnumVariantValue::U32(
          ::core::num::NonZeroU32::new(3u32).unwrap(),
        ),
      }
      .build()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<__GROST_FLAVOR__: ?::core::marker::Sized>
    ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>
    for ColorVariantReflection<
      ::grost::__private::reflection::encode::EncodeReflection<
        ::grost::__private::reflection::EnumVariantReflection,
      >,
      __GROST_FLAVOR__,
      3u32,
    >
  {
    type Reflection = ::grost::__private::varing::utils::Buffer<{ 5usize + 1 }>;
    const REFLECTION: &Self::Reflection = &::grost::__private::varing::encode_u32_varint(3u32);
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<__GROST_FLAVOR__: ?::core::marker::Sized>
    ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>
    for ColorVariantReflection<
      ::grost::__private::reflection::Len<
        ::grost::__private::reflection::encode::EncodeReflection<
          ::grost::__private::reflection::EnumVariantReflection,
        >,
      >,
      __GROST_FLAVOR__,
      3u32,
    >
  {
    type Reflection = ::core::primitive::usize;
    const REFLECTION: &Self::Reflection = &{
      <ColorVariantReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
          ::grost::__private::reflection::EnumVariantReflection,
        >,
        __GROST_FLAVOR__,
        3u32,
      > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::REFLECTION
        .len()
    };
  }
  #[automatically_derived]
  impl ::core::fmt::Display for Color {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
      match self.try_as_str() {
        ::core::result::Result::Ok(variant_str) => {
          ::core::write!(f, "{variant_str}")
        }
        ::core::result::Result::Err(val) => ::core::write!(f, "Unknown({})", val),
      }
    }
  }
  #[automatically_derived]
  impl Color {
    /// Try to return the enum variant as a `str`, if the variant is unknown, it will return the value of the unknown variant.
    #[inline]
    pub const fn try_as_str(
      &self,
    ) -> ::core::result::Result<&'static ::core::primitive::str, ::core::primitive::u32> {
      ::core::result::Result::Ok(match self {
        Self::Red => "Red",
        Self::Green => "Green",
        Self::Blue => "Blue",
        Self::Unknown(val) => return ::core::result::Result::Err(*val),
      })
    }
  }
  #[automatically_derived]
  impl ::core::fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
      ::core::write!(f, "Fail to parse `Color`, unknown {}", self._priv)
    }
  }
  #[automatically_derived]
  impl ::core::error::Error for ParseColorError {}
  #[automatically_derived]
  impl ::core::str::FromStr for Color {
    type Err = ParseColorError;
    fn from_str(src: &::core::primitive::str) -> ::core::result::Result<Self, Self::Err> {
      match src.trim() {
        "red" | "RED" | "Red" => ::core::result::Result::Ok(Self::Red),
        "green" | "GREEN" | "Green" => ::core::result::Result::Ok(Self::Green),
        "blue" | "BLUE" | "Blue" => ::core::result::Result::Ok(Self::Blue),
        val => {
          if let ::core::option::Option::Some(remaining) = val
            .strip_prefix("Unknown(")
            .or_else(|| val.strip_prefix("unknown("))
          {
            if let ::core::option::Option::Some(remaining) = remaining.strip_suffix(')') {
              if let ::core::result::Result::Ok(val) =
                <::core::primitive::u32 as ::core::str::FromStr>::from_str(remaining)
              {
                return ::core::result::Result::Ok(::core::convert::From::from(val));
              }
            }
          }
          if let ::core::result::Result::Ok(val) =
            <::core::primitive::u32 as ::core::str::FromStr>::from_str(val)
          {
            return ::core::result::Result::Ok(::core::convert::From::from(val));
          }
          ::core::result::Result::Err(ParseColorError {
            _priv: ::std::string::ToString::to_string(src),
          })
        }
      }
    }
  }
  #[automatically_derived]
  impl<'a> ::core::convert::TryFrom<&'a ::core::primitive::str> for Color {
    type Error = ParseColorError;
    fn try_from(src: &'a ::core::primitive::str) -> ::core::result::Result<Self, Self::Error> {
      ::core::str::FromStr::from_str(src)
    }
  }
  #[automatically_derived]
  impl Color {
    ///Returns `true` if the enum variant is [`Color::Red`]
    #[inline]
    pub const fn is_red(&self) -> ::core::primitive::bool {
      ::core::matches!(self, Self::Red)
    }
    ///Returns `true` if the enum variant is [`Color::Green`]
    #[inline]
    pub const fn is_green(&self) -> ::core::primitive::bool {
      ::core::matches!(self, Self::Green)
    }
    ///Returns `true` if the enum variant is [`Color::Blue`]
    #[inline]
    pub const fn is_blue(&self) -> ::core::primitive::bool {
      ::core::matches!(self, Self::Blue)
    }
  }
  #[automatically_derived]
  impl ::core::convert::From<::core::primitive::u32> for Color {
    #[inline]
    fn from(repr: ::core::primitive::u32) -> Self {
      Self::from_u32(repr)
    }
  }
  #[automatically_derived]
  impl ::core::convert::From<Color> for ::core::primitive::u32 {
    #[inline]
    fn from(e: Color) -> Self {
      e.as_u32()
    }
  }
  #[automatically_derived]
  impl<'a> ::core::convert::TryFrom<&'a [::core::primitive::u8]> for Color {
    type Error = ::grost::__private::varing::DecodeError;
    #[inline]
    fn try_from(src: &'a [::core::primitive::u8]) -> ::core::result::Result<Self, Self::Error> {
      Self::decode(src).map(|(_, this)| this)
    }
  }
  #[automatically_derived]
  impl ::grost::__private::varing::Varint for Color {
    const MAX_ENCODED_LEN: ::core::primitive::usize = 5usize;
    const MIN_ENCODED_LEN: ::core::primitive::usize = 1usize;
    #[inline]
    fn encode(
      &self,
      buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<::core::primitive::usize, ::grost::__private::varing::EncodeError>
    {
      Self::encode_to(self, buf)
    }
    #[inline]
    fn encoded_len(&self) -> ::core::primitive::usize {
      Self::encoded_len(self)
    }
    #[inline]
    fn decode(
      src: &[::core::primitive::u8],
    ) -> ::core::result::Result<
      (::core::primitive::usize, Self),
      ::grost::__private::varing::DecodeError,
    > {
      Self::decode(src)
    }
  }
  #[automatically_derived]
  impl Color {
    /// Returns the enum variant from the raw representation.
    #[inline]
    pub const fn from_u32(repr: ::core::primitive::u32) -> Self {
      match repr {
        1u32 => Self::Red,
        2u32 => Self::Green,
        3u32 => Self::Blue,
        val => Self::Unknown(val),
      }
    }
    /// Returns the raw representation of the enum variant.
    #[inline]
    pub const fn as_u32(&self) -> ::core::primitive::u32 {
      match self {
        Self::Red => 1u32,
        Self::Green => 2u32,
        Self::Blue => 3u32,
        Self::Unknown(val) => *val,
      }
    }
    /// Returns the encoded bytes of the enum variant.
    #[inline]
    pub const fn encode(&self) -> ::grost::__private::varing::utils::Buffer<{ 5usize + 1 }> {
      match self {
        Self::Red => {
          *<ColorVariantReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
              ::grost::__private::reflection::EnumVariantReflection,
            >,
            (),
            1u32,
          > as ::grost::__private::reflection::Reflectable<()>>::REFLECTION
        }
        Self::Green => {
          *<ColorVariantReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
              ::grost::__private::reflection::EnumVariantReflection,
            >,
            (),
            2u32,
          > as ::grost::__private::reflection::Reflectable<()>>::REFLECTION
        }
        Self::Blue => {
          *<ColorVariantReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
              ::grost::__private::reflection::EnumVariantReflection,
            >,
            (),
            3u32,
          > as ::grost::__private::reflection::Reflectable<()>>::REFLECTION
        }
        Self::Unknown(val) => ::grost::__private::varing::encode_u32_varint(*val),
      }
    }
    /// Returns the encoded bytes of the enum variant.
    #[inline]
    pub const fn encode_to(
      &self,
      buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<::core::primitive::usize, ::grost::__private::varing::EncodeError>
    {
      match self {
        Self::Red => {
          const ENCODED_LEN: ::core::primitive::usize = *<ColorVariantReflection<
            ::grost::__private::reflection::Len<
              ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::EnumVariantReflection,
              >,
            >,
            (),
            1u32,
          > as ::grost::__private::reflection::Reflectable<()>>::REFLECTION;
          let buf_len = buf.len();
          if buf_len < ENCODED_LEN {
            return ::core::result::Result::Err(
              ::grost::__private::varing::EncodeError::underflow(ENCODED_LEN, buf_len),
            );
          }
          let (b, _) = buf.split_at_mut(ENCODED_LEN);
          b.copy_from_slice(
            <ColorVariantReflection<
              ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::EnumVariantReflection,
              >,
              (),
              1u32,
            > as ::grost::__private::reflection::Reflectable<()>>::REFLECTION
              .as_slice(),
          );
          ::core::result::Result::Ok(ENCODED_LEN)
        }
        Self::Green => {
          const ENCODED_LEN: ::core::primitive::usize = *<ColorVariantReflection<
            ::grost::__private::reflection::Len<
              ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::EnumVariantReflection,
              >,
            >,
            (),
            2u32,
          > as ::grost::__private::reflection::Reflectable<()>>::REFLECTION;
          let buf_len = buf.len();
          if buf_len < ENCODED_LEN {
            return ::core::result::Result::Err(
              ::grost::__private::varing::EncodeError::underflow(ENCODED_LEN, buf_len),
            );
          }
          let (b, _) = buf.split_at_mut(ENCODED_LEN);
          b.copy_from_slice(
            <ColorVariantReflection<
              ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::EnumVariantReflection,
              >,
              (),
              2u32,
            > as ::grost::__private::reflection::Reflectable<()>>::REFLECTION
              .as_slice(),
          );
          ::core::result::Result::Ok(ENCODED_LEN)
        }
        Self::Blue => {
          const ENCODED_LEN: ::core::primitive::usize = *<ColorVariantReflection<
            ::grost::__private::reflection::Len<
              ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::EnumVariantReflection,
              >,
            >,
            (),
            3u32,
          > as ::grost::__private::reflection::Reflectable<()>>::REFLECTION;
          let buf_len = buf.len();
          if buf_len < ENCODED_LEN {
            return ::core::result::Result::Err(
              ::grost::__private::varing::EncodeError::underflow(ENCODED_LEN, buf_len),
            );
          }
          let (b, _) = buf.split_at_mut(ENCODED_LEN);
          b.copy_from_slice(
            <ColorVariantReflection<
              ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::EnumVariantReflection,
              >,
              (),
              3u32,
            > as ::grost::__private::reflection::Reflectable<()>>::REFLECTION
              .as_slice(),
          );
          ::core::result::Result::Ok(ENCODED_LEN)
        }
        Self::Unknown(val) => ::grost::__private::varing::encode_u32_varint_to(*val, buf),
      }
    }
    /// Decodes the enum variant from the encoded bytes.
    ///
    /// Returns the number of bytes readed and `Self`.
    #[inline]
    pub const fn decode(
      src: &[::core::primitive::u8],
    ) -> ::core::result::Result<
      (::core::primitive::usize, Self),
      ::grost::__private::varing::DecodeError,
    > {
      ::core::result::Result::Ok(match src {
        [1u8] => ([1u8].len(), Self::Red),
        [2u8] => ([2u8].len(), Self::Green),
        [3u8] => ([3u8].len(), Self::Blue),
        src => {
          return match ::grost::__private::varing::decode_u32_varint(src) {
            ::core::result::Result::Ok((read, val)) => {
              ::core::result::Result::Ok((read, Self::Unknown(val)))
            }
            ::core::result::Result::Err(e) => ::core::result::Result::Err(e),
          };
        }
      })
    }
    /// Returns the encoded length of the enum variant.
    #[inline]
    pub const fn encoded_len(&self) -> ::core::primitive::usize {
      match self {
        Self::Red => {
          *<ColorVariantReflection<
            ::grost::__private::reflection::Len<
              ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::EnumVariantReflection,
              >,
            >,
            (),
            1u32,
          > as ::grost::__private::reflection::Reflectable<()>>::REFLECTION
        }
        Self::Green => {
          *<ColorVariantReflection<
            ::grost::__private::reflection::Len<
              ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::EnumVariantReflection,
              >,
            >,
            (),
            2u32,
          > as ::grost::__private::reflection::Reflectable<()>>::REFLECTION
        }
        Self::Blue => {
          *<ColorVariantReflection<
            ::grost::__private::reflection::Len<
              ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::EnumVariantReflection,
              >,
            >,
            (),
            3u32,
          > as ::grost::__private::reflection::Reflectable<()>>::REFLECTION
        }
        Self::Unknown(val) => ::grost::__private::varing::encoded_u32_varint_len(*val),
      }
    }
  }
};
const _: () = {
  ::grost::__private::network_varint!(Color);
};
