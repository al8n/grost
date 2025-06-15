#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use grost::{
  Decode, PartialDecode,
  buffer::Buf,
  flavors::{DefaultWireFormat, Network, WireFormat, network::LengthDelimited},
  selection::{Selectable, Selector},
};
use grost_derive::{Object, object};
use std::marker::PhantomData;
fn default_array<const N: usize>() -> [u8; N] {
  [0; N]
}
#[grost(
  flavor(default(encode(skip_default, enum = "grost::flavors::network::encoding::enumeration")),),
  generic(flavor = "F")
)]
pub struct User {
  #[grost(
    tag = 2,
    schema(description = "The nick name of the user"),
    selector(select = "all"),
    string
  )]
  name: String,
  #[grost(tag = 3, scalar, schema(description = "The age of the user"), copy)]
  age: u8,
  #[grost(
    tag = 4,
    schema(description = "The email of the user"),
    partial_decoded(copy),
    optional(list(string))
  )]
  emails: Option<Vec<String>>,
}
#[automatically_derived]
impl ::core::fmt::Debug for User {
  #[inline]
  fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
    ::core::fmt::Formatter::debug_struct_field3_finish(
      f,
      "User",
      "name",
      &self.name,
      "age",
      &self.age,
      "emails",
      &&self.emails,
    )
  }
}
#[automatically_derived]
impl ::core::clone::Clone for User {
  #[inline]
  fn clone(&self) -> User {
    User {
      name: ::core::clone::Clone::clone(&self.name),
      age: ::core::clone::Clone::clone(&self.age),
      emails: ::core::clone::Clone::clone(&self.emails),
    }
  }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for User {}
#[automatically_derived]
impl ::core::cmp::PartialEq for User {
  #[inline]
  fn eq(&self, other: &User) -> bool {
    self.name == other.name && self.age == other.age && self.emails == other.emails
  }
}
#[automatically_derived]
impl ::core::cmp::Eq for User {
  #[inline]
  #[doc(hidden)]
  #[coverage(off)]
  fn assert_receiver_is_total_eq(&self) -> () {
    let _: ::core::cmp::AssertParamIsEq<String>;
    let _: ::core::cmp::AssertParamIsEq<u8>;
    let _: ::core::cmp::AssertParamIsEq<Option<Vec<String>>>;
  }
}
/// Field indexer for the struct [`User`]
#[repr(u32)]
pub enum UserIndex {
  /// The field indexer for the field `name`
  Name = 2u32,
  /// The field indexer for the field `age`
  Age = 3u32,
  /// The field indexer for the field `emails`
  Emails = 4u32,
}
#[automatically_derived]
impl ::core::clone::Clone for UserIndex {
  #[inline]
  fn clone(&self) -> UserIndex {
    *self
  }
}
#[automatically_derived]
impl ::core::marker::Copy for UserIndex {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for UserIndex {}
#[automatically_derived]
impl ::core::cmp::PartialEq for UserIndex {
  #[inline]
  fn eq(&self, other: &UserIndex) -> bool {
    let __self_discr = ::core::intrinsics::discriminant_value(self);
    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
    __self_discr == __arg1_discr
  }
}
#[automatically_derived]
impl ::core::cmp::Eq for UserIndex {
  #[inline]
  #[doc(hidden)]
  #[coverage(off)]
  fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for UserIndex {
  #[inline]
  fn partial_cmp(&self, other: &UserIndex) -> ::core::option::Option<::core::cmp::Ordering> {
    let __self_discr = ::core::intrinsics::discriminant_value(self);
    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
    ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
  }
}
#[automatically_derived]
impl ::core::cmp::Ord for UserIndex {
  #[inline]
  fn cmp(&self, other: &UserIndex) -> ::core::cmp::Ordering {
    let __self_discr = ::core::intrinsics::discriminant_value(self);
    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
    ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
  }
}
#[automatically_derived]
impl ::core::hash::Hash for UserIndex {
  #[inline]
  fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
    let __self_discr = ::core::intrinsics::discriminant_value(self);
    ::core::hash::Hash::hash(&__self_discr, state)
  }
}
#[automatically_derived]
impl ::core::fmt::Debug for UserIndex {
  #[inline]
  fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
    ::core::fmt::Formatter::write_str(
      f,
      match self {
        UserIndex::Name => "Name",
        UserIndex::Age => "Age",
        UserIndex::Emails => "Emails",
      },
    )
  }
}
/// Partial struct for the [`User`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct PartialUser<__GROST_UNKNOWN_BUFFER__>
where
  String: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
  <String as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output:
    ::core::marker::Sized,
  u8: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
  <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output:
    ::core::marker::Sized,
  Option<Vec<String>>: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
  <Option<Vec<String>> as ::grost::__private::convert::State<
    ::grost::__private::convert::Flatten,
  >>::Output: ::core::marker::Sized,
{
  __grost_unknown_buffer__: ::core::option::Option<__GROST_UNKNOWN_BUFFER__>,
  name: ::core::option::Option<
    <String as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
  >,
  age: ::core::option::Option<
    <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
  >,
  emails: ::core::option::Option<
    <Option<Vec<String>> as ::grost::__private::convert::State<
      ::grost::__private::convert::Flatten,
    >>::Output,
  >,
}
/// Partial reference struct for the struct [`User`]
#[allow(clippy::type_complexity, non_camel_case_types)]
pub struct PartialDecodedUser<
    '__grost_lifetime__,
    F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    __GROST_UNKNOWN_BUFFER__,
>
where
    ::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        2u32,
    >: ::grost::__private::reflection::Reflectable<User>,
    <::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        2u32,
    > as ::grost::__private::reflection::Reflectable<
        User,
    >>::Reflection: ::grost::__private::flavors::WireFormat<F>,
    String: ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >,
    <String as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >>::Output: ::core::marker::Sized,
    ::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        3u32,
    >: ::grost::__private::reflection::Reflectable<User>,
    <::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        3u32,
    > as ::grost::__private::reflection::Reflectable<
        User,
    >>::Reflection: ::grost::__private::flavors::WireFormat<F>,
    u8: ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >,
    <u8 as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >>::Output: ::core::marker::Sized,
    ::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        4u32,
    >: ::grost::__private::reflection::Reflectable<User>,
    <::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        4u32,
    > as ::grost::__private::reflection::Reflectable<
        User,
    >>::Reflection: ::grost::__private::flavors::WireFormat<F>,
    Option<
        Vec<String>,
    >: ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                4u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >,
    <Option<
        Vec<String>,
    > as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                4u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >>::Output: ::core::marker::Sized + ::core::marker::Copy,
{
    __grost_unknown_buffer__: ::core::option::Option<__GROST_UNKNOWN_BUFFER__>,
    name: ::core::option::Option<
        <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output,
    >,
    age: ::core::option::Option<
        <u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output,
    >,
    emails: ::core::option::Option<
        <Option<
            Vec<String>,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output,
    >,
}
/// The selection type for [`User`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct UserSelector<F>
where
    ::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        2u32,
    >: ::grost::__private::reflection::Reflectable<User>,
    <::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        2u32,
    > as ::grost::__private::reflection::Reflectable<
        User,
    >>::Reflection: ::grost::__private::flavors::WireFormat<F>,
    String: ::grost::__private::selection::Selectable<
        F,
        <::grost::__private::reflection::WireFormatReflection<
            User,
            F,
            2u32,
        > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
    >,
    ::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        3u32,
    >: ::grost::__private::reflection::Reflectable<User>,
    <::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        3u32,
    > as ::grost::__private::reflection::Reflectable<
        User,
    >>::Reflection: ::grost::__private::flavors::WireFormat<F>,
    u8: ::grost::__private::selection::Selectable<
        F,
        <::grost::__private::reflection::WireFormatReflection<
            User,
            F,
            3u32,
        > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
    >,
    ::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        4u32,
    >: ::grost::__private::reflection::Reflectable<User>,
    <::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        4u32,
    > as ::grost::__private::reflection::Reflectable<
        User,
    >>::Reflection: ::grost::__private::flavors::WireFormat<F>,
    Option<
        Vec<String>,
    >: ::grost::__private::selection::Selectable<
        F,
        <::grost::__private::reflection::WireFormatReflection<
            User,
            F,
            4u32,
        > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
    >,
    F: ::grost::__private::flavors::Flavor,
{
    name: <String as ::grost::__private::selection::Selectable<
        F,
        <::grost::__private::reflection::WireFormatReflection<
            User,
            F,
            2u32,
        > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
    >>::Selector,
    age: <u8 as ::grost::__private::selection::Selectable<
        F,
        <::grost::__private::reflection::WireFormatReflection<
            User,
            F,
            3u32,
        > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
    >>::Selector,
    emails: <Option<
        Vec<String>,
    > as ::grost::__private::selection::Selectable<
        F,
        <::grost::__private::reflection::WireFormatReflection<
            User,
            F,
            4u32,
        > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
    >>::Selector,
}
/// An iterator over the selected fields of the [`UserSelector`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct UserSelectorIter<
    '__grost_lifetime__,
    F,
    const __GROST_SELECTED__: ::core::primitive::bool = true,
>
where
    ::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        2u32,
    >: ::grost::__private::reflection::Reflectable<User>,
    <::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        2u32,
    > as ::grost::__private::reflection::Reflectable<
        User,
    >>::Reflection: ::grost::__private::flavors::WireFormat<F>,
    String: ::grost::__private::selection::Selectable<
        F,
        <::grost::__private::reflection::WireFormatReflection<
            User,
            F,
            2u32,
        > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
    >,
    ::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        3u32,
    >: ::grost::__private::reflection::Reflectable<User>,
    <::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        3u32,
    > as ::grost::__private::reflection::Reflectable<
        User,
    >>::Reflection: ::grost::__private::flavors::WireFormat<F>,
    u8: ::grost::__private::selection::Selectable<
        F,
        <::grost::__private::reflection::WireFormatReflection<
            User,
            F,
            3u32,
        > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
    >,
    ::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        4u32,
    >: ::grost::__private::reflection::Reflectable<User>,
    <::grost::__private::reflection::WireFormatReflection<
        User,
        F,
        4u32,
    > as ::grost::__private::reflection::Reflectable<
        User,
    >>::Reflection: ::grost::__private::flavors::WireFormat<F>,
    Option<
        Vec<String>,
    >: ::grost::__private::selection::Selectable<
        F,
        <::grost::__private::reflection::WireFormatReflection<
            User,
            F,
            4u32,
        > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
    >,
    F: ::grost::__private::flavors::Flavor,
{
    selector: &'__grost_lifetime__ UserSelector<F>,
    index: ::core::option::Option<UserIndex>,
    num: ::core::primitive::usize,
    yielded: ::core::primitive::usize,
}
const _: () = {
  impl ::core::default::Default for User {
    fn default() -> Self {
      Self::new()
    }
  }
  impl User {
    /// Creates a new instance of the object with default values.
    pub fn new() -> Self {
      Self {
        name: (::core::default::Default::default)(),
        age: (::core::default::Default::default)(),
        emails: (::core::default::Default::default)(),
      }
    }
  }
  impl User {
    /// Returns a reference to the `name`
    #[inline]
    const fn name_ref(&self) -> &String {
      &self.name
    }
    /// Returns a mutable reference to the `name`
    #[inline]
    const fn name_mut(&mut self) -> &mut String {
      &mut self.name
    }
    /// Set the `name` to the given value
    #[inline]
    fn set_name(&mut self, value: String) -> &mut Self {
      self.name = value;
      self
    }
    /// Set the `name` to the given value
    #[inline]
    fn with_name(mut self, value: String) -> Self {
      self.name = value;
      self
    }
    /// Returns a reference to the `age`
    #[inline]
    const fn age_ref(&self) -> &u8 {
      &self.age
    }
    /// Returns a mutable reference to the `age`
    #[inline]
    const fn age_mut(&mut self) -> &mut u8 {
      &mut self.age
    }
    /// Set the `age` to the given value
    #[inline]
    const fn set_age(&mut self, value: u8) -> &mut Self {
      self.age = value;
      self
    }
    /// Set the `age` to the given value
    #[inline]
    const fn with_age(mut self, value: u8) -> Self {
      self.age = value;
      self
    }
    /// Returns a reference to the `emails`
    #[inline]
    const fn emails_ref(&self) -> &Option<Vec<String>> {
      &self.emails
    }
    /// Returns a mutable reference to the `emails`
    #[inline]
    const fn emails_mut(&mut self) -> &mut Option<Vec<String>> {
      &mut self.emails
    }
    /// Set the `emails` to the given value
    #[inline]
    fn set_emails(&mut self, value: Option<Vec<String>>) -> &mut Self {
      self.emails = value;
      self
    }
    /// Set the `emails` to the given value
    #[inline]
    fn with_emails(mut self, value: Option<Vec<String>>) -> Self {
      self.emails = value;
      self
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<__GROST_UNKNOWN_BUFFER__> ::core::default::Default for PartialUser<__GROST_UNKNOWN_BUFFER__>
  where
    String: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
    <String as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output:
      ::core::marker::Sized,
    u8: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
    <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output:
      ::core::marker::Sized,
    Option<Vec<String>>: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
    <Option<Vec<String>> as ::grost::__private::convert::State<
      ::grost::__private::convert::Flatten,
    >>::Output: ::core::marker::Sized,
  {
    fn default() -> Self {
      Self::new()
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<__GROST_UNKNOWN_BUFFER__, __GROST_FLATTEN_STATE__: ?::core::marker::Sized>
    ::grost::__private::convert::State<
      ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
    > for PartialUser<__GROST_UNKNOWN_BUFFER__>
  where
    String: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
    <String as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output:
      ::core::marker::Sized,
    u8: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
    <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output:
      ::core::marker::Sized,
    Option<Vec<String>>: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
    <Option<Vec<String>> as ::grost::__private::convert::State<
      ::grost::__private::convert::Flatten,
    >>::Output: ::core::marker::Sized,
  {
    type Output = Self;
    type Input = Self;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<__GROST_UNKNOWN_BUFFER__> PartialUser<__GROST_UNKNOWN_BUFFER__>
  where
    String: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
    <String as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output:
      ::core::marker::Sized,
    u8: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
    <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output:
      ::core::marker::Sized,
    Option<Vec<String>>: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
    <Option<Vec<String>> as ::grost::__private::convert::State<
      ::grost::__private::convert::Flatten,
    >>::Output: ::core::marker::Sized,
  {
    /// Creates an empty partial struct.
    #[inline]
    pub const fn new() -> Self {
      Self {
        __grost_unknown_buffer__: ::core::option::Option::None,
        name: ::core::option::Option::None,
        age: ::core::option::Option::None,
        emails: ::core::option::Option::None,
      }
    }
    /// Returns `true` if the partial object is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
      self.name.is_none() && self.age.is_none() && self.emails.is_none()
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
    /// Returns a reference to the `name`
    #[inline]
    const fn name_ref(
      &self,
    ) -> ::core::option::Option<
      &<String as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
    > {
      self.name.as_ref()
    }
    /// Returns a mutable reference to the `name`
    #[inline]
        const fn name_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
    >{
      self.name.as_mut()
    }
    /// Returns a reference to the `name` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `name` is `None`
    #[inline]
    const fn unwrap_name_ref(
      &self,
    ) -> &<String as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output
    {
      match self.name.as_ref() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => {
          ::core::panicking::panic_fmt(format_args!("`name` is `None`"));
        }
      }
    }
    /// Returns a mutable reference to the `name` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `name` is `None`
    #[inline]
        const fn unwrap_name_mut(
            &mut self,
        ) -> &mut <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Flatten,
    >>::Output{
      match self.name.as_mut() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => {
          ::core::panicking::panic_fmt(format_args!("`name` is `None`"));
        }
      }
    }
    /// Takes the value of `name` out if it is not `None`
    #[inline]
    const fn take_name(
      &mut self,
    ) -> ::core::option::Option<
      <String as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
    > {
      self.name.take()
    }
    /// Clear the value of `name`
    #[inline]
    fn clear_name(&mut self) -> &mut Self {
      self.name = ::core::option::Option::None;
      self
    }
    /// Set the `name` to the given value
    #[inline]
    fn set_name(
      &mut self,
      value: <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
    ) -> &mut Self {
      self.name = ::core::option::Option::Some(value);
      self
    }
    /// Update the `name` to the given value or clear the `name`
    #[inline]
    fn update_name(
      &mut self,
      value: ::core::option::Option<
                <String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            >,
    ) -> &mut Self {
      self.name = value;
      self
    }
    /// Set the `name` to the given value
    #[inline]
    fn with_name(
      mut self,
      value: <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
    ) -> Self {
      self.name = ::core::option::Option::Some(value);
      self
    }
    /// Clear the value of `name`
    #[inline]
    fn without_name(mut self) -> Self {
      self.name = ::core::option::Option::None;
      self
    }
    /// Update the `name` to the given value or clear the `name`
    #[inline]
    fn maybe_name(
      mut self,
      value: ::core::option::Option<
                <String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            >,
    ) -> Self {
      self.name = value;
      self
    }
    /// Returns a reference to the `age`
    #[inline]
    const fn age_ref(
      &self,
    ) -> ::core::option::Option<
      &<u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
    > {
      self.age.as_ref()
    }
    /// Returns a mutable reference to the `age`
    #[inline]
    const fn age_mut(
      &mut self,
    ) -> ::core::option::Option<
      &mut <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
    > {
      self.age.as_mut()
    }
    /// Returns a reference to the `age` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `age` is `None`
    #[inline]
    const fn unwrap_age_ref(
      &self,
    ) -> &<u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output
    {
      match self.age.as_ref() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => {
          ::core::panicking::panic_fmt(format_args!("`age` is `None`"));
        }
      }
    }
    /// Returns a mutable reference to the `age` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `age` is `None`
    #[inline]
    const fn unwrap_age_mut(
      &mut self,
    ) -> &mut <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output
    {
      match self.age.as_mut() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => {
          ::core::panicking::panic_fmt(format_args!("`age` is `None`"));
        }
      }
    }
    /// Takes the value of `age` out if it is not `None`
    #[inline]
    const fn take_age(
      &mut self,
    ) -> ::core::option::Option<
      <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
    > {
      self.age.take()
    }
    /// Clear the value of `age`
    #[inline]
    const fn clear_age(&mut self) -> &mut Self {
      self.age = ::core::option::Option::None;
      self
    }
    /// Set the `age` to the given value
    #[inline]
    const fn set_age(
      &mut self,
      value: <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
    ) -> &mut Self {
      self.age = ::core::option::Option::Some(value);
      self
    }
    /// Update the `age` to the given value or clear the `age`
    #[inline]
    const fn update_age(
      &mut self,
      value: ::core::option::Option<
        <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
      >,
    ) -> &mut Self {
      self.age = value;
      self
    }
    /// Set the `age` to the given value
    #[inline]
    const fn with_age(
      mut self,
      value: <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
    ) -> Self {
      self.age = ::core::option::Option::Some(value);
      self
    }
    /// Clear the value of `age`
    #[inline]
    const fn without_age(mut self) -> Self {
      self.age = ::core::option::Option::None;
      self
    }
    /// Update the `age` to the given value or clear the `age`
    #[inline]
    const fn maybe_age(
      mut self,
      value: ::core::option::Option<
        <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
      >,
    ) -> Self {
      self.age = value;
      self
    }
    /// Returns a reference to the `emails`
    #[inline]
    const fn emails_ref(
      &self,
    ) -> ::core::option::Option<
      &<Option<Vec<String>> as ::grost::__private::convert::State<
        ::grost::__private::convert::Flatten,
      >>::Output,
    > {
      self.emails.as_ref()
    }
    /// Returns a mutable reference to the `emails`
    #[inline]
    const fn emails_mut(
      &mut self,
    ) -> ::core::option::Option<
      &mut <Option<Vec<String>> as ::grost::__private::convert::State<
        ::grost::__private::convert::Flatten,
      >>::Output,
    > {
      self.emails.as_mut()
    }
    /// Returns a reference to the `emails` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `emails` is `None`
    #[inline]
    const fn unwrap_emails_ref(
      &self,
    ) -> &<Option<Vec<String>> as ::grost::__private::convert::State<
      ::grost::__private::convert::Flatten,
    >>::Output {
      match self.emails.as_ref() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => {
          ::core::panicking::panic_fmt(format_args!("`emails` is `None`"));
        }
      }
    }
    /// Returns a mutable reference to the `emails` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `emails` is `None`
    #[inline]
    const fn unwrap_emails_mut(
      &mut self,
    ) -> &mut <Option<Vec<String>> as ::grost::__private::convert::State<
      ::grost::__private::convert::Flatten,
    >>::Output {
      match self.emails.as_mut() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => {
          ::core::panicking::panic_fmt(format_args!("`emails` is `None`"));
        }
      }
    }
    /// Takes the value of `emails` out if it is not `None`
    #[inline]
    const fn take_emails(
      &mut self,
    ) -> ::core::option::Option<
      <Option<Vec<String>> as ::grost::__private::convert::State<
        ::grost::__private::convert::Flatten,
      >>::Output,
    > {
      self.emails.take()
    }
    /// Clear the value of `emails`
    #[inline]
    fn clear_emails(&mut self) -> &mut Self {
      self.emails = ::core::option::Option::None;
      self
    }
    /// Set the `emails` to the given value
    #[inline]
    fn set_emails(
      &mut self,
      value: <Option<Vec<String>> as ::grost::__private::convert::State<
        ::grost::__private::convert::Flatten,
      >>::Output,
    ) -> &mut Self {
      self.emails = ::core::option::Option::Some(value);
      self
    }
    /// Update the `emails` to the given value or clear the `emails`
    #[inline]
    fn update_emails(
      &mut self,
      value: ::core::option::Option<
        <Option<Vec<String>> as ::grost::__private::convert::State<
          ::grost::__private::convert::Flatten,
        >>::Output,
      >,
    ) -> &mut Self {
      self.emails = value;
      self
    }
    /// Set the `emails` to the given value
    #[inline]
    fn with_emails(
      mut self,
      value: <Option<Vec<String>> as ::grost::__private::convert::State<
        ::grost::__private::convert::Flatten,
      >>::Output,
    ) -> Self {
      self.emails = ::core::option::Option::Some(value);
      self
    }
    /// Clear the value of `emails`
    #[inline]
    fn without_emails(mut self) -> Self {
      self.emails = ::core::option::Option::None;
      self
    }
    /// Update the `emails` to the given value or clear the `emails`
    #[inline]
    fn maybe_emails(
      mut self,
      value: ::core::option::Option<
        <Option<Vec<String>> as ::grost::__private::convert::State<
          ::grost::__private::convert::Flatten,
        >>::Output,
      >,
    ) -> Self {
      self.emails = value;
      self
    }
  }
  #[automatically_derived]
  #[allow(clippy::type_complexity, non_camel_case_types)]
  impl<F> ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::ObjectFieldReflection<
      User,
      ::grost::__private::reflection::ObjectField,
      F,
      2u32,
    >
  where
    F: ?::core::marker::Sized,
    ::grost::__private::reflection::TypeReflection<String>:
      ::grost::__private::reflection::Reflectable<
          String,
          Reflection = ::grost::__private::reflection::Type,
        >,
    ::grost::__private::reflection::TypeReflection<u8>: ::grost::__private::reflection::Reflectable<
        u8,
        Reflection = ::grost::__private::reflection::Type,
      >,
    ::grost::__private::reflection::TypeReflection<Option<Vec<String>>>:
      ::grost::__private::reflection::Reflectable<
          Option<Vec<String>>,
          Reflection = ::grost::__private::reflection::Type,
        >,
  {
    type Reflection = ::grost::__private::reflection::ObjectField;
    const REFLECTION: &'static Self::Reflection = &{
      ::grost::__private::reflection::ObjectFieldBuilder {
                name: "name",
                description: "The nick name of the user",
                ty: <::grost::__private::reflection::TypeReflection<
                    String,
                > as ::grost::__private::reflection::Reflectable<String>>::REFLECTION,
            }
                .build()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::WireFormatReflection<
        User,
        ::grost::__private::flavors::Network,
        2u32,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format;
        const REFLECTION: &'static Self::Reflection = &{
            <<String as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::SELF
        };
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
            ::grost::__private::flavors::Network,
            2u32,
        >,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier;
        const REFLECTION: &Self::Reflection = &{
            (::grost::__private::flavors::network::Identifier::new)(
                <<String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                (::grost::__private::flavors::network::Tag::new)(2u32),
            )
        };
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                User,
                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                ::grost::__private::flavors::Network,
                2u32,
            >,
        >,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            (::grost::__private::flavors::network::Identifier::encode)(
                    <::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Network,
                            2u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION,
                )
                .as_slice()
        };
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::Len<
        ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::IdentifierReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    User,
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                    ::grost::__private::flavors::Network,
                    2u32,
                >,
            >,
        >,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = {
            &<::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        User,
                        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                        ::grost::__private::flavors::Network,
                        2u32,
                    >,
                >,
            > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION
                .len()
        };
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::TagReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User,
        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
        ::grost::__private::flavors::Network,
        2u32,
      >,
    >
  {
    type Reflection =
      <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag;
    const REFLECTION: &Self::Reflection =
      &{ (::grost::__private::flavors::network::Tag::new)(2u32) };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User,
          <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
          ::grost::__private::flavors::Network,
          2u32,
        >,
      >,
    >
  {
    type Reflection = [::core::primitive::u8];
    const REFLECTION: &Self::Reflection = {
      (::grost::__private::flavors::network::Tag::encode)(
        <::grost::__private::reflection::TagReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Network,
            2u32,
          >,
        > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION,
      )
      .as_slice()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::Len<
      ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::TagReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Network,
            2u32,
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
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Network,
            2u32,
          >,
        >,
      > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION
        .len()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::WireTypeReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::WireType,
            ::grost::__private::flavors::Network,
            2u32,
        >,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::WireType;
        const REFLECTION: &Self::Reflection = &{
            <<String as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::WIRE_TYPE
        };
    }
  #[automatically_derived]
  #[allow(clippy::type_complexity, non_camel_case_types)]
  impl<F> ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::ObjectFieldReflection<
      User,
      ::grost::__private::reflection::ObjectField,
      F,
      3u32,
    >
  where
    F: ?::core::marker::Sized,
    ::grost::__private::reflection::TypeReflection<String>:
      ::grost::__private::reflection::Reflectable<
          String,
          Reflection = ::grost::__private::reflection::Type,
        >,
    ::grost::__private::reflection::TypeReflection<u8>: ::grost::__private::reflection::Reflectable<
        u8,
        Reflection = ::grost::__private::reflection::Type,
      >,
    ::grost::__private::reflection::TypeReflection<Option<Vec<String>>>:
      ::grost::__private::reflection::Reflectable<
          Option<Vec<String>>,
          Reflection = ::grost::__private::reflection::Type,
        >,
  {
    type Reflection = ::grost::__private::reflection::ObjectField;
    const REFLECTION: &'static Self::Reflection = &{
      ::grost::__private::reflection::ObjectFieldBuilder {
                name: "age",
                description: "The age of the user",
                ty: <::grost::__private::reflection::TypeReflection<
                    u8,
                > as ::grost::__private::reflection::Reflectable<u8>>::REFLECTION,
            }
                .build()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::WireFormatReflection<
        User,
        ::grost::__private::flavors::Network,
        3u32,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format;
        const REFLECTION: &'static Self::Reflection = &{
            <<u8 as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::SELF
        };
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
            ::grost::__private::flavors::Network,
            3u32,
        >,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier;
        const REFLECTION: &Self::Reflection = &{
            (::grost::__private::flavors::network::Identifier::new)(
                <<u8 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                (::grost::__private::flavors::network::Tag::new)(3u32),
            )
        };
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                User,
                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                ::grost::__private::flavors::Network,
                3u32,
            >,
        >,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            (::grost::__private::flavors::network::Identifier::encode)(
                    <::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Network,
                            3u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION,
                )
                .as_slice()
        };
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::Len<
        ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::IdentifierReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    User,
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                    ::grost::__private::flavors::Network,
                    3u32,
                >,
            >,
        >,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = {
            &<::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        User,
                        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                        ::grost::__private::flavors::Network,
                        3u32,
                    >,
                >,
            > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION
                .len()
        };
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::TagReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User,
        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
        ::grost::__private::flavors::Network,
        3u32,
      >,
    >
  {
    type Reflection =
      <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag;
    const REFLECTION: &Self::Reflection =
      &{ (::grost::__private::flavors::network::Tag::new)(3u32) };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User,
          <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
          ::grost::__private::flavors::Network,
          3u32,
        >,
      >,
    >
  {
    type Reflection = [::core::primitive::u8];
    const REFLECTION: &Self::Reflection = {
      (::grost::__private::flavors::network::Tag::encode)(
        <::grost::__private::reflection::TagReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Network,
            3u32,
          >,
        > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION,
      )
      .as_slice()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::Len<
      ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::TagReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Network,
            3u32,
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
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Network,
            3u32,
          >,
        >,
      > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION
        .len()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::WireTypeReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::WireType,
            ::grost::__private::flavors::Network,
            3u32,
        >,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::WireType;
        const REFLECTION: &Self::Reflection = &{
            <<u8 as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::WIRE_TYPE
        };
    }
  #[automatically_derived]
  #[allow(clippy::type_complexity, non_camel_case_types)]
  impl<F> ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::ObjectFieldReflection<
      User,
      ::grost::__private::reflection::ObjectField,
      F,
      4u32,
    >
  where
    F: ?::core::marker::Sized,
    ::grost::__private::reflection::TypeReflection<String>:
      ::grost::__private::reflection::Reflectable<
          String,
          Reflection = ::grost::__private::reflection::Type,
        >,
    ::grost::__private::reflection::TypeReflection<u8>: ::grost::__private::reflection::Reflectable<
        u8,
        Reflection = ::grost::__private::reflection::Type,
      >,
    ::grost::__private::reflection::TypeReflection<Option<Vec<String>>>:
      ::grost::__private::reflection::Reflectable<
          Option<Vec<String>>,
          Reflection = ::grost::__private::reflection::Type,
        >,
  {
    type Reflection = ::grost::__private::reflection::ObjectField;
    const REFLECTION: &'static Self::Reflection = &{
      ::grost::__private::reflection::ObjectFieldBuilder {
                name: "emails",
                description: "The email of the user",
                ty: <::grost::__private::reflection::TypeReflection<
                    Option<Vec<String>>,
                > as ::grost::__private::reflection::Reflectable<
                    Option<Vec<String>>,
                >>::REFLECTION,
            }
                .build()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::WireFormatReflection<
        User,
        ::grost::__private::flavors::Network,
        4u32,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format;
        const REFLECTION: &'static Self::Reflection = &{
            <<Option<
                Vec<String>,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::SELF
        };
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
            ::grost::__private::flavors::Network,
            4u32,
        >,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier;
        const REFLECTION: &Self::Reflection = &{
            (::grost::__private::flavors::network::Identifier::new)(
                <<Option<
                    Vec<String>,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                (::grost::__private::flavors::network::Tag::new)(4u32),
            )
        };
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                User,
                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                ::grost::__private::flavors::Network,
                4u32,
            >,
        >,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            (::grost::__private::flavors::network::Identifier::encode)(
                    <::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Network,
                            4u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION,
                )
                .as_slice()
        };
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::Len<
        ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::IdentifierReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    User,
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                    ::grost::__private::flavors::Network,
                    4u32,
                >,
            >,
        >,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = {
            &<::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        User,
                        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                        ::grost::__private::flavors::Network,
                        4u32,
                    >,
                >,
            > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION
                .len()
        };
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::TagReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User,
        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
        ::grost::__private::flavors::Network,
        4u32,
      >,
    >
  {
    type Reflection =
      <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag;
    const REFLECTION: &Self::Reflection =
      &{ (::grost::__private::flavors::network::Tag::new)(4u32) };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User,
          <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
          ::grost::__private::flavors::Network,
          4u32,
        >,
      >,
    >
  {
    type Reflection = [::core::primitive::u8];
    const REFLECTION: &Self::Reflection = {
      (::grost::__private::flavors::network::Tag::encode)(
        <::grost::__private::reflection::TagReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Network,
            4u32,
          >,
        > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION,
      )
      .as_slice()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::Len<
      ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::TagReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Network,
            4u32,
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
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Network,
            4u32,
          >,
        >,
      > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION
        .len()
    };
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::WireTypeReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::WireType,
            ::grost::__private::flavors::Network,
            4u32,
        >,
    >
    where
        String: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        u8: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
        Option<
            Vec<String>,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format: 'static,
    {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::WireType;
        const REFLECTION: &Self::Reflection = &{
            <<Option<
                Vec<String>,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::WIRE_TYPE
        };
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl ::grost::__private::reflection::Reflectable<User> for User
  where
    ::grost::__private::reflection::TypeReflection<String>:
      ::grost::__private::reflection::Reflectable<
          String,
          Reflection = ::grost::__private::reflection::Type,
        >,
    ::grost::__private::reflection::TypeReflection<u8>: ::grost::__private::reflection::Reflectable<
        u8,
        Reflection = ::grost::__private::reflection::Type,
      >,
    ::grost::__private::reflection::TypeReflection<Option<Vec<String>>>:
      ::grost::__private::reflection::Reflectable<
          Option<Vec<String>>,
          Reflection = ::grost::__private::reflection::Type,
        >,
  {
    type Reflection = ::grost::__private::reflection::Type;
    const REFLECTION: &'static Self::Reflection = &{
      ::grost::__private::reflection::Type::Object(
                &::grost::__private::reflection::ObjectBuilder {
                    name: "User",
                    description: "",
                    fields: &[
                        &::grost::__private::reflection::ObjectFieldBuilder {
                            name: "name",
                            description: "The nick name of the user",
                            ty: <::grost::__private::reflection::TypeReflection<
                                String,
                            > as ::grost::__private::reflection::Reflectable<
                                String,
                            >>::REFLECTION,
                        }
                            .build(),
                        &::grost::__private::reflection::ObjectFieldBuilder {
                            name: "age",
                            description: "The age of the user",
                            ty: <::grost::__private::reflection::TypeReflection<
                                u8,
                            > as ::grost::__private::reflection::Reflectable<
                                u8,
                            >>::REFLECTION,
                        }
                            .build(),
                        &::grost::__private::reflection::ObjectFieldBuilder {
                            name: "emails",
                            description: "The email of the user",
                            ty: <::grost::__private::reflection::TypeReflection<
                                Option<Vec<String>>,
                            > as ::grost::__private::reflection::Reflectable<
                                Option<Vec<String>>,
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
  impl<F> ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::ObjectReflection<
      User,
      ::grost::__private::reflection::Object,
      F,
    >
  where
    F: ?::core::marker::Sized,
    ::grost::__private::reflection::TypeReflection<String>:
      ::grost::__private::reflection::Reflectable<
          String,
          Reflection = ::grost::__private::reflection::Type,
        >,
    ::grost::__private::reflection::TypeReflection<u8>: ::grost::__private::reflection::Reflectable<
        u8,
        Reflection = ::grost::__private::reflection::Type,
      >,
    ::grost::__private::reflection::TypeReflection<Option<Vec<String>>>:
      ::grost::__private::reflection::Reflectable<
          Option<Vec<String>>,
          Reflection = ::grost::__private::reflection::Type,
        >,
  {
    type Reflection = ::grost::__private::reflection::Object;
    const REFLECTION: &'static Self::Reflection = &{
      ::grost::__private::reflection::ObjectBuilder {
                name: "User",
                description: "",
                fields: &[
                    &::grost::__private::reflection::ObjectFieldBuilder {
                        name: "name",
                        description: "The nick name of the user",
                        ty: <::grost::__private::reflection::TypeReflection<
                            String,
                        > as ::grost::__private::reflection::Reflectable<
                            String,
                        >>::REFLECTION,
                    }
                        .build(),
                    &::grost::__private::reflection::ObjectFieldBuilder {
                        name: "age",
                        description: "The age of the user",
                        ty: <::grost::__private::reflection::TypeReflection<
                            u8,
                        > as ::grost::__private::reflection::Reflectable<u8>>::REFLECTION,
                    }
                        .build(),
                    &::grost::__private::reflection::ObjectFieldBuilder {
                        name: "emails",
                        description: "The email of the user",
                        ty: <::grost::__private::reflection::TypeReflection<
                            Option<Vec<String>>,
                        > as ::grost::__private::reflection::Reflectable<
                            Option<Vec<String>>,
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
  impl User {
    /// Returns the reflection of the struct.
    #[inline]
    pub const fn reflection<F>() -> ::grost::__private::reflection::ObjectReflection<
      Self,
      ::grost::__private::reflection::Object,
      F,
    >
    where
      F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
      ::grost::__private::reflection::ObjectReflection::new()
    }
    /// Returns the field reflection of the field `User.name`.
    #[inline]
    const fn name_reflection<F>() -> ::grost::__private::reflection::ObjectFieldReflection<
      User,
      ::grost::__private::reflection::ObjectField,
      F,
      2u32,
    >
    where
      F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
      ::grost::__private::reflection::ObjectFieldReflection::new()
    }
    /// Returns the field reflection of the field `User.age`.
    #[inline]
    const fn age_reflection<F>() -> ::grost::__private::reflection::ObjectFieldReflection<
      User,
      ::grost::__private::reflection::ObjectField,
      F,
      3u32,
    >
    where
      F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
      ::grost::__private::reflection::ObjectFieldReflection::new()
    }
    /// Returns the field reflection of the field `User.emails`.
    #[inline]
    const fn emails_reflection<F>() -> ::grost::__private::reflection::ObjectFieldReflection<
      User,
      ::grost::__private::reflection::ObjectField,
      F,
      4u32,
    >
    where
      F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
      ::grost::__private::reflection::ObjectFieldReflection::new()
    }
  }
};
extern crate test;
#[rustc_test_marker = "t"]
#[doc(hidden)]
pub const t: test::TestDescAndFn = test::TestDescAndFn {
  desc: test::TestDesc {
    name: test::StaticTestName("t"),
    ignore: false,
    ignore_message: ::core::option::Option::None,
    source_file: "grost-derive/tests/generic_object.rs",
    start_line: 163usize,
    start_col: 4usize,
    end_line: 163usize,
    end_col: 5usize,
    compile_fail: false,
    no_run: false,
    should_panic: test::ShouldPanic::No,
    test_type: test::TestType::IntegrationTest,
  },
  testfn: test::StaticTestFn(
    #[coverage(off)]
    || test::assert_test_result(t()),
  ),
};
fn t() {
  let user = PartialDecodedUser::<'_, Network, ()> {
    __grost_unknown_buffer__: ::core::panicking::panic("not yet implemented"),
    name: ::core::panicking::panic("not yet implemented"),
    age: ::core::panicking::panic("not yet implemented"),
    emails: ::core::panicking::panic("not yet implemented"),
  };
  let val = User::emails_reflection();
  let wf = val.wire_format();
  let identifier = val.identifier();
  let encoded_identifier = val.encoded_identifier();
  let object_refl = User::reflection::<Network>();
  {
    ::std::io::_print(format_args!("{0:?}\n", encoded_identifier));
  };
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
  extern crate test;
  test::test_main_static(&[&t])
}
