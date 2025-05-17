#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use grost_derive::Object;
mod sealed {
    pub fn default_user() -> String {
        String::from("user")
    }
}
pub struct User<I> {
    #[grost(
        tag = 1,
        schema(description = "The id of the user"),
        wire = "grost::flavors::network::LengthDelimited",
        selector(select = "all"),
        partial_decoded(copy)
    )]
    id: I,
    #[grost(
        tag = 2,
        schema(description = "The nick name of the user"),
        wire = "grost::flavors::network::LengthDelimited",
        selector(select = "all"),
        partial_decoded(copy),
        default = sealed::default_user,
    )]
    name: String,
    #[grost(
        tag = 3,
        schema(description = "The age of the user"),
        wire = "grost::flavors::network::Varint",
        copy,
        partial_decoded(copy)
    )]
    age: u8,
    #[grost(
        tag = 4,
        schema(description = "The email of the user"),
        wire = "grost::flavors::network::LengthDelimited",
        partial_decoded(copy),
        optional(repeated)
    )]
    emails: Option<Vec<String>>,
}
#[automatically_derived]
impl<I: ::core::fmt::Debug> ::core::fmt::Debug for User<I> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "User",
            "id",
            &self.id,
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
impl<I: ::core::clone::Clone> ::core::clone::Clone for User<I> {
    #[inline]
    fn clone(&self) -> User<I> {
        User {
            id: ::core::clone::Clone::clone(&self.id),
            name: ::core::clone::Clone::clone(&self.name),
            age: ::core::clone::Clone::clone(&self.age),
            emails: ::core::clone::Clone::clone(&self.emails),
        }
    }
}
#[automatically_derived]
impl<I> ::core::marker::StructuralPartialEq for User<I> {}
#[automatically_derived]
impl<I: ::core::cmp::PartialEq> ::core::cmp::PartialEq for User<I> {
    #[inline]
    fn eq(&self, other: &User<I>) -> bool {
        self.id == other.id && self.name == other.name && self.age == other.age
            && self.emails == other.emails
    }
}
#[automatically_derived]
impl<I: ::core::cmp::Eq> ::core::cmp::Eq for User<I> {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<I>;
        let _: ::core::cmp::AssertParamIsEq<String>;
        let _: ::core::cmp::AssertParamIsEq<u8>;
        let _: ::core::cmp::AssertParamIsEq<Option<Vec<String>>>;
    }
}
/// The reflection of the [`User`].
pub struct UserReflection<R: ?::core::marker::Sized, F: ?::core::marker::Sized> {
    _reflect: ::core::marker::PhantomData<R>,
    _flavor: ::core::marker::PhantomData<F>,
}
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct PartialUser<I>
where
    I: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
    <I as ::grost::__private::convert::State<
        ::grost::__private::convert::Flatten,
    >>::Output: ::core::marker::Sized,
{
    id: ::core::option::Option<
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Flatten,
        >>::Output,
    >,
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
        <Option<
            Vec<String>,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Flatten,
        >>::Output,
    >,
}
/// Partial reference struct for the struct [`User`]
#[allow(clippy::type_complexity, non_camel_case_types)]
pub struct PartialDecodedUser<
    '__grost_lifetime__,
    I,
    __GROST_FLAVOR__: ?::core::marker::Sized,
    __GROST_UNKNOWN_BUFFER__,
>
where
    UserReflection<
        (
            ::grost::__private::reflection::WireFormatReflection,
            ::grost::__private::RawTag<1u32>,
        ),
        __GROST_FLAVOR__,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    I: ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >,
    <I as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >>::Output: ::core::marker::Sized + ::core::marker::Copy,
    UserReflection<
        (
            ::grost::__private::reflection::WireFormatReflection,
            ::grost::__private::RawTag<2u32>,
        ),
        __GROST_FLAVOR__,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    String: ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >,
    <String as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >>::Output: ::core::marker::Sized + ::core::marker::Copy,
    UserReflection<
        (
            ::grost::__private::reflection::WireFormatReflection,
            ::grost::__private::RawTag<3u32>,
        ),
        __GROST_FLAVOR__,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    u8: ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >,
    <u8 as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >>::Output: ::core::marker::Sized + ::core::marker::Copy,
    UserReflection<
        (
            ::grost::__private::reflection::WireFormatReflection,
            ::grost::__private::RawTag<4u32>,
        ),
        __GROST_FLAVOR__,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    Option<
        Vec<String>,
    >: ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >,
    <Option<
        Vec<String>,
    > as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >>::Output: ::core::marker::Sized + ::core::marker::Copy,
{
    __grost_unknown_buffer__: ::core::option::Option<__GROST_UNKNOWN_BUFFER__>,
    id: ::core::option::Option<
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output,
    >,
    name: ::core::option::Option<
        <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output,
    >,
    age: ::core::option::Option<
        <u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output,
    >,
    emails: ::core::option::Option<
        <Option<
            Vec<String>,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output,
    >,
}
/// Field indexer for the struct [`User`]
#[repr(u32)]
pub enum UserIndex {
    /// The field indexer for the field `id`
    Id = 0u32,
    /// The field indexer for the field `name`
    Name = 1u32,
    /// The field indexer for the field `age`
    Age = 2u32,
    /// The field indexer for the field `emails`
    Emails = 3u32,
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
            },
        )
    }
}
/// The selection type for [`User`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct UserSelector<I, __GROST_FLAVOR__: ?::core::marker::Sized>
where
    UserReflection<
        (
            ::grost::__private::reflection::WireFormatReflection,
            ::grost::__private::RawTag<1u32>,
        ),
        __GROST_FLAVOR__,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    I: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    UserReflection<
        (
            ::grost::__private::reflection::WireFormatReflection,
            ::grost::__private::RawTag<2u32>,
        ),
        __GROST_FLAVOR__,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    String: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    UserReflection<
        (
            ::grost::__private::reflection::WireFormatReflection,
            ::grost::__private::RawTag<3u32>,
        ),
        __GROST_FLAVOR__,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    u8: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    UserReflection<
        (
            ::grost::__private::reflection::WireFormatReflection,
            ::grost::__private::RawTag<4u32>,
        ),
        __GROST_FLAVOR__,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    Option<
        Vec<String>,
    >: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
{
    id: <I as ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >>::Selector,
    name: <String as ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >>::Selector,
    age: <u8 as ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >>::Selector,
    emails: <Option<
        Vec<String>,
    > as ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >>::Selector,
}
/// An iterator over the selected fields of the [`UserSelector`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct UserSelectorIter<
    '__grost_lifetime__,
    I,
    __GROST_FLAVOR__: ?::core::marker::Sized,
    const __GROST_SELECTED__: ::core::primitive::bool = true,
>
where
    UserReflection<
        (
            ::grost::__private::reflection::WireFormatReflection,
            ::grost::__private::RawTag<1u32>,
        ),
        __GROST_FLAVOR__,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    I: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    UserReflection<
        (
            ::grost::__private::reflection::WireFormatReflection,
            ::grost::__private::RawTag<2u32>,
        ),
        __GROST_FLAVOR__,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    String: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    UserReflection<
        (
            ::grost::__private::reflection::WireFormatReflection,
            ::grost::__private::RawTag<3u32>,
        ),
        __GROST_FLAVOR__,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    u8: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    UserReflection<
        (
            ::grost::__private::reflection::WireFormatReflection,
            ::grost::__private::RawTag<4u32>,
        ),
        __GROST_FLAVOR__,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    Option<
        Vec<String>,
    >: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
{
    selector: &'__grost_lifetime__ UserSelector<I, __GROST_FLAVOR__>,
    index: ::core::option::Option<UserIndex>,
    num: ::core::primitive::usize,
    yielded: ::core::primitive::usize,
}
const _: () = {
    const _: () = {
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized,
        {
            /// Returns the relection to the wire format of the field.
            #[inline]
            pub const fn wire_format(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::TagReflection<
                    <F as ::grost::__private::flavors::Flavor>::Tag,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the relection to a tag of the field.
            #[inline]
            pub const fn tag(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::EncodedTagReflection<
                    <F as ::grost::__private::flavors::Flavor>::Tag,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the relection to the encoded tag of the field.
            #[inline]
            pub const fn encoded_tag(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::Len<
                    ::grost::__private::reflection::EncodedTagReflection<
                        <F as ::grost::__private::flavors::Flavor>::Tag,
                    >,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns length of the relection to the encoded tag of the field.
            #[inline]
            pub const fn encoded_tag_len(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::WireTypeReflection<
                    <F as ::grost::__private::flavors::Flavor>::WireType,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the relection to the wire type of the field.
            #[inline]
            pub const fn wire_type(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::IdentifierReflection<
                    <F as ::grost::__private::flavors::Flavor>::Identifier,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the relection to the identifier of the field.
            #[inline]
            pub const fn identifier(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::EncodedIdentifierReflection<
                    <F as ::grost::__private::flavors::Flavor>::Identifier,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the relection to the encoded identifier of the field.
            #[inline]
            pub const fn encoded_identifier(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::Len<
                    ::grost::__private::reflection::EncodedIdentifierReflection<
                        <F as ::grost::__private::flavors::Flavor>::Identifier,
                    >,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the relection to the encoded identifier of the field.
            #[inline]
            pub const fn encoded_identifier_len(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::encode::EncodeReflection<
                    ::grost::__private::reflection::encode::EncodeField,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the reflection to the encode fn.
            #[inline]
            pub const fn encode(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::encode::EncodeReflection<
                    ::grost::__private::reflection::Len<
                        ::grost::__private::reflection::encode::EncodeField,
                    >,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the reflection to fn which will give the length of the encoded data.
            #[inline]
            pub const fn encoded_len(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::encode::EncodeReflection<
                    ::grost::__private::reflection::encode::EncodeRefField,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the reflection to the reference encode fn.
            #[inline]
            pub const fn encode_ref(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::encode::EncodeReflection<
                    ::grost::__private::reflection::Len<
                        ::grost::__private::reflection::encode::EncodeRefField,
                    >,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the reflection to the reference encode fn which will give the length of the encoded data.
            #[inline]
            pub const fn encoded_ref_len(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::encode::EncodeReflection<
                    ::grost::__private::reflection::encode::PartialEncodeField,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the reflection to the partial encode fn.
            #[inline]
            pub const fn partial_encode(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::encode::EncodeReflection<
                    ::grost::__private::reflection::Len<
                        ::grost::__private::reflection::encode::PartialEncodeField,
                    >,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the reflection to the partial encode fn which will give the length of the encoded data.
            #[inline]
            pub const fn partial_encoded_len(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::encode::EncodeReflection<
                    ::grost::__private::reflection::encode::PartialEncodeRefField,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the reflection to the partial reference encode fn.
            #[inline]
            pub const fn partial_encode_ref(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<
            F,
            const TAG: ::core::primitive::u32,
        > UserReflection<
            (
                ::grost::__private::reflection::encode::EncodeReflection<
                    ::grost::__private::reflection::Len<
                        ::grost::__private::reflection::encode::PartialEncodeRefField,
                    >,
                >,
                ::grost::__private::RawTag<TAG>,
            ),
            F,
        >
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the reflection to the partial reference encode fn which will give the length of the encoded data.
            #[inline]
            pub const fn partial_encoded_ref_len(&self) -> Self {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        impl<R, F> ::core::ops::Deref for UserReflection<R, F>
        where
            R: ?::core::marker::Sized,
            F: ?::core::marker::Sized,
            Self: ::grost::__private::reflection::Reflectable<F>,
        {
            type Target = <Self as ::grost::__private::reflection::Reflectable<
                F,
            >>::Reflection;
            fn deref(&self) -> &Self::Target {
                <Self as ::grost::__private::reflection::Reflectable<F>>::REFLECTION
            }
        }
        #[automatically_derived]
        impl<R, F> ::core::convert::AsRef<<Self as ::core::ops::Deref>::Target>
        for UserReflection<R, F>
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
        impl<R, F> ::core::fmt::Debug for UserReflection<R, F>
        where
            R: ?::core::marker::Sized,
            F: ?::core::marker::Sized,
            Self: ::grost::__private::reflection::Reflectable<F>,
            <Self as ::grost::__private::reflection::Reflectable<
                F,
            >>::Reflection: ::core::fmt::Debug,
        {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                ::core::fmt::Debug::fmt(::core::ops::Deref::deref(self), f)
            }
        }
        #[automatically_derived]
        impl<R, F> ::core::fmt::Display for UserReflection<R, F>
        where
            R: ?::core::marker::Sized,
            F: ?::core::marker::Sized,
            Self: ::grost::__private::reflection::Reflectable<F>,
            <Self as ::grost::__private::reflection::Reflectable<
                F,
            >>::Reflection: ::core::fmt::Display,
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
        for UserReflection<R, F> {
            fn clone(&self) -> Self {
                *self
            }
        }
        #[automatically_derived]
        impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::marker::Copy
        for UserReflection<R, F> {}
        #[automatically_derived]
        impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> UserReflection<R, F> {
            const fn new_in() -> Self {
                Self {
                    _reflect: ::core::marker::PhantomData,
                    _flavor: ::core::marker::PhantomData,
                }
            }
        }
        #[automatically_derived]
        impl<F> UserReflection<::grost::__private::reflection::ObjectReflection<F>, F>
        where
            F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        {
            /// Returns the reflection of the struct.
            #[inline]
            const fn new() -> Self {
                Self::new_in()
            }
            /// Returns the field reflection of the field `User.id`.
            #[inline]
            pub const fn id(
                &self,
            ) -> UserReflection<
                (
                    ::grost::__private::reflection::ObjectFieldReflection<F>,
                    ::grost::__private::RawTag<1u32>,
                ),
                F,
            > {
                UserReflection::new_in()
            }
            /// Returns the field reflection of the field `User.name`.
            #[inline]
            pub const fn name(
                &self,
            ) -> UserReflection<
                (
                    ::grost::__private::reflection::ObjectFieldReflection<F>,
                    ::grost::__private::RawTag<2u32>,
                ),
                F,
            > {
                UserReflection::new_in()
            }
            /// Returns the field reflection of the field `User.age`.
            #[inline]
            pub const fn age(
                &self,
            ) -> UserReflection<
                (
                    ::grost::__private::reflection::ObjectFieldReflection<F>,
                    ::grost::__private::RawTag<3u32>,
                ),
                F,
            > {
                UserReflection::new_in()
            }
            /// Returns the field reflection of the field `User.emails`.
            #[inline]
            pub const fn emails(
                &self,
            ) -> UserReflection<
                (
                    ::grost::__private::reflection::ObjectFieldReflection<F>,
                    ::grost::__private::RawTag<4u32>,
                ),
                F,
            > {
                UserReflection::new_in()
            }
        }
        #[automatically_derived]
        impl<I> User<I> {
            /// Returns the reflection of the struct.
            #[allow(non_camel_case_types)]
            #[inline]
            pub const fn reflection<__GROST_FLAVOR__>() -> UserReflection<
                ::grost::__private::reflection::ObjectReflection<__GROST_FLAVOR__>,
                __GROST_FLAVOR__,
            >
            where
                __GROST_FLAVOR__: ?::core::marker::Sized
                    + ::grost::__private::flavors::Flavor,
            {
                UserReflection::new()
            }
        }
    };
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        I,
        __GROST_FLAVOR__: ?::core::marker::Sized,
    > ::grost::__private::indexer::Indexable<__GROST_FLAVOR__> for User<I> {
        type Indexer = UserIndex;
    }
    #[automatically_derived]
    impl UserIndex {
        /// The number of variants of this field indexer.
        pub const VARIANTS: ::core::primitive::usize = 4usize;
        /// The first field indexer.
        pub const FIRST: Self = Self::Id;
        /// The last field indexer.
        pub const LAST: Self = Self::Emails;
        /// Returns the field reflection of the corresponding field.
        #[allow(non_camel_case_types, clippy::type_complexity)]
        #[inline]
        pub const fn reflection<__GROST_FLAVOR__>(
            &self,
        ) -> &'static ::grost::__private::reflection::ObjectFieldReflection<
            __GROST_FLAVOR__,
        >
        where
            __GROST_FLAVOR__: ::grost::__private::flavors::Flavor
                + ?::core::marker::Sized,
            UserReflection<
                (
                    ::grost::__private::reflection::ObjectFieldReflection<
                        __GROST_FLAVOR__,
                    >,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            >: ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
                Reflection = ::grost::__private::reflection::ObjectFieldReflection<
                    __GROST_FLAVOR__,
                >,
            >,
            UserReflection<
                (
                    ::grost::__private::reflection::ObjectFieldReflection<
                        __GROST_FLAVOR__,
                    >,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            >: ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
                Reflection = ::grost::__private::reflection::ObjectFieldReflection<
                    __GROST_FLAVOR__,
                >,
            >,
            UserReflection<
                (
                    ::grost::__private::reflection::ObjectFieldReflection<
                        __GROST_FLAVOR__,
                    >,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            >: ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
                Reflection = ::grost::__private::reflection::ObjectFieldReflection<
                    __GROST_FLAVOR__,
                >,
            >,
            UserReflection<
                (
                    ::grost::__private::reflection::ObjectFieldReflection<
                        __GROST_FLAVOR__,
                    >,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            >: ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
                Reflection = ::grost::__private::reflection::ObjectFieldReflection<
                    __GROST_FLAVOR__,
                >,
            >,
        {
            match self {
                Self::Id => {
                    <UserReflection<
                        (
                            ::grost::__private::reflection::ObjectFieldReflection<
                                __GROST_FLAVOR__,
                            >,
                            ::grost::__private::RawTag<1u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::REFLECTION
                }
                Self::Name => {
                    <UserReflection<
                        (
                            ::grost::__private::reflection::ObjectFieldReflection<
                                __GROST_FLAVOR__,
                            >,
                            ::grost::__private::RawTag<2u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::REFLECTION
                }
                Self::Age => {
                    <UserReflection<
                        (
                            ::grost::__private::reflection::ObjectFieldReflection<
                                __GROST_FLAVOR__,
                            >,
                            ::grost::__private::RawTag<3u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::REFLECTION
                }
                Self::Emails => {
                    <UserReflection<
                        (
                            ::grost::__private::reflection::ObjectFieldReflection<
                                __GROST_FLAVOR__,
                            >,
                            ::grost::__private::RawTag<4u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::REFLECTION
                }
            }
        }
        /// Returns the next field indexer.
        ///
        /// Returns `None` if there are no more fields.
        #[inline]
        pub const fn next(&self) -> ::core::option::Option<Self> {
            match self {
                Self::Emails => ::core::option::Option::None,
                Self::Id => ::core::option::Option::Some(Self::Name),
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
                Self::Id => ::core::option::Option::None,
                Self::Emails => ::core::option::Option::Some(Self::Age),
                Self::Age => ::core::option::Option::Some(Self::Name),
                Self::Name => ::core::option::Option::Some(Self::Id),
            }
        }
        /// Returns the remaining number of fields.
        #[inline]
        pub const fn remaining(&self) -> ::core::primitive::usize {
            Self::LAST as ::core::primitive::usize
                - *self as ::core::primitive::u32 as ::core::primitive::usize
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
    impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> ::core::fmt::Debug
    for UserSelector<I, __GROST_FLAVOR__>
    where
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        I: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u8: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        Option<
            Vec<String>,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
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
                .finish()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> ::core::cmp::PartialEq
    for UserSelector<I, __GROST_FLAVOR__>
    where
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        I: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u8: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        Option<
            Vec<String>,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        fn eq(&self, other: &Self) -> ::core::primitive::bool {
            self.id == other.id && self.name == other.name && self.age == other.age
                && self.emails == other.emails
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> ::core::cmp::Eq
    for UserSelector<I, __GROST_FLAVOR__>
    where
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        I: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u8: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        Option<
            Vec<String>,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> ::core::hash::Hash
    for UserSelector<I, __GROST_FLAVOR__>
    where
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        I: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u8: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        Option<
            Vec<String>,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
            self.id.hash(state);
            self.name.hash(state);
            self.age.hash(state);
            self.emails.hash(state);
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> ::core::clone::Clone
    for UserSelector<I, __GROST_FLAVOR__>
    where
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        I: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u8: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        Option<
            Vec<String>,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        fn clone(&self) -> Self {
            Self {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
                age: ::core::clone::Clone::clone(&self.age),
                emails: ::core::clone::Clone::clone(&self.emails),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> ::core::marker::Copy
    for UserSelector<I, __GROST_FLAVOR__>
    where
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        I: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u8: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        Option<
            Vec<String>,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        <I as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector: ::core::marker::Copy,
        <String as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector: ::core::marker::Copy,
        <u8 as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector: ::core::marker::Copy,
        <Option<
            Vec<String>,
        > as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector: ::core::marker::Copy,
    {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        I,
        __GROST_FLAVOR__: ?::core::marker::Sized,
    > ::grost::__private::Selector<__GROST_FLAVOR__>
    for UserSelector<I, __GROST_FLAVOR__>
    where
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        I: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u8: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        Option<
            Vec<String>,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
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
            <<I as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::flip(&mut self.id);
            <<String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::flip(&mut self.name);
            <<u8 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::flip(&mut self.age);
            <<Option<
                Vec<String>,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::flip(&mut self.emails);
            self
        }
        fn merge(&mut self, other: Self) -> &mut Self {
            <<I as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::merge(&mut self.id, other.id);
            <<String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::merge(&mut self.name, other.name);
            <<u8 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::merge(&mut self.age, other.age);
            <<Option<
                Vec<String>,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::merge(&mut self.emails, other.emails);
            self
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> ::core::default::Default
    for UserSelector<I, __GROST_FLAVOR__>
    where
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        I: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u8: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        Option<
            Vec<String>,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> UserSelector<I, __GROST_FLAVOR__>
    where
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        I: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u8: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        Option<
            Vec<String>,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        /// The number of options in this selection type.
        pub const OPTIONS: ::core::primitive::usize = 4usize;
        /// Returns a selector with the default values.
        #[inline]
        pub const fn new() -> Self {
            Self {
                id: <<I as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<1u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT,
                name: <<String as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<2u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT,
                age: <<u8 as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<3u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT,
                emails: <<Option<
                    Vec<String>,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<4u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT,
            }
        }
        /// Returns a selector which selects nothing.
        #[inline]
        pub const fn empty() -> Self {
            Self {
                id: <<I as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<1u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE,
                name: <<String as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<2u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE,
                age: <<u8 as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<3u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE,
                emails: <<Option<
                    Vec<String>,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<4u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE,
            }
        }
        /// Returns a selector which selects all.
        #[inline]
        pub const fn all() -> Self {
            Self {
                id: <<I as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<1u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::ALL,
                name: <<String as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<2u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::ALL,
                age: <<u8 as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<3u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::ALL,
                emails: <<Option<
                    Vec<String>,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<4u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::ALL,
            }
        }
        /// Returns `true` if the selector selects nothing.
        #[inline]
        pub fn is_empty(&self) -> ::core::primitive::bool {
            <<I as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.id)
                && <<String as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<2u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_empty(&self.name)
                && <<u8 as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<3u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_empty(&self.age)
                && <<Option<
                    Vec<String>,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<4u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_empty(&self.emails)
        }
        /// Returns `true` if the selector selects all.
        #[inline]
        pub fn is_all(&self) -> ::core::primitive::bool {
            <<I as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_all(&self.id)
                && <<String as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<2u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_all(&self.name)
                && <<u8 as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<3u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_all(&self.age)
                && <<Option<
                    Vec<String>,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<4u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_all(&self.emails)
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
            num
        }
        /// Returns an iterator over the selected fields.
        #[inline]
        pub fn iter_selected(&self) -> UserSelectorIter<'_, I, __GROST_FLAVOR__, true> {
            UserSelectorIter::new(self, self.selected())
        }
        /// Returns an iterator over the unselected fields.
        #[inline]
        pub fn iter_unselected(
            &self,
        ) -> UserSelectorIter<'_, I, __GROST_FLAVOR__, false> {
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
            }
        }
        /// Select the `User.id` field
        #[inline]
        pub fn select_id(&mut self) -> &mut Self {
            self
                .id = <<I as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unselect the `User.id` field
        #[inline]
        pub fn unselect_id(&mut self) -> &mut Self {
            self
                .id = <<I as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Update the `User.id` field
        #[inline]
        pub fn update_id(
            &mut self,
            value: <I as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> &mut Self {
            self.id = value;
            self
        }
        /// Set or unset the `User.id` field
        #[inline]
        pub fn maybe_id(
            mut self,
            val: <I as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> Self {
            self.id = val;
            self
        }
        /// Get a reference to the selector of `User.id` field
        #[inline]
        pub const fn id_ref(
            &self,
        ) -> &<I as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &self.id
        }
        /// Get a mutable reference to the selector of `User.id` field
        #[inline]
        pub const fn id_mut(
            &mut self,
        ) -> &mut <I as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &mut self.id
        }
        /// Set the `User.id` field
        #[inline]
        pub fn with_id(mut self) -> Self {
            self
                .id = <<I as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unset the `User.id` field
        #[inline]
        pub fn without_id(mut self) -> Self {
            self
                .id = <<I as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Returns `true` if the `User.id` field is selected
        #[inline]
        pub fn is_id_selected(&self) -> ::core::primitive::bool {
            !<<I as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.id)
        }
        /// Returns `true` if the `User.id` field is unselected
        #[inline]
        pub fn is_id_unselected(&self) -> ::core::primitive::bool {
            <<I as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.id)
        }
        /// Select the `User.name` field
        #[inline]
        pub fn select_name(&mut self) -> &mut Self {
            self
                .name = <<String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unselect the `User.name` field
        #[inline]
        pub fn unselect_name(&mut self) -> &mut Self {
            self
                .name = <<String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Update the `User.name` field
        #[inline]
        pub fn update_name(
            &mut self,
            value: <String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> &mut Self {
            self.name = value;
            self
        }
        /// Set or unset the `User.name` field
        #[inline]
        pub fn maybe_name(
            mut self,
            val: <String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> Self {
            self.name = val;
            self
        }
        /// Get a reference to the selector of `User.name` field
        #[inline]
        pub const fn name_ref(
            &self,
        ) -> &<String as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &self.name
        }
        /// Get a mutable reference to the selector of `User.name` field
        #[inline]
        pub const fn name_mut(
            &mut self,
        ) -> &mut <String as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &mut self.name
        }
        /// Set the `User.name` field
        #[inline]
        pub fn with_name(mut self) -> Self {
            self
                .name = <<String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unset the `User.name` field
        #[inline]
        pub fn without_name(mut self) -> Self {
            self
                .name = <<String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Returns `true` if the `User.name` field is selected
        #[inline]
        pub fn is_name_selected(&self) -> ::core::primitive::bool {
            !<<String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.name)
        }
        /// Returns `true` if the `User.name` field is unselected
        #[inline]
        pub fn is_name_unselected(&self) -> ::core::primitive::bool {
            <<String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.name)
        }
        /// Select the `User.age` field
        #[inline]
        pub fn select_age(&mut self) -> &mut Self {
            self
                .age = <<u8 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unselect the `User.age` field
        #[inline]
        pub fn unselect_age(&mut self) -> &mut Self {
            self
                .age = <<u8 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Update the `User.age` field
        #[inline]
        pub fn update_age(
            &mut self,
            value: <u8 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> &mut Self {
            self.age = value;
            self
        }
        /// Set or unset the `User.age` field
        #[inline]
        pub fn maybe_age(
            mut self,
            val: <u8 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> Self {
            self.age = val;
            self
        }
        /// Get a reference to the selector of `User.age` field
        #[inline]
        pub const fn age_ref(
            &self,
        ) -> &<u8 as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &self.age
        }
        /// Get a mutable reference to the selector of `User.age` field
        #[inline]
        pub const fn age_mut(
            &mut self,
        ) -> &mut <u8 as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &mut self.age
        }
        /// Set the `User.age` field
        #[inline]
        pub fn with_age(mut self) -> Self {
            self
                .age = <<u8 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unset the `User.age` field
        #[inline]
        pub fn without_age(mut self) -> Self {
            self
                .age = <<u8 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Returns `true` if the `User.age` field is selected
        #[inline]
        pub fn is_age_selected(&self) -> ::core::primitive::bool {
            !<<u8 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.age)
        }
        /// Returns `true` if the `User.age` field is unselected
        #[inline]
        pub fn is_age_unselected(&self) -> ::core::primitive::bool {
            <<u8 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.age)
        }
        /// Select the `User.emails` field
        #[inline]
        pub fn select_emails(&mut self) -> &mut Self {
            self
                .emails = <<Option<
                Vec<String>,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unselect the `User.emails` field
        #[inline]
        pub fn unselect_emails(&mut self) -> &mut Self {
            self
                .emails = <<Option<
                Vec<String>,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Update the `User.emails` field
        #[inline]
        pub fn update_emails(
            &mut self,
            value: <Option<
                Vec<String>,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> &mut Self {
            self.emails = value;
            self
        }
        /// Set or unset the `User.emails` field
        #[inline]
        pub fn maybe_emails(
            mut self,
            val: <Option<
                Vec<String>,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> Self {
            self.emails = val;
            self
        }
        /// Get a reference to the selector of `User.emails` field
        #[inline]
        pub const fn emails_ref(
            &self,
        ) -> &<Option<
            Vec<String>,
        > as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &self.emails
        }
        /// Get a mutable reference to the selector of `User.emails` field
        #[inline]
        pub const fn emails_mut(
            &mut self,
        ) -> &mut <Option<
            Vec<String>,
        > as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &mut self.emails
        }
        /// Set the `User.emails` field
        #[inline]
        pub fn with_emails(mut self) -> Self {
            self
                .emails = <<Option<
                Vec<String>,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unset the `User.emails` field
        #[inline]
        pub fn without_emails(mut self) -> Self {
            self
                .emails = <<Option<
                Vec<String>,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Returns `true` if the `User.emails` field is selected
        #[inline]
        pub fn is_emails_selected(&self) -> ::core::primitive::bool {
            !<<Option<
                Vec<String>,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.emails)
        }
        /// Returns `true` if the `User.emails` field is unselected
        #[inline]
        pub fn is_emails_unselected(&self) -> ::core::primitive::bool {
            <<Option<
                Vec<String>,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.emails)
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I,
        __GROST_FLAVOR__: ?::core::marker::Sized,
        const __GROST_SELECTED__: ::core::primitive::bool,
    > UserSelectorIter<'__grost_lifetime__, I, __GROST_FLAVOR__, __GROST_SELECTED__>
    where
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        I: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<1u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<2u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u8: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<3u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        Option<
            Vec<String>,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserReflection<
                (
                    ::grost::__private::reflection::WireFormatReflection,
                    ::grost::__private::RawTag<4u32>,
                ),
                __GROST_FLAVOR__,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        #[inline]
        const fn new(
            selector: &'__grost_lifetime__ UserSelector<I, __GROST_FLAVOR__>,
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
        I,
        __GROST_FLAVOR__: ?::core::marker::Sized,
        __GROST_UNKNOWN_BUFFER__,
    > ::core::default::Default
    for PartialDecodedUser<
        '__grost_lifetime__,
        I,
        __GROST_FLAVOR__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        String: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u8: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        Option<
            Vec<String>,
        >: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
    {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I,
        __GROST_FLAVOR__: ?::core::marker::Sized,
        __GROST_UNKNOWN_BUFFER__,
        __GROST_FLATTEN_STATE__: ?::core::marker::Sized,
    > ::grost::__private::convert::State<
        ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
    >
    for PartialDecodedUser<
        '__grost_lifetime__,
        I,
        __GROST_FLAVOR__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        String: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u8: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        Option<
            Vec<String>,
        >: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
    {
        type Output = Self;
        type Input = Self;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        I,
        __GROST_FLAVOR__: ?::core::marker::Sized,
        __GROST_UNKNOWN_BUFFER__,
    > PartialDecodedUser<
        '__grost_lifetime__,
        I,
        __GROST_FLAVOR__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<1u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<1u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<2u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        String: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<2u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<3u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u8: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<3u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        UserReflection<
            (
                ::grost::__private::reflection::WireFormatReflection,
                ::grost::__private::RawTag<4u32>,
            ),
            __GROST_FLAVOR__,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        Option<
            Vec<String>,
        >: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <Option<
            Vec<String>,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserReflection<
                    (
                        ::grost::__private::reflection::WireFormatReflection,
                        ::grost::__private::RawTag<4u32>,
                    ),
                    __GROST_FLAVOR__,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
    {
        /// Creates an empty partial struct.
        #[inline]
        pub const fn new() -> Self {
            Self {
                id: ::core::option::Option::None,
                name: ::core::option::Option::None,
                age: ::core::option::Option::None,
                emails: ::core::option::Option::None,
                __grost_unknown_buffer__: ::core::option::Option::None,
            }
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
        /// Returns a reference to the `id`
        #[inline]
        pub const fn id_ref(
            &self,
        ) -> ::core::option::Option<
            &<I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<1u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.id.as_ref()
        }
        /// Returns a mutable reference to the `id`
        #[inline]
        pub const fn id_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<1u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.id.as_mut()
        }
        /// Takes the value of `id` out if it is not `None`
        #[inline]
        pub const fn take_id(
            &mut self,
        ) -> ::core::option::Option<
            <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<1u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.id.take()
        }
        /// Clear the value of `id`
        #[inline]
        pub const fn clear_id(&mut self) -> &mut Self {
            self.id = ::core::option::Option::None;
            self
        }
        /// Set the `id` to the given value
        #[inline]
        pub const fn set_id(
            &mut self,
            value: <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<1u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> &mut Self {
            self.id = ::core::option::Option::Some(value);
            self
        }
        /// Update the `id` to the given value or clear the `id`
        #[inline]
        pub const fn update_id(
            &mut self,
            value: ::core::option::Option<
                <I as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        __GROST_FLAVOR__,
                        <UserReflection<
                            (
                                ::grost::__private::reflection::WireFormatReflection,
                                ::grost::__private::RawTag<1u32>,
                            ),
                            __GROST_FLAVOR__,
                        > as ::grost::__private::reflection::Reflectable<
                            __GROST_FLAVOR__,
                        >>::Reflection,
                    >,
                >>::Output,
            >,
        ) -> &mut Self {
            self.id = value;
            self
        }
        /// Set the `id` to the given value
        #[inline]
        pub const fn with_id(
            mut self,
            value: <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<1u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> Self {
            self.id = ::core::option::Option::Some(value);
            self
        }
        /// Clear the value of `id`
        #[inline]
        pub const fn without_id(mut self) -> Self {
            self.id = ::core::option::Option::None;
            self
        }
        /// Update the `id` to the given value or clear the `id`
        #[inline]
        pub const fn maybe_id(
            mut self,
            value: ::core::option::Option<
                <I as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        __GROST_FLAVOR__,
                        <UserReflection<
                            (
                                ::grost::__private::reflection::WireFormatReflection,
                                ::grost::__private::RawTag<1u32>,
                            ),
                            __GROST_FLAVOR__,
                        > as ::grost::__private::reflection::Reflectable<
                            __GROST_FLAVOR__,
                        >>::Reflection,
                    >,
                >>::Output,
            >,
        ) -> Self {
            self.id = value;
            self
        }
        /// Returns a reference to the `name`
        #[inline]
        pub const fn name_ref(
            &self,
        ) -> ::core::option::Option<
            &<String as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<2u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.name.as_ref()
        }
        /// Returns a mutable reference to the `name`
        #[inline]
        pub const fn name_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<2u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.name.as_mut()
        }
        /// Takes the value of `name` out if it is not `None`
        #[inline]
        pub const fn take_name(
            &mut self,
        ) -> ::core::option::Option<
            <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<2u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.name.take()
        }
        /// Clear the value of `name`
        #[inline]
        pub const fn clear_name(&mut self) -> &mut Self {
            self.name = ::core::option::Option::None;
            self
        }
        /// Set the `name` to the given value
        #[inline]
        pub const fn set_name(
            &mut self,
            value: <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<2u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> &mut Self {
            self.name = ::core::option::Option::Some(value);
            self
        }
        /// Update the `name` to the given value or clear the `name`
        #[inline]
        pub const fn update_name(
            &mut self,
            value: ::core::option::Option<
                <String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        __GROST_FLAVOR__,
                        <UserReflection<
                            (
                                ::grost::__private::reflection::WireFormatReflection,
                                ::grost::__private::RawTag<2u32>,
                            ),
                            __GROST_FLAVOR__,
                        > as ::grost::__private::reflection::Reflectable<
                            __GROST_FLAVOR__,
                        >>::Reflection,
                    >,
                >>::Output,
            >,
        ) -> &mut Self {
            self.name = value;
            self
        }
        /// Set the `name` to the given value
        #[inline]
        pub const fn with_name(
            mut self,
            value: <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<2u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> Self {
            self.name = ::core::option::Option::Some(value);
            self
        }
        /// Clear the value of `name`
        #[inline]
        pub const fn without_name(mut self) -> Self {
            self.name = ::core::option::Option::None;
            self
        }
        /// Update the `name` to the given value or clear the `name`
        #[inline]
        pub const fn maybe_name(
            mut self,
            value: ::core::option::Option<
                <String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        __GROST_FLAVOR__,
                        <UserReflection<
                            (
                                ::grost::__private::reflection::WireFormatReflection,
                                ::grost::__private::RawTag<2u32>,
                            ),
                            __GROST_FLAVOR__,
                        > as ::grost::__private::reflection::Reflectable<
                            __GROST_FLAVOR__,
                        >>::Reflection,
                    >,
                >>::Output,
            >,
        ) -> Self {
            self.name = value;
            self
        }
        /// Returns a reference to the `age`
        #[inline]
        pub const fn age_ref(
            &self,
        ) -> ::core::option::Option<
            &<u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<3u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.age.as_ref()
        }
        /// Returns a mutable reference to the `age`
        #[inline]
        pub const fn age_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<3u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.age.as_mut()
        }
        /// Takes the value of `age` out if it is not `None`
        #[inline]
        pub const fn take_age(
            &mut self,
        ) -> ::core::option::Option<
            <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<3u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.age.take()
        }
        /// Clear the value of `age`
        #[inline]
        pub const fn clear_age(&mut self) -> &mut Self {
            self.age = ::core::option::Option::None;
            self
        }
        /// Set the `age` to the given value
        #[inline]
        pub const fn set_age(
            &mut self,
            value: <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<3u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> &mut Self {
            self.age = ::core::option::Option::Some(value);
            self
        }
        /// Update the `age` to the given value or clear the `age`
        #[inline]
        pub const fn update_age(
            &mut self,
            value: ::core::option::Option<
                <u8 as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        __GROST_FLAVOR__,
                        <UserReflection<
                            (
                                ::grost::__private::reflection::WireFormatReflection,
                                ::grost::__private::RawTag<3u32>,
                            ),
                            __GROST_FLAVOR__,
                        > as ::grost::__private::reflection::Reflectable<
                            __GROST_FLAVOR__,
                        >>::Reflection,
                    >,
                >>::Output,
            >,
        ) -> &mut Self {
            self.age = value;
            self
        }
        /// Set the `age` to the given value
        #[inline]
        pub const fn with_age(
            mut self,
            value: <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<3u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> Self {
            self.age = ::core::option::Option::Some(value);
            self
        }
        /// Clear the value of `age`
        #[inline]
        pub const fn without_age(mut self) -> Self {
            self.age = ::core::option::Option::None;
            self
        }
        /// Update the `age` to the given value or clear the `age`
        #[inline]
        pub const fn maybe_age(
            mut self,
            value: ::core::option::Option<
                <u8 as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        __GROST_FLAVOR__,
                        <UserReflection<
                            (
                                ::grost::__private::reflection::WireFormatReflection,
                                ::grost::__private::RawTag<3u32>,
                            ),
                            __GROST_FLAVOR__,
                        > as ::grost::__private::reflection::Reflectable<
                            __GROST_FLAVOR__,
                        >>::Reflection,
                    >,
                >>::Output,
            >,
        ) -> Self {
            self.age = value;
            self
        }
        /// Returns a reference to the `emails`
        #[inline]
        pub const fn emails_ref(
            &self,
        ) -> ::core::option::Option<
            &<Option<
                Vec<String>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<4u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.emails.as_ref()
        }
        /// Returns a mutable reference to the `emails`
        #[inline]
        pub const fn emails_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <Option<
                Vec<String>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<4u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.emails.as_mut()
        }
        /// Takes the value of `emails` out if it is not `None`
        #[inline]
        pub const fn take_emails(
            &mut self,
        ) -> ::core::option::Option<
            <Option<
                Vec<String>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<4u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.emails.take()
        }
        /// Clear the value of `emails`
        #[inline]
        pub const fn clear_emails(&mut self) -> &mut Self {
            self.emails = ::core::option::Option::None;
            self
        }
        /// Set the `emails` to the given value
        #[inline]
        pub const fn set_emails(
            &mut self,
            value: <Option<
                Vec<String>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<4u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> &mut Self {
            self.emails = ::core::option::Option::Some(value);
            self
        }
        /// Update the `emails` to the given value or clear the `emails`
        #[inline]
        pub const fn update_emails(
            &mut self,
            value: ::core::option::Option<
                <Option<
                    Vec<String>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        __GROST_FLAVOR__,
                        <UserReflection<
                            (
                                ::grost::__private::reflection::WireFormatReflection,
                                ::grost::__private::RawTag<4u32>,
                            ),
                            __GROST_FLAVOR__,
                        > as ::grost::__private::reflection::Reflectable<
                            __GROST_FLAVOR__,
                        >>::Reflection,
                    >,
                >>::Output,
            >,
        ) -> &mut Self {
            self.emails = value;
            self
        }
        /// Set the `emails` to the given value
        #[inline]
        pub const fn with_emails(
            mut self,
            value: <Option<
                Vec<String>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserReflection<
                        (
                            ::grost::__private::reflection::WireFormatReflection,
                            ::grost::__private::RawTag<4u32>,
                        ),
                        __GROST_FLAVOR__,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> Self {
            self.emails = ::core::option::Option::Some(value);
            self
        }
        /// Clear the value of `emails`
        #[inline]
        pub const fn without_emails(mut self) -> Self {
            self.emails = ::core::option::Option::None;
            self
        }
        /// Update the `emails` to the given value or clear the `emails`
        #[inline]
        pub const fn maybe_emails(
            mut self,
            value: ::core::option::Option<
                <Option<
                    Vec<String>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        __GROST_FLAVOR__,
                        <UserReflection<
                            (
                                ::grost::__private::reflection::WireFormatReflection,
                                ::grost::__private::RawTag<4u32>,
                            ),
                            __GROST_FLAVOR__,
                        > as ::grost::__private::reflection::Reflectable<
                            __GROST_FLAVOR__,
                        >>::Reflection,
                    >,
                >>::Output,
            >,
        ) -> Self {
            self.emails = value;
            self
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I> ::core::default::Default for PartialUser<I>
    where
        I: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Flatten,
        >>::Output: ::core::marker::Sized,
    {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        I,
        __GROST_FLATTEN_STATE__: ?::core::marker::Sized,
    > ::grost::__private::convert::State<
        ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
    > for PartialUser<I>
    where
        I: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Flatten,
        >>::Output: ::core::marker::Sized,
    {
        type Output = Self;
        type Input = Self;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I> PartialUser<I>
    where
        I: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Flatten,
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
            }
        }
        /// Returns a reference to the `id`
        #[inline]
        pub const fn id_ref(
            &self,
        ) -> ::core::option::Option<
            &<I as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
        > {
            self.id.as_ref()
        }
        /// Returns a mutable reference to the `id`
        #[inline]
        pub const fn id_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
        > {
            self.id.as_mut()
        }
        /// Takes the value of `id` out if it is not `None`
        #[inline]
        pub const fn take_id(
            &mut self,
        ) -> ::core::option::Option<
            <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
        > {
            self.id.take()
        }
        /// Clear the value of `id`
        #[inline]
        pub fn clear_id(&mut self) -> &mut Self {
            self.id = ::core::option::Option::None;
            self
        }
        /// Set the `id` to the given value
        #[inline]
        pub fn set_id(
            &mut self,
            value: <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
        ) -> &mut Self {
            self.id = ::core::option::Option::Some(value);
            self
        }
        /// Update the `id` to the given value or clear the `id`
        #[inline]
        pub fn update_id(
            &mut self,
            value: ::core::option::Option<
                <I as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            >,
        ) -> &mut Self {
            self.id = value;
            self
        }
        /// Set the `id` to the given value
        #[inline]
        pub fn with_id(
            mut self,
            value: <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
        ) -> Self {
            self.id = ::core::option::Option::Some(value);
            self
        }
        /// Clear the value of `id`
        #[inline]
        pub fn without_id(mut self) -> Self {
            self.id = ::core::option::Option::None;
            self
        }
        /// Update the `id` to the given value or clear the `id`
        #[inline]
        pub fn maybe_id(
            mut self,
            value: ::core::option::Option<
                <I as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            >,
        ) -> Self {
            self.id = value;
            self
        }
        /// Returns a reference to the `name`
        #[inline]
        pub const fn name_ref(
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
        pub const fn name_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
        > {
            self.name.as_mut()
        }
        /// Takes the value of `name` out if it is not `None`
        #[inline]
        pub const fn take_name(
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
        pub fn clear_name(&mut self) -> &mut Self {
            self.name = ::core::option::Option::None;
            self
        }
        /// Set the `name` to the given value
        #[inline]
        pub fn set_name(
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
        pub fn update_name(
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
        pub fn with_name(
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
        pub fn without_name(mut self) -> Self {
            self.name = ::core::option::Option::None;
            self
        }
        /// Update the `name` to the given value or clear the `name`
        #[inline]
        pub fn maybe_name(
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
        pub const fn age_ref(
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
        pub const fn age_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
        > {
            self.age.as_mut()
        }
        /// Takes the value of `age` out if it is not `None`
        #[inline]
        pub const fn take_age(
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
        pub const fn clear_age(&mut self) -> &mut Self {
            self.age = ::core::option::Option::None;
            self
        }
        /// Set the `age` to the given value
        #[inline]
        pub const fn set_age(
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
        pub const fn update_age(
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
        pub const fn with_age(
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
        pub const fn without_age(mut self) -> Self {
            self.age = ::core::option::Option::None;
            self
        }
        /// Update the `age` to the given value or clear the `age`
        #[inline]
        pub const fn maybe_age(
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
        pub const fn emails_ref(
            &self,
        ) -> ::core::option::Option<
            &<Option<
                Vec<String>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
        > {
            self.emails.as_ref()
        }
        /// Returns a mutable reference to the `emails`
        #[inline]
        pub const fn emails_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <Option<
                Vec<String>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
        > {
            self.emails.as_mut()
        }
        /// Takes the value of `emails` out if it is not `None`
        #[inline]
        pub const fn take_emails(
            &mut self,
        ) -> ::core::option::Option<
            <Option<
                Vec<String>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
        > {
            self.emails.take()
        }
        /// Clear the value of `emails`
        #[inline]
        pub fn clear_emails(&mut self) -> &mut Self {
            self.emails = ::core::option::Option::None;
            self
        }
        /// Set the `emails` to the given value
        #[inline]
        pub fn set_emails(
            &mut self,
            value: <Option<
                Vec<String>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
        ) -> &mut Self {
            self.emails = ::core::option::Option::Some(value);
            self
        }
        /// Update the `emails` to the given value or clear the `emails`
        #[inline]
        pub fn update_emails(
            &mut self,
            value: ::core::option::Option<
                <Option<
                    Vec<String>,
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
        pub fn with_emails(
            mut self,
            value: <Option<
                Vec<String>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
        ) -> Self {
            self.emails = ::core::option::Option::Some(value);
            self
        }
        /// Clear the value of `emails`
        #[inline]
        pub fn without_emails(mut self) -> Self {
            self.emails = ::core::option::Option::None;
            self
        }
        /// Update the `emails` to the given value or clear the `emails`
        #[inline]
        pub fn maybe_emails(
            mut self,
            value: ::core::option::Option<
                <Option<
                    Vec<String>,
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
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I> ::core::default::Default for User<I>
    where
        I: ::core::default::Default,
    {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I> User<I>
    where
        I: ::core::default::Default,
    {
        /// Creates a new instance with default values.
        #[inline]
        pub fn new() -> Self {
            Self {
                id: ::core::default::Default::default(),
                name: sealed::default_user(),
                age: ::core::default::Default::default(),
                emails: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        I,
        __GROST_FLATTEN_STATE__: ?::core::marker::Sized,
    > ::grost::__private::convert::State<
        ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
    > for User<I> {
        type Output = Self;
        type Input = Self;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I> User<I> {
        /// Returns a reference to the `id`
        #[inline]
        pub const fn id_ref(&self) -> &I {
            &self.id
        }
        /// Returns a mutable reference to the `id`
        #[inline]
        pub const fn id_mut(&mut self) -> &mut I {
            &mut self.id
        }
        /// Set the `id` to the given value
        #[inline]
        pub fn set_id(&mut self, value: I) -> &mut Self {
            self.id = value;
            self
        }
        /// Set the `id` to the given value
        #[inline]
        pub fn with_id(mut self, value: I) -> Self {
            self.id = value;
            self
        }
        /// Returns a reference to the `name`
        #[inline]
        pub const fn name_ref(&self) -> &String {
            &self.name
        }
        /// Returns a mutable reference to the `name`
        #[inline]
        pub const fn name_mut(&mut self) -> &mut String {
            &mut self.name
        }
        /// Set the `name` to the given value
        #[inline]
        pub fn set_name(&mut self, value: String) -> &mut Self {
            self.name = value;
            self
        }
        /// Set the `name` to the given value
        #[inline]
        pub fn with_name(mut self, value: String) -> Self {
            self.name = value;
            self
        }
        /// Returns a reference to the `age`
        #[inline]
        pub const fn age_ref(&self) -> &u8 {
            &self.age
        }
        /// Returns a mutable reference to the `age`
        #[inline]
        pub const fn age_mut(&mut self) -> &mut u8 {
            &mut self.age
        }
        /// Set the `age` to the given value
        #[inline]
        pub const fn set_age(&mut self, value: u8) -> &mut Self {
            self.age = value;
            self
        }
        /// Set the `age` to the given value
        #[inline]
        pub const fn with_age(mut self, value: u8) -> Self {
            self.age = value;
            self
        }
        /// Returns a reference to the `emails`
        #[inline]
        pub const fn emails_ref(&self) -> &Option<Vec<String>> {
            &self.emails
        }
        /// Returns a mutable reference to the `emails`
        #[inline]
        pub const fn emails_mut(&mut self) -> &mut Option<Vec<String>> {
            &mut self.emails
        }
        /// Set the `emails` to the given value
        #[inline]
        pub fn set_emails(&mut self, value: Option<Vec<String>>) -> &mut Self {
            self.emails = value;
            self
        }
        /// Set the `emails` to the given value
        #[inline]
        pub fn with_emails(mut self, value: Option<Vec<String>>) -> Self {
            self.emails = value;
            self
        }
    }
};
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl ::grost::__private::reflection::Reflectable<::grost::__private::flavors::Network>
for UserReflection<
    (
        ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::RawTag<1u32>,
    ),
    ::grost::__private::flavors::Network,
> {
    type Reflection = ::grost::__private::reflection::ObjectFieldReflection<
        ::grost::__private::flavors::Network,
    >;
    const REFLECTION: &Self::Reflection = &{
        ::grost::__private::reflection::ObjectFieldReflectionBuilder::<
            ::grost::__private::flavors::Network,
        > {
            identifier: ::grost::__private::flavors::network::Identifier::new(
                <grost::flavors::network::LengthDelimited as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                ::grost::__private::flavors::network::Tag::new(1u32),
            ),
            name: "id",
            ty: ::core::any::type_name::<I>,
            schema_name: "id",
            schema_type: <::grost::__private::reflection::SchemaTypeReflection<
                I,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION,
        }
            .build()
    };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl ::grost::__private::reflection::Reflectable<::grost::__private::flavors::Network>
for UserReflection<
    (
        ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::RawTag<2u32>,
    ),
    ::grost::__private::flavors::Network,
> {
    type Reflection = ::grost::__private::reflection::ObjectFieldReflection<
        ::grost::__private::flavors::Network,
    >;
    const REFLECTION: &Self::Reflection = &{
        ::grost::__private::reflection::ObjectFieldReflectionBuilder::<
            ::grost::__private::flavors::Network,
        > {
            identifier: ::grost::__private::flavors::network::Identifier::new(
                <grost::flavors::network::LengthDelimited as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                ::grost::__private::flavors::network::Tag::new(2u32),
            ),
            name: "name",
            ty: ::core::any::type_name::<String>,
            schema_name: "name",
            schema_type: <::grost::__private::reflection::SchemaTypeReflection<
                String,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION,
        }
            .build()
    };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl ::grost::__private::reflection::Reflectable<::grost::__private::flavors::Network>
for UserReflection<
    (
        ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::RawTag<3u32>,
    ),
    ::grost::__private::flavors::Network,
> {
    type Reflection = ::grost::__private::reflection::ObjectFieldReflection<
        ::grost::__private::flavors::Network,
    >;
    const REFLECTION: &Self::Reflection = &{
        ::grost::__private::reflection::ObjectFieldReflectionBuilder::<
            ::grost::__private::flavors::Network,
        > {
            identifier: ::grost::__private::flavors::network::Identifier::new(
                <grost::flavors::network::Varint as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                ::grost::__private::flavors::network::Tag::new(3u32),
            ),
            name: "age",
            ty: ::core::any::type_name::<u8>,
            schema_name: "age",
            schema_type: <::grost::__private::reflection::SchemaTypeReflection<
                u8,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION,
        }
            .build()
    };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl ::grost::__private::reflection::Reflectable<::grost::__private::flavors::Network>
for UserReflection<
    (
        ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::RawTag<4u32>,
    ),
    ::grost::__private::flavors::Network,
> {
    type Reflection = ::grost::__private::reflection::ObjectFieldReflection<
        ::grost::__private::flavors::Network,
    >;
    const REFLECTION: &Self::Reflection = &{
        ::grost::__private::reflection::ObjectFieldReflectionBuilder::<
            ::grost::__private::flavors::Network,
        > {
            identifier: ::grost::__private::flavors::network::Identifier::new(
                <grost::flavors::network::LengthDelimited as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                ::grost::__private::flavors::network::Tag::new(4u32),
            ),
            name: "emails",
            ty: ::core::any::type_name::<Option<Vec<String>>>,
            schema_name: "emails",
            schema_type: <::grost::__private::reflection::SchemaTypeReflection<
                Option<Vec<String>>,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION,
        }
            .build()
    };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl ::grost::__private::reflection::Reflectable<::grost::__private::flavors::Network>
for UserReflection<
    (
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::RawTag<1u32>,
    ),
    ::grost::__private::flavors::Network,
> {
    type Reflection = ::grost::__private::flavors::network::Identifier;
    const REFLECTION: &Self::Reflection = &{
        <UserReflection<
            (
                ::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
                ::grost::__private::RawTag<1u32>,
            ),
            ::grost::__private::flavors::Network,
        > as ::grost::__private::reflection::Reflectable<
            ::grost::__private::flavors::Network,
        >>::REFLECTION
            .identifier()
    };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl ::grost::__private::reflection::Reflectable<::grost::__private::flavors::Network>
for UserReflection<
    (
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::RawTag<2u32>,
    ),
    ::grost::__private::flavors::Network,
> {
    type Reflection = ::grost::__private::flavors::network::Identifier;
    const REFLECTION: &Self::Reflection = &{
        <UserReflection<
            (
                ::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
                ::grost::__private::RawTag<2u32>,
            ),
            ::grost::__private::flavors::Network,
        > as ::grost::__private::reflection::Reflectable<
            ::grost::__private::flavors::Network,
        >>::REFLECTION
            .identifier()
    };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl ::grost::__private::reflection::Reflectable<::grost::__private::flavors::Network>
for UserReflection<
    (
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::RawTag<3u32>,
    ),
    ::grost::__private::flavors::Network,
> {
    type Reflection = ::grost::__private::flavors::network::Identifier;
    const REFLECTION: &Self::Reflection = &{
        <UserReflection<
            (
                ::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
                ::grost::__private::RawTag<3u32>,
            ),
            ::grost::__private::flavors::Network,
        > as ::grost::__private::reflection::Reflectable<
            ::grost::__private::flavors::Network,
        >>::REFLECTION
            .identifier()
    };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl ::grost::__private::reflection::Reflectable<::grost::__private::flavors::Network>
for UserReflection<
    (
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::RawTag<4u32>,
    ),
    ::grost::__private::flavors::Network,
> {
    type Reflection = ::grost::__private::flavors::network::Identifier;
    const REFLECTION: &Self::Reflection = &{
        <UserReflection<
            (
                ::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
                ::grost::__private::RawTag<4u32>,
            ),
            ::grost::__private::flavors::Network,
        > as ::grost::__private::reflection::Reflectable<
            ::grost::__private::flavors::Network,
        >>::REFLECTION
            .identifier()
    };
}
extern crate test;
#[rustc_test_marker = "t"]
#[doc(hidden)]
pub const t: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("t"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "grost-derive/tests/object.rs",
        start_line: 82usize,
        start_col: 4usize,
        end_line: 82usize,
        end_col: 5usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(t())),
};
fn t() {}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&t])
}
