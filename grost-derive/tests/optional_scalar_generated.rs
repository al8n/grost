#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use grost_derive::Object;
struct OptionalScalar {
  #[grost(tag = 1, optional(scalar))]
  foo: Option<u32>,
}
#[automatically_derived]
impl ::core::fmt::Debug for OptionalScalar {
  #[inline]
  fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
    ::core::fmt::Formatter::debug_struct_field1_finish(f, "OptionalScalar", "foo", &&self.foo)
  }
}
#[automatically_derived]
impl ::core::clone::Clone for OptionalScalar {
  #[inline]
  fn clone(&self) -> OptionalScalar {
    OptionalScalar {
      foo: ::core::clone::Clone::clone(&self.foo),
    }
  }
}
/// Field indexer for the struct [`OptionalScalar`]
#[repr(u32)]
enum OptionalScalarIndex {
  /// The field indexer for the field `foo`
  Foo = 1u32,
}
#[automatically_derived]
impl ::core::clone::Clone for OptionalScalarIndex {
  #[inline]
  fn clone(&self) -> OptionalScalarIndex {
    *self
  }
}
#[automatically_derived]
impl ::core::marker::Copy for OptionalScalarIndex {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for OptionalScalarIndex {}
#[automatically_derived]
impl ::core::cmp::PartialEq for OptionalScalarIndex {
  #[inline]
  fn eq(&self, other: &OptionalScalarIndex) -> bool {
    true
  }
}
#[automatically_derived]
impl ::core::cmp::Eq for OptionalScalarIndex {
  #[inline]
  #[doc(hidden)]
  #[coverage(off)]
  fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for OptionalScalarIndex {
  #[inline]
  fn partial_cmp(
    &self,
    other: &OptionalScalarIndex,
  ) -> ::core::option::Option<::core::cmp::Ordering> {
    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
  }
}
#[automatically_derived]
impl ::core::cmp::Ord for OptionalScalarIndex {
  #[inline]
  fn cmp(&self, other: &OptionalScalarIndex) -> ::core::cmp::Ordering {
    ::core::cmp::Ordering::Equal
  }
}
#[automatically_derived]
impl ::core::hash::Hash for OptionalScalarIndex {
  #[inline]
  fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
}
#[automatically_derived]
impl ::core::fmt::Debug for OptionalScalarIndex {
  #[inline]
  fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
    ::core::fmt::Formatter::write_str(f, "Foo")
  }
}
/// The selection type for [`OptionalScalar`]
#[allow(non_camel_case_types, clippy::type_complexity)]
struct OptionalScalarSelector {
  foo: <Option<u32> as ::grost::__private::selection::Selectable<
    ::grost::__private::flavors::Groto,
  >>::Selector,
}
/// An iterator over the selected fields of the [`OptionalScalarSelector`]
#[allow(non_camel_case_types, clippy::type_complexity)]
struct OptionalScalarSelectorIter<
  '__grost_lifetime__,
  const __GROST_SELECTED__: ::core::primitive::bool = true,
> {
  selector: &'__grost_lifetime__ OptionalScalarSelector,
  index: ::core::option::Option<OptionalScalarIndex>,
  num: ::core::primitive::usize,
  yielded: ::core::primitive::usize,
}
/// Partial struct for the [`PartialOptionalScalar`]
#[allow(non_camel_case_types, clippy::type_complexity)]
struct PartialOptionalScalar {
  foo: <Option<u32> as ::grost::__private::convert::State<
    ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
  >>::Output,
}
/// Partial reference struct for the struct [`OptionalScalar`]
#[allow(non_camel_case_types, clippy::type_complexity)]
struct PartialOptionalScalarRef<
  '__grost_lifetime__,
  __GROST_READ_BUFFER__,
  __GROST_UNKNOWN_BUFFER__,
> {
  __grost_unknown_buffer__: ::core::option::Option<__GROST_UNKNOWN_BUFFER__>,
  __grost_read_buffer__: ::core::option::Option<__GROST_READ_BUFFER__>,
  foo: <Option<u32> as ::grost::__private::convert::State<
    ::grost::__private::convert::PartialRef<
      '__grost_lifetime__,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
      <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
        ::grost::__private::flavors::Groto,
      >>::Format,
      ::grost::__private::flavors::Groto,
    >,
  >>::Output,
}
const _: () = {
  impl ::core::default::Default for OptionalScalar {
    fn default() -> Self {
      Self::new()
    }
  }
  impl OptionalScalar {
    /// Creates a new instance of the object with default values.
    pub fn new() -> Self {
      Self {
        foo: (::core::default::Default::default)(),
      }
    }
  }
  impl OptionalScalar {
    /// Returns a reference to the `foo`
    #[inline]
    const fn foo_ref(&self) -> &Option<u32> {
      &self.foo
    }
    /// Returns a mutable reference to the `foo`
    #[inline]
    const fn foo_mut(&mut self) -> &mut Option<u32> {
      &mut self.foo
    }
    /// Set the `foo` to the given value
    #[inline]
    fn set_foo(&mut self, value: Option<u32>) -> &mut Self {
      self.foo = value;
      self
    }
    /// Set the `foo` to the given value
    #[inline]
    fn with_foo(mut self, value: Option<u32>) -> Self {
      self.foo = value;
      self
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<__GROST_FLATTEN_STATE__: ?::core::marker::Sized>
    ::grost::__private::convert::State<
      ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
    > for OptionalScalar
  {
    type Output = Self;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl
    ::grost::__private::convert::State<
      ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
    > for OptionalScalar
  {
    type Output = PartialOptionalScalar;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl
    ::grost::__private::convert::State<
      ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
    > for PartialOptionalScalar
  {
    type Output = Self;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, __GROST_READ_BUFFER__, __GROST_UNKNOWN_BUFFER__>
    ::grost::__private::convert::State<
      ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
    >
    for PartialOptionalScalarRef<
      '__grost_lifetime__,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
    >
  {
    type Output = Self;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, __GROST_READ_BUFFER__, __GROST_UNKNOWN_BUFFER__>
    ::grost::__private::convert::State<
      ::grost::__private::convert::PartialRef<
        '__grost_lifetime__,
        ::grost::__private::flavors::Groto,
        ::grost::__private::flavors::groto::LengthDelimited,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
      >,
    > for OptionalScalar
  {
    type Output = PartialOptionalScalarRef<
      '__grost_lifetime__,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
    >;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, __GROST_READ_BUFFER__, __GROST_UNKNOWN_BUFFER__>
    ::grost::__private::convert::State<
      ::grost::__private::convert::PartialRef<
        '__grost_lifetime__,
        ::grost::__private::flavors::Groto,
        ::grost::__private::flavors::groto::LengthDelimited,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
      >,
    > for PartialOptionalScalar
  {
    type Output = PartialOptionalScalarRef<
      '__grost_lifetime__,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
    >;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, __GROST_READ_BUFFER__, __GROST_UNKNOWN_BUFFER__>
    ::grost::__private::convert::State<
      ::grost::__private::convert::PartialRef<
        '__grost_lifetime__,
        ::grost::__private::flavors::Groto,
        ::grost::__private::flavors::groto::LengthDelimited,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >
    for PartialOptionalScalarRef<
      '__grost_lifetime__,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
    >
  {
    type Output = Self;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl ::core::default::Default for PartialOptionalScalar {
    fn default() -> Self {
      Self::new()
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<__GROST_FLATTEN_STATE__: ?::core::marker::Sized>
    ::grost::__private::convert::State<
      ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
    > for PartialOptionalScalar
  {
    type Output = Self;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl PartialOptionalScalar {
    /// Creates an empty partial struct.
    #[inline]
    pub const fn new() -> Self {
      Self {
        foo: ::core::option::Option::None,
      }
    }
    /// Returns `true` if the partial object is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
      self.foo.is_none()
    }
    /// Returns a reference to the `foo`
    #[inline]
    const fn foo_ref(
      &self,
    ) -> ::core::option::Option<
      &<Option<u32> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output,
    > {
      self.foo.as_ref()
    }
    /// Returns a mutable reference to the `foo`
    #[inline]
    const fn foo_mut(
      &mut self,
    ) -> ::core::option::Option<
      &mut <Option<u32> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output,
    > {
      self.foo.as_mut()
    }
    /// Returns a reference to the `foo` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `foo` is `None`
    #[inline]
    const fn unwrap_foo_ref(
      &self,
    ) -> &<Option<u32> as ::grost::__private::convert::State<
      ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
    >>::Output {
      match self.foo.as_ref() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => {
          ::core::panicking::panic_fmt(format_args!("`foo` is `None`"));
        }
      }
    }
    /// Returns a mutable reference to the `foo` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `foo` is `None`
    #[inline]
    const fn unwrap_foo_mut(
      &mut self,
    ) -> &mut <Option<u32> as ::grost::__private::convert::State<
      ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
    >>::Output {
      match self.foo.as_mut() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => {
          ::core::panicking::panic_fmt(format_args!("`foo` is `None`"));
        }
      }
    }
    /// Takes the value of `foo` out if it is not `None`
    #[inline]
    const fn take_foo(
      &mut self,
    ) -> ::core::option::Option<
      <Option<u32> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output,
    > {
      self.foo.take()
    }
    /// Clear the value of `foo`
    #[inline]
    fn clear_foo(&mut self) -> &mut Self {
      self.foo = ::core::option::Option::None;
      self
    }
    /// Set the `foo` to the given value
    #[inline]
    fn set_foo(
      &mut self,
      value: <Option<u32> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output,
    ) -> &mut Self {
      self.foo = ::core::option::Option::Some(value);
      self
    }
    /// Update the `foo` to the given value or clear the `foo`
    #[inline]
    fn update_foo(
      &mut self,
      value: ::core::option::Option<
        <Option<u32> as ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >>::Output,
      >,
    ) -> &mut Self {
      self.foo = value;
      self
    }
    /// Set the `foo` to the given value
    #[inline]
    fn with_foo(
      mut self,
      value: <Option<u32> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output,
    ) -> Self {
      self.foo = ::core::option::Option::Some(value);
      self
    }
    /// Clear the value of `foo`
    #[inline]
    fn without_foo(mut self) -> Self {
      self.foo = ::core::option::Option::None;
      self
    }
    /// Update the `foo` to the given value or clear the `foo`
    #[inline]
    fn maybe_foo(
      mut self,
      value: ::core::option::Option<
        <Option<u32> as ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >>::Output,
      >,
    ) -> Self {
      self.foo = value;
      self
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, __GROST_READ_BUFFER__, __GROST_UNKNOWN_BUFFER__>
    ::core::default::Default
    for PartialOptionalScalarRef<
      '__grost_lifetime__,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
    >
  {
    fn default() -> Self {
      Self::new()
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<
    '__grost_lifetime__,
    __GROST_READ_BUFFER__,
    __GROST_UNKNOWN_BUFFER__,
    __GROST_FLATTEN_STATE__: ?::core::marker::Sized,
  >
    ::grost::__private::convert::State<
      ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
    >
    for PartialOptionalScalarRef<
      '__grost_lifetime__,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
    >
  {
    type Output = Self;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, __GROST_READ_BUFFER__, __GROST_UNKNOWN_BUFFER__>
    PartialOptionalScalarRef<'__grost_lifetime__, __GROST_READ_BUFFER__, __GROST_UNKNOWN_BUFFER__>
  {
    /// Creates an empty partial struct.
    #[inline]
    pub const fn new() -> Self {
      Self {
        foo: ::core::option::Option::None,
        __grost_unknown_buffer__: ::core::option::Option::None,
        __grost_read_buffer__: ::core::option::Option::None,
      }
    }
    /// Returns `true` if the partial struct is empty, which means all fields are `None`.
    #[inline]
    pub const fn is_empty(&self) -> bool {
      self.__grost_unknown_buffer__.is_none() && self.foo.is_none()
    }
    /// Returns the original encoded type of the partial decoded object.
    #[inline]
    pub const fn raw(&self) -> ::core::option::Option<&__GROST_READ_BUFFER__> {
      self.__grost_read_buffer__.as_ref()
    }
    /// Returns a reference to the unknown buffer, which holds the unknown data when decoding.
    #[inline]
    pub const fn unknown_buffer(&self) -> ::core::option::Option<&__GROST_UNKNOWN_BUFFER__> {
      self.__grost_unknown_buffer__.as_ref()
    }
    /// Returns a mutable reference to the unknown buffer, which holds the unknown data when decoding.
    #[inline]
    pub const fn unknown_buffer_mut(
      &mut self,
    ) -> ::core::option::Option<&mut __GROST_UNKNOWN_BUFFER__> {
      self.__grost_unknown_buffer__.as_mut()
    }
    /// Takes the unknown buffer out if the unknown buffer is not `None`.
    #[inline]
    pub const fn take_unknown_buffer(
      &mut self,
    ) -> ::core::option::Option<__GROST_UNKNOWN_BUFFER__> {
      self.__grost_unknown_buffer__.take()
    }
    /// Set the value of unknown buffer
    #[inline]
    pub fn set_unknown_buffer(&mut self, buffer: __GROST_UNKNOWN_BUFFER__) -> &mut Self {
      self.__grost_unknown_buffer__ = ::core::option::Option::Some(buffer);
      self
    }
    /// Clears the unknown buffer.
    #[inline]
    pub fn clear_unknown_buffer(&mut self) -> &mut Self {
      self.__grost_unknown_buffer__ = ::core::option::Option::None;
      self
    }
    /// Set the value of unknown buffer
    #[inline]
    pub fn with_unknown_buffer(mut self, buffer: __GROST_UNKNOWN_BUFFER__) -> Self {
      self.__grost_unknown_buffer__ = ::core::option::Option::Some(buffer);
      self
    }
    /// Clears the unknown buffer.
    #[inline]
    pub fn without_unknown_buffer(mut self) -> Self {
      self.__grost_unknown_buffer__ = ::core::option::Option::None;
      self
    }
    /// Returns a reference to the `foo`
    #[inline]
    const fn foo_ref(
      &self,
    ) -> ::core::option::Option<
      &<Option<u32> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_UNKNOWN_BUFFER__,
          <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output,
    > {
      self.foo.as_ref()
    }
    /// Returns a mutable reference to the `foo`
    #[inline]
    const fn foo_mut(
      &mut self,
    ) -> ::core::option::Option<
      &mut <Option<u32> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_UNKNOWN_BUFFER__,
          <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output,
    > {
      self.foo.as_mut()
    }
    /// Returns a reference to the `foo` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `foo` is `None`
    #[inline]
    const fn unwrap_foo_ref(
      &self,
    ) -> &<Option<u32> as ::grost::__private::convert::State<
      ::grost::__private::convert::PartialRef<
        '__grost_lifetime__,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
        <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
          ::grost::__private::flavors::Groto,
        >>::Format,
        ::grost::__private::flavors::Groto,
      >,
    >>::Output {
      match self.foo.as_ref() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => {
          ::core::panicking::panic_fmt(format_args!("`foo` is `None`"));
        }
      }
    }
    /// Returns a mutable reference to the `foo` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `foo` is `None`
    #[inline]
    const fn unwrap_foo_mut(
      &mut self,
    ) -> &mut <Option<u32> as ::grost::__private::convert::State<
      ::grost::__private::convert::PartialRef<
        '__grost_lifetime__,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
        <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
          ::grost::__private::flavors::Groto,
        >>::Format,
        ::grost::__private::flavors::Groto,
      >,
    >>::Output {
      match self.foo.as_mut() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => {
          ::core::panicking::panic_fmt(format_args!("`foo` is `None`"));
        }
      }
    }
    /// Takes the value of `foo` out if it is not `None`
    #[inline]
    const fn take_foo(
      &mut self,
    ) -> ::core::option::Option<
      <Option<u32> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_UNKNOWN_BUFFER__,
          <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output,
    > {
      self.foo.take()
    }
    /// Clear the value of `foo`
    #[inline]
    fn clear_foo(&mut self) -> &mut Self {
      self.foo = ::core::option::Option::None;
      self
    }
    /// Set the `foo` to the given value
    #[inline]
    fn set_foo(
      &mut self,
      value: <Option<u32> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_UNKNOWN_BUFFER__,
          <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output,
    ) -> &mut Self {
      self.foo = ::core::option::Option::Some(value);
      self
    }
    /// Update the `foo` to the given value or clear the `foo`
    #[inline]
    fn update_foo(
      &mut self,
      value: ::core::option::Option<
        <Option<u32> as ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
            <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >>::Output,
      >,
    ) -> &mut Self {
      self.foo = value;
      self
    }
    /// Set the `foo` to the given value
    #[inline]
    fn with_foo(
      mut self,
      value: <Option<u32> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_UNKNOWN_BUFFER__,
          <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output,
    ) -> Self {
      self.foo = ::core::option::Option::Some(value);
      self
    }
    /// Clear the value of `foo`
    #[inline]
    fn without_foo(mut self) -> Self {
      self.foo = ::core::option::Option::None;
      self
    }
    /// Update the `foo` to the given value or clear the `foo`
    #[inline]
    fn maybe_foo(
      mut self,
      value: ::core::option::Option<
        <Option<u32> as ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
            <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >>::Output,
      >,
    ) -> Self {
      self.foo = value;
      self
    }
  }
  #[automatically_derived]
  #[allow(clippy::type_complexity, non_camel_case_types)]
  impl ::grost::__private::reflection::Reflectable<OptionalScalar>
    for ::grost::__private::reflection::ObjectFieldReflection<
      OptionalScalar,
      ::grost::__private::reflection::ObjectField,
      ::grost::__private::flavors::Groto,
      1u32,
    >
  {
    type Reflection = ::grost::__private::reflection::ObjectField;
    const REFLECTION: &'static Self::Reflection = &{
      ::grost::__private::reflection::ObjectFieldBuilder {
                name: "foo",
                description: "",
                ty: <::grost::__private::reflection::SchemaTypeReflection<
                    Option<u32>,
                > as ::grost::__private::reflection::Reflectable<
                    Option<u32>,
                >>::REFLECTION,
            }
                .build()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<OptionalScalar>
    for ::grost::__private::reflection::WireFormatReflection<
      OptionalScalar,
      ::grost::__private::flavors::Groto,
      1u32,
    >
  {
    type Reflection = <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
      ::grost::__private::flavors::Groto,
    >>::Format;
    const REFLECTION: &'static Self::Reflection = &{
      <<Option<
                u32,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Groto,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Groto,
            >>::SELF
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<OptionalScalar>
    for ::grost::__private::reflection::IdentifierReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        OptionalScalar,
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
        ::grost::__private::flavors::Groto,
        1u32,
      >,
    >
  {
    type Reflection =
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier;
    const REFLECTION: &Self::Reflection = &{
      (::grost::__private::flavors::groto::Identifier::new)(
        <<Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
          ::grost::__private::flavors::Groto,
        >>::Format as ::grost::__private::flavors::WireFormat<
          ::grost::__private::flavors::Groto,
        >>::WIRE_TYPE,
        (::grost::__private::flavors::groto::Tag::new)(1u32),
      )
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<OptionalScalar>
    for ::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          OptionalScalar,
          <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
          ::grost::__private::flavors::Groto,
          1u32,
        >,
      >,
    >
  {
    type Reflection = [::core::primitive::u8];
    const REFLECTION: &Self::Reflection = {
      (::grost::__private::flavors::groto::Identifier::encode)(
        <::grost::__private::reflection::IdentifierReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            OptionalScalar,
            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
            ::grost::__private::flavors::Groto,
            1u32,
          >,
        > as ::grost::__private::reflection::Reflectable<OptionalScalar>>::REFLECTION,
      )
      .as_slice()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<OptionalScalar>
    for ::grost::__private::reflection::Len<
      ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::IdentifierReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            OptionalScalar,
            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
            ::grost::__private::flavors::Groto,
            1u32,
          >,
        >,
      >,
    >
  {
    type Reflection = ::core::primitive::usize;
    const REFLECTION: &Self::Reflection = {
      &<::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::IdentifierReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            OptionalScalar,
            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
            ::grost::__private::flavors::Groto,
            1u32,
          >,
        >,
      > as ::grost::__private::reflection::Reflectable<OptionalScalar>>::REFLECTION
        .len()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<OptionalScalar>
    for ::grost::__private::reflection::TagReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        OptionalScalar,
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
        ::grost::__private::flavors::Groto,
        1u32,
      >,
    >
  {
    type Reflection =
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag;
    const REFLECTION: &Self::Reflection = &{ (::grost::__private::flavors::groto::Tag::new)(1u32) };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<OptionalScalar>
    for ::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          OptionalScalar,
          <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
          ::grost::__private::flavors::Groto,
          1u32,
        >,
      >,
    >
  {
    type Reflection = [::core::primitive::u8];
    const REFLECTION: &Self::Reflection = {
      (::grost::__private::flavors::groto::Tag::encode)(
        <::grost::__private::reflection::TagReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            OptionalScalar,
            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Groto,
            1u32,
          >,
        > as ::grost::__private::reflection::Reflectable<OptionalScalar>>::REFLECTION,
      )
      .as_slice()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<OptionalScalar>
    for ::grost::__private::reflection::Len<
      ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::TagReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            OptionalScalar,
            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Groto,
            1u32,
          >,
        >,
      >,
    >
  {
    type Reflection = ::core::primitive::usize;
    const REFLECTION: &Self::Reflection = {
      &<::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::TagReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            OptionalScalar,
            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Groto,
            1u32,
          >,
        >,
      > as ::grost::__private::reflection::Reflectable<OptionalScalar>>::REFLECTION
        .len()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<OptionalScalar>
    for ::grost::__private::reflection::WireSchemaTypeReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        OptionalScalar,
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::WireType,
        ::grost::__private::flavors::Groto,
        1u32,
      >,
    >
  {
    type Reflection =
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::WireType;
    const REFLECTION: &Self::Reflection = &{
      <<Option<
                u32,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Groto,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Groto,
            >>::WIRE_TYPE
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<OptionalScalar> for OptionalScalar {
    type Reflection = ::grost::__private::reflection::SchemaType;
    const REFLECTION: &'static Self::Reflection = &{
      ::grost::__private::reflection::SchemaType::Object(
                &::grost::__private::reflection::ObjectBuilder {
                    name: "OptionalScalar",
                    description: "",
                    fields: &[
                        &::grost::__private::reflection::ObjectFieldBuilder {
                            name: "foo",
                            description: "",
                            ty: <::grost::__private::reflection::SchemaTypeReflection<
                                Option<u32>,
                            > as ::grost::__private::reflection::Reflectable<
                                Option<u32>,
                            >>::REFLECTION,
                        }
                            .build(),
                    ],
                }
                    .build(),
            )
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<OptionalScalar>
    for ::grost::__private::reflection::Reflection<
      OptionalScalar,
      ::grost::__private::reflection::Object,
      ::grost::__private::flavors::Groto,
    >
  {
    type Reflection = ::grost::__private::reflection::Object;
    const REFLECTION: &'static Self::Reflection = &{
      ::grost::__private::reflection::ObjectBuilder {
                name: "OptionalScalar",
                description: "",
                fields: &[
                    &::grost::__private::reflection::ObjectFieldBuilder {
                        name: "foo",
                        description: "",
                        ty: <::grost::__private::reflection::SchemaTypeReflection<
                            Option<u32>,
                        > as ::grost::__private::reflection::Reflectable<
                            Option<u32>,
                        >>::REFLECTION,
                    }
                        .build(),
                ],
            }
                .build()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl OptionalScalar {
    /// Returns the reflection of the struct.
    #[inline]
    pub const fn reflection() -> ::grost::__private::reflection::Reflection<
      Self,
      ::grost::__private::reflection::Object,
      ::grost::__private::flavors::Groto,
    >
    where
      ::grost::__private::flavors::Groto: ::grost::__private::flavors::Flavor,
    {
      ::grost::__private::reflection::Reflection::new()
    }
    /// Returns the field reflection of the field `OptionalScalar.foo`.
    #[inline]
    const fn foo_reflection() -> ::grost::__private::reflection::ObjectFieldReflection<
      OptionalScalar,
      ::grost::__private::reflection::ObjectField,
      ::grost::__private::flavors::Groto,
      1u32,
    >
    where
      ::grost::__private::flavors::Groto: ::grost::__private::flavors::Flavor,
    {
      ::grost::__private::reflection::ObjectFieldReflection::new()
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl ::grost::__private::indexer::Indexable<::grost::__private::flavors::Groto> for OptionalScalar {
    type Indexer = OptionalScalarIndex;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl OptionalScalarIndex {
    /// Returns the field reflection of the corresponding field.
    #[allow(non_camel_case_types, clippy::type_complexity)]
    #[inline]
    pub const fn reflection(&self) -> &'static ::grost::__private::reflection::ObjectField {
      match self {
        Self::Foo => {
          <::grost::__private::reflection::ObjectFieldReflection<
            OptionalScalar,
            ::grost::__private::reflection::ObjectField,
            ::grost::__private::flavors::Groto,
            1u32,
          > as ::grost::__private::reflection::Reflectable<OptionalScalar>>::REFLECTION
        }
      }
    }
  }
  #[automatically_derived]
  impl OptionalScalarIndex {
    /// The number of variants of this field indexer.
    pub const VARIANTS: ::core::primitive::usize = 1usize;
    /// The first field indexer.
    pub const FIRST: Self = Self::Foo;
    /// The last field indexer.
    pub const LAST: Self = Self::Foo;
    /// Returns the next field indexer.
    ///
    /// Returns `None` if there are no more fields.
    #[inline]
    pub const fn next(&self) -> ::core::option::Option<Self> {
      match self {
        Self::Foo => ::core::option::Option::None,
      }
    }
    /// Returns the previous field indexer.
    ///
    /// Returns `None` if there are no previous fields.
    #[inline]
    pub const fn prev(&self) -> ::core::option::Option<Self> {
      match self {
        Self::Foo => ::core::option::Option::None,
      }
    }
    /// Returns the remaining number of fields.
    #[inline]
    pub const fn remaining(&self) -> ::core::primitive::usize {
      Self::LAST.index() - self.index()
    }
    const fn index(&self) -> ::core::primitive::usize {
      match self {
        Self::Foo => 0usize,
      }
    }
  }
  #[automatically_derived]
  impl ::core::iter::Iterator for OptionalScalarIndex {
    type Item = Self;
    fn next(&mut self) -> ::core::option::Option<Self> {
      Self::next(self)
    }
    fn size_hint(
      &self,
    ) -> (
      ::core::primitive::usize,
      ::core::option::Option<::core::primitive::usize>,
    ) {
      let remaining = self.remaining();
      (remaining, ::core::option::Option::Some(remaining))
    }
  }
  #[automatically_derived]
  impl ::core::iter::DoubleEndedIterator for OptionalScalarIndex {
    fn next_back(&mut self) -> ::core::option::Option<Self> {
      Self::prev(self)
    }
  }
  #[automatically_derived]
  impl ::core::iter::FusedIterator for OptionalScalarIndex {}
  #[automatically_derived]
  impl ::core::iter::ExactSizeIterator for OptionalScalarIndex {
    fn len(&self) -> ::core::primitive::usize {
      self.remaining()
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>
    for OptionalScalar
  {
    type Selector = OptionalScalarSelector;
    fn is_empty(&self) -> ::core::primitive::bool {
      false
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>
    for PartialOptionalScalar
  {
    type Selector = OptionalScalarSelector;
    fn is_empty(&self) -> ::core::primitive::bool {
      Self::is_empty(self)
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, __GROST_READ_BUFFER__, __GROST_UNKNOWN_BUFFER__>
    ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>
    for PartialOptionalScalarRef<
      '__grost_lifetime__,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
    >
  {
    type Selector = OptionalScalarSelector;
    fn is_empty(&self) -> ::core::primitive::bool {
      Self::is_empty(self)
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl ::core::fmt::Debug for OptionalScalarSelector {
    fn fmt(
      &self,
      f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::result::Result<(), ::core::fmt::Error> {
      f.debug_struct("OptionalScalarSelector")
        .field("foo", &self.foo)
        .finish()
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl ::core::cmp::PartialEq for OptionalScalarSelector {
    fn eq(&self, other: &Self) -> ::core::primitive::bool {
      self.foo == other.foo
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl ::core::cmp::Eq for OptionalScalarSelector {}
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl ::core::hash::Hash for OptionalScalarSelector {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
      self.foo.hash(state);
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl ::core::clone::Clone for OptionalScalarSelector {
    fn clone(&self) -> Self {
      Self {
        foo: ::core::clone::Clone::clone(&self.foo),
      }
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl ::core::marker::Copy for OptionalScalarSelector where
    <Option<u32> as ::grost::__private::selection::Selectable<
      ::grost::__private::flavors::Groto,
    >>::Selector: ::core::marker::Copy
  {
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl ::grost::__private::selection::Selector<::grost::__private::flavors::Groto>
    for OptionalScalarSelector
  {
    const ALL: Self = Self::all();
    const DEFAULT: Self = Self::new();
    const NONE: Self = Self::empty();
    fn selected(&self) -> ::core::primitive::usize {
      Self::selected(self)
    }
    fn unselected(&self) -> ::core::primitive::usize {
      Self::unselected(self)
    }
    fn flip(&mut self) -> &mut Self {
      <<Option<u32> as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Groto,
      >>::Selector as ::grost::__private::selection::Selector<
        ::grost::__private::flavors::Groto,
      >>::flip(&mut self.foo);
      self
    }
    fn merge(&mut self, other: Self) -> &mut Self {
      <<Option<u32> as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Groto,
      >>::Selector as ::grost::__private::selection::Selector<
        ::grost::__private::flavors::Groto,
      >>::merge(&mut self.foo, other.foo);
      self
    }
  }
  #[automatically_derived]
  impl OptionalScalarSelector {
    /// Returns a selector with the default values.
    #[inline]
    pub const fn new() -> Self {
      Self {
        foo: <<Option<u32> as ::grost::__private::selection::Selectable<
          ::grost::__private::flavors::Groto,
        >>::Selector as ::grost::__private::selection::Selector<
          ::grost::__private::flavors::Groto,
        >>::DEFAULT,
      }
    }
    /// Returns a selector which selects nothing.
    #[inline]
    pub const fn empty() -> Self {
      Self {
        foo: <<Option<u32> as ::grost::__private::selection::Selectable<
          ::grost::__private::flavors::Groto,
        >>::Selector as ::grost::__private::selection::Selector<
          ::grost::__private::flavors::Groto,
        >>::NONE,
      }
    }
    /// Returns a selector which selects all.
    #[inline]
    pub const fn all() -> Self {
      Self {
        foo: <<Option<u32> as ::grost::__private::selection::Selectable<
          ::grost::__private::flavors::Groto,
        >>::Selector as ::grost::__private::selection::Selector<
          ::grost::__private::flavors::Groto,
        >>::ALL,
      }
    }
    /// Returns `true` if the selector selects nothing.
    #[inline]
    pub fn is_empty(&self) -> ::core::primitive::bool {
      <<Option<
                u32,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Groto,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Groto,
            >>::is_empty(&self.foo)
    }
    /// Returns `true` if the selector selects all.
    #[inline]
    pub fn is_all(&self) -> ::core::primitive::bool {
      <<Option<
                u32,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Groto,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Groto,
            >>::is_all(&self.foo)
    }
    /// Returns the number of selected fields.
    #[inline]
    pub fn selected(&self) -> ::core::primitive::usize {
      let mut num = 0;
      if self.is_foo_selected() {
        num += 1;
      }
      num
    }
    /// Returns the number of unselected fields.
    #[inline]
    pub fn unselected(&self) -> ::core::primitive::usize {
      let mut num = 0;
      if self.is_foo_unselected() {
        num += 1;
      }
      num
    }
    /// Returns an iterator over the selected fields.
    #[inline]
    pub fn iter_selected<'__grost_lifetime__>(
      &'__grost_lifetime__ self,
    ) -> OptionalScalarSelectorIter<'__grost_lifetime__, true> {
      OptionalScalarSelectorIter::new(self, self.selected())
    }
    /// Returns an iterator over the unselected fields.
    #[inline]
    pub fn iter_unselected<'__grost_lifetime__>(
      &'__grost_lifetime__ self,
    ) -> OptionalScalarSelectorIter<'__grost_lifetime__, false> {
      OptionalScalarSelectorIter::new(self, self.unselected())
    }
    /// Returns `true` if such field is selected.
    #[inline]
    pub fn is_selected(&self, idx: OptionalScalarIndex) -> ::core::primitive::bool {
      match idx {
        OptionalScalarIndex::Foo => self.is_foo_selected(),
      }
    }
    /// Returns `true` if such field is unselected.
    #[inline]
    pub fn is_unselected(&self, idx: OptionalScalarIndex) -> ::core::primitive::bool {
      match idx {
        OptionalScalarIndex::Foo => self.is_foo_unselected(),
      }
    }
    /// Select the `OptionalScalar.foo` field
    #[inline]
    pub fn select_foo(&mut self) -> &mut Self {
      self.foo = <<Option<u32> as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Groto,
      >>::Selector as ::grost::__private::selection::Selector<
        ::grost::__private::flavors::Groto,
      >>::DEFAULT;
      self
    }
    /// Unselect the `OptionalScalar.foo` field
    #[inline]
    pub fn unselect_foo(&mut self) -> &mut Self {
      self.foo = <<Option<u32> as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Groto,
      >>::Selector as ::grost::__private::selection::Selector<
        ::grost::__private::flavors::Groto,
      >>::NONE;
      self
    }
    /// Update the `OptionalScalar.foo` field
    #[inline]
    pub fn update_foo(
      &mut self,
      value: <Option<u32> as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Groto,
      >>::Selector,
    ) -> &mut Self {
      self.foo = value;
      self
    }
    /// Set or unset the `OptionalScalar.foo` field
    #[inline]
    pub fn maybe_foo(
      mut self,
      val: <Option<u32> as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Groto,
      >>::Selector,
    ) -> Self {
      self.foo = val;
      self
    }
    /// Get a reference to the selector of `OptionalScalar.foo` field
    #[inline]
    pub const fn foo_ref(
      &self,
    ) -> &<Option<u32> as ::grost::__private::selection::Selectable<
      ::grost::__private::flavors::Groto,
    >>::Selector {
      &self.foo
    }
    /// Get a mutable reference to the selector of `OptionalScalar.foo` field
    #[inline]
    pub const fn foo_mut(
      &mut self,
    ) -> &mut <Option<u32> as ::grost::__private::selection::Selectable<
      ::grost::__private::flavors::Groto,
    >>::Selector {
      &mut self.foo
    }
    /// Set the `OptionalScalar.foo` field
    #[inline]
    pub fn with_foo(mut self) -> Self {
      self.foo = <<Option<u32> as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Groto,
      >>::Selector as ::grost::__private::selection::Selector<
        ::grost::__private::flavors::Groto,
      >>::DEFAULT;
      self
    }
    /// Unset the `OptionalScalar.foo` field
    #[inline]
    pub fn without_foo(mut self) -> Self {
      self.foo = <<Option<u32> as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Groto,
      >>::Selector as ::grost::__private::selection::Selector<
        ::grost::__private::flavors::Groto,
      >>::NONE;
      self
    }
    /// Returns `true` if the `OptionalScalar.foo` field is selected
    #[inline]
    pub fn is_foo_selected(&self) -> ::core::primitive::bool {
      !<<Option<
                u32,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Groto,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Groto,
            >>::is_empty(&self.foo)
    }
    /// Returns `true` if the `OptionalScalar.foo` field is unselected
    #[inline]
    pub fn is_foo_unselected(&self) -> ::core::primitive::bool {
      <<Option<
                u32,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Groto,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Groto,
            >>::is_empty(&self.foo)
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, const __GROST_SELECTED__: ::core::primitive::bool>
    OptionalScalarSelectorIter<'__grost_lifetime__, __GROST_SELECTED__>
  {
    #[inline]
    const fn new(
      selector: &'__grost_lifetime__ OptionalScalarSelector,
      num: ::core::primitive::usize,
    ) -> Self {
      Self {
        selector,
        index: ::core::option::Option::Some(OptionalScalarIndex::FIRST),
        num,
        yielded: 0,
      }
    }
    /// Returns the exact remaining length of the iterator.
    #[inline]
    pub const fn remaining(&self) -> ::core::primitive::usize {
      self.num - self.yielded
    }
    /// Returns `true` if the iterator is empty.
    #[inline]
    pub const fn is_empty(&self) -> ::core::primitive::bool {
      self.remaining() == 0
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, const __GROST_SELECTED__: ::core::primitive::bool>
    ::core::iter::Iterator for OptionalScalarSelectorIter<'__grost_lifetime__, __GROST_SELECTED__>
  {
    type Item = &'static ::grost::__private::reflection::ObjectField;
    fn next(&mut self) -> ::core::option::Option<Self::Item> {
      if self.yielded >= self.num {
        return ::core::option::Option::None;
      }
      let mut current_index = self.index;
      while let ::core::option::Option::Some(idx) = current_index {
        if __GROST_SELECTED__ {
          if self.selector.is_selected(idx) {
            self.index = idx.next();
            self.yielded += 1;
            return ::core::option::Option::Some(idx.reflection());
          }
        } else if self.selector.is_unselected(idx) {
          self.index = idx.next();
          self.yielded += 1;
          return ::core::option::Option::Some(idx.reflection());
        }
        current_index = idx.next();
      }
      ::core::option::Option::None
    }
    fn size_hint(
      &self,
    ) -> (
      ::core::primitive::usize,
      ::core::option::Option<::core::primitive::usize>,
    ) {
      let remaining = self.remaining();
      (remaining, ::core::option::Option::Some(remaining))
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, const __GROST_SELECTED__: ::core::primitive::bool>
    ::core::iter::FusedIterator
    for OptionalScalarSelectorIter<'__grost_lifetime__, __GROST_SELECTED__>
  {
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, const __GROST_SELECTED__: ::core::primitive::bool>
    ::core::iter::ExactSizeIterator
    for OptionalScalarSelectorIter<'__grost_lifetime__, __GROST_SELECTED__>
  {
    fn len(&self) -> ::core::primitive::usize {
      self.remaining()
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl
    ::grost::__private::convert::Transform<
      Self,
      Self,
      ::grost::__private::flavors::groto::LengthDelimited,
      ::grost::__private::flavors::Groto,
    > for OptionalScalar
  {
    fn transform(
      input: Self,
    ) -> ::core::result::Result<
      Self,
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
    > {
      ::core::result::Result::Ok(input)
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, __GROST_READ_BUFFER__, __GROST_UNKNOWN_BUFFER__>
    ::grost::__private::decode::Decode<
      '__grost_lifetime__,
      ::grost::__private::flavors::Groto,
      ::grost::__private::flavors::groto::LengthDelimited,
      Self,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
    > for PartialOptionalScalar
  {
    fn decode(
      context: &'__grost_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
      src: __GROST_READ_BUFFER__,
    ) -> ::core::result::Result<
      (::core::primitive::usize, Self),
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
    >
    where
      Self: ::core::marker::Sized + '__grost_lifetime__,
      __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf + '__grost_lifetime__,
      __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
          <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
            __GROST_READ_BUFFER__,
          >,
        > + '__grost_lifetime__,
    {
      <PartialOptionalScalar as ::grost::__private::decode::Decode<
        '__grost_lifetime__,
        ::grost::__private::flavors::Groto,
        ::grost::__private::flavors::groto::LengthDelimited,
        PartialOptionalScalarRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_UNKNOWN_BUFFER__,
        >,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
      >>::decode(context, src)
      .and_then(|(read, input)| {
        <PartialOptionalScalar as ::grost::__private::convert::Transform<
          PartialOptionalScalarRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
          >,
          PartialOptionalScalar,
          ::grost::__private::flavors::groto::LengthDelimited,
          ::grost::__private::flavors::Groto,
        >>::transform(input)
        .map(|input| (read, input))
      })
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, __GROST_READ_BUFFER__, __GROST_UNKNOWN_BUFFER__>
    ::grost::__private::decode::Decode<
      '__grost_lifetime__,
      ::grost::__private::flavors::Groto,
      ::grost::__private::flavors::groto::LengthDelimited,
      Self,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
    > for OptionalScalar
  {
    fn decode(
      context: &'__grost_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
      src: __GROST_READ_BUFFER__,
    ) -> ::core::result::Result<
      (::core::primitive::usize, Self),
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
    >
    where
      Self: ::core::marker::Sized + '__grost_lifetime__,
      __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf + '__grost_lifetime__,
      __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
          <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
            __GROST_READ_BUFFER__,
          >,
        > + '__grost_lifetime__,
    {
      <PartialOptionalScalar as ::grost::__private::decode::Decode<
        '__grost_lifetime__,
        ::grost::__private::flavors::Groto,
        ::grost::__private::flavors::groto::LengthDelimited,
        PartialOptionalScalar,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
      >>::decode(context, src)
      .and_then(|(read, input)| {
        <OptionalScalar as ::grost::__private::convert::Transform<
          PartialOptionalScalar,
          OptionalScalar,
          ::grost::__private::flavors::groto::LengthDelimited,
          ::grost::__private::flavors::Groto,
        >>::transform(input)
        .map(|input| (read, input))
      })
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl
    ::grost::__private::convert::Transform<
      Self,
      Self,
      ::grost::__private::flavors::groto::LengthDelimited,
      ::grost::__private::flavors::Groto,
    > for PartialOptionalScalar
  {
    fn transform(
      input: Self,
    ) -> ::core::result::Result<
      Self,
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
    > {
      ::core::result::Result::Ok(input)
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl
    ::grost::__private::convert::Transform<
      PartialOptionalScalar,
      OptionalScalar,
      ::grost::__private::flavors::groto::LengthDelimited,
      ::grost::__private::flavors::Groto,
    > for OptionalScalar
  {
    fn transform(
      input: PartialOptionalScalar,
    ) -> ::core::result::Result<
      OptionalScalar,
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
    > {
      ::core::result::Result::Ok(Self { foo: input.foo })
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl
    ::grost::__private::convert::PartialTransform<
      Self,
      ::core::option::Option<Self>,
      ::grost::__private::flavors::groto::LengthDelimited,
      ::grost::__private::flavors::Groto,
    > for PartialOptionalScalar
  {
    fn partial_transform(
      input: PartialOptionalScalar,
      selector: &OptionalScalarSelector,
    ) -> ::core::result::Result<
      ::core::option::Option<PartialOptionalScalar>,
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
    > {
      let mut this = Self::new();
      if let ::core::option::Option::Some(value) = input.foo {
        if selector.is_foo_selected() {
          this.foo = <<Option<u32> as ::grost::__private::convert::State<
            ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
          >>::Output as ::grost::__private::convert::PartialTransform<
            <Option<u32> as ::grost::__private::convert::State<
              ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output,
            ::core::option::Option<
              <Option<u32> as ::grost::__private::convert::State<
                ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
              >>::Output,
            >,
            <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >>::partial_transform(value, selector.foo_ref())?;
        }
      }
      ::core::result::Result::Ok((!this.is_empty()).then_some(this))
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, __GROST_READ_BUFFER__, __GROST_UNKNOWN_BUFFER__>
    ::grost::__private::convert::Transform<
      Self,
      Self,
      ::grost::__private::flavors::groto::LengthDelimited,
      ::grost::__private::flavors::Groto,
    >
    for PartialOptionalScalarRef<
      '__grost_lifetime__,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
    >
  {
    fn transform(
      input: Self,
    ) -> ::core::result::Result<
      Self,
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
    > {
      ::core::result::Result::Ok(input)
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<
    '__grost_lifetime__,
    '__grost_decode_lifetime__,
    __GROST_READ_BUFFER__,
    __GROST_UNKNOWN_BUFFER__,
  >
    ::grost::__private::decode::Decode<
      '__grost_decode_lifetime__,
      ::grost::__private::flavors::Groto,
      ::grost::__private::flavors::groto::LengthDelimited,
      Self,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
    >
    for PartialOptionalScalarRef<
      '__grost_lifetime__,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
    >
  where
    '__grost_decode_lifetime__: '__grost_lifetime__,
  {
    fn decode(
      context: &'__grost_decode_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
      src: __GROST_READ_BUFFER__,
    ) -> ::core::result::Result<
      (::core::primitive::usize, Self),
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
    >
    where
      Self: ::core::marker::Sized + '__grost_decode_lifetime__,
      __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf + '__grost_decode_lifetime__,
      __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
          <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
            __GROST_READ_BUFFER__,
          >,
        > + '__grost_decode_lifetime__,
    {
      let buf = src.as_bytes();
      let buf_len = buf.len();
      let mut this = Self::new();
      let mut offset = 0;
      while offset < buf_len {
        let (read, identifier) = <<::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier as ::grost::__private::flavors::Identifier<
                    ::grost::__private::flavors::Groto,
                >>::decode::<&[::core::primitive::u8]>(&buf[offset..])?;
        offset += read;
        match &identifier {
                    <::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            OptionalScalar,
                            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Groto,
                            1u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        OptionalScalar,
                    >>::REFLECTION => {
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::core::convert::Into::into(
                                    ::grost::__private::error::Error::buffer_underflow(),
                                ),
                            );
                        }
                        if this.foo.is_some() {
                            return ::core::result::Result::Err(
                                ::core::convert::Into::into(
                                    ::grost::__private::error::Error::duplicated_field(
                                        "foo",
                                        ::core::any::type_name::<Option<u32>>(),
                                        *<::grost::__private::reflection::IdentifierReflection<
                                            ::grost::__private::reflection::ObjectFieldReflection<
                                                OptionalScalar,
                                                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                                                ::grost::__private::flavors::Groto,
                                                1u32,
                                            >,
                                        > as ::grost::__private::reflection::Reflectable<
                                            OptionalScalar,
                                        >>::REFLECTION,
                                    ),
                                ),
                            );
                        }
                        let (read, value) = (<Option<
                            u32,
                        > as ::grost::__private::decode::Decode<
                            '__grost_decode_lifetime__,
                            ::grost::__private::flavors::Groto,
                            <Option<
                                u32,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Groto,
                            >>::Format,
                            <Option<
                                u32,
                            > as ::grost::__private::convert::State<
                                ::grost::__private::convert::PartialRef<
                                    '__grost_lifetime__,
                                    __GROST_READ_BUFFER__,
                                    __GROST_UNKNOWN_BUFFER__,
                                    <Option<
                                        u32,
                                    > as ::grost::__private::flavors::DefaultWireFormat<
                                        ::grost::__private::flavors::Groto,
                                    >>::Format,
                                    ::grost::__private::flavors::Groto,
                                >,
                            >>::Output,
                            __GROST_READ_BUFFER__,
                            __GROST_UNKNOWN_BUFFER__,
                        >>::decode)(context, src.slice(offset..))?;
                        this.foo = ::core::option::Option::Some(value);
                        offset += read;
                    }
                    _ => {
                        if context.err_on_unknown() {
                            return ::core::result::Result::Err(
                                ::core::convert::Into::into(
                                    ::grost::__private::error::Error::unknown_identifier(
                                        ::core::any::type_name::<OptionalScalar>(),
                                        identifier,
                                    ),
                                ),
                            );
                        }
                        if context.skip_unknown() {
                            if offset >= buf_len {
                                return ::core::result::Result::Err(
                                    ::core::convert::Into::into(
                                        ::grost::__private::error::Error::buffer_underflow(),
                                    ),
                                );
                            }
                            offset
                                += <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::skip(
                                    context,
                                    identifier.wire_type(),
                                    src.slice(offset..),
                                )?;
                        } else {
                            let encoded_len = <<::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier as ::grost::__private::flavors::Identifier<
                                ::grost::__private::flavors::Groto,
                            >>::encoded_len(&identifier);
                            let (read, unknown) = <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::decode_unknown(
                                context,
                                src.slice(offset - encoded_len..),
                            )?;
                            offset += read;
                            let unknowns_mut = this
                                .__grost_unknown_buffer__
                                .get_or_insert_with(|| __GROST_UNKNOWN_BUFFER__::new());
                            if let ::core::option::Option::Some(unknown) = unknowns_mut
                                .push(unknown)
                            {
                                let len = ::grost::__private::Buffer::len(unknowns_mut);
                                return ::core::result::Result::Err(
                                    ::core::convert::Into::into(
                                        ::grost::__private::error::Error::buffer_overflow(
                                            len,
                                            ::core::num::NonZeroUsize::new(len + 1).unwrap(),
                                        ),
                                    ),
                                );
                            }
                        }
                    }
                }
      }
      ::core::result::Result::Ok((offset, this))
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<
    '__grost_lifetime__,
    '__grost_decode_lifetime__,
    __GROST_READ_BUFFER__,
    __GROST_UNKNOWN_BUFFER__,
  >
    ::grost::__private::decode::Decode<
      '__grost_decode_lifetime__,
      ::grost::__private::flavors::Groto,
      ::grost::__private::flavors::groto::LengthDelimited,
      PartialOptionalScalarRef<
        '__grost_lifetime__,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
      >,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
    > for OptionalScalar
  where
    '__grost_decode_lifetime__: '__grost_lifetime__,
  {
    fn decode(
      context: &'__grost_decode_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
      src: __GROST_READ_BUFFER__,
    ) -> ::core::result::Result<
      (
        ::core::primitive::usize,
        PartialOptionalScalarRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_UNKNOWN_BUFFER__,
        >,
      ),
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
    >
    where
      PartialOptionalScalarRef<
        '__grost_lifetime__,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
      >: ::core::marker::Sized + '__grost_decode_lifetime__,
      __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf + '__grost_decode_lifetime__,
      __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
          <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
            __GROST_READ_BUFFER__,
          >,
        > + '__grost_decode_lifetime__,
    {
      <PartialOptionalScalarRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            > as ::grost::__private::decode::Decode<
                '__grost_decode_lifetime__,
                ::grost::__private::flavors::Groto,
                ::grost::__private::flavors::groto::LengthDelimited,
                PartialOptionalScalarRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >>::decode(context, src)
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<
    '__grost_lifetime__,
    '__grost_decode_lifetime__,
    __GROST_READ_BUFFER__,
    __GROST_UNKNOWN_BUFFER__,
  >
    ::grost::__private::decode::Decode<
      '__grost_decode_lifetime__,
      ::grost::__private::flavors::Groto,
      ::grost::__private::flavors::groto::LengthDelimited,
      PartialOptionalScalarRef<
        '__grost_lifetime__,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
      >,
      __GROST_READ_BUFFER__,
      __GROST_UNKNOWN_BUFFER__,
    > for PartialOptionalScalar
  where
    '__grost_decode_lifetime__: '__grost_lifetime__,
  {
    fn decode(
      context: &'__grost_decode_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
      src: __GROST_READ_BUFFER__,
    ) -> ::core::result::Result<
      (
        ::core::primitive::usize,
        PartialOptionalScalarRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_UNKNOWN_BUFFER__,
        >,
      ),
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
    >
    where
      PartialOptionalScalarRef<
        '__grost_lifetime__,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
      >: ::core::marker::Sized + '__grost_decode_lifetime__,
      __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf + '__grost_decode_lifetime__,
      __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
          <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
            __GROST_READ_BUFFER__,
          >,
        > + '__grost_decode_lifetime__,
    {
      <PartialOptionalScalarRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            > as ::grost::__private::decode::Decode<
                '__grost_decode_lifetime__,
                ::grost::__private::flavors::Groto,
                ::grost::__private::flavors::groto::LengthDelimited,
                PartialOptionalScalarRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >>::decode(context, src)
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, __GROST_READ_BUFFER__, __GROST_UNKNOWN_BUFFER__>
    ::grost::__private::convert::Transform<
      PartialOptionalScalarRef<
        '__grost_lifetime__,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
      >,
      PartialOptionalScalar,
      ::grost::__private::flavors::groto::LengthDelimited,
      ::grost::__private::flavors::Groto,
    > for PartialOptionalScalar
  where
    __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf + '__grost_lifetime__,
    __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
          __GROST_READ_BUFFER__,
        >,
      > + '__grost_lifetime__,
  {
    fn transform(
      input: PartialOptionalScalarRef<
        '__grost_lifetime__,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
      >,
    ) -> ::core::result::Result<
      Self,
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
    > {
      let mut this = Self::new();
      if let ::core::option::Option::Some(value) = input.foo {
        this.foo =
          ::core::option::Option::Some(<<Option<u32> as ::grost::__private::convert::State<
            ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
          >>::Output as ::grost::__private::convert::Transform<
            <Option<u32> as ::grost::__private::convert::State<
              ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
                <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
                  ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
              >,
            >>::Output,
            <Option<u32> as ::grost::__private::convert::State<
              ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output,
            <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >>::transform(value)?);
      }
      ::core::result::Result::Ok(this)
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<'__grost_lifetime__, __GROST_READ_BUFFER__, __GROST_UNKNOWN_BUFFER__>
    ::grost::__private::convert::PartialTransform<
      PartialOptionalScalarRef<
        '__grost_lifetime__,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
      >,
      ::core::option::Option<PartialOptionalScalar>,
      ::grost::__private::flavors::groto::LengthDelimited,
      ::grost::__private::flavors::Groto,
    > for PartialOptionalScalar
  where
    __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf + '__grost_lifetime__,
    __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
          __GROST_READ_BUFFER__,
        >,
      > + '__grost_lifetime__,
  {
    fn partial_transform(
      input: PartialOptionalScalarRef<
        '__grost_lifetime__,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
      >,
      selector: &OptionalScalarSelector,
    ) -> ::core::result::Result<
      ::core::option::Option<Self>,
      <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
    > {
      let mut this = Self::new();
      if let ::core::option::Option::Some(value) = input.foo {
        if selector.is_foo_selected() {
          this.foo = <<Option<u32> as ::grost::__private::convert::State<
            ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
          >>::Output as ::grost::__private::convert::PartialTransform<
            <Option<u32> as ::grost::__private::convert::State<
              ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
                <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
                  ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
              >,
            >>::Output,
            ::core::option::Option<
              <Option<u32> as ::grost::__private::convert::State<
                ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
              >>::Output,
            >,
            <Option<u32> as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >>::partial_transform(value, selector.foo_ref())?;
        }
      }
      ::core::result::Result::Ok((!this.is_empty()).then_some(this))
    }
  }
};
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
  extern crate test;
  test::test_main_static(&[])
}
