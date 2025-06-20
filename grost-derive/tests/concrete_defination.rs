#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use std::marker::PhantomData;
use grost::{
    Decode, Flavor,
    flavors::{DefaultWireFormat, Network, WireFormat, network::LengthDelimited},
    selection::{Selectable, Selector},
};
use grost_derive::{Object, object};
fn default_array<const N: usize>() -> [u8; N] {
    [0; N]
}
fn error_name<'a>() -> Result<&'a str, <Network as Flavor>::Error> {
    Ok("name")
}
#[grost(
    flavor(
        default(
            encode(skip_default, enum = "grost::flavors::network::encoding::enumeration")
        ),
    ),
)]
pub struct User<I: Default> {
    #[grost(
        tag = 1,
        schema(description = "The id of the user"),
        selector(select = "all"),
        flavor(default = "grost::flavors::network::LengthDelimited"),
        bytes
    )]
    id: I,
    #[grost(
        tag = 2,
        schema(description = "The nick name of the user"),
        selector(select = "all"),
        string,
        flavor(default(decode(ok_or_else = "error_name")))
    )]
    name: String,
    #[grost(tag = 3, scalar, schema(description = "The age of the user"), copy)]
    age: u8,
    #[grost(tag = 4, schema(description = "The email of the user"), list(string))]
    emails: Vec<String>,
    #[grost(
        tag = 5,
        schema(description = "The linkin link of the user"),
        partial_decoded(copy),
        optional(string)
    )]
    linkin: Option<String>,
}
#[automatically_derived]
impl<I: ::core::fmt::Debug + Default> ::core::fmt::Debug for User<I> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(
            f,
            "User",
            "id",
            &self.id,
            "name",
            &self.name,
            "age",
            &self.age,
            "emails",
            &self.emails,
            "linkin",
            &&self.linkin,
        )
    }
}
#[automatically_derived]
impl<I: ::core::clone::Clone + Default> ::core::clone::Clone for User<I> {
    #[inline]
    fn clone(&self) -> User<I> {
        User {
            id: ::core::clone::Clone::clone(&self.id),
            name: ::core::clone::Clone::clone(&self.name),
            age: ::core::clone::Clone::clone(&self.age),
            emails: ::core::clone::Clone::clone(&self.emails),
            linkin: ::core::clone::Clone::clone(&self.linkin),
        }
    }
}
#[automatically_derived]
impl<I: Default> ::core::marker::StructuralPartialEq for User<I> {}
#[automatically_derived]
impl<I: ::core::cmp::PartialEq + Default> ::core::cmp::PartialEq for User<I> {
    #[inline]
    fn eq(&self, other: &User<I>) -> bool {
        self.id == other.id && self.name == other.name && self.age == other.age
            && self.emails == other.emails && self.linkin == other.linkin
    }
}
#[automatically_derived]
impl<I: ::core::cmp::Eq + Default> ::core::cmp::Eq for User<I> {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<I>;
        let _: ::core::cmp::AssertParamIsEq<String>;
        let _: ::core::cmp::AssertParamIsEq<u8>;
        let _: ::core::cmp::AssertParamIsEq<Vec<String>>;
        let _: ::core::cmp::AssertParamIsEq<Option<String>>;
    }
}
/// Field indexer for the struct [`User`]
#[repr(u32)]
pub enum UserIndex {
    /// The field indexer for the field `id`
    Id = 1u32,
    /// The field indexer for the field `name`
    Name = 2u32,
    /// The field indexer for the field `age`
    Age = 3u32,
    /// The field indexer for the field `emails`
    Emails = 4u32,
    /// The field indexer for the field `linkin`
    Linkin = 5u32,
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
    fn partial_cmp(
        &self,
        other: &UserIndex,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
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
                UserIndex::Id => "Id",
                UserIndex::Name => "Name",
                UserIndex::Age => "Age",
                UserIndex::Emails => "Emails",
                UserIndex::Linkin => "Linkin",
            },
        )
    }
}
/// The selection type for [`User`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct UserSelector<I: Default>
where
    I: ::grost::__private::selection::Selectable<::grost::__private::flavors::Network>,
{
    id: <I as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Network,
    >>::Selector,
    name: <String as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Network,
    >>::Selector,
    age: <u8 as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Network,
    >>::Selector,
    emails: <Vec<
        String,
    > as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Network,
    >>::Selector,
    linkin: <Option<
        String,
    > as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Network,
    >>::Selector,
}
/// An iterator over the selected fields of the [`UserSelector`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct UserSelectorIter<
    '__grost_lifetime__,
    I: Default,
    const __GROST_SELECTED__: ::core::primitive::bool = true,
>
where
    I: ::grost::__private::selection::Selectable<::grost::__private::flavors::Network>,
{
    selector: &'__grost_lifetime__ UserSelector<I>,
    index: ::core::option::Option<UserIndex>,
    num: ::core::primitive::usize,
    yielded: ::core::primitive::usize,
}
/// Partial struct for the [`PartialUser`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct PartialUser<I: Default> {
    id: ::core::option::Option<I>,
    name: ::core::option::Option<String>,
    age: ::core::option::Option<u8>,
    emails: ::core::option::Option<Vec<String>>,
    linkin: ::core::option::Option<Option<String>>,
}
/// Partial reference struct for the struct [`User`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct PartialDecodedUser<
    '__grost_lifetime__,
    I: Default,
    __GROST_READ_BUFFER__,
    __GROST_UNKNOWN_BUFFER__,
>
where
    I: ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            ::grost::__private::flavors::Network,
            grost::flavors::network::LengthDelimited,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >,
    <I as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            ::grost::__private::flavors::Network,
            grost::flavors::network::LengthDelimited,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >>::Output: ::core::marker::Sized,
{
    __grost_unknown_buffer__: ::core::option::Option<__GROST_UNKNOWN_BUFFER__>,
    __grost_read_buffer__: ::core::option::Option<__GROST_READ_BUFFER__>,
    id: ::core::option::Option<
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output,
    >,
    name: ::core::option::Option<
        <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output,
    >,
    age: ::core::option::Option<
        <u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <u8 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output,
    >,
    emails: ::core::option::Option<
        <Vec<
            String,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <Vec<
                    String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output,
    >,
    linkin: ::core::option::Option<
        <Option<
            String,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <Option<
                    String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output,
    >,
}
const _: () = {
    impl<I: Default> ::core::default::Default for User<I> {
        fn default() -> Self {
            Self::new()
        }
    }
    impl<I: Default> User<I> {
        /// Creates a new instance of the object with default values.
        pub fn new() -> Self {
            Self {
                id: (::core::default::Default::default)(),
                name: (::core::default::Default::default)(),
                age: (::core::default::Default::default)(),
                emails: (::core::default::Default::default)(),
                linkin: (::core::default::Default::default)(),
            }
        }
    }
    impl<I: Default> User<I> {
        /// Returns a reference to the `id`
        #[inline]
        const fn id_ref(&self) -> &I {
            &self.id
        }
        /// Returns a mutable reference to the `id`
        #[inline]
        const fn id_mut(&mut self) -> &mut I {
            &mut self.id
        }
        /// Set the `id` to the given value
        #[inline]
        fn set_id(&mut self, value: I) -> &mut Self {
            self.id = value;
            self
        }
        /// Set the `id` to the given value
        #[inline]
        fn with_id(mut self, value: I) -> Self {
            self.id = value;
            self
        }
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
        const fn emails_ref(&self) -> &Vec<String> {
            &self.emails
        }
        /// Returns a mutable reference to the `emails`
        #[inline]
        const fn emails_mut(&mut self) -> &mut Vec<String> {
            &mut self.emails
        }
        /// Set the `emails` to the given value
        #[inline]
        fn set_emails(&mut self, value: Vec<String>) -> &mut Self {
            self.emails = value;
            self
        }
        /// Set the `emails` to the given value
        #[inline]
        fn with_emails(mut self, value: Vec<String>) -> Self {
            self.emails = value;
            self
        }
        /// Returns a reference to the `linkin`
        #[inline]
        const fn linkin_ref(&self) -> &Option<String> {
            &self.linkin
        }
        /// Returns a mutable reference to the `linkin`
        #[inline]
        const fn linkin_mut(&mut self) -> &mut Option<String> {
            &mut self.linkin
        }
        /// Set the `linkin` to the given value
        #[inline]
        fn set_linkin(&mut self, value: Option<String>) -> &mut Self {
            self.linkin = value;
            self
        }
        /// Set the `linkin` to the given value
        #[inline]
        fn with_linkin(mut self, value: Option<String>) -> Self {
            self.linkin = value;
            self
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I: Default> ::core::default::Default for PartialUser<I> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        I: Default,
        __GROST_FLATTEN_STATE__: ?::core::marker::Sized,
    > ::grost::__private::convert::State<
        ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
    > for PartialUser<I> {
        type Output = Self;
        type Input = Self;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> PartialUser<I> {
        /// Creates an empty partial struct.
        #[inline]
        pub const fn new() -> Self {
            Self {
                id: ::core::option::Option::None,
                name: ::core::option::Option::None,
                age: ::core::option::Option::None,
                emails: ::core::option::Option::None,
                linkin: ::core::option::Option::None,
            }
        }
        /// Returns `true` if the partial object is empty.
        #[inline]
        pub const fn is_empty(&self) -> bool {
            self.id.is_none() && self.name.is_none() && self.age.is_none()
                && self.emails.is_none() && self.linkin.is_none()
        }
        /// Returns a reference to the `id`
        #[inline]
        const fn id_ref(&self) -> ::core::option::Option<&I> {
            self.id.as_ref()
        }
        /// Returns a mutable reference to the `id`
        #[inline]
        const fn id_mut(&mut self) -> ::core::option::Option<&mut I> {
            self.id.as_mut()
        }
        /// Returns a reference to the `id` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `id` is `None`
        #[inline]
        const fn unwrap_id_ref(&self) -> &I {
            match self.id.as_ref() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => {
                    ::core::panicking::panic_fmt(format_args!("`id` is `None`"));
                }
            }
        }
        /// Returns a mutable reference to the `id` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `id` is `None`
        #[inline]
        const fn unwrap_id_mut(&mut self) -> &mut I {
            match self.id.as_mut() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => {
                    ::core::panicking::panic_fmt(format_args!("`id` is `None`"));
                }
            }
        }
        /// Takes the value of `id` out if it is not `None`
        #[inline]
        const fn take_id(&mut self) -> ::core::option::Option<I> {
            self.id.take()
        }
        /// Clear the value of `id`
        #[inline]
        fn clear_id(&mut self) -> &mut Self {
            self.id = ::core::option::Option::None;
            self
        }
        /// Set the `id` to the given value
        #[inline]
        fn set_id(&mut self, value: I) -> &mut Self {
            self.id = ::core::option::Option::Some(value);
            self
        }
        /// Update the `id` to the given value or clear the `id`
        #[inline]
        fn update_id(&mut self, value: ::core::option::Option<I>) -> &mut Self {
            self.id = value;
            self
        }
        /// Set the `id` to the given value
        #[inline]
        fn with_id(mut self, value: I) -> Self {
            self.id = ::core::option::Option::Some(value);
            self
        }
        /// Clear the value of `id`
        #[inline]
        fn without_id(mut self) -> Self {
            self.id = ::core::option::Option::None;
            self
        }
        /// Update the `id` to the given value or clear the `id`
        #[inline]
        fn maybe_id(mut self, value: ::core::option::Option<I>) -> Self {
            self.id = value;
            self
        }
        /// Returns a reference to the `name`
        #[inline]
        const fn name_ref(&self) -> ::core::option::Option<&String> {
            self.name.as_ref()
        }
        /// Returns a mutable reference to the `name`
        #[inline]
        const fn name_mut(&mut self) -> ::core::option::Option<&mut String> {
            self.name.as_mut()
        }
        /// Returns a reference to the `name` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `name` is `None`
        #[inline]
        const fn unwrap_name_ref(&self) -> &String {
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
        const fn unwrap_name_mut(&mut self) -> &mut String {
            match self.name.as_mut() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => {
                    ::core::panicking::panic_fmt(format_args!("`name` is `None`"));
                }
            }
        }
        /// Takes the value of `name` out if it is not `None`
        #[inline]
        const fn take_name(&mut self) -> ::core::option::Option<String> {
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
        fn set_name(&mut self, value: String) -> &mut Self {
            self.name = ::core::option::Option::Some(value);
            self
        }
        /// Update the `name` to the given value or clear the `name`
        #[inline]
        fn update_name(&mut self, value: ::core::option::Option<String>) -> &mut Self {
            self.name = value;
            self
        }
        /// Set the `name` to the given value
        #[inline]
        fn with_name(mut self, value: String) -> Self {
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
        fn maybe_name(mut self, value: ::core::option::Option<String>) -> Self {
            self.name = value;
            self
        }
        /// Returns a reference to the `age`
        #[inline]
        const fn age_ref(&self) -> ::core::option::Option<&u8> {
            self.age.as_ref()
        }
        /// Returns a mutable reference to the `age`
        #[inline]
        const fn age_mut(&mut self) -> ::core::option::Option<&mut u8> {
            self.age.as_mut()
        }
        /// Returns a reference to the `age` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `age` is `None`
        #[inline]
        const fn unwrap_age_ref(&self) -> &u8 {
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
        const fn unwrap_age_mut(&mut self) -> &mut u8 {
            match self.age.as_mut() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => {
                    ::core::panicking::panic_fmt(format_args!("`age` is `None`"));
                }
            }
        }
        /// Takes the value of `age` out if it is not `None`
        #[inline]
        const fn take_age(&mut self) -> ::core::option::Option<u8> {
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
        const fn set_age(&mut self, value: u8) -> &mut Self {
            self.age = ::core::option::Option::Some(value);
            self
        }
        /// Update the `age` to the given value or clear the `age`
        #[inline]
        const fn update_age(&mut self, value: ::core::option::Option<u8>) -> &mut Self {
            self.age = value;
            self
        }
        /// Set the `age` to the given value
        #[inline]
        const fn with_age(mut self, value: u8) -> Self {
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
        const fn maybe_age(mut self, value: ::core::option::Option<u8>) -> Self {
            self.age = value;
            self
        }
        /// Returns a reference to the `emails`
        #[inline]
        const fn emails_ref(&self) -> ::core::option::Option<&Vec<String>> {
            self.emails.as_ref()
        }
        /// Returns a mutable reference to the `emails`
        #[inline]
        const fn emails_mut(&mut self) -> ::core::option::Option<&mut Vec<String>> {
            self.emails.as_mut()
        }
        /// Returns a reference to the `emails` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `emails` is `None`
        #[inline]
        const fn unwrap_emails_ref(&self) -> &Vec<String> {
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
        const fn unwrap_emails_mut(&mut self) -> &mut Vec<String> {
            match self.emails.as_mut() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => {
                    ::core::panicking::panic_fmt(format_args!("`emails` is `None`"));
                }
            }
        }
        /// Takes the value of `emails` out if it is not `None`
        #[inline]
        const fn take_emails(&mut self) -> ::core::option::Option<Vec<String>> {
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
        fn set_emails(&mut self, value: Vec<String>) -> &mut Self {
            self.emails = ::core::option::Option::Some(value);
            self
        }
        /// Update the `emails` to the given value or clear the `emails`
        #[inline]
        fn update_emails(
            &mut self,
            value: ::core::option::Option<Vec<String>>,
        ) -> &mut Self {
            self.emails = value;
            self
        }
        /// Set the `emails` to the given value
        #[inline]
        fn with_emails(mut self, value: Vec<String>) -> Self {
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
        fn maybe_emails(mut self, value: ::core::option::Option<Vec<String>>) -> Self {
            self.emails = value;
            self
        }
        /// Returns a reference to the `linkin`
        #[inline]
        const fn linkin_ref(&self) -> ::core::option::Option<&Option<String>> {
            self.linkin.as_ref()
        }
        /// Returns a mutable reference to the `linkin`
        #[inline]
        const fn linkin_mut(&mut self) -> ::core::option::Option<&mut Option<String>> {
            self.linkin.as_mut()
        }
        /// Returns a reference to the `linkin` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `linkin` is `None`
        #[inline]
        const fn unwrap_linkin_ref(&self) -> &Option<String> {
            match self.linkin.as_ref() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => {
                    ::core::panicking::panic_fmt(format_args!("`linkin` is `None`"));
                }
            }
        }
        /// Returns a mutable reference to the `linkin` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `linkin` is `None`
        #[inline]
        const fn unwrap_linkin_mut(&mut self) -> &mut Option<String> {
            match self.linkin.as_mut() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => {
                    ::core::panicking::panic_fmt(format_args!("`linkin` is `None`"));
                }
            }
        }
        /// Takes the value of `linkin` out if it is not `None`
        #[inline]
        const fn take_linkin(&mut self) -> ::core::option::Option<Option<String>> {
            self.linkin.take()
        }
        /// Clear the value of `linkin`
        #[inline]
        fn clear_linkin(&mut self) -> &mut Self {
            self.linkin = ::core::option::Option::None;
            self
        }
        /// Set the `linkin` to the given value
        #[inline]
        fn set_linkin(&mut self, value: Option<String>) -> &mut Self {
            self.linkin = ::core::option::Option::Some(value);
            self
        }
        /// Update the `linkin` to the given value or clear the `linkin`
        #[inline]
        fn update_linkin(
            &mut self,
            value: ::core::option::Option<Option<String>>,
        ) -> &mut Self {
            self.linkin = value;
            self
        }
        /// Set the `linkin` to the given value
        #[inline]
        fn with_linkin(mut self, value: Option<String>) -> Self {
            self.linkin = ::core::option::Option::Some(value);
            self
        }
        /// Clear the value of `linkin`
        #[inline]
        fn without_linkin(mut self) -> Self {
            self.linkin = ::core::option::Option::None;
            self
        }
        /// Update the `linkin` to the given value or clear the `linkin`
        #[inline]
        fn maybe_linkin(
            mut self,
            value: ::core::option::Option<Option<String>>,
        ) -> Self {
            self.linkin = value;
            self
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I: Default,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > ::core::default::Default
    for PartialDecodedUser<
        '__grost_lifetime__,
        I,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized,
    {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I: Default,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
        __GROST_FLATTEN_STATE__: ?::core::marker::Sized,
    > ::grost::__private::convert::State<
        ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
    >
    for PartialDecodedUser<
        '__grost_lifetime__,
        I,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized,
    {
        type Output = Self;
        type Input = Self;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I: Default,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > PartialDecodedUser<
        '__grost_lifetime__,
        I,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized,
    {
        /// Creates an empty partial struct.
        #[inline]
        pub const fn new() -> Self {
            Self {
                id: ::core::option::Option::None,
                name: ::core::option::Option::None,
                age: ::core::option::Option::None,
                emails: ::core::option::Option::None,
                linkin: ::core::option::Option::None,
                __grost_unknown_buffer__: ::core::option::Option::None,
                __grost_read_buffer__: ::core::option::Option::None,
            }
        }
        /// Returns `true` if the partial struct is empty, which means all fields are `None`.
        #[inline]
        pub const fn is_empty(&self) -> bool {
            self.__grost_unknown_buffer__.is_none() && self.id.is_none()
                && self.name.is_none() && self.age.is_none() && self.emails.is_none()
                && self.linkin.is_none()
        }
        /// Returns the original encoded type of the partial decoded object.
        #[inline]
        pub const fn raw(&self) -> ::core::option::Option<&__GROST_READ_BUFFER__> {
            self.__grost_read_buffer__.as_ref()
        }
        /// Returns a reference to the unknown buffer, which holds the unknown data when decoding.
        #[inline]
        pub const fn unknown_buffer(
            &self,
        ) -> ::core::option::Option<&__GROST_UNKNOWN_BUFFER__> {
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
        pub fn set_unknown_buffer(
            &mut self,
            buffer: __GROST_UNKNOWN_BUFFER__,
        ) -> &mut Self {
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
        /// Returns a reference to the `id`
        #[inline]
        const fn id_ref(
            &self,
        ) -> ::core::option::Option<
            &<I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    grost::flavors::network::LengthDelimited,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.id.as_ref()
        }
        /// Returns a mutable reference to the `id`
        #[inline]
        const fn id_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    grost::flavors::network::LengthDelimited,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.id.as_mut()
        }
        /// Returns a reference to the `id` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `id` is `None`
        #[inline]
        const fn unwrap_id_ref(
            &self,
        ) -> &<I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output {
            match self.id.as_ref() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => {
                    ::core::panicking::panic_fmt(format_args!("`id` is `None`"));
                }
            }
        }
        /// Returns a mutable reference to the `id` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `id` is `None`
        #[inline]
        const fn unwrap_id_mut(
            &mut self,
        ) -> &mut <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output {
            match self.id.as_mut() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => {
                    ::core::panicking::panic_fmt(format_args!("`id` is `None`"));
                }
            }
        }
        /// Takes the value of `id` out if it is not `None`
        #[inline]
        const fn take_id(
            &mut self,
        ) -> ::core::option::Option<
            <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    grost::flavors::network::LengthDelimited,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.id.take()
        }
        /// Clear the value of `id`
        #[inline]
        fn clear_id(&mut self) -> &mut Self {
            self.id = ::core::option::Option::None;
            self
        }
        /// Set the `id` to the given value
        #[inline]
        fn set_id(
            &mut self,
            value: <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    grost::flavors::network::LengthDelimited,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        ) -> &mut Self {
            self.id = ::core::option::Option::Some(value);
            self
        }
        /// Update the `id` to the given value or clear the `id`
        #[inline]
        fn update_id(
            &mut self,
            value: ::core::option::Option<
                <I as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        ::grost::__private::flavors::Network,
                        grost::flavors::network::LengthDelimited,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                >>::Output,
            >,
        ) -> &mut Self {
            self.id = value;
            self
        }
        /// Set the `id` to the given value
        #[inline]
        fn with_id(
            mut self,
            value: <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    grost::flavors::network::LengthDelimited,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        ) -> Self {
            self.id = ::core::option::Option::Some(value);
            self
        }
        /// Clear the value of `id`
        #[inline]
        fn without_id(mut self) -> Self {
            self.id = ::core::option::Option::None;
            self
        }
        /// Update the `id` to the given value or clear the `id`
        #[inline]
        fn maybe_id(
            mut self,
            value: ::core::option::Option<
                <I as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        ::grost::__private::flavors::Network,
                        grost::flavors::network::LengthDelimited,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                >>::Output,
            >,
        ) -> Self {
            self.id = value;
            self
        }
        /// Returns a reference to the `name`
        #[inline]
        const fn name_ref(
            &self,
        ) -> ::core::option::Option<
            &<String as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.name.as_ref()
        }
        /// Returns a mutable reference to the `name`
        #[inline]
        const fn name_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
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
        ) -> &<String as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output {
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
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output {
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
            <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
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
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
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
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        ::grost::__private::flavors::Network,
                        <String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
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
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
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
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        ::grost::__private::flavors::Network,
                        <String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
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
            &<u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <u8 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.age.as_ref()
        }
        /// Returns a mutable reference to the `age`
        #[inline]
        const fn age_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <u8 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
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
        ) -> &<u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <u8 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output {
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
        ) -> &mut <u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <u8 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output {
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
            <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <u8 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.age.take()
        }
        /// Clear the value of `age`
        #[inline]
        fn clear_age(&mut self) -> &mut Self {
            self.age = ::core::option::Option::None;
            self
        }
        /// Set the `age` to the given value
        #[inline]
        fn set_age(
            &mut self,
            value: <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <u8 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        ) -> &mut Self {
            self.age = ::core::option::Option::Some(value);
            self
        }
        /// Update the `age` to the given value or clear the `age`
        #[inline]
        fn update_age(
            &mut self,
            value: ::core::option::Option<
                <u8 as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        ::grost::__private::flavors::Network,
                        <u8 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                >>::Output,
            >,
        ) -> &mut Self {
            self.age = value;
            self
        }
        /// Set the `age` to the given value
        #[inline]
        fn with_age(
            mut self,
            value: <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <u8 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        ) -> Self {
            self.age = ::core::option::Option::Some(value);
            self
        }
        /// Clear the value of `age`
        #[inline]
        fn without_age(mut self) -> Self {
            self.age = ::core::option::Option::None;
            self
        }
        /// Update the `age` to the given value or clear the `age`
        #[inline]
        fn maybe_age(
            mut self,
            value: ::core::option::Option<
                <u8 as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        ::grost::__private::flavors::Network,
                        <u8 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                >>::Output,
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
            &<Vec<
                String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <Vec<
                        String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.emails.as_ref()
        }
        /// Returns a mutable reference to the `emails`
        #[inline]
        const fn emails_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <Vec<
                String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <Vec<
                        String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
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
        ) -> &<Vec<
            String,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <Vec<
                    String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
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
        ) -> &mut <Vec<
            String,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <Vec<
                    String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
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
            <Vec<
                String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <Vec<
                        String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
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
            value: <Vec<
                String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <Vec<
                        String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
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
                <Vec<
                    String,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        ::grost::__private::flavors::Network,
                        <Vec<
                            String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
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
            value: <Vec<
                String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <Vec<
                        String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
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
                <Vec<
                    String,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        ::grost::__private::flavors::Network,
                        <Vec<
                            String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                >>::Output,
            >,
        ) -> Self {
            self.emails = value;
            self
        }
        /// Returns a reference to the `linkin`
        #[inline]
        const fn linkin_ref(
            &self,
        ) -> ::core::option::Option<
            &<Option<
                String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <Option<
                        String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.linkin.as_ref()
        }
        /// Returns a mutable reference to the `linkin`
        #[inline]
        const fn linkin_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <Option<
                String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <Option<
                        String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.linkin.as_mut()
        }
        /// Returns a reference to the `linkin` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `linkin` is `None`
        #[inline]
        const fn unwrap_linkin_ref(
            &self,
        ) -> &<Option<
            String,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <Option<
                    String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output {
            match self.linkin.as_ref() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => {
                    ::core::panicking::panic_fmt(format_args!("`linkin` is `None`"));
                }
            }
        }
        /// Returns a mutable reference to the `linkin` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `linkin` is `None`
        #[inline]
        const fn unwrap_linkin_mut(
            &mut self,
        ) -> &mut <Option<
            String,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <Option<
                    String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output {
            match self.linkin.as_mut() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => {
                    ::core::panicking::panic_fmt(format_args!("`linkin` is `None`"));
                }
            }
        }
        /// Takes the value of `linkin` out if it is not `None`
        #[inline]
        const fn take_linkin(
            &mut self,
        ) -> ::core::option::Option<
            <Option<
                String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <Option<
                        String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.linkin.take()
        }
        /// Clear the value of `linkin`
        #[inline]
        const fn clear_linkin(&mut self) -> &mut Self {
            self.linkin = ::core::option::Option::None;
            self
        }
        /// Set the `linkin` to the given value
        #[inline]
        const fn set_linkin(
            &mut self,
            value: <Option<
                String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <Option<
                        String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        ) -> &mut Self {
            self.linkin = ::core::option::Option::Some(value);
            self
        }
        /// Update the `linkin` to the given value or clear the `linkin`
        #[inline]
        const fn update_linkin(
            &mut self,
            value: ::core::option::Option<
                <Option<
                    String,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        ::grost::__private::flavors::Network,
                        <Option<
                            String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                >>::Output,
            >,
        ) -> &mut Self {
            self.linkin = value;
            self
        }
        /// Set the `linkin` to the given value
        #[inline]
        const fn with_linkin(
            mut self,
            value: <Option<
                String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    <Option<
                        String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        ) -> Self {
            self.linkin = ::core::option::Option::Some(value);
            self
        }
        /// Clear the value of `linkin`
        #[inline]
        const fn without_linkin(mut self) -> Self {
            self.linkin = ::core::option::Option::None;
            self
        }
        /// Update the `linkin` to the given value or clear the `linkin`
        #[inline]
        const fn maybe_linkin(
            mut self,
            value: ::core::option::Option<
                <Option<
                    String,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        ::grost::__private::flavors::Network,
                        <Option<
                            String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                >>::Output,
            >,
        ) -> Self {
            self.linkin = value;
            self
        }
    }
    #[automatically_derived]
    #[allow(clippy::type_complexity, non_camel_case_types)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::reflection::ObjectField,
        ::grost::__private::flavors::Network,
        1u32,
    >
    where
        ::grost::__private::reflection::TypeReflection<
            I,
        >: ::grost::__private::reflection::Reflectable<
            I,
            Reflection = ::grost::__private::reflection::Type,
        >,
    {
        type Reflection = ::grost::__private::reflection::ObjectField;
        const REFLECTION: &'static Self::Reflection = &{
            ::grost::__private::reflection::ObjectFieldBuilder {
                name: "id",
                description: "The id of the user",
                ty: <::grost::__private::reflection::TypeReflection<
                    I,
                > as ::grost::__private::reflection::Reflectable<I>>::REFLECTION,
            }
                .build()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = grost::flavors::network::LengthDelimited;
        const REFLECTION: &'static Self::Reflection = &{
            <grost::flavors::network::LengthDelimited as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::SELF
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
            ::grost::__private::flavors::Network,
            1u32,
        >,
    > {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier;
        const REFLECTION: &Self::Reflection = &{
            (::grost::__private::flavors::network::Identifier::new)(
                <grost::flavors::network::LengthDelimited as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                (::grost::__private::flavors::network::Tag::new)(1u32),
            )
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                User<I>,
                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                ::grost::__private::flavors::Network,
                1u32,
            >,
        >,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            (::grost::__private::flavors::network::Identifier::encode)(
                    <::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Network,
                            1u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION,
                )
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::Len<
        ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::IdentifierReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    User<I>,
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                    ::grost::__private::flavors::Network,
                    1u32,
                >,
            >,
        >,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = {
            &<::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                        ::grost::__private::flavors::Network,
                        1u32,
                    >,
                >,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Network,
            1u32,
        >,
    > {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag;
        const REFLECTION: &Self::Reflection = &{
            (::grost::__private::flavors::network::Tag::new)(1u32)
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::TagReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                User<I>,
                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                ::grost::__private::flavors::Network,
                1u32,
            >,
        >,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            (::grost::__private::flavors::network::Tag::encode)(
                    <::grost::__private::reflection::TagReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                            ::grost::__private::flavors::Network,
                            1u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION,
                )
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::Len<
        ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::TagReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    User<I>,
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                    ::grost::__private::flavors::Network,
                    1u32,
                >,
            >,
        >,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = {
            &<::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::TagReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                        ::grost::__private::flavors::Network,
                        1u32,
                    >,
                >,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::WireTypeReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::WireType,
            ::grost::__private::flavors::Network,
            1u32,
        >,
    > {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::WireType;
        const REFLECTION: &Self::Reflection = &{
            <grost::flavors::network::LengthDelimited as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::WIRE_TYPE
        };
    }
    #[automatically_derived]
    #[allow(clippy::type_complexity, non_camel_case_types)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::reflection::ObjectField,
        ::grost::__private::flavors::Network,
        2u32,
    > {
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
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        2u32,
    > {
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
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
            ::grost::__private::flavors::Network,
            2u32,
        >,
    > {
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
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                User<I>,
                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                ::grost::__private::flavors::Network,
                2u32,
            >,
        >,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            (::grost::__private::flavors::network::Identifier::encode)(
                    <::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Network,
                            2u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION,
                )
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::Len<
        ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::IdentifierReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    User<I>,
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                    ::grost::__private::flavors::Network,
                    2u32,
                >,
            >,
        >,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = {
            &<::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                        ::grost::__private::flavors::Network,
                        2u32,
                    >,
                >,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Network,
            2u32,
        >,
    > {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag;
        const REFLECTION: &Self::Reflection = &{
            (::grost::__private::flavors::network::Tag::new)(2u32)
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::TagReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                User<I>,
                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                ::grost::__private::flavors::Network,
                2u32,
            >,
        >,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            (::grost::__private::flavors::network::Tag::encode)(
                    <::grost::__private::reflection::TagReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                            ::grost::__private::flavors::Network,
                            2u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION,
                )
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::Len<
        ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::TagReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    User<I>,
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                    ::grost::__private::flavors::Network,
                    2u32,
                >,
            >,
        >,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = {
            &<::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::TagReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                        ::grost::__private::flavors::Network,
                        2u32,
                    >,
                >,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::WireTypeReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::WireType,
            ::grost::__private::flavors::Network,
            2u32,
        >,
    > {
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
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::reflection::ObjectField,
        ::grost::__private::flavors::Network,
        3u32,
    > {
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
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        3u32,
    > {
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
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
            ::grost::__private::flavors::Network,
            3u32,
        >,
    > {
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
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                User<I>,
                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                ::grost::__private::flavors::Network,
                3u32,
            >,
        >,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            (::grost::__private::flavors::network::Identifier::encode)(
                    <::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Network,
                            3u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION,
                )
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::Len<
        ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::IdentifierReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    User<I>,
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                    ::grost::__private::flavors::Network,
                    3u32,
                >,
            >,
        >,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = {
            &<::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                        ::grost::__private::flavors::Network,
                        3u32,
                    >,
                >,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Network,
            3u32,
        >,
    > {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag;
        const REFLECTION: &Self::Reflection = &{
            (::grost::__private::flavors::network::Tag::new)(3u32)
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::TagReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                User<I>,
                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                ::grost::__private::flavors::Network,
                3u32,
            >,
        >,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            (::grost::__private::flavors::network::Tag::encode)(
                    <::grost::__private::reflection::TagReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                            ::grost::__private::flavors::Network,
                            3u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION,
                )
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::Len<
        ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::TagReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    User<I>,
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                    ::grost::__private::flavors::Network,
                    3u32,
                >,
            >,
        >,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = {
            &<::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::TagReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                        ::grost::__private::flavors::Network,
                        3u32,
                    >,
                >,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::WireTypeReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::WireType,
            ::grost::__private::flavors::Network,
            3u32,
        >,
    > {
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
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::reflection::ObjectField,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = ::grost::__private::reflection::ObjectField;
        const REFLECTION: &'static Self::Reflection = &{
            ::grost::__private::reflection::ObjectFieldBuilder {
                name: "emails",
                description: "The email of the user",
                ty: <::grost::__private::reflection::TypeReflection<
                    Vec<String>,
                > as ::grost::__private::reflection::Reflectable<
                    Vec<String>,
                >>::REFLECTION,
            }
                .build()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = <Vec<
            String,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format;
        const REFLECTION: &'static Self::Reflection = &{
            <<Vec<
                String,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::SELF
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
            ::grost::__private::flavors::Network,
            4u32,
        >,
    > {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier;
        const REFLECTION: &Self::Reflection = &{
            (::grost::__private::flavors::network::Identifier::new)(
                <<Vec<
                    String,
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
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                User<I>,
                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                ::grost::__private::flavors::Network,
                4u32,
            >,
        >,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            (::grost::__private::flavors::network::Identifier::encode)(
                    <::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Network,
                            4u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION,
                )
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::Len<
        ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::IdentifierReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    User<I>,
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                    ::grost::__private::flavors::Network,
                    4u32,
                >,
            >,
        >,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = {
            &<::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                        ::grost::__private::flavors::Network,
                        4u32,
                    >,
                >,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Network,
            4u32,
        >,
    > {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag;
        const REFLECTION: &Self::Reflection = &{
            (::grost::__private::flavors::network::Tag::new)(4u32)
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::TagReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                User<I>,
                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                ::grost::__private::flavors::Network,
                4u32,
            >,
        >,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            (::grost::__private::flavors::network::Tag::encode)(
                    <::grost::__private::reflection::TagReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                            ::grost::__private::flavors::Network,
                            4u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION,
                )
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::Len<
        ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::TagReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    User<I>,
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                    ::grost::__private::flavors::Network,
                    4u32,
                >,
            >,
        >,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = {
            &<::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::TagReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                        ::grost::__private::flavors::Network,
                        4u32,
                    >,
                >,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::WireTypeReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::WireType,
            ::grost::__private::flavors::Network,
            4u32,
        >,
    > {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::WireType;
        const REFLECTION: &Self::Reflection = &{
            <<Vec<
                String,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::WIRE_TYPE
        };
    }
    #[automatically_derived]
    #[allow(clippy::type_complexity, non_camel_case_types)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::reflection::ObjectField,
        ::grost::__private::flavors::Network,
        5u32,
    > {
        type Reflection = ::grost::__private::reflection::ObjectField;
        const REFLECTION: &'static Self::Reflection = &{
            ::grost::__private::reflection::ObjectFieldBuilder {
                name: "linkin",
                description: "The linkin link of the user",
                ty: <::grost::__private::reflection::TypeReflection<
                    Option<String>,
                > as ::grost::__private::reflection::Reflectable<
                    Option<String>,
                >>::REFLECTION,
            }
                .build()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        5u32,
    > {
        type Reflection = <Option<
            String,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format;
        const REFLECTION: &'static Self::Reflection = &{
            <<Option<
                String,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::SELF
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
            ::grost::__private::flavors::Network,
            5u32,
        >,
    > {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier;
        const REFLECTION: &Self::Reflection = &{
            (::grost::__private::flavors::network::Identifier::new)(
                <<Option<
                    String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                (::grost::__private::flavors::network::Tag::new)(5u32),
            )
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                User<I>,
                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                ::grost::__private::flavors::Network,
                5u32,
            >,
        >,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            (::grost::__private::flavors::network::Identifier::encode)(
                    <::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Network,
                            5u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION,
                )
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::Len<
        ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::IdentifierReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    User<I>,
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                    ::grost::__private::flavors::Network,
                    5u32,
                >,
            >,
        >,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = {
            &<::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                        ::grost::__private::flavors::Network,
                        5u32,
                    >,
                >,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Network,
            5u32,
        >,
    > {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag;
        const REFLECTION: &Self::Reflection = &{
            (::grost::__private::flavors::network::Tag::new)(5u32)
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::TagReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                User<I>,
                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                ::grost::__private::flavors::Network,
                5u32,
            >,
        >,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            (::grost::__private::flavors::network::Tag::encode)(
                    <::grost::__private::reflection::TagReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                            ::grost::__private::flavors::Network,
                            5u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION,
                )
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::Len<
        ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::TagReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    User<I>,
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                    ::grost::__private::flavors::Network,
                    5u32,
                >,
            >,
        >,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = {
            &<::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::TagReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag,
                        ::grost::__private::flavors::Network,
                        5u32,
                    >,
                >,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::WireTypeReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::WireType,
            ::grost::__private::flavors::Network,
            5u32,
        >,
    > {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::WireType;
        const REFLECTION: &Self::Reflection = &{
            <<Option<
                String,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::WIRE_TYPE
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>> for User<I>
    where
        ::grost::__private::reflection::TypeReflection<
            I,
        >: ::grost::__private::reflection::Reflectable<
            I,
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
                            name: "id",
                            description: "The id of the user",
                            ty: <::grost::__private::reflection::TypeReflection<
                                I,
                            > as ::grost::__private::reflection::Reflectable<
                                I,
                            >>::REFLECTION,
                        }
                            .build(),
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
                                Vec<String>,
                            > as ::grost::__private::reflection::Reflectable<
                                Vec<String>,
                            >>::REFLECTION,
                        }
                            .build(),
                        &::grost::__private::reflection::ObjectFieldBuilder {
                            name: "linkin",
                            description: "The linkin link of the user",
                            ty: <::grost::__private::reflection::TypeReflection<
                                Option<String>,
                            > as ::grost::__private::reflection::Reflectable<
                                Option<String>,
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
    impl<I: Default> ::grost::__private::reflection::Reflectable<User<I>>
    for ::grost::__private::reflection::ObjectReflection<
        User<I>,
        ::grost::__private::reflection::Object,
        ::grost::__private::flavors::Network,
    >
    where
        ::grost::__private::reflection::TypeReflection<
            I,
        >: ::grost::__private::reflection::Reflectable<
            I,
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
                        name: "id",
                        description: "The id of the user",
                        ty: <::grost::__private::reflection::TypeReflection<
                            I,
                        > as ::grost::__private::reflection::Reflectable<I>>::REFLECTION,
                    }
                        .build(),
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
                            Vec<String>,
                        > as ::grost::__private::reflection::Reflectable<
                            Vec<String>,
                        >>::REFLECTION,
                    }
                        .build(),
                    &::grost::__private::reflection::ObjectFieldBuilder {
                        name: "linkin",
                        description: "The linkin link of the user",
                        ty: <::grost::__private::reflection::TypeReflection<
                            Option<String>,
                        > as ::grost::__private::reflection::Reflectable<
                            Option<String>,
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
    impl<I: Default> User<I> {
        /// Returns the reflection of the struct.
        #[inline]
        pub const fn reflection() -> ::grost::__private::reflection::ObjectReflection<
            Self,
            ::grost::__private::reflection::Object,
            ::grost::__private::flavors::Network,
        >
        where
            ::grost::__private::flavors::Network: ::grost::__private::flavors::Flavor,
        {
            ::grost::__private::reflection::ObjectReflection::new()
        }
        /// Returns the field reflection of the field `User.id`.
        #[inline]
        const fn id_reflection() -> ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            ::grost::__private::reflection::ObjectField,
            ::grost::__private::flavors::Network,
            1u32,
        >
        where
            ::grost::__private::flavors::Network: ::grost::__private::flavors::Flavor,
        {
            ::grost::__private::reflection::ObjectFieldReflection::new()
        }
        /// Returns the field reflection of the field `User.name`.
        #[inline]
        const fn name_reflection() -> ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            ::grost::__private::reflection::ObjectField,
            ::grost::__private::flavors::Network,
            2u32,
        >
        where
            ::grost::__private::flavors::Network: ::grost::__private::flavors::Flavor,
        {
            ::grost::__private::reflection::ObjectFieldReflection::new()
        }
        /// Returns the field reflection of the field `User.age`.
        #[inline]
        const fn age_reflection() -> ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            ::grost::__private::reflection::ObjectField,
            ::grost::__private::flavors::Network,
            3u32,
        >
        where
            ::grost::__private::flavors::Network: ::grost::__private::flavors::Flavor,
        {
            ::grost::__private::reflection::ObjectFieldReflection::new()
        }
        /// Returns the field reflection of the field `User.emails`.
        #[inline]
        const fn emails_reflection() -> ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            ::grost::__private::reflection::ObjectField,
            ::grost::__private::flavors::Network,
            4u32,
        >
        where
            ::grost::__private::flavors::Network: ::grost::__private::flavors::Flavor,
        {
            ::grost::__private::reflection::ObjectFieldReflection::new()
        }
        /// Returns the field reflection of the field `User.linkin`.
        #[inline]
        const fn linkin_reflection() -> ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            ::grost::__private::reflection::ObjectField,
            ::grost::__private::flavors::Network,
            5u32,
        >
        where
            ::grost::__private::flavors::Network: ::grost::__private::flavors::Flavor,
        {
            ::grost::__private::reflection::ObjectFieldReflection::new()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        I: Default,
    > ::grost::__private::indexer::Indexable<::grost::__private::flavors::Network>
    for User<I> {
        type Indexer = UserIndex;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl UserIndex {
        /// Returns the field reflection of the corresponding field.
        #[allow(non_camel_case_types, clippy::type_complexity)]
        #[inline]
        pub const fn reflection<I: Default>(
            &self,
        ) -> &'static ::grost::__private::reflection::ObjectField
        where
            ::grost::__private::reflection::TypeReflection<
                I,
            >: ::grost::__private::reflection::Reflectable<
                I,
                Reflection = ::grost::__private::reflection::Type,
            >,
        {
            match self {
                Self::Id => {
                    <::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        ::grost::__private::reflection::ObjectField,
                        ::grost::__private::flavors::Network,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION
                }
                Self::Name => {
                    <::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        ::grost::__private::reflection::ObjectField,
                        ::grost::__private::flavors::Network,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION
                }
                Self::Age => {
                    <::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        ::grost::__private::reflection::ObjectField,
                        ::grost::__private::flavors::Network,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION
                }
                Self::Emails => {
                    <::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        ::grost::__private::reflection::ObjectField,
                        ::grost::__private::flavors::Network,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION
                }
                Self::Linkin => {
                    <::grost::__private::reflection::ObjectFieldReflection<
                        User<I>,
                        ::grost::__private::reflection::ObjectField,
                        ::grost::__private::flavors::Network,
                        5u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION
                }
            }
        }
    }
    #[automatically_derived]
    impl UserIndex {
        /// The number of variants of this field indexer.
        pub const VARIANTS: ::core::primitive::usize = 5usize;
        /// The first field indexer.
        pub const FIRST: Self = Self::Id;
        /// The last field indexer.
        pub const LAST: Self = Self::Linkin;
        /// Returns the next field indexer.
        ///
        /// Returns `None` if there are no more fields.
        #[inline]
        pub const fn next(&self) -> ::core::option::Option<Self> {
            match self {
                Self::Linkin => ::core::option::Option::None,
                Self::Id => ::core::option::Option::Some(Self::Name),
                Self::Name => ::core::option::Option::Some(Self::Age),
                Self::Age => ::core::option::Option::Some(Self::Emails),
                Self::Emails => ::core::option::Option::Some(Self::Linkin),
            }
        }
        /// Returns the previous field indexer.
        ///
        /// Returns `None` if there are no previous fields.
        #[inline]
        pub const fn prev(&self) -> ::core::option::Option<Self> {
            match self {
                Self::Id => ::core::option::Option::None,
                Self::Linkin => ::core::option::Option::Some(Self::Emails),
                Self::Emails => ::core::option::Option::Some(Self::Age),
                Self::Age => ::core::option::Option::Some(Self::Name),
                Self::Name => ::core::option::Option::Some(Self::Id),
            }
        }
        /// Returns the remaining number of fields.
        #[inline]
        pub const fn remaining(&self) -> ::core::primitive::usize {
            Self::LAST.index() - self.index()
        }
        const fn index(&self) -> ::core::primitive::usize {
            match self {
                Self::Id => 0usize,
                Self::Name => 1usize,
                Self::Age => 2usize,
                Self::Emails => 3usize,
                Self::Linkin => 4usize,
            }
        }
    }
    #[automatically_derived]
    impl ::core::iter::Iterator for UserIndex {
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
    impl ::core::iter::DoubleEndedIterator for UserIndex {
        fn next_back(&mut self) -> ::core::option::Option<Self> {
            Self::prev(self)
        }
    }
    #[automatically_derived]
    impl ::core::iter::FusedIterator for UserIndex {}
    #[automatically_derived]
    impl ::core::iter::ExactSizeIterator for UserIndex {
        fn len(&self) -> ::core::primitive::usize {
            self.remaining()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        I: Default,
    > ::grost::__private::selection::Selectable<::grost::__private::flavors::Network>
    for User<I>
    where
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
    {
        type Selector = UserSelector<I>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        I: Default,
    > ::grost::__private::selection::Selectable<::grost::__private::flavors::Network>
    for PartialUser<I>
    where
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
    {
        type Selector = UserSelector<I>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I: Default,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > ::grost::__private::selection::Selectable<::grost::__private::flavors::Network>
    for PartialDecodedUser<
        '__grost_lifetime__,
        I,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized,
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
    {
        type Selector = UserSelector<I>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I: Default> ::core::fmt::Debug for UserSelector<I>
    where
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
    {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            f.debug_struct("UserSelector")
                .field("id", &self.id)
                .field("name", &self.name)
                .field("age", &self.age)
                .field("emails", &self.emails)
                .field("linkin", &self.linkin)
                .finish()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I: Default> ::core::cmp::PartialEq for UserSelector<I>
    where
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
    {
        fn eq(&self, other: &Self) -> ::core::primitive::bool {
            self.id == other.id && self.name == other.name && self.age == other.age
                && self.emails == other.emails && self.linkin == other.linkin
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I: Default> ::core::cmp::Eq for UserSelector<I>
    where
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
    {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I: Default> ::core::hash::Hash for UserSelector<I>
    where
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
    {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
            self.id.hash(state);
            self.name.hash(state);
            self.age.hash(state);
            self.emails.hash(state);
            self.linkin.hash(state);
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I: Default> ::core::clone::Clone for UserSelector<I>
    where
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
    {
        fn clone(&self) -> Self {
            Self {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
                age: ::core::clone::Clone::clone(&self.age),
                emails: ::core::clone::Clone::clone(&self.emails),
                linkin: ::core::clone::Clone::clone(&self.linkin),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I: Default> ::core::marker::Copy for UserSelector<I>
    where
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
        <I as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector: ::core::marker::Copy,
        <String as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector: ::core::marker::Copy,
        <u8 as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector: ::core::marker::Copy,
        <Vec<
            String,
        > as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector: ::core::marker::Copy,
        <Option<
            String,
        > as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector: ::core::marker::Copy,
    {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        I: Default,
    > ::grost::__private::selection::Selector<::grost::__private::flavors::Network>
    for UserSelector<I>
    where
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
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
            <<I as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::flip(&mut self.id);
            <<String as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::flip(&mut self.name);
            <<u8 as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::flip(&mut self.age);
            <<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::flip(&mut self.emails);
            <<Option<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::flip(&mut self.linkin);
            self
        }
        fn merge(&mut self, other: Self) -> &mut Self {
            <<I as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::merge(&mut self.id, other.id);
            <<String as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::merge(&mut self.name, other.name);
            <<u8 as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::merge(&mut self.age, other.age);
            <<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::merge(&mut self.emails, other.emails);
            <<Option<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::merge(&mut self.linkin, other.linkin);
            self
        }
    }
    #[automatically_derived]
    impl<I: Default> UserSelector<I>
    where
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
    {
        /// Returns a selector with the default values.
        #[inline]
        pub const fn new() -> Self {
            Self {
                id: <<I as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::DEFAULT,
                name: <<String as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::DEFAULT,
                age: <<u8 as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::DEFAULT,
                emails: <<Vec<
                    String,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::DEFAULT,
                linkin: <<Option<
                    String,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::DEFAULT,
            }
        }
        /// Returns a selector which selects nothing.
        #[inline]
        pub const fn empty() -> Self {
            Self {
                id: <<I as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::NONE,
                name: <<String as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::NONE,
                age: <<u8 as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::NONE,
                emails: <<Vec<
                    String,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::NONE,
                linkin: <<Option<
                    String,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::NONE,
            }
        }
        /// Returns a selector which selects all.
        #[inline]
        pub const fn all() -> Self {
            Self {
                id: <<I as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::ALL,
                name: <<String as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::ALL,
                age: <<u8 as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::ALL,
                emails: <<Vec<
                    String,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::ALL,
                linkin: <<Option<
                    String,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::ALL,
            }
        }
        /// Returns `true` if the selector selects nothing.
        #[inline]
        pub fn is_empty(&self) -> ::core::primitive::bool {
            <<I as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::is_empty(&self.id)
                && <<String as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::is_empty(&self.name)
                && <<u8 as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::is_empty(&self.age)
                && <<Vec<
                    String,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::is_empty(&self.emails)
                && <<Option<
                    String,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::is_empty(&self.linkin)
        }
        /// Returns `true` if the selector selects all.
        #[inline]
        pub fn is_all(&self) -> ::core::primitive::bool {
            <<I as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::is_all(&self.id)
                && <<String as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::is_all(&self.name)
                && <<u8 as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::is_all(&self.age)
                && <<Vec<
                    String,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::is_all(&self.emails)
                && <<Option<
                    String,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Network,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Network,
                >>::is_all(&self.linkin)
        }
        /// Returns the number of selected fields.
        #[inline]
        pub fn selected(&self) -> ::core::primitive::usize {
            let mut num = 0;
            if self.is_id_selected() {
                num += 1;
            }
            if self.is_name_selected() {
                num += 1;
            }
            if self.is_age_selected() {
                num += 1;
            }
            if self.is_emails_selected() {
                num += 1;
            }
            if self.is_linkin_selected() {
                num += 1;
            }
            num
        }
        /// Returns the number of unselected fields.
        #[inline]
        pub fn unselected(&self) -> ::core::primitive::usize {
            let mut num = 0;
            if self.is_id_unselected() {
                num += 1;
            }
            if self.is_name_unselected() {
                num += 1;
            }
            if self.is_age_unselected() {
                num += 1;
            }
            if self.is_emails_unselected() {
                num += 1;
            }
            if self.is_linkin_unselected() {
                num += 1;
            }
            num
        }
        /// Returns an iterator over the selected fields.
        #[inline]
        pub fn iter_selected<'__grost_lifetime__>(
            &'__grost_lifetime__ self,
        ) -> UserSelectorIter<'__grost_lifetime__, I, true> {
            UserSelectorIter::new(self, self.selected())
        }
        /// Returns an iterator over the unselected fields.
        #[inline]
        pub fn iter_unselected<'__grost_lifetime__>(
            &'__grost_lifetime__ self,
        ) -> UserSelectorIter<'__grost_lifetime__, I, false> {
            UserSelectorIter::new(self, self.unselected())
        }
        /// Returns `true` if such field is selected.
        #[inline]
        pub fn is_selected(&self, idx: UserIndex) -> ::core::primitive::bool {
            match idx {
                UserIndex::Id => self.is_id_selected(),
                UserIndex::Name => self.is_name_selected(),
                UserIndex::Age => self.is_age_selected(),
                UserIndex::Emails => self.is_emails_selected(),
                UserIndex::Linkin => self.is_linkin_selected(),
            }
        }
        /// Returns `true` if such field is unselected.
        #[inline]
        pub fn is_unselected(&self, idx: UserIndex) -> ::core::primitive::bool {
            match idx {
                UserIndex::Id => self.is_id_unselected(),
                UserIndex::Name => self.is_name_unselected(),
                UserIndex::Age => self.is_age_unselected(),
                UserIndex::Emails => self.is_emails_unselected(),
                UserIndex::Linkin => self.is_linkin_unselected(),
            }
        }
        /// Select the `User.id` field
        #[inline]
        pub fn select_id(&mut self) -> &mut Self {
            self
                .id = <<I as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::DEFAULT;
            self
        }
        /// Unselect the `User.id` field
        #[inline]
        pub fn unselect_id(&mut self) -> &mut Self {
            self
                .id = <<I as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::NONE;
            self
        }
        /// Update the `User.id` field
        #[inline]
        pub fn update_id(
            &mut self,
            value: <I as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector,
        ) -> &mut Self {
            self.id = value;
            self
        }
        /// Set or unset the `User.id` field
        #[inline]
        pub fn maybe_id(
            mut self,
            val: <I as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector,
        ) -> Self {
            self.id = val;
            self
        }
        /// Get a reference to the selector of `User.id` field
        #[inline]
        pub const fn id_ref(
            &self,
        ) -> &<I as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector {
            &self.id
        }
        /// Get a mutable reference to the selector of `User.id` field
        #[inline]
        pub const fn id_mut(
            &mut self,
        ) -> &mut <I as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector {
            &mut self.id
        }
        /// Set the `User.id` field
        #[inline]
        pub fn with_id(mut self) -> Self {
            self
                .id = <<I as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::DEFAULT;
            self
        }
        /// Unset the `User.id` field
        #[inline]
        pub fn without_id(mut self) -> Self {
            self
                .id = <<I as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::NONE;
            self
        }
        /// Returns `true` if the `User.id` field is selected
        #[inline]
        pub fn is_id_selected(&self) -> ::core::primitive::bool {
            !<<I as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::is_empty(&self.id)
        }
        /// Returns `true` if the `User.id` field is unselected
        #[inline]
        pub fn is_id_unselected(&self) -> ::core::primitive::bool {
            <<I as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::is_empty(&self.id)
        }
        /// Select the `User.name` field
        #[inline]
        pub fn select_name(&mut self) -> &mut Self {
            self
                .name = <<String as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::DEFAULT;
            self
        }
        /// Unselect the `User.name` field
        #[inline]
        pub fn unselect_name(&mut self) -> &mut Self {
            self
                .name = <<String as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::NONE;
            self
        }
        /// Update the `User.name` field
        #[inline]
        pub fn update_name(
            &mut self,
            value: <String as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector,
        ) -> &mut Self {
            self.name = value;
            self
        }
        /// Set or unset the `User.name` field
        #[inline]
        pub fn maybe_name(
            mut self,
            val: <String as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector,
        ) -> Self {
            self.name = val;
            self
        }
        /// Get a reference to the selector of `User.name` field
        #[inline]
        pub const fn name_ref(
            &self,
        ) -> &<String as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector {
            &self.name
        }
        /// Get a mutable reference to the selector of `User.name` field
        #[inline]
        pub const fn name_mut(
            &mut self,
        ) -> &mut <String as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector {
            &mut self.name
        }
        /// Set the `User.name` field
        #[inline]
        pub fn with_name(mut self) -> Self {
            self
                .name = <<String as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::DEFAULT;
            self
        }
        /// Unset the `User.name` field
        #[inline]
        pub fn without_name(mut self) -> Self {
            self
                .name = <<String as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::NONE;
            self
        }
        /// Returns `true` if the `User.name` field is selected
        #[inline]
        pub fn is_name_selected(&self) -> ::core::primitive::bool {
            !<<String as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::is_empty(&self.name)
        }
        /// Returns `true` if the `User.name` field is unselected
        #[inline]
        pub fn is_name_unselected(&self) -> ::core::primitive::bool {
            <<String as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::is_empty(&self.name)
        }
        /// Select the `User.age` field
        #[inline]
        pub fn select_age(&mut self) -> &mut Self {
            self
                .age = <<u8 as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::DEFAULT;
            self
        }
        /// Unselect the `User.age` field
        #[inline]
        pub fn unselect_age(&mut self) -> &mut Self {
            self
                .age = <<u8 as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::NONE;
            self
        }
        /// Update the `User.age` field
        #[inline]
        pub fn update_age(
            &mut self,
            value: <u8 as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector,
        ) -> &mut Self {
            self.age = value;
            self
        }
        /// Set or unset the `User.age` field
        #[inline]
        pub fn maybe_age(
            mut self,
            val: <u8 as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector,
        ) -> Self {
            self.age = val;
            self
        }
        /// Get a reference to the selector of `User.age` field
        #[inline]
        pub const fn age_ref(
            &self,
        ) -> &<u8 as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector {
            &self.age
        }
        /// Get a mutable reference to the selector of `User.age` field
        #[inline]
        pub const fn age_mut(
            &mut self,
        ) -> &mut <u8 as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector {
            &mut self.age
        }
        /// Set the `User.age` field
        #[inline]
        pub fn with_age(mut self) -> Self {
            self
                .age = <<u8 as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::DEFAULT;
            self
        }
        /// Unset the `User.age` field
        #[inline]
        pub fn without_age(mut self) -> Self {
            self
                .age = <<u8 as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::NONE;
            self
        }
        /// Returns `true` if the `User.age` field is selected
        #[inline]
        pub fn is_age_selected(&self) -> ::core::primitive::bool {
            !<<u8 as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::is_empty(&self.age)
        }
        /// Returns `true` if the `User.age` field is unselected
        #[inline]
        pub fn is_age_unselected(&self) -> ::core::primitive::bool {
            <<u8 as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::is_empty(&self.age)
        }
        /// Select the `User.emails` field
        #[inline]
        pub fn select_emails(&mut self) -> &mut Self {
            self
                .emails = <<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::DEFAULT;
            self
        }
        /// Unselect the `User.emails` field
        #[inline]
        pub fn unselect_emails(&mut self) -> &mut Self {
            self
                .emails = <<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::NONE;
            self
        }
        /// Update the `User.emails` field
        #[inline]
        pub fn update_emails(
            &mut self,
            value: <Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector,
        ) -> &mut Self {
            self.emails = value;
            self
        }
        /// Set or unset the `User.emails` field
        #[inline]
        pub fn maybe_emails(
            mut self,
            val: <Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector,
        ) -> Self {
            self.emails = val;
            self
        }
        /// Get a reference to the selector of `User.emails` field
        #[inline]
        pub const fn emails_ref(
            &self,
        ) -> &<Vec<
            String,
        > as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector {
            &self.emails
        }
        /// Get a mutable reference to the selector of `User.emails` field
        #[inline]
        pub const fn emails_mut(
            &mut self,
        ) -> &mut <Vec<
            String,
        > as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector {
            &mut self.emails
        }
        /// Set the `User.emails` field
        #[inline]
        pub fn with_emails(mut self) -> Self {
            self
                .emails = <<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::DEFAULT;
            self
        }
        /// Unset the `User.emails` field
        #[inline]
        pub fn without_emails(mut self) -> Self {
            self
                .emails = <<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::NONE;
            self
        }
        /// Returns `true` if the `User.emails` field is selected
        #[inline]
        pub fn is_emails_selected(&self) -> ::core::primitive::bool {
            !<<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::is_empty(&self.emails)
        }
        /// Returns `true` if the `User.emails` field is unselected
        #[inline]
        pub fn is_emails_unselected(&self) -> ::core::primitive::bool {
            <<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::is_empty(&self.emails)
        }
        /// Select the `User.linkin` field
        #[inline]
        pub fn select_linkin(&mut self) -> &mut Self {
            self
                .linkin = <<Option<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::DEFAULT;
            self
        }
        /// Unselect the `User.linkin` field
        #[inline]
        pub fn unselect_linkin(&mut self) -> &mut Self {
            self
                .linkin = <<Option<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::NONE;
            self
        }
        /// Update the `User.linkin` field
        #[inline]
        pub fn update_linkin(
            &mut self,
            value: <Option<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector,
        ) -> &mut Self {
            self.linkin = value;
            self
        }
        /// Set or unset the `User.linkin` field
        #[inline]
        pub fn maybe_linkin(
            mut self,
            val: <Option<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector,
        ) -> Self {
            self.linkin = val;
            self
        }
        /// Get a reference to the selector of `User.linkin` field
        #[inline]
        pub const fn linkin_ref(
            &self,
        ) -> &<Option<
            String,
        > as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector {
            &self.linkin
        }
        /// Get a mutable reference to the selector of `User.linkin` field
        #[inline]
        pub const fn linkin_mut(
            &mut self,
        ) -> &mut <Option<
            String,
        > as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >>::Selector {
            &mut self.linkin
        }
        /// Set the `User.linkin` field
        #[inline]
        pub fn with_linkin(mut self) -> Self {
            self
                .linkin = <<Option<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::DEFAULT;
            self
        }
        /// Unset the `User.linkin` field
        #[inline]
        pub fn without_linkin(mut self) -> Self {
            self
                .linkin = <<Option<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::NONE;
            self
        }
        /// Returns `true` if the `User.linkin` field is selected
        #[inline]
        pub fn is_linkin_selected(&self) -> ::core::primitive::bool {
            !<<Option<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::is_empty(&self.linkin)
        }
        /// Returns `true` if the `User.linkin` field is unselected
        #[inline]
        pub fn is_linkin_unselected(&self) -> ::core::primitive::bool {
            <<Option<
                String,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Network,
            >>::Selector as ::grost::__private::selection::Selector<
                ::grost::__private::flavors::Network,
            >>::is_empty(&self.linkin)
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I: Default,
        const __GROST_SELECTED__: ::core::primitive::bool,
    > UserSelectorIter<'__grost_lifetime__, I, __GROST_SELECTED__>
    where
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
    {
        #[inline]
        const fn new(
            selector: &'__grost_lifetime__ UserSelector<I>,
            num: ::core::primitive::usize,
        ) -> Self {
            Self {
                selector,
                index: ::core::option::Option::Some(UserIndex::FIRST),
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
    impl<
        '__grost_lifetime__,
        I: Default,
        const __GROST_SELECTED__: ::core::primitive::bool,
    > ::core::iter::Iterator
    for UserSelectorIter<'__grost_lifetime__, I, __GROST_SELECTED__>
    where
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::reflection::TypeReflection<
            I,
        >: ::grost::__private::reflection::Reflectable<
            I,
            Reflection = ::grost::__private::reflection::Type,
        >,
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
                        return ::core::option::Option::Some(idx.reflection::<I>());
                    }
                } else if self.selector.is_unselected(idx) {
                    self.index = idx.next();
                    self.yielded += 1;
                    return ::core::option::Option::Some(idx.reflection::<I>());
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
    impl<
        '__grost_lifetime__,
        I: Default,
        const __GROST_SELECTED__: ::core::primitive::bool,
    > ::core::iter::FusedIterator
    for UserSelectorIter<'__grost_lifetime__, I, __GROST_SELECTED__>
    where
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::reflection::TypeReflection<
            I,
        >: ::grost::__private::reflection::Reflectable<
            I,
            Reflection = ::grost::__private::reflection::Type,
        >,
    {}
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I: Default,
        const __GROST_SELECTED__: ::core::primitive::bool,
    > ::core::iter::ExactSizeIterator
    for UserSelectorIter<'__grost_lifetime__, I, __GROST_SELECTED__>
    where
        I: ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::reflection::TypeReflection<
            I,
        >: ::grost::__private::reflection::Reflectable<
            I,
            Reflection = ::grost::__private::reflection::Type,
        >,
    {
        fn len(&self) -> ::core::primitive::usize {
            self.remaining()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I: Default,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > ::grost::__private::Decode<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
        Self,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > for User<I>
    where
        I: ::grost::__private::Decode<
            '__grost_lifetime__,
            ::grost::__private::flavors::Network,
            grost::flavors::network::LengthDelimited,
            I,
            __GROST_UNKNOWN_BUFFER__,
        >,
    {
        fn decode(
            context: &'__grost_lifetime__ <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            src: __GROST_READ_BUFFER__,
        ) -> ::core::result::Result<
            (::core::primitive::usize, Self),
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Error,
        >
        where
            Self: ::core::marker::Sized + '__grost_lifetime__,
            __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf<
                '__grost_lifetime__,
            >,
            __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Unknown<
                        __GROST_READ_BUFFER__,
                    >,
                > + '__grost_lifetime__,
        {
            ::core::panicking::panic("not yet implemented")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        I: Default,
    > ::grost::__private::decode::Transform<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
        Self,
    > for User<I> {
        fn transform(
            input: Self,
        ) -> ::core::result::Result<
            Self,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Error,
        > {
            ::core::result::Result::Ok(input)
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I: Default,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > ::grost::__private::Decode<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
        Self,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > for PartialUser<I>
    where
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized,
        I: ::grost::__private::Decode<
            '__grost_lifetime__,
            ::grost::__private::flavors::Network,
            grost::flavors::network::LengthDelimited,
            <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    grost::flavors::network::LengthDelimited,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >,
        I: ::grost::__private::Transform<
            ::grost::__private::flavors::Network,
            grost::flavors::network::LengthDelimited,
            <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    grost::flavors::network::LengthDelimited,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        >,
    {
        fn decode(
            context: &'__grost_lifetime__ <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            src: __GROST_READ_BUFFER__,
        ) -> ::core::result::Result<
            (::core::primitive::usize, Self),
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Error,
        >
        where
            Self: ::core::marker::Sized + '__grost_lifetime__,
            __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf<
                '__grost_lifetime__,
            >,
            __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Unknown<
                        __GROST_READ_BUFFER__,
                    >,
                > + '__grost_lifetime__,
        {
            <PartialUser<
                I,
            > as ::grost::__private::decode::Decode<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
                PartialDecodedUser<
                    '__grost_lifetime__,
                    I,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >>::decode(context, src)
                .and_then(|(read, input)| {
                    <PartialUser<
                        I,
                    > as ::grost::__private::decode::Transform<
                        ::grost::__private::flavors::Network,
                        ::grost::__private::flavors::network::LengthDelimited,
                        PartialDecodedUser<
                            '__grost_lifetime__,
                            I,
                            __GROST_READ_BUFFER__,
                            __GROST_UNKNOWN_BUFFER__,
                        >,
                    >>::transform(input)
                        .map(|input| (read, input))
                })
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        I: Default,
    > ::grost::__private::decode::Transform<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
        Self,
    > for PartialUser<I> {
        fn transform(
            input: Self,
        ) -> ::core::result::Result<
            Self,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Error,
        > {
            ::core::result::Result::Ok(input)
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I: Default,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > ::grost::__private::decode::Transform<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
        Self,
    >
    for PartialDecodedUser<
        '__grost_lifetime__,
        I,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized,
    {
        fn transform(
            input: Self,
        ) -> ::core::result::Result<
            Self,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Error,
        > {
            ::core::result::Result::Ok(input)
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        '__grost_decode_lifetime__,
        I: Default,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > ::grost::__private::Decode<
        '__grost_decode_lifetime__,
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
        Self,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    >
    for PartialDecodedUser<
        '__grost_lifetime__,
        I,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized,
        I: ::grost::__private::Decode<
            '__grost_decode_lifetime__,
            ::grost::__private::flavors::Network,
            grost::flavors::network::LengthDelimited,
            <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    grost::flavors::network::LengthDelimited,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >,
        '__grost_decode_lifetime__: '__grost_lifetime__,
    {
        fn decode(
            context: &'__grost_decode_lifetime__ <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            src: __GROST_READ_BUFFER__,
        ) -> ::core::result::Result<
            (::core::primitive::usize, Self),
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Error,
        >
        where
            Self: ::core::marker::Sized + '__grost_decode_lifetime__,
            __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf<
                '__grost_decode_lifetime__,
            >,
            __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Unknown<
                        __GROST_READ_BUFFER__,
                    >,
                > + '__grost_decode_lifetime__,
        {
            let buf = src.as_bytes();
            let buf_len = buf.len();
            let mut this = Self::new();
            let mut offset = 0;
            while offset < buf_len {
                let (read, identifier) = <<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier as ::grost::__private::flavors::Identifier<
                    ::grost::__private::flavors::Network,
                >>::decode::<&[::core::primitive::u8]>(&buf[offset..])?;
                offset += read;
                match &identifier {
                    <::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Network,
                            1u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION => {
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::core::convert::Into::into(
                                    ::grost::__private::error::Error::buffer_underflow(),
                                ),
                            );
                        }
                        if this.id.is_some() {
                            return ::core::result::Result::Err(
                                ::core::convert::Into::into(
                                    ::grost::__private::error::Error::duplicated_field(
                                        "id",
                                        ::core::any::type_name::<I>(),
                                        *<::grost::__private::reflection::IdentifierReflection<
                                            ::grost::__private::reflection::ObjectFieldReflection<
                                                User<I>,
                                                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                                                ::grost::__private::flavors::Network,
                                                1u32,
                                            >,
                                        > as ::grost::__private::reflection::Reflectable<
                                            User<I>,
                                        >>::REFLECTION,
                                    ),
                                ),
                            );
                        }
                        let (read, value) = <I as ::grost::__private::Decode<
                            '__grost_decode_lifetime__,
                            ::grost::__private::flavors::Network,
                            grost::flavors::network::LengthDelimited,
                            <I as ::grost::__private::convert::State<
                                ::grost::__private::convert::Decoded<
                                    '__grost_lifetime__,
                                    ::grost::__private::flavors::Network,
                                    grost::flavors::network::LengthDelimited,
                                    __GROST_READ_BUFFER__,
                                    __GROST_UNKNOWN_BUFFER__,
                                >,
                            >>::Output,
                            __GROST_READ_BUFFER__,
                            __GROST_UNKNOWN_BUFFER__,
                        >>::decode(context, src.slice(offset..))?;
                        this.id = ::core::option::Option::Some(value);
                        offset += read;
                    }
                    <::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Network,
                            2u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION => {
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::core::convert::Into::into(
                                    ::grost::__private::error::Error::buffer_underflow(),
                                ),
                            );
                        }
                        if this.name.is_some() {
                            return ::core::result::Result::Err(
                                ::core::convert::Into::into(
                                    ::grost::__private::error::Error::duplicated_field(
                                        "name",
                                        ::core::any::type_name::<String>(),
                                        *<::grost::__private::reflection::IdentifierReflection<
                                            ::grost::__private::reflection::ObjectFieldReflection<
                                                User<I>,
                                                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                                                ::grost::__private::flavors::Network,
                                                2u32,
                                            >,
                                        > as ::grost::__private::reflection::Reflectable<
                                            User<I>,
                                        >>::REFLECTION,
                                    ),
                                ),
                            );
                        }
                        let (read, value) = <String as ::grost::__private::Decode<
                            '__grost_decode_lifetime__,
                            ::grost::__private::flavors::Network,
                            <String as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                            <String as ::grost::__private::convert::State<
                                ::grost::__private::convert::Decoded<
                                    '__grost_lifetime__,
                                    ::grost::__private::flavors::Network,
                                    <String as ::grost::__private::flavors::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                    __GROST_READ_BUFFER__,
                                    __GROST_UNKNOWN_BUFFER__,
                                >,
                            >>::Output,
                            __GROST_READ_BUFFER__,
                            __GROST_UNKNOWN_BUFFER__,
                        >>::decode(context, src.slice(offset..))?;
                        this.name = ::core::option::Option::Some(value);
                        offset += read;
                    }
                    <::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Network,
                            3u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION => {
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::core::convert::Into::into(
                                    ::grost::__private::error::Error::buffer_underflow(),
                                ),
                            );
                        }
                        if this.age.is_some() {
                            return ::core::result::Result::Err(
                                ::core::convert::Into::into(
                                    ::grost::__private::error::Error::duplicated_field(
                                        "age",
                                        ::core::any::type_name::<u8>(),
                                        *<::grost::__private::reflection::IdentifierReflection<
                                            ::grost::__private::reflection::ObjectFieldReflection<
                                                User<I>,
                                                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                                                ::grost::__private::flavors::Network,
                                                3u32,
                                            >,
                                        > as ::grost::__private::reflection::Reflectable<
                                            User<I>,
                                        >>::REFLECTION,
                                    ),
                                ),
                            );
                        }
                        let (read, value) = <u8 as ::grost::__private::Decode<
                            '__grost_decode_lifetime__,
                            ::grost::__private::flavors::Network,
                            <u8 as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                            <u8 as ::grost::__private::convert::State<
                                ::grost::__private::convert::Decoded<
                                    '__grost_lifetime__,
                                    ::grost::__private::flavors::Network,
                                    <u8 as ::grost::__private::flavors::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                    __GROST_READ_BUFFER__,
                                    __GROST_UNKNOWN_BUFFER__,
                                >,
                            >>::Output,
                            __GROST_READ_BUFFER__,
                            __GROST_UNKNOWN_BUFFER__,
                        >>::decode(context, src.slice(offset..))?;
                        this.age = ::core::option::Option::Some(value);
                        offset += read;
                    }
                    <::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Network,
                            4u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION => {
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::core::convert::Into::into(
                                    ::grost::__private::error::Error::buffer_underflow(),
                                ),
                            );
                        }
                        if this.emails.is_some() {
                            return ::core::result::Result::Err(
                                ::core::convert::Into::into(
                                    ::grost::__private::error::Error::duplicated_field(
                                        "emails",
                                        ::core::any::type_name::<Vec<String>>(),
                                        *<::grost::__private::reflection::IdentifierReflection<
                                            ::grost::__private::reflection::ObjectFieldReflection<
                                                User<I>,
                                                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                                                ::grost::__private::flavors::Network,
                                                4u32,
                                            >,
                                        > as ::grost::__private::reflection::Reflectable<
                                            User<I>,
                                        >>::REFLECTION,
                                    ),
                                ),
                            );
                        }
                        let (read, value) = <Vec<
                            String,
                        > as ::grost::__private::Decode<
                            '__grost_decode_lifetime__,
                            ::grost::__private::flavors::Network,
                            <Vec<
                                String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                            <Vec<
                                String,
                            > as ::grost::__private::convert::State<
                                ::grost::__private::convert::Decoded<
                                    '__grost_lifetime__,
                                    ::grost::__private::flavors::Network,
                                    <Vec<
                                        String,
                                    > as ::grost::__private::flavors::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                    __GROST_READ_BUFFER__,
                                    __GROST_UNKNOWN_BUFFER__,
                                >,
                            >>::Output,
                            __GROST_READ_BUFFER__,
                            __GROST_UNKNOWN_BUFFER__,
                        >>::decode(context, src.slice(offset..))?;
                        this.emails = ::core::option::Option::Some(value);
                        offset += read;
                    }
                    <::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            User<I>,
                            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Network,
                            5u32,
                        >,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::REFLECTION => {
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::core::convert::Into::into(
                                    ::grost::__private::error::Error::buffer_underflow(),
                                ),
                            );
                        }
                        if this.linkin.is_some() {
                            return ::core::result::Result::Err(
                                ::core::convert::Into::into(
                                    ::grost::__private::error::Error::duplicated_field(
                                        "linkin",
                                        ::core::any::type_name::<Option<String>>(),
                                        *<::grost::__private::reflection::IdentifierReflection<
                                            ::grost::__private::reflection::ObjectFieldReflection<
                                                User<I>,
                                                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier,
                                                ::grost::__private::flavors::Network,
                                                5u32,
                                            >,
                                        > as ::grost::__private::reflection::Reflectable<
                                            User<I>,
                                        >>::REFLECTION,
                                    ),
                                ),
                            );
                        }
                        let (read, value) = <Option<
                            String,
                        > as ::grost::__private::Decode<
                            '__grost_decode_lifetime__,
                            ::grost::__private::flavors::Network,
                            <Option<
                                String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                            <Option<
                                String,
                            > as ::grost::__private::convert::State<
                                ::grost::__private::convert::Decoded<
                                    '__grost_lifetime__,
                                    ::grost::__private::flavors::Network,
                                    <Option<
                                        String,
                                    > as ::grost::__private::flavors::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                    __GROST_READ_BUFFER__,
                                    __GROST_UNKNOWN_BUFFER__,
                                >,
                            >>::Output,
                            __GROST_READ_BUFFER__,
                            __GROST_UNKNOWN_BUFFER__,
                        >>::decode(context, src.slice(offset..))?;
                        this.linkin = ::core::option::Option::Some(value);
                        offset += read;
                    }
                    _ => {
                        if context.err_on_unknown() {
                            return ::core::result::Result::Err(
                                ::core::convert::Into::into(
                                    ::grost::__private::error::Error::unknown_identifier(
                                        ::core::any::type_name::<User<I>>(),
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
                                += <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::skip(
                                    context,
                                    identifier.wire_type(),
                                    src.slice(offset..),
                                )?;
                        } else {
                            let encoded_len = <<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Identifier as ::grost::__private::flavors::Identifier<
                                ::grost::__private::flavors::Network,
                            >>::encoded_len(&identifier);
                            let (read, unknown) = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::decode_unknown(
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
            if this.name.is_none() {
                this.name = ::core::option::Option::Some((error_name)()?);
            }
            ::core::result::Result::Ok((offset, this))
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        '__grost_decode_lifetime__,
        I: Default,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > ::grost::__private::Decode<
        '__grost_decode_lifetime__,
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
        PartialDecodedUser<
            '__grost_lifetime__,
            I,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > for User<I>
    where
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized,
        I: ::grost::__private::Decode<
            '__grost_decode_lifetime__,
            ::grost::__private::flavors::Network,
            grost::flavors::network::LengthDelimited,
            <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    grost::flavors::network::LengthDelimited,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >,
        '__grost_decode_lifetime__: '__grost_lifetime__,
    {
        fn decode(
            context: &'__grost_decode_lifetime__ <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            src: __GROST_READ_BUFFER__,
        ) -> ::core::result::Result<
            (
                ::core::primitive::usize,
                PartialDecodedUser<
                    '__grost_lifetime__,
                    I,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            ),
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Error,
        >
        where
            PartialDecodedUser<
                '__grost_lifetime__,
                I,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >: ::core::marker::Sized + '__grost_decode_lifetime__,
            __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf<
                '__grost_decode_lifetime__,
            >,
            __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Unknown<
                        __GROST_READ_BUFFER__,
                    >,
                > + '__grost_decode_lifetime__,
        {
            <PartialDecodedUser<
                '__grost_lifetime__,
                I,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            > as ::grost::__private::Decode<
                '__grost_decode_lifetime__,
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
                PartialDecodedUser<
                    '__grost_lifetime__,
                    I,
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
        I: Default,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > ::grost::__private::Decode<
        '__grost_decode_lifetime__,
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
        PartialDecodedUser<
            '__grost_lifetime__,
            I,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > for PartialUser<I>
    where
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized,
        I: ::grost::__private::Decode<
            '__grost_decode_lifetime__,
            ::grost::__private::flavors::Network,
            grost::flavors::network::LengthDelimited,
            <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    grost::flavors::network::LengthDelimited,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >,
        '__grost_decode_lifetime__: '__grost_lifetime__,
    {
        fn decode(
            context: &'__grost_decode_lifetime__ <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            src: __GROST_READ_BUFFER__,
        ) -> ::core::result::Result<
            (
                ::core::primitive::usize,
                PartialDecodedUser<
                    '__grost_lifetime__,
                    I,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            ),
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Error,
        >
        where
            PartialDecodedUser<
                '__grost_lifetime__,
                I,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >: ::core::marker::Sized + '__grost_decode_lifetime__,
            __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf<
                '__grost_decode_lifetime__,
            >,
            __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
                    <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Unknown<
                        __GROST_READ_BUFFER__,
                    >,
                > + '__grost_decode_lifetime__,
        {
            <PartialDecodedUser<
                '__grost_lifetime__,
                I,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            > as ::grost::__private::Decode<
                '__grost_decode_lifetime__,
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
                PartialDecodedUser<
                    '__grost_lifetime__,
                    I,
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
        I: Default,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > ::grost::__private::decode::Transform<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
        PartialDecodedUser<
            '__grost_lifetime__,
            I,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >,
    > for PartialUser<I>
    where
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized,
        I: ::grost::__private::Transform<
            ::grost::__private::flavors::Network,
            grost::flavors::network::LengthDelimited,
            <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Network,
                    grost::flavors::network::LengthDelimited,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        >,
        __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf<'__grost_lifetime__>,
        __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
                <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Unknown<
                    __GROST_READ_BUFFER__,
                >,
            > + '__grost_lifetime__,
    {
        fn transform(
            input: PartialDecodedUser<
                '__grost_lifetime__,
                I,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        ) -> ::core::result::Result<
            Self,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Error,
        > {
            let mut this = Self::new();
            if let ::core::option::Option::Some(value) = input.id {
                this
                    .id = ::core::option::Option::Some(
                    <I as ::grost::__private::decode::Transform<
                        ::grost::__private::flavors::Network,
                        grost::flavors::network::LengthDelimited,
                        <I as ::grost::__private::convert::State<
                            ::grost::__private::convert::Decoded<
                                '__grost_lifetime__,
                                ::grost::__private::flavors::Network,
                                grost::flavors::network::LengthDelimited,
                                __GROST_READ_BUFFER__,
                                __GROST_UNKNOWN_BUFFER__,
                            >,
                        >>::Output,
                    >>::transform(value)?,
                );
            }
            if let ::core::option::Option::Some(value) = input.name {
                this
                    .name = ::core::option::Option::Some(
                    <String as ::grost::__private::decode::Transform<
                        ::grost::__private::flavors::Network,
                        <String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                        <String as ::grost::__private::convert::State<
                            ::grost::__private::convert::Decoded<
                                '__grost_lifetime__,
                                ::grost::__private::flavors::Network,
                                <String as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                                __GROST_READ_BUFFER__,
                                __GROST_UNKNOWN_BUFFER__,
                            >,
                        >>::Output,
                    >>::transform(value)?,
                );
            }
            if let ::core::option::Option::Some(value) = input.age {
                this
                    .age = ::core::option::Option::Some(
                    <u8 as ::grost::__private::decode::Transform<
                        ::grost::__private::flavors::Network,
                        <u8 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                        <u8 as ::grost::__private::convert::State<
                            ::grost::__private::convert::Decoded<
                                '__grost_lifetime__,
                                ::grost::__private::flavors::Network,
                                <u8 as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                                __GROST_READ_BUFFER__,
                                __GROST_UNKNOWN_BUFFER__,
                            >,
                        >>::Output,
                    >>::transform(value)?,
                );
            }
            if let ::core::option::Option::Some(value) = input.emails {
                this
                    .emails = ::core::option::Option::Some(
                    <Vec<
                        String,
                    > as ::grost::__private::decode::Transform<
                        ::grost::__private::flavors::Network,
                        <Vec<
                            String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                        <Vec<
                            String,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Decoded<
                                '__grost_lifetime__,
                                ::grost::__private::flavors::Network,
                                <Vec<
                                    String,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                                __GROST_READ_BUFFER__,
                                __GROST_UNKNOWN_BUFFER__,
                            >,
                        >>::Output,
                    >>::transform(value)?,
                );
            }
            if let ::core::option::Option::Some(value) = input.linkin {
                this
                    .linkin = ::core::option::Option::Some(
                    <Option<
                        String,
                    > as ::grost::__private::decode::Transform<
                        ::grost::__private::flavors::Network,
                        <Option<
                            String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                        <Option<
                            String,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Decoded<
                                '__grost_lifetime__,
                                ::grost::__private::flavors::Network,
                                <Option<
                                    String,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                                __GROST_READ_BUFFER__,
                                __GROST_UNKNOWN_BUFFER__,
                            >,
                        >>::Output,
                    >>::transform(value)?,
                );
            }
            ::core::result::Result::Ok(this)
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I: Default,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            ::grost::__private::flavors::Network,
            ::grost::__private::flavors::network::LengthDelimited,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >,
    > for User<I>
    where
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized,
    {
        type Input = &'__grost_lifetime__ [::core::primitive::u8];
        type Output = PartialDecodedUser<
            '__grost_lifetime__,
            I,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I: Default,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            ::grost::__private::flavors::Network,
            ::grost::__private::flavors::network::LengthDelimited,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >,
    > for PartialUser<I>
    where
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized,
    {
        type Input = &'__grost_lifetime__ [::core::primitive::u8];
        type Output = PartialDecodedUser<
            '__grost_lifetime__,
            I,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I: Default,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            ::grost::__private::flavors::Network,
            ::grost::__private::flavors::network::LengthDelimited,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >
    for PartialDecodedUser<
        '__grost_lifetime__,
        I,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                grost::flavors::network::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized,
    {
        type Input = &'__grost_lifetime__ [::core::primitive::u8];
        type Output = Self;
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
        source_file: "grost-derive/tests/concrete_object.rs",
        start_line: 166usize,
        start_col: 4usize,
        end_line: 166usize,
        end_col: 5usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(t())),
};
fn t() {
    let val = User::<String>::emails_reflection();
    let wf = val.wire_format();
    let identifier = val.identifier();
    let encoded_identifier = val.encoded_identifier();
    let object_refl = User::<String>::reflection();
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
