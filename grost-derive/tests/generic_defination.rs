#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use std::marker::PhantomData;
use grost::{
    Decode, PartialDecode, buffer::Buf,
    flavors::{DefaultWireFormat, Network, WireFormat, network::LengthDelimited},
    selection::{Selectable, Selector},
};
use grost_derive::{Object, object};
fn default_array<const N: usize>() -> [u8; N] {
    [0; N]
}
#[grost(
    flavor(
        default(
            encode(skip_default, enum = "grost::flavors::network::encoding::enumeration")
        ),
    ),
    generic(flavor = "F"),
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
        list(string)
    )]
    emails: Vec<String>,
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
        let _: ::core::cmp::AssertParamIsEq<Vec<String>>;
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
                UserIndex::Name => "Name",
                UserIndex::Age => "Age",
                UserIndex::Emails => "Emails",
            },
        )
    }
}
/// Partial struct for the [`User`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct PartialUser<__GROST_UNKNOWN_BUFFER__> {
    __grost_unknown_buffer__: ::core::option::Option<__GROST_UNKNOWN_BUFFER__>,
    name: ::core::option::Option<
        <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Flatten,
        >>::Output,
    >,
    age: ::core::option::Option<
        <u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Flatten,
        >>::Output,
    >,
    emails: ::core::option::Option<
        <Vec<
            String,
        > as ::grost::__private::convert::State<
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
    Vec<
        String,
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
    <Vec<
        String,
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
        <Vec<
            String,
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
    Vec<
        String,
    >: ::grost::__private::selection::Selectable<
        F,
        <::grost::__private::reflection::WireFormatReflection<
            User,
            F,
            4u32,
        > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
    >,
    F: ::grost::__private::flavors::Flavor + ?::core::marker::Sized,
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
    emails: <Vec<
        String,
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
    Vec<
        String,
    >: ::grost::__private::selection::Selectable<
        F,
        <::grost::__private::reflection::WireFormatReflection<
            User,
            F,
            4u32,
        > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
    >,
    F: ::grost::__private::flavors::Flavor + ?::core::marker::Sized,
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
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<F> ::grost::__private::indexer::Indexable<F> for User {
        type Indexer = UserIndex;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl UserIndex {
        /// Returns the field reflection of the corresponding field.
        #[allow(non_camel_case_types, clippy::type_complexity)]
        #[inline]
        pub const fn reflection<F>(
            &self,
        ) -> &'static ::grost::__private::reflection::ObjectField
        where
            ::grost::__private::reflection::ObjectFieldReflection<
                User,
                ::grost::__private::reflection::ObjectField,
                F,
                2u32,
            >: ::grost::__private::reflection::Reflectable<
                User,
                Reflection = ::grost::__private::reflection::ObjectField,
            >,
            ::grost::__private::reflection::ObjectFieldReflection<
                User,
                ::grost::__private::reflection::ObjectField,
                F,
                3u32,
            >: ::grost::__private::reflection::Reflectable<
                User,
                Reflection = ::grost::__private::reflection::ObjectField,
            >,
            ::grost::__private::reflection::ObjectFieldReflection<
                User,
                ::grost::__private::reflection::ObjectField,
                F,
                4u32,
            >: ::grost::__private::reflection::Reflectable<
                User,
                Reflection = ::grost::__private::reflection::ObjectField,
            >,
        {
            match self {
                Self::Name => {
                    <::grost::__private::reflection::ObjectFieldReflection<
                        User,
                        ::grost::__private::reflection::ObjectField,
                        F,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION
                }
                Self::Age => {
                    <::grost::__private::reflection::ObjectFieldReflection<
                        User,
                        ::grost::__private::reflection::ObjectField,
                        F,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION
                }
                Self::Emails => {
                    <::grost::__private::reflection::ObjectFieldReflection<
                        User,
                        ::grost::__private::reflection::ObjectField,
                        F,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::REFLECTION
                }
            }
        }
    }
    #[automatically_derived]
    impl UserIndex {
        /// The number of variants of this field indexer.
        pub const VARIANTS: ::core::primitive::usize = 3usize;
        /// The first field indexer.
        pub const FIRST: Self = Self::Name;
        /// The last field indexer.
        pub const LAST: Self = Self::Emails;
        /// Returns the next field indexer.
        ///
        /// Returns `None` if there are no more fields.
        #[inline]
        pub const fn next(&self) -> ::core::option::Option<Self> {
            match self {
                Self::Emails => ::core::option::Option::None,
                Self::Name => ::core::option::Option::Some(Self::Age),
                Self::Age => ::core::option::Option::Some(Self::Emails),
            }
        }
        /// Returns the previous field indexer.
        ///
        /// Returns `None` if there are no previous fields.
        #[inline]
        pub const fn prev(&self) -> ::core::option::Option<Self> {
            match self {
                Self::Name => ::core::option::Option::None,
                Self::Emails => ::core::option::Option::Some(Self::Age),
                Self::Age => ::core::option::Option::Some(Self::Name),
            }
        }
        /// Returns the remaining number of fields.
        #[inline]
        pub const fn remaining(&self) -> ::core::primitive::usize {
            Self::LAST.index() - self.index()
        }
        const fn index(&self) -> ::core::primitive::usize {
            match self {
                Self::Name => 0usize,
                Self::Age => 1usize,
                Self::Emails => 2usize,
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
    #[allow(non_camel_case_types)]
    impl<__GROST_UNKNOWN_BUFFER__> ::core::default::Default
    for PartialUser<__GROST_UNKNOWN_BUFFER__> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        __GROST_UNKNOWN_BUFFER__,
        __GROST_FLATTEN_STATE__: ?::core::marker::Sized,
    > ::grost::__private::convert::State<
        ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
    > for PartialUser<__GROST_UNKNOWN_BUFFER__> {
        type Output = Self;
        type Input = Self;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<__GROST_UNKNOWN_BUFFER__> PartialUser<__GROST_UNKNOWN_BUFFER__> {
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
        /// Returns a reference to the `name`
        #[inline]
        const fn name_ref(
            &self,
        ) -> ::core::option::Option<
            &<String as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
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
                ::grost::__private::convert::Flatten,
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
            ::grost::__private::convert::Flatten,
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
            ::grost::__private::convert::Flatten,
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
                ::grost::__private::convert::Flatten,
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
            &<u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
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
                ::grost::__private::convert::Flatten,
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
            ::grost::__private::convert::Flatten,
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
            ::grost::__private::convert::Flatten,
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
                ::grost::__private::convert::Flatten,
            >>::Output,
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
                <u8 as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
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
                <u8 as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
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
            &mut <Vec<
                String,
            > as ::grost::__private::convert::State<
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
        ) -> &<Vec<
            String,
        > as ::grost::__private::convert::State<
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
        ) -> &mut <Vec<
            String,
        > as ::grost::__private::convert::State<
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
            <Vec<
                String,
            > as ::grost::__private::convert::State<
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
            value: <Vec<
                String,
            > as ::grost::__private::convert::State<
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
                <Vec<
                    String,
                > as ::grost::__private::convert::State<
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
            value: <Vec<
                String,
            > as ::grost::__private::convert::State<
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
                <Vec<
                    String,
                > as ::grost::__private::convert::State<
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
    impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User,
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
    > {
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
    > {
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
    > {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag;
        const REFLECTION: &Self::Reflection = &{
            (::grost::__private::flavors::network::Tag::new)(2u32)
        };
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
    > {
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
    > {
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
    impl<F> ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::ObjectFieldReflection<
        User,
        ::grost::__private::reflection::ObjectField,
        F,
        3u32,
    >
    where
        F: ?::core::marker::Sized,
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
    impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User,
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
    > {
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
    > {
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
    > {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag;
        const REFLECTION: &Self::Reflection = &{
            (::grost::__private::flavors::network::Tag::new)(3u32)
        };
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
    > {
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
    > {
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
    impl<F> ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::ObjectFieldReflection<
        User,
        ::grost::__private::reflection::ObjectField,
        F,
        4u32,
    >
    where
        F: ?::core::marker::Sized,
    {
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
    impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::WireFormatReflection<
        User,
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
    impl ::grost::__private::reflection::Reflectable<User>
    for ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            User,
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
    > {
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
    > {
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
    > {
        type Reflection = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Tag;
        const REFLECTION: &Self::Reflection = &{
            (::grost::__private::flavors::network::Tag::new)(4u32)
        };
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
    > {
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
    > {
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
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<User> for User {
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
                                Vec<String>,
                            > as ::grost::__private::reflection::Reflectable<
                                Vec<String>,
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
                            Vec<String>,
                        > as ::grost::__private::reflection::Reflectable<
                            Vec<String>,
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
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for User {
        type Selector = UserSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        __GROST_UNKNOWN_BUFFER__,
    > ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for PartialUser<__GROST_UNKNOWN_BUFFER__> {
        type Selector = UserSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        __GROST_UNKNOWN_BUFFER__,
    > ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    >
    for PartialDecodedUser<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        __GROST_UNKNOWN_BUFFER__,
    > {
        type Selector = UserSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<F> ::core::fmt::Debug for UserSelector<F>
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
        Vec<
            String,
        >: ::grost::__private::selection::Selectable<
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                4u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >,
        F: ::grost::__private::flavors::Flavor + ?::core::marker::Sized,
    {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            f.debug_struct("UserSelector")
                .field("name", &self.name)
                .field("age", &self.age)
                .field("emails", &self.emails)
                .finish()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<F> ::core::cmp::PartialEq for UserSelector<F>
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
        Vec<
            String,
        >: ::grost::__private::selection::Selectable<
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                4u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >,
        F: ::grost::__private::flavors::Flavor + ?::core::marker::Sized,
    {
        fn eq(&self, other: &Self) -> ::core::primitive::bool {
            self.name == other.name && self.age == other.age
                && self.emails == other.emails
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<F> ::core::cmp::Eq for UserSelector<F>
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
        Vec<
            String,
        >: ::grost::__private::selection::Selectable<
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                4u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >,
        F: ::grost::__private::flavors::Flavor + ?::core::marker::Sized,
    {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<F> ::core::hash::Hash for UserSelector<F>
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
        Vec<
            String,
        >: ::grost::__private::selection::Selectable<
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                4u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >,
        F: ::grost::__private::flavors::Flavor + ?::core::marker::Sized,
    {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
            self.name.hash(state);
            self.age.hash(state);
            self.emails.hash(state);
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<F> ::core::clone::Clone for UserSelector<F>
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
        Vec<
            String,
        >: ::grost::__private::selection::Selectable<
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                4u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >,
        F: ::grost::__private::flavors::Flavor + ?::core::marker::Sized,
    {
        fn clone(&self) -> Self {
            Self {
                name: ::core::clone::Clone::clone(&self.name),
                age: ::core::clone::Clone::clone(&self.age),
                emails: ::core::clone::Clone::clone(&self.emails),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<F> ::core::marker::Copy for UserSelector<F>
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
        Vec<
            String,
        >: ::grost::__private::selection::Selectable<
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                4u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >,
        F: ::grost::__private::flavors::Flavor + ?::core::marker::Sized,
        <String as ::grost::__private::selection::Selectable<
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >>::Selector: ::core::marker::Copy,
        <u8 as ::grost::__private::selection::Selectable<
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >>::Selector: ::core::marker::Copy,
        <Vec<
            String,
        > as ::grost::__private::selection::Selectable<
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                4u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >>::Selector: ::core::marker::Copy,
    {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<F> ::grost::__private::selection::Selector<F> for UserSelector<F>
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
        Vec<
            String,
        >: ::grost::__private::selection::Selectable<
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                4u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >,
        F: ::grost::__private::flavors::Flavor + ?::core::marker::Sized,
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
            <<String as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                F,
            >>::flip(&mut self.name);
            <<u8 as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                F,
            >>::flip(&mut self.age);
            <<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                F,
            >>::flip(&mut self.emails);
            self
        }
        fn merge(&mut self, other: Self) -> &mut Self {
            <<String as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                F,
            >>::merge(&mut self.name, other.name);
            <<u8 as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                F,
            >>::merge(&mut self.age, other.age);
            <<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                F,
            >>::merge(&mut self.emails, other.emails);
            self
        }
    }
    #[automatically_derived]
    impl<F> UserSelector<F>
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
        Vec<
            String,
        >: ::grost::__private::selection::Selectable<
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                4u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >,
        F: ::grost::__private::flavors::Flavor + ?::core::marker::Sized,
    {
        /// Returns a selector with the default values.
        #[inline]
        pub const fn new() -> Self {
            Self {
                name: <<String as ::grost::__private::selection::Selectable<
                    F,
                    <::grost::__private::reflection::WireFormatReflection<
                        User,
                        F,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<F>>::DEFAULT,
                age: <<u8 as ::grost::__private::selection::Selectable<
                    F,
                    <::grost::__private::reflection::WireFormatReflection<
                        User,
                        F,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<F>>::DEFAULT,
                emails: <<Vec<
                    String,
                > as ::grost::__private::selection::Selectable<
                    F,
                    <::grost::__private::reflection::WireFormatReflection<
                        User,
                        F,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<F>>::DEFAULT,
            }
        }
        /// Returns a selector which selects nothing.
        #[inline]
        pub const fn empty() -> Self {
            Self {
                name: <<String as ::grost::__private::selection::Selectable<
                    F,
                    <::grost::__private::reflection::WireFormatReflection<
                        User,
                        F,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<F>>::NONE,
                age: <<u8 as ::grost::__private::selection::Selectable<
                    F,
                    <::grost::__private::reflection::WireFormatReflection<
                        User,
                        F,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<F>>::NONE,
                emails: <<Vec<
                    String,
                > as ::grost::__private::selection::Selectable<
                    F,
                    <::grost::__private::reflection::WireFormatReflection<
                        User,
                        F,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<F>>::NONE,
            }
        }
        /// Returns a selector which selects all.
        #[inline]
        pub const fn all() -> Self {
            Self {
                name: <<String as ::grost::__private::selection::Selectable<
                    F,
                    <::grost::__private::reflection::WireFormatReflection<
                        User,
                        F,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<F>>::ALL,
                age: <<u8 as ::grost::__private::selection::Selectable<
                    F,
                    <::grost::__private::reflection::WireFormatReflection<
                        User,
                        F,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<F>>::ALL,
                emails: <<Vec<
                    String,
                > as ::grost::__private::selection::Selectable<
                    F,
                    <::grost::__private::reflection::WireFormatReflection<
                        User,
                        F,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<F>>::ALL,
            }
        }
        /// Returns `true` if the selector selects nothing.
        #[inline]
        pub fn is_empty(&self) -> ::core::primitive::bool {
            <<String as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                F,
            >>::is_empty(&self.name)
                && <<u8 as ::grost::__private::selection::Selectable<
                    F,
                    <::grost::__private::reflection::WireFormatReflection<
                        User,
                        F,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    F,
                >>::is_empty(&self.age)
                && <<Vec<
                    String,
                > as ::grost::__private::selection::Selectable<
                    F,
                    <::grost::__private::reflection::WireFormatReflection<
                        User,
                        F,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    F,
                >>::is_empty(&self.emails)
        }
        /// Returns `true` if the selector selects all.
        #[inline]
        pub fn is_all(&self) -> ::core::primitive::bool {
            <<String as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                F,
            >>::is_all(&self.name)
                && <<u8 as ::grost::__private::selection::Selectable<
                    F,
                    <::grost::__private::reflection::WireFormatReflection<
                        User,
                        F,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    F,
                >>::is_all(&self.age)
                && <<Vec<
                    String,
                > as ::grost::__private::selection::Selectable<
                    F,
                    <::grost::__private::reflection::WireFormatReflection<
                        User,
                        F,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    F,
                >>::is_all(&self.emails)
        }
        /// Returns the number of selected fields.
        #[inline]
        pub fn selected(&self) -> ::core::primitive::usize {
            let mut num = 0;
            if self.is_name_selected() {
                num += 1;
            }
            if self.is_age_selected() {
                num += 1;
            }
            if self.is_emails_selected() {
                num += 1;
            }
            num
        }
        /// Returns the number of unselected fields.
        #[inline]
        pub fn unselected(&self) -> ::core::primitive::usize {
            let mut num = 0;
            if self.is_name_unselected() {
                num += 1;
            }
            if self.is_age_unselected() {
                num += 1;
            }
            if self.is_emails_unselected() {
                num += 1;
            }
            num
        }
        /// Returns an iterator over the selected fields.
        #[inline]
        pub fn iter_selected<'__grost_lifetime__>(
            &'__grost_lifetime__ self,
        ) -> UserSelectorIter<'__grost_lifetime__, F, true> {
            UserSelectorIter::new(self, self.selected())
        }
        /// Returns an iterator over the unselected fields.
        #[inline]
        pub fn iter_unselected<'__grost_lifetime__>(
            &'__grost_lifetime__ self,
        ) -> UserSelectorIter<'__grost_lifetime__, F, false> {
            UserSelectorIter::new(self, self.unselected())
        }
        /// Returns `true` if such field is selected.
        #[inline]
        pub fn is_selected(&self, idx: UserIndex) -> ::core::primitive::bool {
            match idx {
                UserIndex::Name => self.is_name_selected(),
                UserIndex::Age => self.is_age_selected(),
                UserIndex::Emails => self.is_emails_selected(),
            }
        }
        /// Returns `true` if such field is unselected.
        #[inline]
        pub fn is_unselected(&self, idx: UserIndex) -> ::core::primitive::bool {
            match idx {
                UserIndex::Name => self.is_name_unselected(),
                UserIndex::Age => self.is_age_unselected(),
                UserIndex::Emails => self.is_emails_unselected(),
            }
        }
        /// Select the `User.name` field
        #[inline]
        pub fn select_name(&mut self) -> &mut Self {
            self
                .name = <<String as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<F>>::DEFAULT;
            self
        }
        /// Unselect the `User.name` field
        #[inline]
        pub fn unselect_name(&mut self) -> &mut Self {
            self
                .name = <<String as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<F>>::NONE;
            self
        }
        /// Update the `User.name` field
        #[inline]
        pub fn update_name(
            &mut self,
            value: <String as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
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
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
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
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >>::Selector {
            &self.name
        }
        /// Get a mutable reference to the selector of `User.name` field
        #[inline]
        pub const fn name_mut(
            &mut self,
        ) -> &mut <String as ::grost::__private::selection::Selectable<
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >>::Selector {
            &mut self.name
        }
        /// Set the `User.name` field
        #[inline]
        pub fn with_name(mut self) -> Self {
            self
                .name = <<String as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<F>>::DEFAULT;
            self
        }
        /// Unset the `User.name` field
        #[inline]
        pub fn without_name(mut self) -> Self {
            self
                .name = <<String as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<F>>::NONE;
            self
        }
        /// Returns `true` if the `User.name` field is selected
        #[inline]
        pub fn is_name_selected(&self) -> ::core::primitive::bool {
            !<<String as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                F,
            >>::is_empty(&self.name)
        }
        /// Returns `true` if the `User.name` field is unselected
        #[inline]
        pub fn is_name_unselected(&self) -> ::core::primitive::bool {
            <<String as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                F,
            >>::is_empty(&self.name)
        }
        /// Select the `User.age` field
        #[inline]
        pub fn select_age(&mut self) -> &mut Self {
            self
                .age = <<u8 as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<F>>::DEFAULT;
            self
        }
        /// Unselect the `User.age` field
        #[inline]
        pub fn unselect_age(&mut self) -> &mut Self {
            self
                .age = <<u8 as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<F>>::NONE;
            self
        }
        /// Update the `User.age` field
        #[inline]
        pub fn update_age(
            &mut self,
            value: <u8 as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
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
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
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
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >>::Selector {
            &self.age
        }
        /// Get a mutable reference to the selector of `User.age` field
        #[inline]
        pub const fn age_mut(
            &mut self,
        ) -> &mut <u8 as ::grost::__private::selection::Selectable<
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >>::Selector {
            &mut self.age
        }
        /// Set the `User.age` field
        #[inline]
        pub fn with_age(mut self) -> Self {
            self
                .age = <<u8 as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<F>>::DEFAULT;
            self
        }
        /// Unset the `User.age` field
        #[inline]
        pub fn without_age(mut self) -> Self {
            self
                .age = <<u8 as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<F>>::NONE;
            self
        }
        /// Returns `true` if the `User.age` field is selected
        #[inline]
        pub fn is_age_selected(&self) -> ::core::primitive::bool {
            !<<u8 as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                F,
            >>::is_empty(&self.age)
        }
        /// Returns `true` if the `User.age` field is unselected
        #[inline]
        pub fn is_age_unselected(&self) -> ::core::primitive::bool {
            <<u8 as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                F,
            >>::is_empty(&self.age)
        }
        /// Select the `User.emails` field
        #[inline]
        pub fn select_emails(&mut self) -> &mut Self {
            self
                .emails = <<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<F>>::DEFAULT;
            self
        }
        /// Unselect the `User.emails` field
        #[inline]
        pub fn unselect_emails(&mut self) -> &mut Self {
            self
                .emails = <<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<F>>::NONE;
            self
        }
        /// Update the `User.emails` field
        #[inline]
        pub fn update_emails(
            &mut self,
            value: <Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
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
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
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
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                4u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
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
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                4u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
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
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<F>>::DEFAULT;
            self
        }
        /// Unset the `User.emails` field
        #[inline]
        pub fn without_emails(mut self) -> Self {
            self
                .emails = <<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<F>>::NONE;
            self
        }
        /// Returns `true` if the `User.emails` field is selected
        #[inline]
        pub fn is_emails_selected(&self) -> ::core::primitive::bool {
            !<<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                F,
            >>::is_empty(&self.emails)
        }
        /// Returns `true` if the `User.emails` field is unselected
        #[inline]
        pub fn is_emails_unselected(&self) -> ::core::primitive::bool {
            <<Vec<
                String,
            > as ::grost::__private::selection::Selectable<
                F,
                <::grost::__private::reflection::WireFormatReflection<
                    User,
                    F,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                F,
            >>::is_empty(&self.emails)
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        F,
        const __GROST_SELECTED__: ::core::primitive::bool,
    > UserSelectorIter<'__grost_lifetime__, F, __GROST_SELECTED__>
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
        Vec<
            String,
        >: ::grost::__private::selection::Selectable<
            F,
            <::grost::__private::reflection::WireFormatReflection<
                User,
                F,
                4u32,
            > as ::grost::__private::reflection::Reflectable<User>>::Reflection,
        >,
        F: ::grost::__private::flavors::Flavor + ?::core::marker::Sized,
    {
        #[inline]
        const fn new(
            selector: &'__grost_lifetime__ UserSelector<F>,
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
        start_line: 162usize,
        start_col: 4usize,
        end_line: 162usize,
        end_col: 5usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(t())),
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
