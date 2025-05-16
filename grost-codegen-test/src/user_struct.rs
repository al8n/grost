#![no_implicit_prelude]

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
///A user struct
pub struct User {
    name: ::std::string::String,
    age: u32,
    email: ::core::option::Option<::std::string::String>,
}
/// The partial struct of [`User`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct PartialUser {
    name: ::core::option::Option<::std::string::String>,
    age: ::core::option::Option<u32>,
    email: ::core::option::Option<::std::string::String>,
}
#[allow(clippy::type_complexity, non_camel_case_types)]
pub struct PartialUserRef<
    '__grost_lifetime__,
    __GROST_FLAVOR__,
    __GROST_UNKNOWN_BUFFER__ = (),
>
where
    UserFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        1u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::std::string::String: ::grost::__private::convert::State<
        ::grost::__private::convert::Encoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >,
    <::std::string::String as ::grost::__private::convert::State<
        ::grost::__private::convert::Encoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >>::Output: ::core::marker::Sized,
    UserFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        2u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    u32: ::grost::__private::convert::State<
        ::grost::__private::convert::Encoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >,
    <u32 as ::grost::__private::convert::State<
        ::grost::__private::convert::Encoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >>::Output: ::core::marker::Sized,
    UserFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        3u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::core::option::Option<
        ::std::string::String,
    >: ::grost::__private::convert::State<
        ::grost::__private::convert::Encoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >,
    <::core::option::Option<
        ::std::string::String,
    > as ::grost::__private::convert::State<
        ::grost::__private::convert::Encoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >>::Output: ::core::marker::Sized,
{
    name: ::core::option::Option<
        <::std::string::String as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output,
    >,
    age: ::core::option::Option<
        <u32 as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output,
    >,
    email: ::core::option::Option<
        <::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output,
    >,
    __grost_unknown_buffer__: ::core::option::Option<__GROST_UNKNOWN_BUFFER__>,
}
/// Field indexer for the struct [`User`]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::cmp::PartialOrd,
    ::core::cmp::Ord,
    ::core::hash::Hash,
    ::core::fmt::Debug
)]
#[repr(u32)]
pub enum UserFieldIndex {
    /// The field indexer for the field `name`
    Name = 0u32,
    /// The field indexer for the field `age`
    Age = 1u32,
    /// The field indexer for the field `email`
    Email = 2u32,
}
/// The field reflection of the [`User`]'s fields.
pub struct UserFieldReflection<
    R: ?::core::marker::Sized,
    F: ?::core::marker::Sized,
    const TAG: ::core::primitive::u32,
> {
    _reflect: ::core::marker::PhantomData<R>,
    _flavor: ::core::marker::PhantomData<F>,
}
/// The reflection of the [`User`].
pub struct UserReflection<R: ?::core::marker::Sized, F: ?::core::marker::Sized> {
    _reflect: ::core::marker::PhantomData<R>,
    _flavor: ::core::marker::PhantomData<F>,
}
/// The selection type for User
#[allow(non_camel_case_types)]
pub struct UserSelector<__GROST_FLAVOR__>
where
    UserFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        1u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::std::string::String: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    UserFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        2u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    u32: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    UserFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        3u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::core::option::Option<
        ::std::string::String,
    >: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
{
    name: <::std::string::String as ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >>::Selector,
    age: <u32 as ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >>::Selector,
    email: <::core::option::Option<
        ::std::string::String,
    > as ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >>::Selector,
}
/// An iterator over the selected fields of the [`UserSelector`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct UserSelectorIter<
    '__grost_lifetime__,
    __GROST_FLAVOR__,
    const N: ::core::primitive::bool = true,
>
where
    UserFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        1u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::std::string::String: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    UserFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        2u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    u32: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    UserFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        3u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::core::option::Option<
        ::std::string::String,
    >: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
{
    selector: &'__grost_lifetime__ UserSelector<__GROST_FLAVOR__>,
    index: ::core::option::Option<UserFieldIndex>,
    num: ::core::primitive::usize,
    yielded: ::core::primitive::usize,
}
#[derive(::core::fmt::Debug, ::core::clone::Clone)]
///A comment struct
pub struct Comment {
    user: User,
    replyer: ::core::option::Option<User>,
    title: ::std::string::String,
    content: ::core::option::Option<::std::string::String>,
}
/// The partial struct of [`Comment`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct PartialComment {
    user: ::core::option::Option<User>,
    replyer: ::core::option::Option<User>,
    title: ::core::option::Option<::std::string::String>,
    content: ::core::option::Option<::std::string::String>,
}
#[allow(clippy::type_complexity, non_camel_case_types)]
pub struct PartialCommentRef<
    '__grost_lifetime__,
    __GROST_FLAVOR__,
    __GROST_UNKNOWN_BUFFER__ = (),
>
where
    CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        1u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    User: ::grost::__private::convert::State<
        ::grost::__private::convert::Encoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >,
    <User as ::grost::__private::convert::State<
        ::grost::__private::convert::Encoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >>::Output: ::core::marker::Sized,
    CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        2u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::core::option::Option<
        User,
    >: ::grost::__private::convert::State<
        ::grost::__private::convert::Encoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >,
    <::core::option::Option<
        User,
    > as ::grost::__private::convert::State<
        ::grost::__private::convert::Encoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >>::Output: ::core::marker::Sized,
    CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        3u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::std::string::String: ::grost::__private::convert::State<
        ::grost::__private::convert::Encoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >,
    <::std::string::String as ::grost::__private::convert::State<
        ::grost::__private::convert::Encoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >>::Output: ::core::marker::Sized,
    CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        4u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::core::option::Option<
        ::std::string::String,
    >: ::grost::__private::convert::State<
        ::grost::__private::convert::Encoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >,
    <::core::option::Option<
        ::std::string::String,
    > as ::grost::__private::convert::State<
        ::grost::__private::convert::Encoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    >>::Output: ::core::marker::Sized,
{
    user: ::core::option::Option<
        <User as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output,
    >,
    replyer: ::core::option::Option<
        <::core::option::Option<
            User,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output,
    >,
    title: ::core::option::Option<
        <::std::string::String as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output,
    >,
    content: ::core::option::Option<
        <::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output,
    >,
    __grost_unknown_buffer__: ::core::option::Option<__GROST_UNKNOWN_BUFFER__>,
}
/// Field indexer for the struct [`Comment`]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::cmp::PartialOrd,
    ::core::cmp::Ord,
    ::core::hash::Hash,
    ::core::fmt::Debug
)]
#[repr(u32)]
pub enum CommentFieldIndex {
    /// The field indexer for the field `user`
    User = 0u32,
    /// The field indexer for the field `replyer`
    Replyer = 1u32,
    /// The field indexer for the field `title`
    Title = 2u32,
    /// The field indexer for the field `content`
    Content = 3u32,
}
/// The field reflection of the [`Comment`]'s fields.
pub struct CommentFieldReflection<
    R: ?::core::marker::Sized,
    F: ?::core::marker::Sized,
    const TAG: ::core::primitive::u32,
> {
    _reflect: ::core::marker::PhantomData<R>,
    _flavor: ::core::marker::PhantomData<F>,
}
/// The reflection of the [`Comment`].
pub struct CommentReflection<R: ?::core::marker::Sized, F: ?::core::marker::Sized> {
    _reflect: ::core::marker::PhantomData<R>,
    _flavor: ::core::marker::PhantomData<F>,
}
/// The selection type for Comment
#[allow(non_camel_case_types)]
pub struct CommentSelector<__GROST_FLAVOR__>
where
    CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        1u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    User: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        2u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::core::option::Option<
        User,
    >: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        3u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::std::string::String: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        4u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::core::option::Option<
        ::std::string::String,
    >: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            4u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
{
    user: <User as ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >>::Selector,
    replyer: <::core::option::Option<
        User,
    > as ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >>::Selector,
    title: <::std::string::String as ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >>::Selector,
    content: <::core::option::Option<
        ::std::string::String,
    > as ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            4u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >>::Selector,
}
/// An iterator over the selected fields of the [`CommentSelector`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct CommentSelectorIter<
    '__grost_lifetime__,
    __GROST_FLAVOR__,
    const N: ::core::primitive::bool = true,
>
where
    CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        1u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    User: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        2u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::core::option::Option<
        User,
    >: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        3u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::std::string::String: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
    CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        __GROST_FLAVOR__,
        4u32,
    >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
    ::core::option::Option<
        ::std::string::String,
    >: ::grost::__private::Selectable<
        __GROST_FLAVOR__,
        <CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            4u32,
        > as ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>>::Reflection,
    >,
{
    selector: &'__grost_lifetime__ CommentSelector<__GROST_FLAVOR__>,
    index: ::core::option::Option<CommentFieldIndex>,
    num: ::core::primitive::usize,
    yielded: ::core::primitive::usize,
}
const _: () = {
    #[automatically_derived]
    impl ::core::default::Default for User {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl User {
        /// Returns a new default instance of the struct
        pub fn new() -> Self {
            Self {
                name: ::core::default::Default::default(),
                age: ::core::default::Default::default(),
                email: ::core::default::Default::default(),
            }
        }
        /// Gets the reference of the field `name`.
        #[inline]
        pub fn name_ref(&self) -> &::std::string::String {
            &self.name
        }
        /// Gets the mutable reference of the field `name`.
        #[inline]
        pub fn name_mut(&mut self) -> &mut ::std::string::String {
            &mut self.name
        }
        /// Sets the `name`.
        #[inline]
        pub fn set_name(&mut self, name: ::std::string::String) -> &mut Self {
            self.name = name;
            self
        }
        /// Sets the `name`.
        #[inline]
        pub fn with_name(mut self, name: ::std::string::String) -> Self {
            self.name = name;
            self
        }
        /// Gets the reference of the field `age`.
        #[inline]
        pub const fn age_ref(&self) -> u32 {
            self.age
        }
        /// Gets the mutable reference of the field `age`.
        #[inline]
        pub const fn age_mut(&mut self) -> &mut u32 {
            &mut self.age
        }
        /// Sets the `age`.
        #[inline]
        pub const fn set_age(&mut self, age: u32) -> &mut Self {
            self.age = age;
            self
        }
        /// Sets the `age`.
        #[inline]
        pub const fn with_age(mut self, age: u32) -> Self {
            self.age = age;
            self
        }
        /// Gets the reference of the field `email`.
        #[inline]
        pub fn email_ref(&self) -> ::core::option::Option<&::std::string::String> {
            ::core::option::Option::as_ref(&self.email)
        }
        /// Gets the mutable reference of the field `email`.
        #[inline]
        pub fn email_mut(
            &mut self,
        ) -> ::core::option::Option<&mut ::std::string::String> {
            ::core::option::Option::as_mut(&mut self.email)
        }
        /// Sets the `email`.
        #[inline]
        pub fn set_email(
            &mut self,
            email: ::core::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.email = email;
            self
        }
        /// Sets the `email`.
        #[inline]
        pub fn with_email(
            mut self,
            email: ::core::option::Option<::std::string::String>,
        ) -> Self {
            self.email = email;
            self
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::default::Default for PartialUser {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl PartialUser {
        /// Creates an empty partial struct.
        pub const fn new() -> Self {
            Self {
                name: ::core::option::Option::None,
                age: ::core::option::Option::None,
                email: ::core::option::Option::None,
            }
        }
        #[inline]
        pub const fn name_ref(&self) -> ::core::option::Option<&::std::string::String> {
            self.name.as_ref()
        }
        #[inline]
        pub const fn name_mut(
            &mut self,
        ) -> ::core::option::Option<&mut ::std::string::String> {
            self.name.as_mut()
        }
        #[inline]
        pub const fn take_name(
            &mut self,
        ) -> ::core::option::Option<::std::string::String> {
            self.name.take()
        }
        #[inline]
        pub fn clear_name(&mut self) -> &mut Self {
            self.name = ::core::option::Option::None;
            self
        }
        #[inline]
        pub fn set_name(&mut self, value: ::std::string::String) -> &mut Self {
            self.name = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_name(mut self, value: ::std::string::String) -> Self {
            self.name = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn without_name(mut self) -> Self {
            self.name = ::core::option::Option::None;
            self
        }
        #[inline]
        pub const fn age_ref(&self) -> ::core::option::Option<&u32> {
            self.age.as_ref()
        }
        #[inline]
        pub const fn age_mut(&mut self) -> ::core::option::Option<&mut u32> {
            self.age.as_mut()
        }
        #[inline]
        pub const fn take_age(&mut self) -> ::core::option::Option<u32> {
            self.age.take()
        }
        #[inline]
        pub const fn clear_age(&mut self) -> &mut Self {
            self.age = ::core::option::Option::None;
            self
        }
        #[inline]
        pub const fn set_age(&mut self, value: u32) -> &mut Self {
            self.age = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub const fn with_age(mut self, value: u32) -> Self {
            self.age = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub const fn without_age(mut self) -> Self {
            self.age = ::core::option::Option::None;
            self
        }
        #[inline]
        pub const fn email_ref(&self) -> ::core::option::Option<&::std::string::String> {
            self.email.as_ref()
        }
        #[inline]
        pub const fn email_mut(
            &mut self,
        ) -> ::core::option::Option<&mut ::std::string::String> {
            self.email.as_mut()
        }
        #[inline]
        pub const fn take_email(
            &mut self,
        ) -> ::core::option::Option<::std::string::String> {
            self.email.take()
        }
        #[inline]
        pub fn clear_email(&mut self) -> &mut Self {
            self.email = ::core::option::Option::None;
            self
        }
        #[inline]
        pub fn set_email(&mut self, value: ::std::string::String) -> &mut Self {
            self.email = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_email(mut self, value: ::std::string::String) -> Self {
            self.email = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn without_email(mut self) -> Self {
            self.email = ::core::option::Option::None;
            self
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        __GROST_FLAVOR__,
        __GROST_UNKNOWN_BUFFER__,
    > ::core::default::Default
    for PartialUserRef<'__grost_lifetime__, __GROST_FLAVOR__, __GROST_UNKNOWN_BUFFER__>
    where
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <::std::string::String as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u32: ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <u32 as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
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
        __GROST_FLAVOR__,
        __GROST_UNKNOWN_BUFFER__,
    > PartialUserRef<'__grost_lifetime__, __GROST_FLAVOR__, __GROST_UNKNOWN_BUFFER__>
    where
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <::std::string::String as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u32: ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <u32 as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized,
    {
        /// Creates an empty partial struct.
        #[inline]
        pub const fn new() -> Self {
            Self {
                name: ::core::option::Option::None,
                age: ::core::option::Option::None,
                email: ::core::option::Option::None,
                __grost_unknown_buffer__: ::core::option::Option::None,
            }
        }
        #[inline]
        pub const fn name_ref(
            &self,
        ) -> ::core::option::Option<
            &<::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.name.as_ref()
        }
        #[inline]
        pub const fn name_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.name.as_mut()
        }
        #[inline]
        pub const fn take_name(
            &mut self,
        ) -> ::core::option::Option<
            <::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.name.take()
        }
        #[inline]
        pub fn clear_name(&mut self) -> &mut Self {
            self.name = ::core::option::Option::None;
            self
        }
        #[inline]
        pub fn set_name(
            &mut self,
            value: <::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> &mut Self {
            self.name = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_name(
            mut self,
            value: <::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> Self {
            self.name = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn without_name(mut self) -> Self {
            self.name = ::core::option::Option::None;
            self
        }
        #[inline]
        pub const fn age_ref(
            &self,
        ) -> ::core::option::Option<
            &<u32 as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.age.as_ref()
        }
        #[inline]
        pub const fn age_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <u32 as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.age.as_mut()
        }
        #[inline]
        pub const fn take_age(
            &mut self,
        ) -> ::core::option::Option<
            <u32 as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.age.take()
        }
        #[inline]
        pub fn clear_age(&mut self) -> &mut Self {
            self.age = ::core::option::Option::None;
            self
        }
        #[inline]
        pub fn set_age(
            &mut self,
            value: <u32 as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> &mut Self {
            self.age = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_age(
            mut self,
            value: <u32 as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> Self {
            self.age = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn without_age(mut self) -> Self {
            self.age = ::core::option::Option::None;
            self
        }
        #[inline]
        pub const fn email_ref(
            &self,
        ) -> ::core::option::Option<
            &<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.email.as_ref()
        }
        #[inline]
        pub const fn email_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.email.as_mut()
        }
        #[inline]
        pub const fn take_email(
            &mut self,
        ) -> ::core::option::Option<
            <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.email.take()
        }
        #[inline]
        pub fn clear_email(&mut self) -> &mut Self {
            self.email = ::core::option::Option::None;
            self
        }
        #[inline]
        pub fn set_email(
            &mut self,
            value: <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> &mut Self {
            self.email = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_email(
            mut self,
            value: <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> Self {
            self.email = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn without_email(mut self) -> Self {
            self.email = ::core::option::Option::None;
            self
        }
    }
};
const _: () = {
    #[automatically_derived]
    impl<R, F, const TAG: ::core::primitive::u32> ::core::ops::Deref
    for UserFieldReflection<R, F, TAG>
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
    impl<
        R,
        F,
        const TAG: ::core::primitive::u32,
    > ::core::convert::AsRef<<Self as ::core::ops::Deref>::Target>
    for UserFieldReflection<R, F, TAG>
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
    impl<R, F, const TAG: ::core::primitive::u32> ::core::fmt::Debug
    for UserFieldReflection<R, F, TAG>
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
    impl<R, F, const TAG: ::core::primitive::u32> ::core::fmt::Display
    for UserFieldReflection<R, F, TAG>
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
    #[allow(clippy::type_complexity, non_camel_case_types)]
    impl<R, F, const TAG: ::core::primitive::u32> UserFieldReflection<R, F, TAG>
    where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized,
    {
        #[inline]
        const fn new_in() -> Self {
            Self {
                _reflect: ::core::marker::PhantomData,
                _flavor: ::core::marker::PhantomData,
            }
        }
        /// Returns the reflection of the field.
        #[inline]
        const fn new() -> Self {
            Self::new_in()
        }
        /// Returns the relection to the wire format of the field.
        #[inline]
        pub const fn wire_format(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
    }
    #[automatically_derived]
    #[allow(clippy::type_complexity)]
    impl<
        F,
        const TAG: ::core::primitive::u32,
    > UserFieldReflection<::grost::__private::reflection::ObjectFieldReflection<F>, F, TAG>
    where
        F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
        /// Returns the relection to a tag of the field.
        #[inline]
        pub const fn tag(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::TagReflection<F::Tag>,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the relection to the encoded tag of the field.
        #[inline]
        pub const fn encoded_tag(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::EncodedTagReflection<F::Identifier>,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the relection to the encoded tag of the field.
        #[inline]
        pub const fn encoded_tag_len(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::EncodedTagReflection<F::Identifier>,
            >,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the relection to the wire type of the field.
        #[inline]
        pub const fn wire_type(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::WireTypeReflection<F::WireType>,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the relection to the identifier of the field.
        #[inline]
        pub const fn identifier(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::IdentifierReflection<F::Identifier>,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the relection to the encoded identifier of the field.
        #[inline]
        pub const fn encoded_identifier(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::EncodedIdentifierReflection<F::Identifier>,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the relection to the encoded identifier of the field.
        #[inline]
        pub const fn encoded_identifier_len(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::EncodedIdentifierReflection<
                    F::Identifier,
                >,
            >,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the reflection to the encode fn.
        #[inline]
        pub const fn encode(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::encode::EncodeField,
            >,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the reflection to fn which will give the length of the encoded data.
        #[inline]
        pub const fn encoded_len(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::Len<
                    ::grost::__private::reflection::encode::EncodeField,
                >,
            >,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the reflection to the reference encode fn.
        #[inline]
        pub const fn encode_ref(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::encode::EncodeRefField,
            >,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the reflection to the reference encode fn which will give the length of the encoded data.
        #[inline]
        pub const fn encoded_ref_len(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::Len<
                    ::grost::__private::reflection::encode::EncodeRefField,
                >,
            >,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the reflection to the partial encode fn.
        #[inline]
        pub const fn partial_encode(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::encode::PartialEncodeField,
            >,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the reflection to the partial encode fn which will give the length of the encoded data.
        #[inline]
        pub const fn partial_encoded_len(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::Len<
                    ::grost::__private::reflection::encode::PartialEncodeField,
                >,
            >,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the reflection to the partial reference encode fn.
        #[inline]
        pub const fn partial_encode_ref(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::encode::PartialEncodeRefField,
            >,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the reflection to the partial reference encode fn which will give the length of the encoded data.
        #[inline]
        pub const fn partial_encoded_ref_len(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::Len<
                    ::grost::__private::reflection::encode::PartialEncodeRefField,
                >,
            >,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
    }
    #[automatically_derived]
    impl<R, F, const TAG: ::core::primitive::u32> ::core::clone::Clone
    for UserFieldReflection<R, F, TAG>
    where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized,
    {
        fn clone(&self) -> Self {
            *self
        }
    }
    #[automatically_derived]
    impl<R, F, const TAG: ::core::primitive::u32> ::core::marker::Copy
    for UserFieldReflection<R, F, TAG>
    where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized,
    {}
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
        /// Returns the field reflection of the field `User.name`.
        #[inline]
        pub const fn name(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::ObjectFieldReflection<F>,
            F,
            1u32,
        > {
            UserFieldReflection::new()
        }
        /// Returns the field reflection of the field `User.age`.
        #[inline]
        pub const fn age(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::ObjectFieldReflection<F>,
            F,
            2u32,
        > {
            UserFieldReflection::new()
        }
        /// Returns the field reflection of the field `User.email`.
        #[inline]
        pub const fn email(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::ObjectFieldReflection<F>,
            F,
            3u32,
        > {
            UserFieldReflection::new()
        }
    }
    #[automatically_derived]
    impl User {
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
const _: () = {
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        __GROST_FLAVOR__: ?::core::marker::Sized,
    > ::grost::__private::indexer::Indexable<__GROST_FLAVOR__> for User {
        type Indexer = UserFieldIndex;
    }
    #[automatically_derived]
    impl UserFieldIndex {
        /// The number of variants of this field indexer.
        pub const VARIANTS: ::core::primitive::usize = 3usize;
        /// The first field indexer.
        pub const FIRST: Self = Self::Name;
        /// The last field indexer.
        pub const LAST: Self = Self::Email;
        /// Returns the field reflection of the corresponding field.
        #[allow(non_camel_case_types, clippy::type_complexity)]
        #[inline]
        pub const fn reflection<__GROST_FLAVOR__>(
            &self,
        ) -> &'static ::grost::__private::reflection::ObjectFieldReflection<__GROST_FLAVOR__>
        where
            __GROST_FLAVOR__: ::grost::__private::flavors::Flavor
                + ?::core::marker::Sized,
            UserFieldReflection<
                ::grost::__private::reflection::ObjectFieldReflection<__GROST_FLAVOR__>,
                __GROST_FLAVOR__,
                1u32,
            >: ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
                Reflection = ::grost::__private::reflection::ObjectFieldReflection<
                    __GROST_FLAVOR__,
                >,
            >,
            UserFieldReflection<
                ::grost::__private::reflection::ObjectFieldReflection<__GROST_FLAVOR__>,
                __GROST_FLAVOR__,
                2u32,
            >: ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
                Reflection = ::grost::__private::reflection::ObjectFieldReflection<
                    __GROST_FLAVOR__,
                >,
            >,
            UserFieldReflection<
                ::grost::__private::reflection::ObjectFieldReflection<__GROST_FLAVOR__>,
                __GROST_FLAVOR__,
                3u32,
            >: ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
                Reflection = ::grost::__private::reflection::ObjectFieldReflection<
                    __GROST_FLAVOR__,
                >,
            >,
        {
            match self {
                Self::Name => {
                    <UserFieldReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            __GROST_FLAVOR__,
                        >,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::REFLECTION
                }
                Self::Age => {
                    <UserFieldReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            __GROST_FLAVOR__,
                        >,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::REFLECTION
                }
                Self::Email => {
                    <UserFieldReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            __GROST_FLAVOR__,
                        >,
                        __GROST_FLAVOR__,
                        3u32,
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
                Self::Email => ::core::option::Option::None,
                Self::Name => ::core::option::Option::Some(Self::Age),
                Self::Age => ::core::option::Option::Some(Self::Email),
            }
        }
        /// Returns the previous field indexer.
        ///
        /// Returns `None` if there are no previous fields.
        #[inline]
        pub const fn prev(&self) -> ::core::option::Option<Self> {
            match self {
                Self::Name => ::core::option::Option::None,
                Self::Email => ::core::option::Option::Some(Self::Age),
                Self::Age => ::core::option::Option::Some(Self::Name),
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
    impl ::core::iter::Iterator for UserFieldIndex {
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
    impl ::core::iter::DoubleEndedIterator for UserFieldIndex {
        fn next_back(&mut self) -> ::core::option::Option<Self> {
            Self::prev(self)
        }
    }
    #[automatically_derived]
    impl ::core::iter::FusedIterator for UserFieldIndex {}
    #[automatically_derived]
    impl ::core::iter::ExactSizeIterator for UserFieldIndex {
        fn len(&self) -> ::core::primitive::usize {
            self.remaining()
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<__GROST_FLAVOR__> ::core::fmt::Debug for UserSelector<__GROST_FLAVOR__>
    where
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u32: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::write!(f, "UserSelector")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<__GROST_FLAVOR__> ::core::cmp::PartialEq for UserSelector<__GROST_FLAVOR__>
    where
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u32: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        fn eq(&self, other: &Self) -> ::core::primitive::bool {
            self.name == other.name && self.age == other.age && self.email == other.email
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<__GROST_FLAVOR__> ::core::cmp::Eq for UserSelector<__GROST_FLAVOR__>
    where
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u32: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<__GROST_FLAVOR__> ::core::hash::Hash for UserSelector<__GROST_FLAVOR__>
    where
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u32: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
            self.name.hash(state);
            self.age.hash(state);
            self.email.hash(state);
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<__GROST_FLAVOR__> ::core::clone::Clone for UserSelector<__GROST_FLAVOR__>
    where
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u32: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        fn clone(&self) -> Self {
            Self {
                name: ::core::clone::Clone::clone(&self.name),
                age: ::core::clone::Clone::clone(&self.age),
                email: ::core::clone::Clone::clone(&self.email),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<__GROST_FLAVOR__> ::grost::__private::Selector<__GROST_FLAVOR__>
    for UserSelector<__GROST_FLAVOR__>
    where
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u32: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
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
            <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::flip(&mut self.name);
            <<u32 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::flip(&mut self.age);
            <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::flip(&mut self.email);
            self
        }
        fn merge(&mut self, other: Self) -> &mut Self {
            <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::merge(&mut self.name, other.name);
            <<u32 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::merge(&mut self.age, other.age);
            <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::merge(&mut self.email, other.email);
            self
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<__GROST_FLAVOR__> ::core::default::Default for UserSelector<__GROST_FLAVOR__>
    where
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u32: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
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
    impl<__GROST_FLAVOR__> UserSelector<__GROST_FLAVOR__>
    where
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u32: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        /// The number of options in this selection type.
        pub const OPTIONS: ::core::primitive::usize = 3usize;
        /// Returns a selector with the default values.
        #[inline]
        pub const fn new() -> Self {
            Self {
                name: <<::std::string::String as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT,
                age: <<u32 as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT,
                email: <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
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
                name: <<::std::string::String as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE,
                age: <<u32 as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE,
                email: <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
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
                name: <<::std::string::String as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::ALL,
                age: <<u32 as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::ALL,
                email: <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::ALL,
            }
        }
        /// Returns `true` if the selector selects nothing.
        #[inline]
        pub fn is_empty(&self) -> ::core::primitive::bool {
            <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.name)
                && <<u32 as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_empty(&self.age)
                && <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_empty(&self.email)
        }
        /// Returns `true` if the selector selects all.
        #[inline]
        pub fn is_all(&self) -> ::core::primitive::bool {
            <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_all(&self.name)
                && <<u32 as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_all(&self.age)
                && <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <UserFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_all(&self.email)
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
            if self.is_email_selected() {
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
            if self.is_email_unselected() {
                num += 1;
            }
            num
        }
        /// Returns an iterator over the selected fields.
        #[inline]
        pub fn iter_selected(&self) -> UserSelectorIter<__GROST_FLAVOR__, true> {
            UserSelectorIter::new(self, self.selected())
        }
        /// Returns an iterator over the unselected fields.
        #[inline]
        pub fn iter_unselected(&self) -> UserSelectorIter<__GROST_FLAVOR__, false> {
            UserSelectorIter::new(self, self.unselected())
        }
        /// Returns `true` if such field is selected.
        #[inline]
        pub fn is_selected(&self, idx: UserFieldIndex) -> ::core::primitive::bool {
            match idx {
                UserFieldIndex::Name => self.is_name_selected(),
                UserFieldIndex::Age => self.is_age_selected(),
                UserFieldIndex::Email => self.is_email_selected(),
            }
        }
        /// Returns `true` if such field is unselected.
        #[inline]
        pub fn is_unselected(&self, idx: UserFieldIndex) -> ::core::primitive::bool {
            match idx {
                UserFieldIndex::Name => self.is_name_unselected(),
                UserFieldIndex::Age => self.is_age_unselected(),
                UserFieldIndex::Email => self.is_email_unselected(),
            }
        }
        /// Select the `User.name` field
        #[inline]
        pub fn select_name(&mut self) -> &mut Self {
            self.name = <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unselect the `User.name` field
        #[inline]
        pub fn unselect_name(&mut self) -> &mut Self {
            self.name = <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
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
            value: <::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
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
            val: <::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
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
        ) -> &<::std::string::String as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
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
        ) -> &mut <::std::string::String as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &mut self.name
        }
        /// Set the `User.name` field
        #[inline]
        pub fn with_name(mut self) -> Self {
            self.name = <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unset the `User.name` field
        #[inline]
        pub fn without_name(mut self) -> Self {
            self.name = <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Returns `true` if the `User.name` field is selected
        #[inline]
        pub fn is_name_selected(&self) -> ::core::primitive::bool {
            !<<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
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
            <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
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
            self.age = <<u32 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unselect the `User.age` field
        #[inline]
        pub fn unselect_age(&mut self) -> &mut Self {
            self.age = <<u32 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
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
            value: <u32 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
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
            val: <u32 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
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
        ) -> &<u32 as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
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
        ) -> &mut <u32 as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &mut self.age
        }
        /// Set the `User.age` field
        #[inline]
        pub fn with_age(mut self) -> Self {
            self.age = <<u32 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unset the `User.age` field
        #[inline]
        pub fn without_age(mut self) -> Self {
            self.age = <<u32 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Returns `true` if the `User.age` field is selected
        #[inline]
        pub fn is_age_selected(&self) -> ::core::primitive::bool {
            !<<u32 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
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
            <<u32 as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.age)
        }
        /// Select the `User.email` field
        #[inline]
        pub fn select_email(&mut self) -> &mut Self {
            self.email = <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unselect the `User.email` field
        #[inline]
        pub fn unselect_email(&mut self) -> &mut Self {
            self.email = <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Update the `User.email` field
        #[inline]
        pub fn update_email(
            &mut self,
            value: <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> &mut Self {
            self.email = value;
            self
        }
        /// Set or unset the `User.email` field
        #[inline]
        pub fn maybe_email(
            mut self,
            val: <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> Self {
            self.email = val;
            self
        }
        /// Get a reference to the selector of `User.email` field
        #[inline]
        pub const fn email_ref(
            &self,
        ) -> &<::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &self.email
        }
        /// Get a mutable reference to the selector of `User.email` field
        #[inline]
        pub const fn email_mut(
            &mut self,
        ) -> &mut <::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &mut self.email
        }
        /// Set the `User.email` field
        #[inline]
        pub fn with_email(mut self) -> Self {
            self.email = <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unset the `User.email` field
        #[inline]
        pub fn without_email(mut self) -> Self {
            self.email = <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Returns `true` if the `User.email` field is selected
        #[inline]
        pub fn is_email_selected(&self) -> ::core::primitive::bool {
            !<<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.email)
        }
        /// Returns `true` if the `User.email` field is unselected
        #[inline]
        pub fn is_email_unselected(&self) -> ::core::primitive::bool {
            <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <UserFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.email)
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        __GROST_FLAVOR__,
        const N: ::core::primitive::bool,
    > UserSelectorIter<'__grost_lifetime__, __GROST_FLAVOR__, N>
    where
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        u32: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <UserFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        #[inline]
        const fn new(
            selector: &'__grost_lifetime__ UserSelector<__GROST_FLAVOR__>,
            num: ::core::primitive::usize,
        ) -> Self {
            Self {
                selector,
                index: ::core::option::Option::Some(UserFieldIndex::FIRST),
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
const _: () = {
    ::grost::__private::encoded_state(
        & '__grost_lifetime__::grost::__private::flavors::Network : User as
        ::grost::__private::flavors::network::LengthDelimited => PartialUserRef <
        '__grost_lifetime__, ::grost::__private::flavors::Network >
    );
    ::grost::__private::encoded_state(
        & '__grost_lifetime__::grost::__private::flavors::Network : PartialUser as
        ::grost::__private::flavors::network::LengthDelimited => PartialUserRef <
        '__grost_lifetime__, ::grost::__private::flavors::Network >
    );
    ::grost::__private::encoded_state(
        & '__grost_lifetime__::grost::__private::flavors::Network : PartialUserRef <
        '__grost_lifetime__, ::grost::__private::flavors::Network > as
        ::grost::__private::flavors::network::LengthDelimited => PartialUserRef <
        '__grost_lifetime__, ::grost::__private::flavors::Network >
    );
    #[automatically_derived]
    impl ::grost::__private::flavors::DefaultWireFormat<
        ::grost::__private::flavors::Network,
    > for User {
        type Format = ::grost::__private::flavors::network::LengthDelimited;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for User {
        type Selector = UserSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::Repeated<
            ::grost::__private::flavors::network::LengthDelimited,
        >,
    > for User {
        type Selector = UserSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        const I: ::core::primitive::u32,
    > ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::Repeated<
            ::grost::__private::flavors::network::Stream<
                ::grost::__private::flavors::network::LengthDelimited,
                I,
            >,
        >,
    > for User {
        type Selector = UserSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for PartialUser {
        type Selector = UserSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::Repeated<
            ::grost::__private::flavors::network::LengthDelimited,
        >,
    > for PartialUser {
        type Selector = UserSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        const I: ::core::primitive::u32,
    > ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::Repeated<
            ::grost::__private::flavors::network::Stream<
                ::grost::__private::flavors::network::LengthDelimited,
                I,
            >,
        >,
    > for PartialUser {
        type Selector = UserSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        '__grost_lifetime__,
    > ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for PartialUserRef<'__grost_lifetime__, ::grost::__private::flavors::Network> {
        type Selector = UserSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        '__grost_lifetime__,
    > ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::Repeated<
            ::grost::__private::flavors::network::LengthDelimited,
        >,
    > for PartialUserRef<'__grost_lifetime__, ::grost::__private::flavors::Network> {
        type Selector = UserSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        '__grost_lifetime__,
        const I: ::core::primitive::u32,
    > ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::Repeated<
            ::grost::__private::flavors::network::Stream<
                ::grost::__private::flavors::network::LengthDelimited,
                I,
            >,
        >,
    > for PartialUserRef<'__grost_lifetime__, ::grost::__private::flavors::Network> {
        type Selector = UserSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    impl<'__grost_lifetime__, const N: ::core::primitive::bool> ::core::iter::Iterator
    for UserSelectorIter<'__grost_lifetime__, ::grost::__private::flavors::Network, N> {
        type Item = &'static ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >;
        fn next(&mut self) -> ::core::option::Option<Self::Item> {
            loop {
                if self.yielded >= self.num {
                    return ::core::option::Option::None;
                }
                match self.index {
                    ::core::option::Option::Some(index) => {
                        match self.selector[(index, N)] {
                            ::core::option::Option::Some(reflection) => {
                                self.index = index.next();
                                self.yielded += 1;
                                return ::core::option::Option::Some(reflection);
                            }
                            ::core::option::Option::None => {
                                self.index = index.next();
                            }
                        }
                    }
                    ::core::option::Option::None => return ::core::option::Option::None,
                }
            }
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
    impl<
        '__grost_lifetime__,
        const N: ::core::primitive::bool,
    > ::core::iter::FusedIterator
    for UserSelectorIter<'__grost_lifetime__, ::grost::__private::flavors::Network, N> {}
    #[automatically_derived]
    impl<
        '__grost_lifetime__,
        const N: ::core::primitive::bool,
    > ::core::iter::ExactSizeIterator
    for UserSelectorIter<'__grost_lifetime__, ::grost::__private::flavors::Network, N> {
        #[inline]
        fn len(&self) -> ::core::primitive::usize {
            self.remaining()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >;
        const REFLECTION: &Self::Reflection = &::grost::__private::reflection::ObjectFieldReflectionBuilder::<
            ::grost::__private::flavors::Network,
        > {
            identifier: ::grost::__private::flavors::network::Identifier::new(
                <<::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                ::grost::__private::flavors::network::Tag::new(1u32),
            ),
            name: "name",
            ty: ::core::any::type_name::<::std::string::String>,
            schema_name: "name",
            schema_type: ::grost::__private::reflection::Type::<
                ::grost::__private::flavors::Network,
            >::Primitive {
                name: "String!",
                description: "",
            },
        }
            .build();
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >;
        const REFLECTION: &Self::Reflection = &::grost::__private::reflection::ObjectFieldReflectionBuilder::<
            ::grost::__private::flavors::Network,
        > {
            identifier: ::grost::__private::flavors::network::Identifier::new(
                <<u32 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                ::grost::__private::flavors::network::Tag::new(2u32),
            ),
            name: "age",
            ty: ::core::any::type_name::<u32>,
            schema_name: "age",
            schema_type: ::grost::__private::reflection::Type::<
                ::grost::__private::flavors::Network,
            >::Primitive {
                name: "u32!",
                description: "",
            },
        }
            .build();
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >;
        const REFLECTION: &Self::Reflection = &::grost::__private::reflection::ObjectFieldReflectionBuilder::<
            ::grost::__private::flavors::Network,
        > {
            identifier: ::grost::__private::flavors::network::Identifier::new(
                <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                ::grost::__private::flavors::network::Tag::new(3u32),
            ),
            name: "email",
            ty: ::core::any::type_name::<::core::option::Option<::std::string::String>>,
            schema_name: "email",
            schema_type: ::grost::__private::reflection::Type::<
                ::grost::__private::flavors::Network,
            >::Optional(
                &::grost::__private::reflection::Type::<
                    ::grost::__private::flavors::Network,
                >::Primitive {
                    name: "String",
                    description: "",
                },
            ),
        }
            .build();
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::TagReflection<
            ::grost::__private::flavors::network::Tag,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = ::grost::__private::flavors::network::Tag;
        const REFLECTION: &Self::Reflection = &{
            <UserFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .tag()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::TagReflection<
            ::grost::__private::flavors::network::Tag,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = ::grost::__private::flavors::network::Tag;
        const REFLECTION: &Self::Reflection = &{
            <UserFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .tag()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::TagReflection<
            ::grost::__private::flavors::network::Tag,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = ::grost::__private::flavors::network::Tag;
        const REFLECTION: &Self::Reflection = &{
            <UserFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .tag()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::EncodedTagReflection<
            ::grost::__private::flavors::network::Tag,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            <UserFieldReflection<
                ::grost::__private::reflection::TagReflection<
                    ::grost::__private::flavors::network::Tag,
                >,
                ::grost::__private::flavors::Network,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .encode()
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::EncodedTagReflection<
            ::grost::__private::flavors::network::Tag,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            <UserFieldReflection<
                ::grost::__private::reflection::TagReflection<
                    ::grost::__private::flavors::network::Tag,
                >,
                ::grost::__private::flavors::Network,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .encode()
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::EncodedTagReflection<
            ::grost::__private::flavors::network::Tag,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            <UserFieldReflection<
                ::grost::__private::reflection::TagReflection<
                    ::grost::__private::flavors::network::Tag,
                >,
                ::grost::__private::flavors::Network,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .encode()
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodedTagReflection<
                ::grost::__private::flavors::network::Tag,
            >,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            <UserFieldReflection<
                ::grost::__private::reflection::EncodedTagReflection<
                    ::grost::__private::flavors::network::Tag,
                >,
                ::grost::__private::flavors::Network,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodedTagReflection<
                ::grost::__private::flavors::network::Tag,
            >,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            <UserFieldReflection<
                ::grost::__private::reflection::EncodedTagReflection<
                    ::grost::__private::flavors::network::Tag,
                >,
                ::grost::__private::flavors::Network,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodedTagReflection<
                ::grost::__private::flavors::network::Tag,
            >,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            <UserFieldReflection<
                ::grost::__private::reflection::EncodedTagReflection<
                    ::grost::__private::flavors::network::Tag,
                >,
                ::grost::__private::flavors::Network,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = ::grost::__private::flavors::network::Identifier;
        const REFLECTION: &Self::Reflection = &{
            <UserFieldReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
                ::grost::__private::flavors::Network,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .identifier()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = ::grost::__private::flavors::network::Identifier;
        const REFLECTION: &Self::Reflection = &{
            <UserFieldReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
                ::grost::__private::flavors::Network,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .identifier()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = ::grost::__private::flavors::network::Identifier;
        const REFLECTION: &Self::Reflection = &{
            <UserFieldReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
                ::grost::__private::flavors::Network,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .identifier()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::EncodedIdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            <UserFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .encode()
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::EncodedIdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            <UserFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .encode()
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::EncodedIdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            <UserFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .encode()
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodedIdentifierReflection<
                ::grost::__private::flavors::network::Identifier,
            >,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            <UserFieldReflection<
                ::grost::__private::reflection::EncodedIdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodedIdentifierReflection<
                ::grost::__private::flavors::network::Identifier,
            >,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            <UserFieldReflection<
                ::grost::__private::reflection::EncodedIdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodedIdentifierReflection<
                ::grost::__private::flavors::network::Identifier,
            >,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            <UserFieldReflection<
                ::grost::__private::reflection::EncodedIdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::WireTypeReflection<
            ::grost::__private::flavors::network::WireType,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = ::grost::__private::flavors::network::WireType;
        const REFLECTION: &::grost::__private::flavors::network::WireType = &{
            <UserFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .wire_type()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::WireTypeReflection<
            ::grost::__private::flavors::network::WireType,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = ::grost::__private::flavors::network::WireType;
        const REFLECTION: &::grost::__private::flavors::network::WireType = &{
            <UserFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .wire_type()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::WireTypeReflection<
            ::grost::__private::flavors::network::WireType,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = ::grost::__private::flavors::network::WireType;
        const REFLECTION: &::grost::__private::flavors::network::WireType = &{
            <UserFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .wire_type()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format;
        const REFLECTION: &<::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format = &{
            <<::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::SELF
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = <u32 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format;
        const REFLECTION: &<u32 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format = &{
            <<u32 as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::SELF
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = <::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format;
        const REFLECTION: &<::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format = &{
            <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::SELF
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    > for User {
        type Reflection = ::grost::__private::reflection::ObjectReflection<
            ::grost::__private::flavors::Network,
        >;
        const REFLECTION: &Self::Reflection = &::grost::__private::reflection::ObjectReflectionBuilder::<
            ::grost::__private::flavors::Network,
        > {
            name: "User",
            schema_name: "User",
            fields: &[
                <UserFieldReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        ::grost::__private::flavors::Network,
                    >,
                    ::grost::__private::flavors::Network,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    ::grost::__private::flavors::Network,
                >>::REFLECTION,
                <UserFieldReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        ::grost::__private::flavors::Network,
                    >,
                    ::grost::__private::flavors::Network,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    ::grost::__private::flavors::Network,
                >>::REFLECTION,
                <UserFieldReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        ::grost::__private::flavors::Network,
                    >,
                    ::grost::__private::flavors::Network,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    ::grost::__private::flavors::Network,
                >>::REFLECTION,
            ],
        }
            .build();
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserReflection<
        ::grost::__private::reflection::ObjectReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
    > {
        type Reflection = ::grost::__private::reflection::ObjectReflection<
            ::grost::__private::flavors::Network,
        >;
        const REFLECTION: &Self::Reflection = <User as ::grost::__private::reflection::Reflectable<
            ::grost::__private::flavors::Network,
        >>::REFLECTION;
    }
    #[automatically_derived]
    impl ::core::ops::Index<(UserFieldIndex, ::core::primitive::bool)>
    for UserSelector<::grost::__private::flavors::Network> {
        type Output = ::core::option::Option<
            &'static ::grost::__private::reflection::ObjectFieldReflection<
                ::grost::__private::flavors::Network,
            >,
        >;
        fn index(
            &self,
            (indexer, select): (UserFieldIndex, ::core::primitive::bool),
        ) -> &Self::Output {
            const NONE: &::core::option::Option<
                &'static ::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
            > = &::core::option::Option::None;
            match indexer {
                UserFieldIndex::Name => {
                    const REFLECTION: ::core::option::Option<
                        &::grost::__private::reflection::ObjectFieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                    > = ::core::option::Option::Some(
                        <UserFieldReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                            ::grost::__private::flavors::Network,
                            1u32,
                        > as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    );
                    match (select, self.name) {
                        (true, true) => &REFLECTION,
                        (true, false) => NONE,
                        (false, true) => NONE,
                        (false, false) => &REFLECTION,
                    }
                }
                UserFieldIndex::Age => {
                    const REFLECTION: ::core::option::Option<
                        &::grost::__private::reflection::ObjectFieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                    > = ::core::option::Option::Some(
                        <UserFieldReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                            ::grost::__private::flavors::Network,
                            2u32,
                        > as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    );
                    match (select, self.age) {
                        (true, true) => &REFLECTION,
                        (true, false) => NONE,
                        (false, true) => NONE,
                        (false, false) => &REFLECTION,
                    }
                }
                UserFieldIndex::Email => {
                    const REFLECTION: ::core::option::Option<
                        &::grost::__private::reflection::ObjectFieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                    > = ::core::option::Option::Some(
                        <UserFieldReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                            ::grost::__private::flavors::Network,
                            3u32,
                        > as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    );
                    match (select, self.email) {
                        (true, true) => &REFLECTION,
                        (true, false) => NONE,
                        (false, true) => NONE,
                        (false, false) => &REFLECTION,
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::ops::Index<UserFieldIndex>
    for UserSelector<::grost::__private::flavors::Network> {
        type Output = ::core::option::Option<
            &'static ::grost::__private::reflection::ObjectFieldReflection<
                ::grost::__private::flavors::Network,
            >,
        >;
        fn index(&self, indexer: UserFieldIndex) -> &Self::Output {
            const NONE: &::core::option::Option<
                &::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
            > = &::core::option::Option::None;
            match indexer {
                UserFieldIndex::Name => {
                    const REFLECTION: ::core::option::Option<
                        &::grost::__private::reflection::ObjectFieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                    > = ::core::option::Option::Some(
                        <UserFieldReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                            ::grost::__private::flavors::Network,
                            1u32,
                        > as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    );
                    if ::grost::__private::Selector::<
                        ::grost::__private::flavors::Network,
                    >::is_empty(&self.name) {
                        NONE
                    } else {
                        &REFLECTION
                    }
                }
                UserFieldIndex::Age => {
                    const REFLECTION: ::core::option::Option<
                        &::grost::__private::reflection::ObjectFieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                    > = ::core::option::Option::Some(
                        <UserFieldReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                            ::grost::__private::flavors::Network,
                            2u32,
                        > as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    );
                    if ::grost::__private::Selector::<
                        ::grost::__private::flavors::Network,
                    >::is_empty(&self.age) {
                        NONE
                    } else {
                        &REFLECTION
                    }
                }
                UserFieldIndex::Email => {
                    const REFLECTION: ::core::option::Option<
                        &::grost::__private::reflection::ObjectFieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                    > = ::core::option::Option::Some(
                        <UserFieldReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                            ::grost::__private::flavors::Network,
                            3u32,
                        > as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    );
                    if ::grost::__private::Selector::<
                        ::grost::__private::flavors::Network,
                    >::is_empty(&self.email) {
                        NONE
                    } else {
                        &REFLECTION
                    }
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::grost::__private::PartialEncode<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for PartialUser {
        fn partial_encode(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
            selector: &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let mut offset = 0;
            let buf_len = buf.len();
            if let ::core::option::Option::Some(ref f) = self.name {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .name()
                        .partial_encode())(f, ctx, &mut buf[offset..], &selector.name)
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::PartialEncode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
            }
            if let ::core::option::Option::Some(ref f) = self.age {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .age()
                        .partial_encode())(f, ctx, &mut buf[offset..], &selector.age)
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::PartialEncode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
            }
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            offset
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .email()
                    .partial_encode())(
                        &self.email,
                        ctx,
                        &mut buf[offset..],
                        &selector.email,
                    )
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        )
                    })?;
            ::core::result::Result::Ok(offset)
        }
        fn partial_encoded_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            selector: &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::primitive::usize {
            let mut len = 0;
            if let ::core::option::Option::Some(ref f) = self.name {
                len
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .name()
                        .partial_encoded_len())(f, ctx, &selector.name);
            }
            if let ::core::option::Option::Some(ref f) = self.age {
                len
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .age()
                        .partial_encoded_len())(f, ctx, &selector.age);
            }
            len
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .email()
                    .partial_encoded_len())(&self.email, ctx, &selector.email);
            len
        }
        fn partial_encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            selector: &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::primitive::usize {
            let encoded_len = <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encoded_len(self, ctx, selector);
            ::grost::__private::varing::encoded_u32_varint_len(
                encoded_len as ::core::primitive::u32,
            ) + encoded_len
        }
        fn partial_encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
            selector: &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let encoded_len = <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encoded_len(self, ctx, selector);
            let buf_len = buf.len();
            let offset = ::grost::__private::varing::encode_u32_varint_to(
                    encoded_len as ::core::primitive::u32,
                    buf,
                )
                .map_err(|e| {
                    ::grost::__private::flavors::network::EncodeError::from_varint_error(
                            e,
                        )
                        .update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_length_delimited_len(
                                self,
                                ctx,
                                selector,
                            ),
                            buf_len,
                        )
                })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_length_delimited_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encode(self, ctx, &mut buf[offset..], selector)
                .map(|write| {
                    #[cfg(debug_assertions)]
                    {
                        ::grost::__private::debug_assert_write_eq::<
                            Self,
                        >(write, encoded_len);
                    }
                    write + offset
                })
                .map_err(|e| {
                    e
                        .update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_length_delimited_len(
                                self,
                                ctx,
                                selector,
                            ),
                            buf_len,
                        )
                })
        }
    }
    #[automatically_derived]
    impl ::grost::__private::PartialEncode<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for User {
        fn partial_encode(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
            selector: &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let mut offset = 0;
            let buf_len = buf.len();
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            offset
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .name()
                    .partial_encode())(
                        &self.name,
                        ctx,
                        &mut buf[offset..],
                        &selector.name,
                    )
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        )
                    })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            offset
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .age()
                    .partial_encode())(&self.age, ctx, &mut buf[offset..], &selector.age)
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        )
                    })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            offset
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .email()
                    .partial_encode())(
                        &self.email,
                        ctx,
                        &mut buf[offset..],
                        &selector.email,
                    )
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        )
                    })?;
            ::core::result::Result::Ok(offset)
        }
        fn partial_encoded_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            selector: &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::primitive::usize {
            let mut len = 0;
            len
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .name()
                    .partial_encoded_len())(&self.name, ctx, &selector.name);
            len
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .age()
                    .partial_encoded_len())(&self.age, ctx, &selector.age);
            len
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .email()
                    .partial_encoded_len())(&self.email, ctx, &selector.email);
            len
        }
        fn partial_encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            selector: &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::primitive::usize {
            let encoded_len = <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encoded_len(self, ctx, selector);
            ::grost::__private::varing::encoded_u32_varint_len(
                encoded_len as ::core::primitive::u32,
            ) + encoded_len
        }
        fn partial_encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
            selector: &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let encoded_len = <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encoded_len(self, ctx, selector);
            let buf_len = buf.len();
            let offset = ::grost::__private::varing::encode_u32_varint_to(
                    encoded_len as ::core::primitive::u32,
                    buf,
                )
                .map_err(|e| {
                    ::grost::__private::flavors::network::EncodeError::from_varint_error(
                            e,
                        )
                        .update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_length_delimited_len(
                                self,
                                ctx,
                                selector,
                            ),
                            buf_len,
                        )
                })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_length_delimited_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encode(self, ctx, &mut buf[offset..], selector)
                .map(|write| {
                    #[cfg(debug_assertions)]
                    {
                        ::grost::__private::debug_assert_write_eq::<
                            Self,
                        >(write, encoded_len);
                    }
                    write + offset
                })
                .map_err(|e| {
                    e
                        .update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_length_delimited_len(
                                self,
                                ctx,
                                selector,
                            ),
                            buf_len,
                        )
                })
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        '__grost_lifetime__,
    > ::grost::__private::PartialEncode<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for PartialUserRef<'__grost_lifetime__, ::grost::__private::flavors::Network> {
        fn partial_encode(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
            selector: &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let mut offset = 0;
            let buf_len = buf.len();
            if let ::core::option::Option::Some(ref f) = self.name {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .name()
                        .partial_encode_ref())(
                            f,
                            ctx,
                            &mut buf[offset..],
                            &selector.name,
                        )
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::PartialEncode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
            }
            if let ::core::option::Option::Some(ref f) = self.age {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .age()
                        .partial_encode_ref())(f, ctx, &mut buf[offset..], &selector.age)
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::PartialEncode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
            }
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            offset
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .email()
                    .partial_encode_ref())(
                        &self.email,
                        ctx,
                        &mut buf[offset..],
                        &selector.email,
                    )
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        )
                    })?;
            ::core::result::Result::Ok(offset)
        }
        fn partial_encoded_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            selector: &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::primitive::usize {
            let mut len = 0;
            if let ::core::option::Option::Some(ref f) = self.name {
                len
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .name()
                        .partial_encoded_ref_len())(f, ctx, &selector.name);
            }
            if let ::core::option::Option::Some(ref f) = self.age {
                len
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .age()
                        .partial_encoded_ref_len())(f, ctx, &selector.age);
            }
            len
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .email()
                    .partial_encoded_ref_len())(&self.email, ctx, &selector.email);
            len
        }
        fn partial_encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            selector: &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::primitive::usize {
            let encoded_len = <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encoded_len(self, ctx, selector);
            ::grost::__private::varing::encoded_u32_varint_len(
                encoded_len as ::core::primitive::u32,
            ) + encoded_len
        }
        fn partial_encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
            selector: &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let encoded_len = <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encoded_len(self, ctx, selector);
            let buf_len = buf.len();
            let offset = ::grost::__private::varing::encode_u32_varint_to(
                    encoded_len as ::core::primitive::u32,
                    buf,
                )
                .map_err(|e| {
                    ::grost::__private::flavors::network::EncodeError::from_varint_error(
                            e,
                        )
                        .update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_length_delimited_len(
                                self,
                                ctx,
                                selector,
                            ),
                            buf_len,
                        )
                })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_length_delimited_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encode(self, ctx, &mut buf[offset..], selector)
                .map(|write| {
                    #[cfg(debug_assertions)]
                    {
                        ::grost::__private::debug_assert_write_eq::<
                            Self,
                        >(write, encoded_len);
                    }
                    write + offset
                })
                .map_err(|e| {
                    e
                        .update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_length_delimited_len(
                                self,
                                ctx,
                                selector,
                            ),
                            buf_len,
                        )
                })
        }
    }
    #[automatically_derived]
    impl ::grost::__private::Encode<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for User {
        fn encode(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let mut offset = 0;
            let buf_len = buf.len();
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            offset
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .name()
                    .encode())(&self.name, ctx, &mut buf[offset..])
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        )
                    })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            offset
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .age()
                    .encode())(&self.age, ctx, &mut buf[offset..])
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        )
                    })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            offset
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .email()
                    .encode())(&self.email, ctx, &mut buf[offset..])
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        )
                    })?;
            ::core::result::Result::Ok(offset)
        }
        fn encoded_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
            let mut len = 0;
            len
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .name()
                    .encoded_len())(&self.name, ctx);
            len
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .age()
                    .encoded_len())(&self.age, ctx);
            len
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .email()
                    .encoded_len())(&self.email, ctx);
            len
        }
        fn encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
            let encoded_len = <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encoded_len(self, ctx);
            ::grost::__private::varing::encoded_u32_varint_len(
                encoded_len as ::core::primitive::u32,
            ) + encoded_len
        }
        fn encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let encoded_len = <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encoded_len(self, ctx);
            let buf_len = buf.len();
            let offset = ::grost::__private::varing::encode_u32_varint_to(
                    encoded_len as ::core::primitive::u32,
                    buf,
                )
                .map_err(|e| {
                    ::grost::__private::flavors::network::EncodeError::from_varint_error(
                            e,
                        )
                        .update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_length_delimited_len(self, ctx),
                            buf_len,
                        )
                })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_length_delimited_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encode(self, ctx, &mut buf[offset..])
                .map(|write| {
                    #[cfg(debug_assertions)]
                    {
                        ::grost::__private::debug_assert_write_eq::<
                            Self,
                        >(write, encoded_len);
                    }
                    write + offset
                })
                .map_err(|e| {
                    e
                        .update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_length_delimited_len(self, ctx),
                            buf_len,
                        )
                })
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::grost::__private::Encode<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for PartialUser {
        fn encode(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let mut offset = 0;
            let buf_len = buf.len();
            if let ::core::option::Option::Some(ref f) = self.name {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .name()
                        .encode())(f, ctx, &mut buf[offset..])
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::Encode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
            }
            if let ::core::option::Option::Some(ref f) = self.age {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .age()
                        .encode())(f, ctx, &mut buf[offset..])
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::Encode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
            }
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            offset
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .email()
                    .encode())(&self.email, ctx, &mut buf[offset..])
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        )
                    })?;
            ::core::result::Result::Ok(offset)
        }
        fn encoded_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
            let mut len = 0;
            if let ::core::option::Option::Some(ref f) = self.name {
                len
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .name()
                        .encoded_len())(f, ctx);
            }
            if let ::core::option::Option::Some(ref f) = self.age {
                len
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .age()
                        .encoded_len())(f, ctx);
            }
            len
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .email()
                    .encoded_len())(&self.email, ctx);
            len
        }
        fn encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
            let encoded_len = <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encoded_len(self, ctx);
            ::grost::__private::varing::encoded_u32_varint_len(
                encoded_len as ::core::primitive::u32,
            ) + encoded_len
        }
        fn encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let encoded_len = <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encoded_len(self, ctx);
            let buf_len = buf.len();
            let offset = ::grost::__private::varing::encode_u32_varint_to(
                    encoded_len as ::core::primitive::u32,
                    buf,
                )
                .map_err(|e| {
                    ::grost::__private::flavors::network::EncodeError::from_varint_error(
                            e,
                        )
                        .update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_length_delimited_len(self, ctx),
                            buf_len,
                        )
                })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_length_delimited_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encode(self, ctx, &mut buf[offset..])
                .map(|write| {
                    #[cfg(debug_assertions)]
                    {
                        ::grost::__private::debug_assert_write_eq::<
                            Self,
                        >(write, encoded_len);
                    }
                    write + offset
                })
                .map_err(|e| {
                    e
                        .update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_length_delimited_len(self, ctx),
                            buf_len,
                        )
                })
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        '__grost_flavor__,
    > ::grost::__private::Encode<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for PartialUserRef<'__grost_flavor__, ::grost::__private::flavors::Network> {
        fn encode(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let mut offset = 0;
            let buf_len = buf.len();
            if let ::core::option::Option::Some(ref f) = self.name {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .name()
                        .encode_ref())(f, ctx, &mut buf[offset..])
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::Encode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
            }
            if let ::core::option::Option::Some(ref f) = self.age {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .age()
                        .encode_ref())(f, ctx, &mut buf[offset..])
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::Encode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
            }
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            offset
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .email()
                    .encode_ref())(&self.email, ctx, &mut buf[offset..])
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        )
                    })?;
            ::core::result::Result::Ok(offset)
        }
        fn encoded_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
            let mut len = 0;
            if let ::core::option::Option::Some(ref f) = self.name {
                len
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .name()
                        .encoded_ref_len())(f, ctx);
            }
            if let ::core::option::Option::Some(ref f) = self.age {
                len
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .age()
                        .encoded_ref_len())(f, ctx);
            }
            len
                += (<User>::reflection::<::grost::__private::flavors::Network>()
                    .email()
                    .encoded_ref_len())(&self.email, ctx);
            len
        }
        fn encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
            let encoded_len = <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encoded_len(self, ctx);
            ::grost::__private::varing::encoded_u32_varint_len(
                encoded_len as ::core::primitive::u32,
            ) + encoded_len
        }
        fn encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let encoded_len = <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encoded_len(self, ctx);
            let buf_len = buf.len();
            let offset = ::grost::__private::varing::encode_u32_varint_to(
                    encoded_len as ::core::primitive::u32,
                    buf,
                )
                .map_err(|e| {
                    ::grost::__private::flavors::network::EncodeError::from_varint_error(
                            e,
                        )
                        .update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_length_delimited_len(self, ctx),
                            buf_len,
                        )
                })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_length_delimited_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encode(self, ctx, &mut buf[offset..])
                .map(|write| {
                    #[cfg(debug_assertions)]
                    {
                        ::grost::__private::debug_assert_write_eq::<
                            Self,
                        >(write, encoded_len);
                    }
                    write + offset
                })
                .map_err(|e| {
                    e
                        .update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_length_delimited_len(self, ctx),
                            buf_len,
                        )
                })
        }
    }
    fn insufficient_buffer_error<T, W>(
        f: &T,
        ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
        selector: ::core::option::Option<
            &<T as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                W,
            >>::Selector,
        >,
        buf_len: ::core::primitive::usize,
    ) -> ::grost::__private::flavors::network::EncodeError
    where
        T: ::grost::__private::PartialEncode<::grost::__private::flavors::Network, W>
            + ::grost::__private::Encode<::grost::__private::flavors::Network, W>
            + ::grost::__private::Selectable<::grost::__private::flavors::Network, W>
            + ?::core::marker::Sized,
        W: ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
    {
        match selector {
            ::core::option::Option::Some(selector) => {
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    <T as ::grost::__private::PartialEncode<
                        ::grost::__private::flavors::Network,
                        W,
                    >>::partial_encoded_len(f, ctx, selector),
                    buf_len,
                )
            }
            ::core::option::Option::None => {
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    <T as ::grost::__private::Encode<
                        ::grost::__private::flavors::Network,
                        W,
                    >>::encoded_length_delimited_len(f, ctx),
                    buf_len,
                )
            }
        }
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::PartialEncodeField,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &::std::string::String,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
            &<::std::string::String as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &::std::string::String,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                const ENCODED_LEN_FN: UserFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::PartialEncodeField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    1u32,
                > = <User>::reflection::<::grost::__private::flavors::Network>()
                    .name()
                    .partial_encoded_len();
                let identifier_len = *<User>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .name()
                    .encoded_identifier_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<User>::reflection::<::grost::__private::flavors::Network>()
                            .name()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                <::std::string::String as ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::partial_encode_length_delimited(f, ctx, &mut buf[offset..], selector)
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::PartialEncodeField,
            >,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &::std::string::String,
            &::grost::__private::flavors::network::Context,
            &<::std::string::String as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::std::string::String,
                ctx: &::grost::__private::flavors::network::Context,
                selector: &<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::primitive::usize {
                (*<User>::reflection::<::grost::__private::flavors::Network>()
                    .name()
                    .encoded_identifier_len())
                    + <::std::string::String as ::grost::__private::PartialEncode<
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::partial_encoded_length_delimited_len(f, ctx, selector)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::EncodeField,
            >,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &::std::string::String,
            &::grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::std::string::String,
                ctx: &::grost::__private::flavors::network::Context,
            ) -> ::core::primitive::usize {
                (*<User>::reflection::<::grost::__private::flavors::Network>()
                    .name()
                    .encoded_identifier_len())
                    + <::std::string::String as ::grost::__private::Encode<
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::encoded_length_delimited_len(f, ctx)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::EncodeField,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &::std::string::String,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &::std::string::String,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                const ENCODED_LEN_FN: UserFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::EncodeField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    1u32,
                > = <User>::reflection::<::grost::__private::flavors::Network>()
                    .name()
                    .encoded_len();
                let identifier_len = *<User>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .name()
                    .encoded_identifier_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<User>::reflection::<::grost::__private::flavors::Network>()
                            .name()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                <::std::string::String as ::grost::__private::Encode<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::encode_length_delimited(f, ctx, &mut buf[offset..])
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::PartialEncodeRefField,
            >,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &<::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
            &<::std::string::String as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &<::std::string::String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
                selector: &<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::primitive::usize {
                (*<User>::reflection::<::grost::__private::flavors::Network>()
                    .name()
                    .encoded_identifier_len())
                    + <<::std::string::String as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output as ::grost::__private::PartialEncode<
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::partial_encoded_length_delimited_len(f, ctx, selector)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::PartialEncodeRefField,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &<::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
            &<::std::string::String as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &<::std::string::String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                let identifier_len = *<User>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .name()
                    .encoded_identifier_len();
                const ENCODED_LEN_FN: UserFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::PartialEncodeRefField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    1u32,
                > = <User>::reflection::<::grost::__private::flavors::Network>()
                    .name()
                    .partial_encoded_ref_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<User>::reflection::<::grost::__private::flavors::Network>()
                            .name()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                <<::std::string::String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output as ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::partial_encode_length_delimited(f, ctx, buf, selector)
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::EncodeRefField,
            >,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &<::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &<::std::string::String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
            ) -> ::core::primitive::usize {
                (*<User>::reflection::<::grost::__private::flavors::Network>()
                    .name()
                    .encoded_identifier_len())
                    + <<::std::string::String as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output as ::grost::__private::Encode<
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::encoded_length_delimited_len(f, ctx)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::EncodeRefField,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &<::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &<::std::string::String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                let identifier_len = *<User>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .name()
                    .encoded_identifier_len();
                const ENCODED_LEN_FN: UserFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::EncodeRefField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    1u32,
                > = <User>::reflection::<::grost::__private::flavors::Network>()
                    .name()
                    .encoded_ref_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<User>::reflection::<::grost::__private::flavors::Network>()
                            .name()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                <<::std::string::String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output as ::grost::__private::Encode<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::encode_length_delimited(f, ctx, buf)
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::PartialEncodeField,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &u32,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
            &<u32 as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <u32 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &u32,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<u32 as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <u32 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                const ENCODED_LEN_FN: UserFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::PartialEncodeField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    2u32,
                > = <User>::reflection::<::grost::__private::flavors::Network>()
                    .age()
                    .partial_encoded_len();
                let identifier_len = *<User>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .age()
                    .encoded_identifier_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<User>::reflection::<::grost::__private::flavors::Network>()
                            .age()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                <u32 as ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::Network,
                    <u32 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::partial_encode_length_delimited(f, ctx, &mut buf[offset..], selector)
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::PartialEncodeField,
            >,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &u32,
            &::grost::__private::flavors::network::Context,
            &<u32 as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <u32 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &u32,
                ctx: &::grost::__private::flavors::network::Context,
                selector: &<u32 as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <u32 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::primitive::usize {
                (*<User>::reflection::<::grost::__private::flavors::Network>()
                    .age()
                    .encoded_identifier_len())
                    + <u32 as ::grost::__private::PartialEncode<
                        ::grost::__private::flavors::Network,
                        <u32 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::partial_encoded_length_delimited_len(f, ctx, selector)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::EncodeField,
            >,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &u32,
            &::grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &u32,
                ctx: &::grost::__private::flavors::network::Context,
            ) -> ::core::primitive::usize {
                (*<User>::reflection::<::grost::__private::flavors::Network>()
                    .age()
                    .encoded_identifier_len())
                    + <u32 as ::grost::__private::Encode<
                        ::grost::__private::flavors::Network,
                        <u32 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::encoded_length_delimited_len(f, ctx)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::EncodeField,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &u32,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &u32,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                const ENCODED_LEN_FN: UserFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::EncodeField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    2u32,
                > = <User>::reflection::<::grost::__private::flavors::Network>()
                    .age()
                    .encoded_len();
                let identifier_len = *<User>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .age()
                    .encoded_identifier_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<User>::reflection::<::grost::__private::flavors::Network>()
                            .age()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                <u32 as ::grost::__private::Encode<
                    ::grost::__private::flavors::Network,
                    <u32 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::encode_length_delimited(f, ctx, &mut buf[offset..])
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::PartialEncodeRefField,
            >,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &<u32 as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <u32 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
            &<u32 as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <u32 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &<u32 as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <u32 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
                selector: &<u32 as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <u32 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::primitive::usize {
                (*<User>::reflection::<::grost::__private::flavors::Network>()
                    .age()
                    .encoded_identifier_len())
                    + <<u32 as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <u32 as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output as ::grost::__private::PartialEncode<
                        ::grost::__private::flavors::Network,
                        <u32 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::partial_encoded_length_delimited_len(f, ctx, selector)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::PartialEncodeRefField,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &<u32 as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <u32 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
            &<u32 as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <u32 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &<u32 as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <u32 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<u32 as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <u32 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                let identifier_len = *<User>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .age()
                    .encoded_identifier_len();
                const ENCODED_LEN_FN: UserFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::PartialEncodeRefField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    2u32,
                > = <User>::reflection::<::grost::__private::flavors::Network>()
                    .age()
                    .partial_encoded_ref_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<User>::reflection::<::grost::__private::flavors::Network>()
                            .age()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                <<u32 as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <u32 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output as ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::Network,
                    <u32 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::partial_encode_length_delimited(f, ctx, buf, selector)
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::EncodeRefField,
            >,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &<u32 as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <u32 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &<u32 as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <u32 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
            ) -> ::core::primitive::usize {
                (*<User>::reflection::<::grost::__private::flavors::Network>()
                    .age()
                    .encoded_identifier_len())
                    + <<u32 as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <u32 as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output as ::grost::__private::Encode<
                        ::grost::__private::flavors::Network,
                        <u32 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::encoded_length_delimited_len(f, ctx)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::EncodeRefField,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &<u32 as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <u32 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &<u32 as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <u32 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                let identifier_len = *<User>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .age()
                    .encoded_identifier_len();
                const ENCODED_LEN_FN: UserFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::EncodeRefField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    2u32,
                > = <User>::reflection::<::grost::__private::flavors::Network>()
                    .age()
                    .encoded_ref_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<User>::reflection::<::grost::__private::flavors::Network>()
                            .age()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                <<u32 as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <u32 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output as ::grost::__private::Encode<
                    ::grost::__private::flavors::Network,
                    <u32 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::encode_length_delimited(f, ctx, buf)
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::PartialEncodeField,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &::core::option::Option<::std::string::String>,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
            &<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &::core::option::Option<::std::string::String>,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                const ENCODED_LEN_FN: UserFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::PartialEncodeField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    3u32,
                > = <User>::reflection::<::grost::__private::flavors::Network>()
                    .email()
                    .partial_encoded_len();
                let identifier_len = *<User>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .email()
                    .encoded_identifier_len();
                match f {
                    ::core::option::Option::None => ::core::result::Result::Ok(0),
                    ::core::option::Option::Some(field) => {
                        let buf_len = buf.len();
                        let mut offset = 0;
                        if offset > buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(f, ctx, selector),
                                    buf_len,
                                ),
                            );
                        }
                        buf[offset..offset + identifier_len]
                            .copy_from_slice(
                                &<User>::reflection::<
                                    ::grost::__private::flavors::Network,
                                >()
                                    .email()
                                    .encoded_identifier(),
                            );
                        offset += identifier_len;
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(f, ctx, selector),
                                    buf_len,
                                ),
                            );
                        }
                        <::std::string::String as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::partial_encode_length_delimited(
                                field,
                                ctx,
                                &mut buf[offset..],
                                selector,
                            )
                            .map(|len| offset + len)
                            .map_err(|e| {
                                e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len)
                            })
                    }
                }
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::PartialEncodeField,
            >,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &::core::option::Option<::std::string::String>,
            &::grost::__private::flavors::network::Context,
            &<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::core::option::Option<::std::string::String>,
                ctx: &::grost::__private::flavors::network::Context,
                selector: &<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::primitive::usize {
                match f {
                    ::core::option::Option::None => 0,
                    ::core::option::Option::Some(f) => {
                        (*<User>::reflection::<::grost::__private::flavors::Network>()
                            .email()
                            .encoded_identifier_len())
                            + <::std::string::String as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >>::partial_encoded_length_delimited_len(f, ctx, selector)
                    }
                }
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::EncodeField,
            >,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &::core::option::Option<::std::string::String>,
            &::grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::core::option::Option<::std::string::String>,
                ctx: &::grost::__private::flavors::network::Context,
            ) -> ::core::primitive::usize {
                match f {
                    ::core::option::Option::None => 0,
                    ::core::option::Option::Some(f) => {
                        (*<User>::reflection::<::grost::__private::flavors::Network>()
                            .email()
                            .encoded_identifier_len())
                            + <::std::string::String as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >>::encoded_length_delimited_len(f, ctx)
                    }
                }
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::EncodeField,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &::core::option::Option<::std::string::String>,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &::core::option::Option<::std::string::String>,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                const ENCODED_LEN_FN: UserFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::EncodeField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    3u32,
                > = <User>::reflection::<::grost::__private::flavors::Network>()
                    .email()
                    .encoded_len();
                let identifier_len = *<User>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .email()
                    .encoded_identifier_len();
                match f {
                    ::core::option::Option::None => ::core::result::Result::Ok(0),
                    ::core::option::Option::Some(field) => {
                        let buf_len = buf.len();
                        let mut offset = 0;
                        if offset > buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(f, ctx),
                                    buf_len,
                                ),
                            );
                        }
                        buf[offset..offset + identifier_len]
                            .copy_from_slice(
                                &<User>::reflection::<
                                    ::grost::__private::flavors::Network,
                                >()
                                    .email()
                                    .encoded_identifier(),
                            );
                        offset += identifier_len;
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(f, ctx),
                                    buf_len,
                                ),
                            );
                        }
                        <::std::string::String as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::encode_length_delimited(field, ctx, &mut buf[offset..])
                            .map(|len| offset + len)
                            .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
                    }
                }
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::PartialEncodeRefField,
            >,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &::core::option::Option<
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
            >,
            &::grost::__private::flavors::network::Context,
            &<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::core::option::Option<
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output,
                >,
                ctx: &::grost::__private::flavors::network::Context,
                selector: &<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::primitive::usize {
                match f {
                    ::core::option::Option::None => 0,
                    ::core::option::Option::Some(f) => {
                        (*<User>::reflection::<::grost::__private::flavors::Network>()
                            .email()
                            .encoded_identifier_len())
                            + <<::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::convert::State<
                                ::grost::__private::convert::Encoded<
                                    '_,
                                    ::grost::__private::flavors::Network,
                                    <::core::option::Option<
                                        ::std::string::String,
                                    > as ::grost::__private::flavors::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                >,
                            >>::Output as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >>::partial_encoded_length_delimited_len(f, ctx, selector)
                    }
                }
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::PartialEncodeRefField,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &::core::option::Option<
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
            >,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
            &<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                field: &::core::option::Option<
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output,
                >,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                match field {
                    ::core::option::Option::None => ::core::result::Result::Ok(0),
                    ::core::option::Option::Some(f) => {
                        let identifier_len = *<User>::reflection::<
                            ::grost::__private::flavors::Network,
                        >()
                            .email()
                            .encoded_identifier_len();
                        const ENCODED_LEN_FN: UserFieldReflection<
                            ::grost::__private::reflection::encode::EncodeReflection<
                                ::grost::__private::reflection::Len<
                                    ::grost::__private::reflection::encode::PartialEncodeRefField,
                                >,
                            >,
                            ::grost::__private::flavors::Network,
                            3u32,
                        > = <User>::reflection::<::grost::__private::flavors::Network>()
                            .email()
                            .partial_encoded_ref_len();
                        let buf_len = buf.len();
                        let mut offset = 0;
                        if offset > buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(field, ctx, selector),
                                    buf_len,
                                ),
                            );
                        }
                        buf[offset..offset + identifier_len]
                            .copy_from_slice(
                                &<User>::reflection::<
                                    ::grost::__private::flavors::Network,
                                >()
                                    .email()
                                    .encoded_identifier(),
                            );
                        offset += identifier_len;
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(field, ctx, selector),
                                    buf_len,
                                ),
                            );
                        }
                        <<::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Encoded<
                                '_,
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >,
                        >>::Output as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::partial_encode_length_delimited(f, ctx, buf, selector)
                            .map(|len| offset + len)
                            .map_err(|e| {
                                e.update((ENCODED_LEN_FN)(field, ctx, selector), buf_len)
                            })
                    }
                }
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::EncodeRefField,
            >,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &::core::option::Option<
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
            >,
            &::grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::core::option::Option<
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output,
                >,
                ctx: &::grost::__private::flavors::network::Context,
            ) -> ::core::primitive::usize {
                match f {
                    ::core::option::Option::None => 0,
                    ::core::option::Option::Some(f) => {
                        (*<User>::reflection::<::grost::__private::flavors::Network>()
                            .email()
                            .encoded_identifier_len())
                            + <<::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::convert::State<
                                ::grost::__private::convert::Encoded<
                                    '_,
                                    ::grost::__private::flavors::Network,
                                    <::core::option::Option<
                                        ::std::string::String,
                                    > as ::grost::__private::flavors::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                >,
                            >>::Output as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >>::encoded_length_delimited_len(f, ctx)
                    }
                }
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for UserFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::EncodeRefField,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &::core::option::Option<
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
            >,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                field: &::core::option::Option<
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output,
                >,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                match field {
                    ::core::option::Option::Some(f) => {
                        let identifier_len = *<User>::reflection::<
                            ::grost::__private::flavors::Network,
                        >()
                            .email()
                            .encoded_identifier_len();
                        const ENCODED_LEN_FN: UserFieldReflection<
                            ::grost::__private::reflection::encode::EncodeReflection<
                                ::grost::__private::reflection::Len<
                                    ::grost::__private::reflection::encode::EncodeRefField,
                                >,
                            >,
                            ::grost::__private::flavors::Network,
                            3u32,
                        > = <User>::reflection::<::grost::__private::flavors::Network>()
                            .email()
                            .encoded_ref_len();
                        let buf_len = buf.len();
                        let mut offset = 0;
                        if offset > buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(field, ctx),
                                    buf_len,
                                ),
                            );
                        }
                        buf[offset..offset + identifier_len]
                            .copy_from_slice(
                                &<User>::reflection::<
                                    ::grost::__private::flavors::Network,
                                >()
                                    .email()
                                    .encoded_identifier(),
                            );
                        offset += identifier_len;
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(field, ctx),
                                    buf_len,
                                ),
                            );
                        }
                        <<::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Encoded<
                                '_,
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >,
                        >>::Output as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::encode_length_delimited(f, ctx, buf)
                            .map(|len| offset + len)
                            .map_err(|e| e.update((ENCODED_LEN_FN)(field, ctx), buf_len))
                    }
                    ::core::option::Option::None => ::core::result::Result::Ok(0),
                }
            }
            encode
        };
    }
};
const _: () = {
    #[automatically_derived]
    impl ::core::default::Default for Comment {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl Comment {
        /// Returns a new default instance of the struct
        pub fn new() -> Self {
            Self {
                user: ::core::default::Default::default(),
                replyer: ::core::default::Default::default(),
                title: ::core::default::Default::default(),
                content: ::core::default::Default::default(),
            }
        }
        /// Gets the reference of the field `user`.
        #[inline]
        pub fn user_ref(&self) -> &User {
            &self.user
        }
        /// Gets the mutable reference of the field `user`.
        #[inline]
        pub fn user_mut(&mut self) -> &mut User {
            &mut self.user
        }
        /// Sets the `user`.
        #[inline]
        pub fn set_user(&mut self, user: User) -> &mut Self {
            self.user = user;
            self
        }
        /// Sets the `user`.
        #[inline]
        pub fn with_user(mut self, user: User) -> Self {
            self.user = user;
            self
        }
        /// Gets the reference of the field `replyer`.
        #[inline]
        pub fn replyer_ref(&self) -> ::core::option::Option<&User> {
            ::core::option::Option::as_ref(&self.replyer)
        }
        /// Gets the mutable reference of the field `replyer`.
        #[inline]
        pub fn replyer_mut(&mut self) -> ::core::option::Option<&mut User> {
            ::core::option::Option::as_mut(&mut self.replyer)
        }
        /// Sets the `replyer`.
        #[inline]
        pub fn set_replyer(
            &mut self,
            replyer: ::core::option::Option<User>,
        ) -> &mut Self {
            self.replyer = replyer;
            self
        }
        /// Sets the `replyer`.
        #[inline]
        pub fn with_replyer(mut self, replyer: ::core::option::Option<User>) -> Self {
            self.replyer = replyer;
            self
        }
        /// Gets the reference of the field `title`.
        #[inline]
        pub fn title_ref(&self) -> &::std::string::String {
            &self.title
        }
        /// Gets the mutable reference of the field `title`.
        #[inline]
        pub fn title_mut(&mut self) -> &mut ::std::string::String {
            &mut self.title
        }
        /// Sets the `title`.
        #[inline]
        pub fn set_title(&mut self, title: ::std::string::String) -> &mut Self {
            self.title = title;
            self
        }
        /// Sets the `title`.
        #[inline]
        pub fn with_title(mut self, title: ::std::string::String) -> Self {
            self.title = title;
            self
        }
        /// Gets the reference of the field `content`.
        #[inline]
        pub fn content_ref(&self) -> ::core::option::Option<&::std::string::String> {
            ::core::option::Option::as_ref(&self.content)
        }
        /// Gets the mutable reference of the field `content`.
        #[inline]
        pub fn content_mut(
            &mut self,
        ) -> ::core::option::Option<&mut ::std::string::String> {
            ::core::option::Option::as_mut(&mut self.content)
        }
        /// Sets the `content`.
        #[inline]
        pub fn set_content(
            &mut self,
            content: ::core::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.content = content;
            self
        }
        /// Sets the `content`.
        #[inline]
        pub fn with_content(
            mut self,
            content: ::core::option::Option<::std::string::String>,
        ) -> Self {
            self.content = content;
            self
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::default::Default for PartialComment {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl PartialComment {
        /// Creates an empty partial struct.
        pub const fn new() -> Self {
            Self {
                user: ::core::option::Option::None,
                replyer: ::core::option::Option::None,
                title: ::core::option::Option::None,
                content: ::core::option::Option::None,
            }
        }
        #[inline]
        pub const fn user_ref(&self) -> ::core::option::Option<&User> {
            self.user.as_ref()
        }
        #[inline]
        pub const fn user_mut(&mut self) -> ::core::option::Option<&mut User> {
            self.user.as_mut()
        }
        #[inline]
        pub const fn take_user(&mut self) -> ::core::option::Option<User> {
            self.user.take()
        }
        #[inline]
        pub fn clear_user(&mut self) -> &mut Self {
            self.user = ::core::option::Option::None;
            self
        }
        #[inline]
        pub fn set_user(&mut self, value: User) -> &mut Self {
            self.user = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_user(mut self, value: User) -> Self {
            self.user = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn without_user(mut self) -> Self {
            self.user = ::core::option::Option::None;
            self
        }
        #[inline]
        pub const fn replyer_ref(&self) -> ::core::option::Option<&User> {
            self.replyer.as_ref()
        }
        #[inline]
        pub const fn replyer_mut(&mut self) -> ::core::option::Option<&mut User> {
            self.replyer.as_mut()
        }
        #[inline]
        pub const fn take_replyer(&mut self) -> ::core::option::Option<User> {
            self.replyer.take()
        }
        #[inline]
        pub fn clear_replyer(&mut self) -> &mut Self {
            self.replyer = ::core::option::Option::None;
            self
        }
        #[inline]
        pub fn set_replyer(&mut self, value: User) -> &mut Self {
            self.replyer = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_replyer(mut self, value: User) -> Self {
            self.replyer = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn without_replyer(mut self) -> Self {
            self.replyer = ::core::option::Option::None;
            self
        }
        #[inline]
        pub const fn title_ref(&self) -> ::core::option::Option<&::std::string::String> {
            self.title.as_ref()
        }
        #[inline]
        pub const fn title_mut(
            &mut self,
        ) -> ::core::option::Option<&mut ::std::string::String> {
            self.title.as_mut()
        }
        #[inline]
        pub const fn take_title(
            &mut self,
        ) -> ::core::option::Option<::std::string::String> {
            self.title.take()
        }
        #[inline]
        pub fn clear_title(&mut self) -> &mut Self {
            self.title = ::core::option::Option::None;
            self
        }
        #[inline]
        pub fn set_title(&mut self, value: ::std::string::String) -> &mut Self {
            self.title = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_title(mut self, value: ::std::string::String) -> Self {
            self.title = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn without_title(mut self) -> Self {
            self.title = ::core::option::Option::None;
            self
        }
        #[inline]
        pub const fn content_ref(
            &self,
        ) -> ::core::option::Option<&::std::string::String> {
            self.content.as_ref()
        }
        #[inline]
        pub const fn content_mut(
            &mut self,
        ) -> ::core::option::Option<&mut ::std::string::String> {
            self.content.as_mut()
        }
        #[inline]
        pub const fn take_content(
            &mut self,
        ) -> ::core::option::Option<::std::string::String> {
            self.content.take()
        }
        #[inline]
        pub fn clear_content(&mut self) -> &mut Self {
            self.content = ::core::option::Option::None;
            self
        }
        #[inline]
        pub fn set_content(&mut self, value: ::std::string::String) -> &mut Self {
            self.content = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_content(mut self, value: ::std::string::String) -> Self {
            self.content = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn without_content(mut self) -> Self {
            self.content = ::core::option::Option::None;
            self
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        __GROST_FLAVOR__,
        __GROST_UNKNOWN_BUFFER__,
    > ::core::default::Default
    for PartialCommentRef<
        '__grost_lifetime__,
        __GROST_FLAVOR__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        User: ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <User as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            User,
        >: ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <::core::option::Option<
            User,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <::std::string::String as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            4u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
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
        __GROST_FLAVOR__,
        __GROST_UNKNOWN_BUFFER__,
    > PartialCommentRef<'__grost_lifetime__, __GROST_FLAVOR__, __GROST_UNKNOWN_BUFFER__>
    where
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        User: ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <User as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            User,
        >: ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <::core::option::Option<
            User,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <::std::string::String as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            4u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >,
        <::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Encoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >,
        >>::Output: ::core::marker::Sized,
    {
        /// Creates an empty partial struct.
        #[inline]
        pub const fn new() -> Self {
            Self {
                user: ::core::option::Option::None,
                replyer: ::core::option::Option::None,
                title: ::core::option::Option::None,
                content: ::core::option::Option::None,
                __grost_unknown_buffer__: ::core::option::Option::None,
            }
        }
        #[inline]
        pub const fn user_ref(
            &self,
        ) -> ::core::option::Option<
            &<User as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.user.as_ref()
        }
        #[inline]
        pub const fn user_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <User as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.user.as_mut()
        }
        #[inline]
        pub const fn take_user(
            &mut self,
        ) -> ::core::option::Option<
            <User as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.user.take()
        }
        #[inline]
        pub fn clear_user(&mut self) -> &mut Self {
            self.user = ::core::option::Option::None;
            self
        }
        #[inline]
        pub fn set_user(
            &mut self,
            value: <User as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> &mut Self {
            self.user = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_user(
            mut self,
            value: <User as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> Self {
            self.user = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn without_user(mut self) -> Self {
            self.user = ::core::option::Option::None;
            self
        }
        #[inline]
        pub const fn replyer_ref(
            &self,
        ) -> ::core::option::Option<
            &<::core::option::Option<
                User,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.replyer.as_ref()
        }
        #[inline]
        pub const fn replyer_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <::core::option::Option<
                User,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.replyer.as_mut()
        }
        #[inline]
        pub const fn take_replyer(
            &mut self,
        ) -> ::core::option::Option<
            <::core::option::Option<
                User,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.replyer.take()
        }
        #[inline]
        pub fn clear_replyer(&mut self) -> &mut Self {
            self.replyer = ::core::option::Option::None;
            self
        }
        #[inline]
        pub fn set_replyer(
            &mut self,
            value: <::core::option::Option<
                User,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> &mut Self {
            self.replyer = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_replyer(
            mut self,
            value: <::core::option::Option<
                User,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> Self {
            self.replyer = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn without_replyer(mut self) -> Self {
            self.replyer = ::core::option::Option::None;
            self
        }
        #[inline]
        pub const fn title_ref(
            &self,
        ) -> ::core::option::Option<
            &<::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.title.as_ref()
        }
        #[inline]
        pub const fn title_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.title.as_mut()
        }
        #[inline]
        pub const fn take_title(
            &mut self,
        ) -> ::core::option::Option<
            <::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.title.take()
        }
        #[inline]
        pub fn clear_title(&mut self) -> &mut Self {
            self.title = ::core::option::Option::None;
            self
        }
        #[inline]
        pub fn set_title(
            &mut self,
            value: <::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> &mut Self {
            self.title = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_title(
            mut self,
            value: <::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> Self {
            self.title = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn without_title(mut self) -> Self {
            self.title = ::core::option::Option::None;
            self
        }
        #[inline]
        pub const fn content_ref(
            &self,
        ) -> ::core::option::Option<
            &<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.content.as_ref()
        }
        #[inline]
        pub const fn content_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.content.as_mut()
        }
        #[inline]
        pub const fn take_content(
            &mut self,
        ) -> ::core::option::Option<
            <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        > {
            self.content.take()
        }
        #[inline]
        pub fn clear_content(&mut self) -> &mut Self {
            self.content = ::core::option::Option::None;
            self
        }
        #[inline]
        pub fn set_content(
            &mut self,
            value: <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> &mut Self {
            self.content = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_content(
            mut self,
            value: <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >,
            >>::Output,
        ) -> Self {
            self.content = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn without_content(mut self) -> Self {
            self.content = ::core::option::Option::None;
            self
        }
    }
};
const _: () = {
    #[automatically_derived]
    impl<R, F, const TAG: ::core::primitive::u32> ::core::ops::Deref
    for CommentFieldReflection<R, F, TAG>
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
    impl<
        R,
        F,
        const TAG: ::core::primitive::u32,
    > ::core::convert::AsRef<<Self as ::core::ops::Deref>::Target>
    for CommentFieldReflection<R, F, TAG>
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
    impl<R, F, const TAG: ::core::primitive::u32> ::core::fmt::Debug
    for CommentFieldReflection<R, F, TAG>
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
    impl<R, F, const TAG: ::core::primitive::u32> ::core::fmt::Display
    for CommentFieldReflection<R, F, TAG>
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
    #[allow(clippy::type_complexity, non_camel_case_types)]
    impl<R, F, const TAG: ::core::primitive::u32> CommentFieldReflection<R, F, TAG>
    where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized,
    {
        #[inline]
        const fn new_in() -> Self {
            Self {
                _reflect: ::core::marker::PhantomData,
                _flavor: ::core::marker::PhantomData,
            }
        }
        /// Returns the reflection of the field.
        #[inline]
        const fn new() -> Self {
            Self::new_in()
        }
        /// Returns the relection to the wire format of the field.
        #[inline]
        pub const fn wire_format(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
    }
    #[automatically_derived]
    #[allow(clippy::type_complexity)]
    impl<
        F,
        const TAG: ::core::primitive::u32,
    > CommentFieldReflection<::grost::__private::reflection::ObjectFieldReflection<F>, F, TAG>
    where
        F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
        /// Returns the relection to a tag of the field.
        #[inline]
        pub const fn tag(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::TagReflection<F::Tag>,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the relection to the encoded tag of the field.
        #[inline]
        pub const fn encoded_tag(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::EncodedTagReflection<F::Identifier>,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the relection to the encoded tag of the field.
        #[inline]
        pub const fn encoded_tag_len(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::EncodedTagReflection<F::Identifier>,
            >,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the relection to the wire type of the field.
        #[inline]
        pub const fn wire_type(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::WireTypeReflection<F::WireType>,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the relection to the identifier of the field.
        #[inline]
        pub const fn identifier(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::IdentifierReflection<F::Identifier>,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the relection to the encoded identifier of the field.
        #[inline]
        pub const fn encoded_identifier(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::EncodedIdentifierReflection<F::Identifier>,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the relection to the encoded identifier of the field.
        #[inline]
        pub const fn encoded_identifier_len(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::EncodedIdentifierReflection<
                    F::Identifier,
                >,
            >,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the reflection to the encode fn.
        #[inline]
        pub const fn encode(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::encode::EncodeField,
            >,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the reflection to fn which will give the length of the encoded data.
        #[inline]
        pub const fn encoded_len(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::Len<
                    ::grost::__private::reflection::encode::EncodeField,
                >,
            >,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the reflection to the reference encode fn.
        #[inline]
        pub const fn encode_ref(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::encode::EncodeRefField,
            >,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the reflection to the reference encode fn which will give the length of the encoded data.
        #[inline]
        pub const fn encoded_ref_len(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::Len<
                    ::grost::__private::reflection::encode::EncodeRefField,
                >,
            >,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the reflection to the partial encode fn.
        #[inline]
        pub const fn partial_encode(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::encode::PartialEncodeField,
            >,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the reflection to the partial encode fn which will give the length of the encoded data.
        #[inline]
        pub const fn partial_encoded_len(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::Len<
                    ::grost::__private::reflection::encode::PartialEncodeField,
                >,
            >,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the reflection to the partial reference encode fn.
        #[inline]
        pub const fn partial_encode_ref(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::encode::PartialEncodeRefField,
            >,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the reflection to the partial reference encode fn which will give the length of the encoded data.
        #[inline]
        pub const fn partial_encoded_ref_len(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::Len<
                    ::grost::__private::reflection::encode::PartialEncodeRefField,
                >,
            >,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
    }
    #[automatically_derived]
    impl<R, F, const TAG: ::core::primitive::u32> ::core::clone::Clone
    for CommentFieldReflection<R, F, TAG>
    where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized,
    {
        fn clone(&self) -> Self {
            *self
        }
    }
    #[automatically_derived]
    impl<R, F, const TAG: ::core::primitive::u32> ::core::marker::Copy
    for CommentFieldReflection<R, F, TAG>
    where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized,
    {}
    #[automatically_derived]
    impl<R, F> ::core::ops::Deref for CommentReflection<R, F>
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
    for CommentReflection<R, F>
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
    impl<R, F> ::core::fmt::Debug for CommentReflection<R, F>
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
    impl<R, F> ::core::fmt::Display for CommentReflection<R, F>
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
    for CommentReflection<R, F> {
        fn clone(&self) -> Self {
            *self
        }
    }
    #[automatically_derived]
    impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::marker::Copy
    for CommentReflection<R, F> {}
    #[automatically_derived]
    impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> CommentReflection<R, F> {
        const fn new_in() -> Self {
            Self {
                _reflect: ::core::marker::PhantomData,
                _flavor: ::core::marker::PhantomData,
            }
        }
    }
    #[automatically_derived]
    impl<F> CommentReflection<::grost::__private::reflection::ObjectReflection<F>, F>
    where
        F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
        /// Returns the reflection of the struct.
        #[inline]
        const fn new() -> Self {
            Self::new_in()
        }
        /// Returns the field reflection of the field `Comment.user`.
        #[inline]
        pub const fn user(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::ObjectFieldReflection<F>,
            F,
            1u32,
        > {
            CommentFieldReflection::new()
        }
        /// Returns the field reflection of the field `Comment.replyer`.
        #[inline]
        pub const fn replyer(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::ObjectFieldReflection<F>,
            F,
            2u32,
        > {
            CommentFieldReflection::new()
        }
        /// Returns the field reflection of the field `Comment.title`.
        #[inline]
        pub const fn title(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::ObjectFieldReflection<F>,
            F,
            3u32,
        > {
            CommentFieldReflection::new()
        }
        /// Returns the field reflection of the field `Comment.content`.
        #[inline]
        pub const fn content(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::ObjectFieldReflection<F>,
            F,
            4u32,
        > {
            CommentFieldReflection::new()
        }
    }
    #[automatically_derived]
    impl Comment {
        /// Returns the reflection of the struct.
        #[allow(non_camel_case_types)]
        #[inline]
        pub const fn reflection<__GROST_FLAVOR__>() -> CommentReflection<
            ::grost::__private::reflection::ObjectReflection<__GROST_FLAVOR__>,
            __GROST_FLAVOR__,
        >
        where
            __GROST_FLAVOR__: ?::core::marker::Sized
                + ::grost::__private::flavors::Flavor,
        {
            CommentReflection::new()
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        __GROST_FLAVOR__: ?::core::marker::Sized,
    > ::grost::__private::indexer::Indexable<__GROST_FLAVOR__> for Comment {
        type Indexer = CommentFieldIndex;
    }
    #[automatically_derived]
    impl CommentFieldIndex {
        /// The number of variants of this field indexer.
        pub const VARIANTS: ::core::primitive::usize = 4usize;
        /// The first field indexer.
        pub const FIRST: Self = Self::User;
        /// The last field indexer.
        pub const LAST: Self = Self::Content;
        /// Returns the field reflection of the corresponding field.
        #[allow(non_camel_case_types, clippy::type_complexity)]
        #[inline]
        pub const fn reflection<__GROST_FLAVOR__>(
            &self,
        ) -> &'static ::grost::__private::reflection::ObjectFieldReflection<__GROST_FLAVOR__>
        where
            __GROST_FLAVOR__: ::grost::__private::flavors::Flavor
                + ?::core::marker::Sized,
            CommentFieldReflection<
                ::grost::__private::reflection::ObjectFieldReflection<__GROST_FLAVOR__>,
                __GROST_FLAVOR__,
                1u32,
            >: ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
                Reflection = ::grost::__private::reflection::ObjectFieldReflection<
                    __GROST_FLAVOR__,
                >,
            >,
            CommentFieldReflection<
                ::grost::__private::reflection::ObjectFieldReflection<__GROST_FLAVOR__>,
                __GROST_FLAVOR__,
                2u32,
            >: ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
                Reflection = ::grost::__private::reflection::ObjectFieldReflection<
                    __GROST_FLAVOR__,
                >,
            >,
            CommentFieldReflection<
                ::grost::__private::reflection::ObjectFieldReflection<__GROST_FLAVOR__>,
                __GROST_FLAVOR__,
                3u32,
            >: ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
                Reflection = ::grost::__private::reflection::ObjectFieldReflection<
                    __GROST_FLAVOR__,
                >,
            >,
            CommentFieldReflection<
                ::grost::__private::reflection::ObjectFieldReflection<__GROST_FLAVOR__>,
                __GROST_FLAVOR__,
                4u32,
            >: ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
                Reflection = ::grost::__private::reflection::ObjectFieldReflection<
                    __GROST_FLAVOR__,
                >,
            >,
        {
            match self {
                Self::User => {
                    <CommentFieldReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            __GROST_FLAVOR__,
                        >,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::REFLECTION
                }
                Self::Replyer => {
                    <CommentFieldReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            __GROST_FLAVOR__,
                        >,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::REFLECTION
                }
                Self::Title => {
                    <CommentFieldReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            __GROST_FLAVOR__,
                        >,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::REFLECTION
                }
                Self::Content => {
                    <CommentFieldReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            __GROST_FLAVOR__,
                        >,
                        __GROST_FLAVOR__,
                        4u32,
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
                Self::Content => ::core::option::Option::None,
                Self::User => ::core::option::Option::Some(Self::Replyer),
                Self::Replyer => ::core::option::Option::Some(Self::Title),
                Self::Title => ::core::option::Option::Some(Self::Content),
            }
        }
        /// Returns the previous field indexer.
        ///
        /// Returns `None` if there are no previous fields.
        #[inline]
        pub const fn prev(&self) -> ::core::option::Option<Self> {
            match self {
                Self::User => ::core::option::Option::None,
                Self::Content => ::core::option::Option::Some(Self::Title),
                Self::Title => ::core::option::Option::Some(Self::Replyer),
                Self::Replyer => ::core::option::Option::Some(Self::User),
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
    impl ::core::iter::Iterator for CommentFieldIndex {
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
    impl ::core::iter::DoubleEndedIterator for CommentFieldIndex {
        fn next_back(&mut self) -> ::core::option::Option<Self> {
            Self::prev(self)
        }
    }
    #[automatically_derived]
    impl ::core::iter::FusedIterator for CommentFieldIndex {}
    #[automatically_derived]
    impl ::core::iter::ExactSizeIterator for CommentFieldIndex {
        fn len(&self) -> ::core::primitive::usize {
            self.remaining()
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<__GROST_FLAVOR__> ::core::fmt::Debug for CommentSelector<__GROST_FLAVOR__>
    where
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        User: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            User,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            4u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::write!(f, "CommentSelector")
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<__GROST_FLAVOR__> ::core::cmp::PartialEq for CommentSelector<__GROST_FLAVOR__>
    where
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        User: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            User,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            4u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        fn eq(&self, other: &Self) -> ::core::primitive::bool {
            self.user == other.user && self.replyer == other.replyer
                && self.title == other.title && self.content == other.content
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<__GROST_FLAVOR__> ::core::cmp::Eq for CommentSelector<__GROST_FLAVOR__>
    where
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        User: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            User,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            4u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<__GROST_FLAVOR__> ::core::hash::Hash for CommentSelector<__GROST_FLAVOR__>
    where
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        User: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            User,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            4u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
            self.user.hash(state);
            self.replyer.hash(state);
            self.title.hash(state);
            self.content.hash(state);
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<__GROST_FLAVOR__> ::core::clone::Clone for CommentSelector<__GROST_FLAVOR__>
    where
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        User: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            User,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            4u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        fn clone(&self) -> Self {
            Self {
                user: ::core::clone::Clone::clone(&self.user),
                replyer: ::core::clone::Clone::clone(&self.replyer),
                title: ::core::clone::Clone::clone(&self.title),
                content: ::core::clone::Clone::clone(&self.content),
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<__GROST_FLAVOR__> ::grost::__private::Selector<__GROST_FLAVOR__>
    for CommentSelector<__GROST_FLAVOR__>
    where
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        User: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            User,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            4u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                4u32,
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
            <<User as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::flip(&mut self.user);
            <<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::flip(&mut self.replyer);
            <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::flip(&mut self.title);
            <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::flip(&mut self.content);
            self
        }
        fn merge(&mut self, other: Self) -> &mut Self {
            <<User as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::merge(&mut self.user, other.user);
            <<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::merge(&mut self.replyer, other.replyer);
            <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::merge(&mut self.title, other.title);
            <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::merge(&mut self.content, other.content);
            self
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<__GROST_FLAVOR__> ::core::default::Default for CommentSelector<__GROST_FLAVOR__>
    where
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        User: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            User,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            4u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                4u32,
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
    impl<__GROST_FLAVOR__> CommentSelector<__GROST_FLAVOR__>
    where
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        User: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            User,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            4u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                4u32,
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
                user: <<User as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT,
                replyer: <<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT,
                title: <<::std::string::String as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT,
                content: <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        4u32,
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
                user: <<User as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE,
                replyer: <<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE,
                title: <<::std::string::String as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE,
                content: <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        4u32,
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
                user: <<User as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::ALL,
                replyer: <<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::ALL,
                title: <<::std::string::String as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::ALL,
                content: <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::ALL,
            }
        }
        /// Returns `true` if the selector selects nothing.
        #[inline]
        pub fn is_empty(&self) -> ::core::primitive::bool {
            <<User as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.user)
                && <<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_empty(&self.replyer)
                && <<::std::string::String as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_empty(&self.title)
                && <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_empty(&self.content)
        }
        /// Returns `true` if the selector selects all.
        #[inline]
        pub fn is_all(&self) -> ::core::primitive::bool {
            <<User as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_all(&self.user)
                && <<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_all(&self.replyer)
                && <<::std::string::String as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_all(&self.title)
                && <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    __GROST_FLAVOR__,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::WireFormatReflection,
                        __GROST_FLAVOR__,
                        4u32,
                    > as ::grost::__private::reflection::Reflectable<
                        __GROST_FLAVOR__,
                    >>::Reflection,
                >>::Selector as ::grost::__private::Selector<
                    __GROST_FLAVOR__,
                >>::is_all(&self.content)
        }
        /// Returns the number of selected fields.
        #[inline]
        pub fn selected(&self) -> ::core::primitive::usize {
            let mut num = 0;
            if self.is_user_selected() {
                num += 1;
            }
            if self.is_replyer_selected() {
                num += 1;
            }
            if self.is_title_selected() {
                num += 1;
            }
            if self.is_content_selected() {
                num += 1;
            }
            num
        }
        /// Returns the number of unselected fields.
        #[inline]
        pub fn unselected(&self) -> ::core::primitive::usize {
            let mut num = 0;
            if self.is_user_unselected() {
                num += 1;
            }
            if self.is_replyer_unselected() {
                num += 1;
            }
            if self.is_title_unselected() {
                num += 1;
            }
            if self.is_content_unselected() {
                num += 1;
            }
            num
        }
        /// Returns an iterator over the selected fields.
        #[inline]
        pub fn iter_selected(&self) -> CommentSelectorIter<__GROST_FLAVOR__, true> {
            CommentSelectorIter::new(self, self.selected())
        }
        /// Returns an iterator over the unselected fields.
        #[inline]
        pub fn iter_unselected(&self) -> CommentSelectorIter<__GROST_FLAVOR__, false> {
            CommentSelectorIter::new(self, self.unselected())
        }
        /// Returns `true` if such field is selected.
        #[inline]
        pub fn is_selected(&self, idx: CommentFieldIndex) -> ::core::primitive::bool {
            match idx {
                CommentFieldIndex::User => self.is_user_selected(),
                CommentFieldIndex::Replyer => self.is_replyer_selected(),
                CommentFieldIndex::Title => self.is_title_selected(),
                CommentFieldIndex::Content => self.is_content_selected(),
            }
        }
        /// Returns `true` if such field is unselected.
        #[inline]
        pub fn is_unselected(&self, idx: CommentFieldIndex) -> ::core::primitive::bool {
            match idx {
                CommentFieldIndex::User => self.is_user_unselected(),
                CommentFieldIndex::Replyer => self.is_replyer_unselected(),
                CommentFieldIndex::Title => self.is_title_unselected(),
                CommentFieldIndex::Content => self.is_content_unselected(),
            }
        }
        /// Select the `Comment.user` field
        #[inline]
        pub fn select_user(&mut self) -> &mut Self {
            self.user = <<User as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unselect the `Comment.user` field
        #[inline]
        pub fn unselect_user(&mut self) -> &mut Self {
            self.user = <<User as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Update the `Comment.user` field
        #[inline]
        pub fn update_user(
            &mut self,
            value: <User as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> &mut Self {
            self.user = value;
            self
        }
        /// Set or unset the `Comment.user` field
        #[inline]
        pub fn maybe_user(
            mut self,
            val: <User as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> Self {
            self.user = val;
            self
        }
        /// Get a reference to the selector of `Comment.user` field
        #[inline]
        pub const fn user_ref(
            &self,
        ) -> &<User as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &self.user
        }
        /// Get a mutable reference to the selector of `Comment.user` field
        #[inline]
        pub const fn user_mut(
            &mut self,
        ) -> &mut <User as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &mut self.user
        }
        /// Set the `Comment.user` field
        #[inline]
        pub fn with_user(mut self) -> Self {
            self.user = <<User as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unset the `Comment.user` field
        #[inline]
        pub fn without_user(mut self) -> Self {
            self.user = <<User as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Returns `true` if the `Comment.user` field is selected
        #[inline]
        pub fn is_user_selected(&self) -> ::core::primitive::bool {
            !<<User as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.user)
        }
        /// Returns `true` if the `Comment.user` field is unselected
        #[inline]
        pub fn is_user_unselected(&self) -> ::core::primitive::bool {
            <<User as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.user)
        }
        /// Select the `Comment.replyer` field
        #[inline]
        pub fn select_replyer(&mut self) -> &mut Self {
            self.replyer = <<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unselect the `Comment.replyer` field
        #[inline]
        pub fn unselect_replyer(&mut self) -> &mut Self {
            self.replyer = <<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Update the `Comment.replyer` field
        #[inline]
        pub fn update_replyer(
            &mut self,
            value: <::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> &mut Self {
            self.replyer = value;
            self
        }
        /// Set or unset the `Comment.replyer` field
        #[inline]
        pub fn maybe_replyer(
            mut self,
            val: <::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> Self {
            self.replyer = val;
            self
        }
        /// Get a reference to the selector of `Comment.replyer` field
        #[inline]
        pub const fn replyer_ref(
            &self,
        ) -> &<::core::option::Option<
            User,
        > as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &self.replyer
        }
        /// Get a mutable reference to the selector of `Comment.replyer` field
        #[inline]
        pub const fn replyer_mut(
            &mut self,
        ) -> &mut <::core::option::Option<
            User,
        > as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &mut self.replyer
        }
        /// Set the `Comment.replyer` field
        #[inline]
        pub fn with_replyer(mut self) -> Self {
            self.replyer = <<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unset the `Comment.replyer` field
        #[inline]
        pub fn without_replyer(mut self) -> Self {
            self.replyer = <<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Returns `true` if the `Comment.replyer` field is selected
        #[inline]
        pub fn is_replyer_selected(&self) -> ::core::primitive::bool {
            !<<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.replyer)
        }
        /// Returns `true` if the `Comment.replyer` field is unselected
        #[inline]
        pub fn is_replyer_unselected(&self) -> ::core::primitive::bool {
            <<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.replyer)
        }
        /// Select the `Comment.title` field
        #[inline]
        pub fn select_title(&mut self) -> &mut Self {
            self.title = <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unselect the `Comment.title` field
        #[inline]
        pub fn unselect_title(&mut self) -> &mut Self {
            self.title = <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Update the `Comment.title` field
        #[inline]
        pub fn update_title(
            &mut self,
            value: <::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> &mut Self {
            self.title = value;
            self
        }
        /// Set or unset the `Comment.title` field
        #[inline]
        pub fn maybe_title(
            mut self,
            val: <::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> Self {
            self.title = val;
            self
        }
        /// Get a reference to the selector of `Comment.title` field
        #[inline]
        pub const fn title_ref(
            &self,
        ) -> &<::std::string::String as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &self.title
        }
        /// Get a mutable reference to the selector of `Comment.title` field
        #[inline]
        pub const fn title_mut(
            &mut self,
        ) -> &mut <::std::string::String as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &mut self.title
        }
        /// Set the `Comment.title` field
        #[inline]
        pub fn with_title(mut self) -> Self {
            self.title = <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unset the `Comment.title` field
        #[inline]
        pub fn without_title(mut self) -> Self {
            self.title = <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Returns `true` if the `Comment.title` field is selected
        #[inline]
        pub fn is_title_selected(&self) -> ::core::primitive::bool {
            !<<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.title)
        }
        /// Returns `true` if the `Comment.title` field is unselected
        #[inline]
        pub fn is_title_unselected(&self) -> ::core::primitive::bool {
            <<::std::string::String as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.title)
        }
        /// Select the `Comment.content` field
        #[inline]
        pub fn select_content(&mut self) -> &mut Self {
            self.content = <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unselect the `Comment.content` field
        #[inline]
        pub fn unselect_content(&mut self) -> &mut Self {
            self.content = <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Update the `Comment.content` field
        #[inline]
        pub fn update_content(
            &mut self,
            value: <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> &mut Self {
            self.content = value;
            self
        }
        /// Set or unset the `Comment.content` field
        #[inline]
        pub fn maybe_content(
            mut self,
            val: <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector,
        ) -> Self {
            self.content = val;
            self
        }
        /// Get a reference to the selector of `Comment.content` field
        #[inline]
        pub const fn content_ref(
            &self,
        ) -> &<::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &self.content
        }
        /// Get a mutable reference to the selector of `Comment.content` field
        #[inline]
        pub const fn content_mut(
            &mut self,
        ) -> &mut <::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >>::Selector {
            &mut self.content
        }
        /// Set the `Comment.content` field
        #[inline]
        pub fn with_content(mut self) -> Self {
            self.content = <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::DEFAULT;
            self
        }
        /// Unset the `Comment.content` field
        #[inline]
        pub fn without_content(mut self) -> Self {
            self.content = <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<__GROST_FLAVOR__>>::NONE;
            self
        }
        /// Returns `true` if the `Comment.content` field is selected
        #[inline]
        pub fn is_content_selected(&self) -> ::core::primitive::bool {
            !<<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.content)
        }
        /// Returns `true` if the `Comment.content` field is unselected
        #[inline]
        pub fn is_content_unselected(&self) -> ::core::primitive::bool {
            <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                __GROST_FLAVOR__,
                <CommentFieldReflection<
                    ::grost::__private::reflection::WireFormatReflection,
                    __GROST_FLAVOR__,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    __GROST_FLAVOR__,
                >>::Reflection,
            >>::Selector as ::grost::__private::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.content)
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
        '__grost_lifetime__,
        __GROST_FLAVOR__,
        const N: ::core::primitive::bool,
    > CommentSelectorIter<'__grost_lifetime__, __GROST_FLAVOR__, N>
    where
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        User: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            User,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::std::string::String: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
        CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection,
            __GROST_FLAVOR__,
            4u32,
        >: ::grost::__private::reflection::Reflectable<__GROST_FLAVOR__>,
        ::core::option::Option<
            ::std::string::String,
        >: ::grost::__private::Selectable<
            __GROST_FLAVOR__,
            <CommentFieldReflection<
                ::grost::__private::reflection::WireFormatReflection,
                __GROST_FLAVOR__,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                __GROST_FLAVOR__,
            >>::Reflection,
        >,
    {
        #[inline]
        const fn new(
            selector: &'__grost_lifetime__ CommentSelector<__GROST_FLAVOR__>,
            num: ::core::primitive::usize,
        ) -> Self {
            Self {
                selector,
                index: ::core::option::Option::Some(CommentFieldIndex::FIRST),
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
const _: () = {
    ::grost::__private::encoded_state(
        & '__grost_lifetime__::grost::__private::flavors::Network : Comment as
        ::grost::__private::flavors::network::LengthDelimited => PartialCommentRef <
        '__grost_lifetime__, ::grost::__private::flavors::Network >
    );
    ::grost::__private::encoded_state(
        & '__grost_lifetime__::grost::__private::flavors::Network : PartialComment as
        ::grost::__private::flavors::network::LengthDelimited => PartialCommentRef <
        '__grost_lifetime__, ::grost::__private::flavors::Network >
    );
    ::grost::__private::encoded_state(
        & '__grost_lifetime__::grost::__private::flavors::Network : PartialCommentRef <
        '__grost_lifetime__, ::grost::__private::flavors::Network > as
        ::grost::__private::flavors::network::LengthDelimited => PartialCommentRef <
        '__grost_lifetime__, ::grost::__private::flavors::Network >
    );
    #[automatically_derived]
    impl ::grost::__private::flavors::DefaultWireFormat<
        ::grost::__private::flavors::Network,
    > for Comment {
        type Format = ::grost::__private::flavors::network::LengthDelimited;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for Comment {
        type Selector = CommentSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::Repeated<
            ::grost::__private::flavors::network::LengthDelimited,
        >,
    > for Comment {
        type Selector = CommentSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        const I: ::core::primitive::u32,
    > ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::Repeated<
            ::grost::__private::flavors::network::Stream<
                ::grost::__private::flavors::network::LengthDelimited,
                I,
            >,
        >,
    > for Comment {
        type Selector = CommentSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for PartialComment {
        type Selector = CommentSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::Repeated<
            ::grost::__private::flavors::network::LengthDelimited,
        >,
    > for PartialComment {
        type Selector = CommentSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        const I: ::core::primitive::u32,
    > ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::Repeated<
            ::grost::__private::flavors::network::Stream<
                ::grost::__private::flavors::network::LengthDelimited,
                I,
            >,
        >,
    > for PartialComment {
        type Selector = CommentSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        '__grost_lifetime__,
    > ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for PartialCommentRef<'__grost_lifetime__, ::grost::__private::flavors::Network> {
        type Selector = CommentSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        '__grost_lifetime__,
    > ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::Repeated<
            ::grost::__private::flavors::network::LengthDelimited,
        >,
    > for PartialCommentRef<'__grost_lifetime__, ::grost::__private::flavors::Network> {
        type Selector = CommentSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        '__grost_lifetime__,
        const I: ::core::primitive::u32,
    > ::grost::__private::Selectable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::Repeated<
            ::grost::__private::flavors::network::Stream<
                ::grost::__private::flavors::network::LengthDelimited,
                I,
            >,
        >,
    > for PartialCommentRef<'__grost_lifetime__, ::grost::__private::flavors::Network> {
        type Selector = CommentSelector<::grost::__private::flavors::Network>;
    }
    #[automatically_derived]
    impl<'__grost_lifetime__, const N: ::core::primitive::bool> ::core::iter::Iterator
    for CommentSelectorIter<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        N,
    > {
        type Item = &'static ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >;
        fn next(&mut self) -> ::core::option::Option<Self::Item> {
            loop {
                if self.yielded >= self.num {
                    return ::core::option::Option::None;
                }
                match self.index {
                    ::core::option::Option::Some(index) => {
                        match self.selector[(index, N)] {
                            ::core::option::Option::Some(reflection) => {
                                self.index = index.next();
                                self.yielded += 1;
                                return ::core::option::Option::Some(reflection);
                            }
                            ::core::option::Option::None => {
                                self.index = index.next();
                            }
                        }
                    }
                    ::core::option::Option::None => return ::core::option::Option::None,
                }
            }
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
    impl<
        '__grost_lifetime__,
        const N: ::core::primitive::bool,
    > ::core::iter::FusedIterator
    for CommentSelectorIter<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        N,
    > {}
    #[automatically_derived]
    impl<
        '__grost_lifetime__,
        const N: ::core::primitive::bool,
    > ::core::iter::ExactSizeIterator
    for CommentSelectorIter<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        N,
    > {
        #[inline]
        fn len(&self) -> ::core::primitive::usize {
            self.remaining()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >;
        const REFLECTION: &Self::Reflection = &::grost::__private::reflection::ObjectFieldReflectionBuilder::<
            ::grost::__private::flavors::Network,
        > {
            identifier: ::grost::__private::flavors::network::Identifier::new(
                <<User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                ::grost::__private::flavors::network::Tag::new(1u32),
            ),
            name: "user",
            ty: ::core::any::type_name::<User>,
            schema_name: "user",
            schema_type: ::grost::__private::reflection::Type::<
                ::grost::__private::flavors::Network,
            >::Object(
                <User as ::grost::__private::reflection::Reflectable<
                    ::grost::__private::flavors::Network,
                >>::REFLECTION,
            ),
        }
            .build();
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >;
        const REFLECTION: &Self::Reflection = &::grost::__private::reflection::ObjectFieldReflectionBuilder::<
            ::grost::__private::flavors::Network,
        > {
            identifier: ::grost::__private::flavors::network::Identifier::new(
                <<::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                ::grost::__private::flavors::network::Tag::new(2u32),
            ),
            name: "replyer",
            ty: ::core::any::type_name::<::core::option::Option<User>>,
            schema_name: "replyer",
            schema_type: ::grost::__private::reflection::Type::<
                ::grost::__private::flavors::Network,
            >::Optional(
                &::grost::__private::reflection::Type::<
                    ::grost::__private::flavors::Network,
                >::Object(
                    <User as ::grost::__private::reflection::Reflectable<
                        ::grost::__private::flavors::Network,
                    >>::REFLECTION,
                ),
            ),
        }
            .build();
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >;
        const REFLECTION: &Self::Reflection = &::grost::__private::reflection::ObjectFieldReflectionBuilder::<
            ::grost::__private::flavors::Network,
        > {
            identifier: ::grost::__private::flavors::network::Identifier::new(
                <<::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                ::grost::__private::flavors::network::Tag::new(3u32),
            ),
            name: "title",
            ty: ::core::any::type_name::<::std::string::String>,
            schema_name: "title",
            schema_type: ::grost::__private::reflection::Type::<
                ::grost::__private::flavors::Network,
            >::Primitive {
                name: "String!",
                description: "",
            },
        }
            .build();
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = ::grost::__private::reflection::ObjectFieldReflection<
            ::grost::__private::flavors::Network,
        >;
        const REFLECTION: &Self::Reflection = &::grost::__private::reflection::ObjectFieldReflectionBuilder::<
            ::grost::__private::flavors::Network,
        > {
            identifier: ::grost::__private::flavors::network::Identifier::new(
                <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Network,
                >>::WIRE_TYPE,
                ::grost::__private::flavors::network::Tag::new(4u32),
            ),
            name: "content",
            ty: ::core::any::type_name::<::core::option::Option<::std::string::String>>,
            schema_name: "content",
            schema_type: ::grost::__private::reflection::Type::<
                ::grost::__private::flavors::Network,
            >::Optional(
                &::grost::__private::reflection::Type::<
                    ::grost::__private::flavors::Network,
                >::Primitive {
                    name: "String",
                    description: "",
                },
            ),
        }
            .build();
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::TagReflection<
            ::grost::__private::flavors::network::Tag,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = ::grost::__private::flavors::network::Tag;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .tag()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::TagReflection<
            ::grost::__private::flavors::network::Tag,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = ::grost::__private::flavors::network::Tag;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .tag()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::TagReflection<
            ::grost::__private::flavors::network::Tag,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = ::grost::__private::flavors::network::Tag;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .tag()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::TagReflection<
            ::grost::__private::flavors::network::Tag,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = ::grost::__private::flavors::network::Tag;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .tag()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::EncodedTagReflection<
            ::grost::__private::flavors::network::Tag,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            <CommentFieldReflection<
                ::grost::__private::reflection::TagReflection<
                    ::grost::__private::flavors::network::Tag,
                >,
                ::grost::__private::flavors::Network,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .encode()
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::EncodedTagReflection<
            ::grost::__private::flavors::network::Tag,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            <CommentFieldReflection<
                ::grost::__private::reflection::TagReflection<
                    ::grost::__private::flavors::network::Tag,
                >,
                ::grost::__private::flavors::Network,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .encode()
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::EncodedTagReflection<
            ::grost::__private::flavors::network::Tag,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            <CommentFieldReflection<
                ::grost::__private::reflection::TagReflection<
                    ::grost::__private::flavors::network::Tag,
                >,
                ::grost::__private::flavors::Network,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .encode()
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::EncodedTagReflection<
            ::grost::__private::flavors::network::Tag,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            <CommentFieldReflection<
                ::grost::__private::reflection::TagReflection<
                    ::grost::__private::flavors::network::Tag,
                >,
                ::grost::__private::flavors::Network,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .encode()
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodedTagReflection<
                ::grost::__private::flavors::network::Tag,
            >,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::EncodedTagReflection<
                    ::grost::__private::flavors::network::Tag,
                >,
                ::grost::__private::flavors::Network,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodedTagReflection<
                ::grost::__private::flavors::network::Tag,
            >,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::EncodedTagReflection<
                    ::grost::__private::flavors::network::Tag,
                >,
                ::grost::__private::flavors::Network,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodedTagReflection<
                ::grost::__private::flavors::network::Tag,
            >,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::EncodedTagReflection<
                    ::grost::__private::flavors::network::Tag,
                >,
                ::grost::__private::flavors::Network,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodedTagReflection<
                ::grost::__private::flavors::network::Tag,
            >,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::EncodedTagReflection<
                    ::grost::__private::flavors::network::Tag,
                >,
                ::grost::__private::flavors::Network,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = ::grost::__private::flavors::network::Identifier;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
                ::grost::__private::flavors::Network,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .identifier()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = ::grost::__private::flavors::network::Identifier;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
                ::grost::__private::flavors::Network,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .identifier()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = ::grost::__private::flavors::network::Identifier;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
                ::grost::__private::flavors::Network,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .identifier()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = ::grost::__private::flavors::network::Identifier;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
                ::grost::__private::flavors::Network,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .identifier()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::EncodedIdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            <CommentFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .encode()
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::EncodedIdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            <CommentFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .encode()
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::EncodedIdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            <CommentFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .encode()
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::EncodedIdentifierReflection<
            ::grost::__private::flavors::network::Identifier,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = [::core::primitive::u8];
        const REFLECTION: &Self::Reflection = {
            <CommentFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .encode()
                .as_slice()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodedIdentifierReflection<
                ::grost::__private::flavors::network::Identifier,
            >,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::EncodedIdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodedIdentifierReflection<
                ::grost::__private::flavors::network::Identifier,
            >,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::EncodedIdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodedIdentifierReflection<
                ::grost::__private::flavors::network::Identifier,
            >,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::EncodedIdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodedIdentifierReflection<
                ::grost::__private::flavors::network::Identifier,
            >,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::EncodedIdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .len()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::WireTypeReflection<
            ::grost::__private::flavors::network::WireType,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = ::grost::__private::flavors::network::WireType;
        const REFLECTION: &::grost::__private::flavors::network::WireType = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                1u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .wire_type()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::WireTypeReflection<
            ::grost::__private::flavors::network::WireType,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = ::grost::__private::flavors::network::WireType;
        const REFLECTION: &::grost::__private::flavors::network::WireType = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                2u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .wire_type()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::WireTypeReflection<
            ::grost::__private::flavors::network::WireType,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = ::grost::__private::flavors::network::WireType;
        const REFLECTION: &::grost::__private::flavors::network::WireType = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                3u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .wire_type()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::WireTypeReflection<
            ::grost::__private::flavors::network::WireType,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = ::grost::__private::flavors::network::WireType;
        const REFLECTION: &::grost::__private::flavors::network::WireType = &{
            <CommentFieldReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::flavors::network::Identifier,
                >,
                ::grost::__private::flavors::Network,
                4u32,
            > as ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::Network,
            >>::REFLECTION
                .wire_type()
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = <User as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format;
        const REFLECTION: &<User as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format = &{
            <<User as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::SELF
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = <::core::option::Option<
            User,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format;
        const REFLECTION: &<::core::option::Option<
            User,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format = &{
            <<::core::option::Option<
                User,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::SELF
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format;
        const REFLECTION: &<::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format = &{
            <<::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::SELF
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::WireFormatReflection,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = <::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format;
        const REFLECTION: &<::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format = &{
            <<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::SELF
        };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    > for Comment {
        type Reflection = ::grost::__private::reflection::ObjectReflection<
            ::grost::__private::flavors::Network,
        >;
        const REFLECTION: &Self::Reflection = &::grost::__private::reflection::ObjectReflectionBuilder::<
            ::grost::__private::flavors::Network,
        > {
            name: "Comment",
            schema_name: "Comment",
            fields: &[
                <CommentFieldReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        ::grost::__private::flavors::Network,
                    >,
                    ::grost::__private::flavors::Network,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<
                    ::grost::__private::flavors::Network,
                >>::REFLECTION,
                <CommentFieldReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        ::grost::__private::flavors::Network,
                    >,
                    ::grost::__private::flavors::Network,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<
                    ::grost::__private::flavors::Network,
                >>::REFLECTION,
                <CommentFieldReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        ::grost::__private::flavors::Network,
                    >,
                    ::grost::__private::flavors::Network,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<
                    ::grost::__private::flavors::Network,
                >>::REFLECTION,
                <CommentFieldReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        ::grost::__private::flavors::Network,
                    >,
                    ::grost::__private::flavors::Network,
                    4u32,
                > as ::grost::__private::reflection::Reflectable<
                    ::grost::__private::flavors::Network,
                >>::REFLECTION,
            ],
        }
            .build();
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentReflection<
        ::grost::__private::reflection::ObjectReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
    > {
        type Reflection = ::grost::__private::reflection::ObjectReflection<
            ::grost::__private::flavors::Network,
        >;
        const REFLECTION: &Self::Reflection = <Comment as ::grost::__private::reflection::Reflectable<
            ::grost::__private::flavors::Network,
        >>::REFLECTION;
    }
    #[automatically_derived]
    impl ::core::ops::Index<(CommentFieldIndex, ::core::primitive::bool)>
    for CommentSelector<::grost::__private::flavors::Network> {
        type Output = ::core::option::Option<
            &'static ::grost::__private::reflection::ObjectFieldReflection<
                ::grost::__private::flavors::Network,
            >,
        >;
        fn index(
            &self,
            (indexer, select): (CommentFieldIndex, ::core::primitive::bool),
        ) -> &Self::Output {
            const NONE: &::core::option::Option<
                &'static ::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
            > = &::core::option::Option::None;
            match indexer {
                CommentFieldIndex::User => {
                    const REFLECTION: ::core::option::Option<
                        &::grost::__private::reflection::ObjectFieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                    > = ::core::option::Option::Some(
                        <CommentFieldReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                            ::grost::__private::flavors::Network,
                            1u32,
                        > as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    );
                    match (select, self.user.is_empty()) {
                        (true, false) => &REFLECTION,
                        (true, true) => NONE,
                        (false, false) => NONE,
                        (false, true) => &REFLECTION,
                    }
                }
                CommentFieldIndex::Replyer => {
                    const REFLECTION: ::core::option::Option<
                        &::grost::__private::reflection::ObjectFieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                    > = ::core::option::Option::Some(
                        <CommentFieldReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                            ::grost::__private::flavors::Network,
                            2u32,
                        > as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    );
                    match (select, self.replyer.is_empty()) {
                        (true, false) => &REFLECTION,
                        (true, true) => NONE,
                        (false, false) => NONE,
                        (false, true) => &REFLECTION,
                    }
                }
                CommentFieldIndex::Title => {
                    const REFLECTION: ::core::option::Option<
                        &::grost::__private::reflection::ObjectFieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                    > = ::core::option::Option::Some(
                        <CommentFieldReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                            ::grost::__private::flavors::Network,
                            3u32,
                        > as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    );
                    match (select, self.title) {
                        (true, true) => &REFLECTION,
                        (true, false) => NONE,
                        (false, true) => NONE,
                        (false, false) => &REFLECTION,
                    }
                }
                CommentFieldIndex::Content => {
                    const REFLECTION: ::core::option::Option<
                        &::grost::__private::reflection::ObjectFieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                    > = ::core::option::Option::Some(
                        <CommentFieldReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                            ::grost::__private::flavors::Network,
                            4u32,
                        > as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    );
                    match (select, self.content) {
                        (true, true) => &REFLECTION,
                        (true, false) => NONE,
                        (false, true) => NONE,
                        (false, false) => &REFLECTION,
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::ops::Index<CommentFieldIndex>
    for CommentSelector<::grost::__private::flavors::Network> {
        type Output = ::core::option::Option<
            &'static ::grost::__private::reflection::ObjectFieldReflection<
                ::grost::__private::flavors::Network,
            >,
        >;
        fn index(&self, indexer: CommentFieldIndex) -> &Self::Output {
            const NONE: &::core::option::Option<
                &::grost::__private::reflection::ObjectFieldReflection<
                    ::grost::__private::flavors::Network,
                >,
            > = &::core::option::Option::None;
            match indexer {
                CommentFieldIndex::User => {
                    const REFLECTION: ::core::option::Option<
                        &::grost::__private::reflection::ObjectFieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                    > = ::core::option::Option::Some(
                        <CommentFieldReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                            ::grost::__private::flavors::Network,
                            1u32,
                        > as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    );
                    if ::grost::__private::Selector::<
                        ::grost::__private::flavors::Network,
                    >::is_empty(&self.user) {
                        NONE
                    } else {
                        &REFLECTION
                    }
                }
                CommentFieldIndex::Replyer => {
                    const REFLECTION: ::core::option::Option<
                        &::grost::__private::reflection::ObjectFieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                    > = ::core::option::Option::Some(
                        <CommentFieldReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                            ::grost::__private::flavors::Network,
                            2u32,
                        > as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    );
                    if ::grost::__private::Selector::<
                        ::grost::__private::flavors::Network,
                    >::is_empty(&self.replyer) {
                        NONE
                    } else {
                        &REFLECTION
                    }
                }
                CommentFieldIndex::Title => {
                    const REFLECTION: ::core::option::Option<
                        &::grost::__private::reflection::ObjectFieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                    > = ::core::option::Option::Some(
                        <CommentFieldReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                            ::grost::__private::flavors::Network,
                            3u32,
                        > as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    );
                    if ::grost::__private::Selector::<
                        ::grost::__private::flavors::Network,
                    >::is_empty(&self.title) {
                        NONE
                    } else {
                        &REFLECTION
                    }
                }
                CommentFieldIndex::Content => {
                    const REFLECTION: ::core::option::Option<
                        &::grost::__private::reflection::ObjectFieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                    > = ::core::option::Option::Some(
                        <CommentFieldReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                            ::grost::__private::flavors::Network,
                            4u32,
                        > as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    );
                    if ::grost::__private::Selector::<
                        ::grost::__private::flavors::Network,
                    >::is_empty(&self.content) {
                        NONE
                    } else {
                        &REFLECTION
                    }
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::grost::__private::PartialEncode<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for PartialComment {
        fn partial_encode(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
            selector: &<Comment as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let mut offset = 0;
            let buf_len = buf.len();
            if let ::core::option::Option::Some(ref f) = self.user {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .user()
                        .partial_encode())(f, ctx, &mut buf[offset..], &selector.user)
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::PartialEncode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
            }
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .replyer()
                    .partial_encode())(
                        &self.replyer,
                        ctx,
                        &mut buf[offset..],
                        &selector.replyer,
                    )
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        )
                    })?;
            if let ::core::option::Option::Some(ref f) = self.title {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .title()
                        .partial_encode())(f, ctx, &mut buf[offset..], &selector.title)
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::PartialEncode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
            }
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .content()
                    .partial_encode())(
                        &self.content,
                        ctx,
                        &mut buf[offset..],
                        &selector.content,
                    )
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        )
                    })?;
            ::core::result::Result::Ok(offset)
        }
        fn partial_encoded_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            selector: &<Comment as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::primitive::usize {
            let mut len = 0;
            if let ::core::option::Option::Some(ref f) = self.user {
                len
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .user()
                        .partial_encoded_len())(f, ctx, &selector.user);
            }
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .replyer()
                    .partial_encoded_len())(&self.replyer, ctx, &selector.replyer);
            if let ::core::option::Option::Some(ref f) = self.title {
                len
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .title()
                        .partial_encoded_len())(f, ctx, &selector.title);
            }
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .content()
                    .partial_encoded_len())(&self.content, ctx, &selector.content);
            len
        }
        fn partial_encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            selector: &<Comment as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::primitive::usize {
            let encoded_len = <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encoded_len(self, ctx, selector);
            ::grost::__private::varing::encoded_u32_varint_len(
                encoded_len as ::core::primitive::u32,
            ) + encoded_len
        }
        fn partial_encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
            selector: &<Comment as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let encoded_len = <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encoded_len(self, ctx, selector);
            let buf_len = buf.len();
            let offset = ::grost::__private::varing::encode_u32_varint_to(
                    encoded_len as ::core::primitive::u32,
                    buf,
                )
                .map_err(|e| {
                    ::grost::__private::flavors::network::EncodeError::from_varint_error(
                            e,
                        )
                        .update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_length_delimited_len(
                                self,
                                ctx,
                                selector,
                            ),
                            buf_len,
                        )
                })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_length_delimited_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encode(self, ctx, &mut buf[offset..], selector)
                .map(|write| {
                    #[cfg(debug_assertions)]
                    {
                        ::grost::__private::debug_assert_write_eq::<
                            Self,
                        >(write, encoded_len);
                    }
                    write + offset
                })
                .map_err(|e| {
                    e
                        .update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_length_delimited_len(
                                self,
                                ctx,
                                selector,
                            ),
                            buf_len,
                        )
                })
        }
    }
    #[automatically_derived]
    impl ::grost::__private::PartialEncode<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for Comment {
        fn partial_encode(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
            selector: &<Comment as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let mut offset = 0;
            let buf_len = buf.len();
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .user()
                    .partial_encode())(
                        &self.user,
                        ctx,
                        &mut buf[offset..],
                        &selector.user,
                    )
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        )
                    })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .replyer()
                    .partial_encode())(
                        &self.replyer,
                        ctx,
                        &mut buf[offset..],
                        &selector.replyer,
                    )
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        )
                    })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .title()
                    .partial_encode())(
                        &self.title,
                        ctx,
                        &mut buf[offset..],
                        &selector.title,
                    )
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        )
                    })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .content()
                    .partial_encode())(
                        &self.content,
                        ctx,
                        &mut buf[offset..],
                        &selector.content,
                    )
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        )
                    })?;
            ::core::result::Result::Ok(offset)
        }
        fn partial_encoded_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            selector: &<Comment as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::primitive::usize {
            let mut len = 0;
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .user()
                    .partial_encoded_len())(&self.user, ctx, &selector.user);
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .replyer()
                    .partial_encoded_len())(&self.replyer, ctx, &selector.replyer);
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .title()
                    .partial_encoded_len())(&self.title, ctx, &selector.title);
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .content()
                    .partial_encoded_len())(&self.content, ctx, &selector.content);
            len
        }
        fn partial_encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            selector: &<Comment as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::primitive::usize {
            let encoded_len = <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encoded_len(self, ctx, selector);
            ::grost::__private::varing::encoded_u32_varint_len(
                encoded_len as ::core::primitive::u32,
            ) + encoded_len
        }
        fn partial_encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
            selector: &<Comment as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let encoded_len = <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encoded_len(self, ctx, selector);
            let buf_len = buf.len();
            let offset = ::grost::__private::varing::encode_u32_varint_to(
                    encoded_len as ::core::primitive::u32,
                    buf,
                )
                .map_err(|e| {
                    ::grost::__private::flavors::network::EncodeError::from_varint_error(
                            e,
                        )
                        .update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_length_delimited_len(
                                self,
                                ctx,
                                selector,
                            ),
                            buf_len,
                        )
                })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_length_delimited_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encode(self, ctx, &mut buf[offset..], selector)
                .map(|write| {
                    #[cfg(debug_assertions)]
                    {
                        ::grost::__private::debug_assert_write_eq::<
                            Self,
                        >(write, encoded_len);
                    }
                    write + offset
                })
                .map_err(|e| {
                    e
                        .update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_length_delimited_len(
                                self,
                                ctx,
                                selector,
                            ),
                            buf_len,
                        )
                })
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        '__grost_lifetime__,
    > ::grost::__private::PartialEncode<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for PartialCommentRef<'__grost_lifetime__, ::grost::__private::flavors::Network> {
        fn partial_encode(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
            selector: &<Comment as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let mut offset = 0;
            let buf_len = buf.len();
            if let ::core::option::Option::Some(ref f) = self.user {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .user()
                        .partial_encode_ref())(
                            f,
                            ctx,
                            &mut buf[offset..],
                            &selector.user,
                        )
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::PartialEncode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
            }
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .replyer()
                    .partial_encode_ref())(
                        &self.replyer,
                        ctx,
                        &mut buf[offset..],
                        &selector.replyer,
                    )
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        )
                    })?;
            if let ::core::option::Option::Some(ref f) = self.title {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .title()
                        .partial_encode_ref())(
                            f,
                            ctx,
                            &mut buf[offset..],
                            &selector.title,
                        )
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::PartialEncode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
            }
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .content()
                    .partial_encode_ref())(
                        &self.content,
                        ctx,
                        &mut buf[offset..],
                        &selector.content,
                    )
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        )
                    })?;
            ::core::result::Result::Ok(offset)
        }
        fn partial_encoded_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            selector: &<Comment as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::primitive::usize {
            let mut len = 0;
            if let ::core::option::Option::Some(ref f) = self.user {
                len
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .user()
                        .partial_encoded_ref_len())(f, ctx, &selector.user);
            }
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .replyer()
                    .partial_encoded_ref_len())(&self.replyer, ctx, &selector.replyer);
            if let ::core::option::Option::Some(ref f) = self.title {
                len
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .title()
                        .partial_encoded_ref_len())(f, ctx, &selector.title);
            }
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .content()
                    .partial_encoded_ref_len())(&self.content, ctx, &selector.content);
            len
        }
        fn partial_encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            selector: &<Comment as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::primitive::usize {
            let encoded_len = <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encoded_len(self, ctx, selector);
            ::grost::__private::varing::encoded_u32_varint_len(
                encoded_len as ::core::primitive::u32,
            ) + encoded_len
        }
        fn partial_encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
            selector: &<Comment as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let encoded_len = <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encoded_len(self, ctx, selector);
            let buf_len = buf.len();
            let offset = ::grost::__private::varing::encode_u32_varint_to(
                    encoded_len as ::core::primitive::u32,
                    buf,
                )
                .map_err(|e| {
                    ::grost::__private::flavors::network::EncodeError::from_varint_error(
                            e,
                        )
                        .update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_length_delimited_len(
                                self,
                                ctx,
                                selector,
                            ),
                            buf_len,
                        )
                })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::partial_encoded_length_delimited_len(self, ctx, selector),
                        buf_len,
                    ),
                );
            }
            <Self as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::partial_encode(self, ctx, &mut buf[offset..], selector)
                .map(|write| {
                    #[cfg(debug_assertions)]
                    {
                        ::grost::__private::debug_assert_write_eq::<
                            Self,
                        >(write, encoded_len);
                    }
                    write + offset
                })
                .map_err(|e| {
                    e
                        .update(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_length_delimited_len(
                                self,
                                ctx,
                                selector,
                            ),
                            buf_len,
                        )
                })
        }
    }
    #[automatically_derived]
    impl ::grost::__private::Encode<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for Comment {
        fn encode(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let mut offset = 0;
            let buf_len = buf.len();
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .user()
                    .encode())(&self.user, ctx, &mut buf[offset..])
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        )
                    })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .replyer()
                    .encode())(&self.replyer, ctx, &mut buf[offset..])
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        )
                    })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .title()
                    .encode())(&self.title, ctx, &mut buf[offset..])
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        )
                    })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .content()
                    .encode())(&self.content, ctx, &mut buf[offset..])
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        )
                    })?;
            ::core::result::Result::Ok(offset)
        }
        fn encoded_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
            let mut len = 0;
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .user()
                    .encoded_len())(&self.user, ctx);
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .replyer()
                    .encoded_len())(&self.replyer, ctx);
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .title()
                    .encoded_len())(&self.title, ctx);
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .content()
                    .encoded_len())(&self.content, ctx);
            len
        }
        fn encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
            let encoded_len = <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encoded_len(self, ctx);
            ::grost::__private::varing::encoded_u32_varint_len(
                encoded_len as ::core::primitive::u32,
            ) + encoded_len
        }
        fn encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let encoded_len = <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encoded_len(self, ctx);
            let buf_len = buf.len();
            let offset = ::grost::__private::varing::encode_u32_varint_to(
                    encoded_len as ::core::primitive::u32,
                    buf,
                )
                .map_err(|e| {
                    ::grost::__private::flavors::network::EncodeError::from_varint_error(
                            e,
                        )
                        .update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_length_delimited_len(self, ctx),
                            buf_len,
                        )
                })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_length_delimited_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encode(self, ctx, &mut buf[offset..])
                .map(|write| {
                    #[cfg(debug_assertions)]
                    {
                        ::grost::__private::debug_assert_write_eq::<
                            Self,
                        >(write, encoded_len);
                    }
                    write + offset
                })
                .map_err(|e| {
                    e
                        .update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_length_delimited_len(self, ctx),
                            buf_len,
                        )
                })
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::grost::__private::Encode<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for PartialComment {
        fn encode(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let mut offset = 0;
            let buf_len = buf.len();
            if let ::core::option::Option::Some(ref f) = self.user {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .user()
                        .encode())(f, ctx, &mut buf[offset..])
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::Encode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
            }
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .replyer()
                    .encode())(&self.replyer, ctx, &mut buf[offset..])
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        )
                    })?;
            if let ::core::option::Option::Some(ref f) = self.title {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .title()
                        .encode())(f, ctx, &mut buf[offset..])
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::Encode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
            }
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .content()
                    .encode())(&self.content, ctx, &mut buf[offset..])
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        )
                    })?;
            ::core::result::Result::Ok(offset)
        }
        fn encoded_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
            let mut len = 0;
            if let ::core::option::Option::Some(ref f) = self.user {
                len
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .user()
                        .encoded_len())(f, ctx);
            }
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .replyer()
                    .encoded_len())(&self.replyer, ctx);
            if let ::core::option::Option::Some(ref f) = self.title {
                len
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .title()
                        .encoded_len())(f, ctx);
            }
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .content()
                    .encoded_len())(&self.content, ctx);
            len
        }
        fn encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
            let encoded_len = <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encoded_len(self, ctx);
            ::grost::__private::varing::encoded_u32_varint_len(
                encoded_len as ::core::primitive::u32,
            ) + encoded_len
        }
        fn encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let encoded_len = <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encoded_len(self, ctx);
            let buf_len = buf.len();
            let offset = ::grost::__private::varing::encode_u32_varint_to(
                    encoded_len as ::core::primitive::u32,
                    buf,
                )
                .map_err(|e| {
                    ::grost::__private::flavors::network::EncodeError::from_varint_error(
                            e,
                        )
                        .update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_length_delimited_len(self, ctx),
                            buf_len,
                        )
                })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_length_delimited_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encode(self, ctx, &mut buf[offset..])
                .map(|write| {
                    #[cfg(debug_assertions)]
                    {
                        ::grost::__private::debug_assert_write_eq::<
                            Self,
                        >(write, encoded_len);
                    }
                    write + offset
                })
                .map_err(|e| {
                    e
                        .update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_length_delimited_len(self, ctx),
                            buf_len,
                        )
                })
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<
        '__grost_flavor__,
    > ::grost::__private::Encode<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for PartialCommentRef<'__grost_flavor__, ::grost::__private::flavors::Network> {
        fn encode(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let mut offset = 0;
            let buf_len = buf.len();
            if let ::core::option::Option::Some(ref f) = self.user {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .user()
                        .encode_ref())(f, ctx, &mut buf[offset..])
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::Encode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
            }
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .replyer()
                    .encode_ref())(&self.replyer, ctx, &mut buf[offset..])
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        )
                    })?;
            if let ::core::option::Option::Some(ref f) = self.title {
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .title()
                        .encode_ref())(f, ctx, &mut buf[offset..])
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::Encode<
                                    ::grost::__private::flavors::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
            }
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            offset
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .content()
                    .encode_ref())(&self.content, ctx, &mut buf[offset..])
                    .map_err(|e| {
                        e.update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_len(self, ctx),
                            buf_len,
                        )
                    })?;
            ::core::result::Result::Ok(offset)
        }
        fn encoded_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
            let mut len = 0;
            if let ::core::option::Option::Some(ref f) = self.user {
                len
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .user()
                        .encoded_ref_len())(f, ctx);
            }
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .replyer()
                    .encoded_ref_len())(&self.replyer, ctx);
            if let ::core::option::Option::Some(ref f) = self.title {
                len
                    += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .title()
                        .encoded_ref_len())(f, ctx);
            }
            len
                += (<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .content()
                    .encoded_ref_len())(&self.content, ctx);
            len
        }
        fn encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
            let encoded_len = <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encoded_len(self, ctx);
            ::grost::__private::varing::encoded_u32_varint_len(
                encoded_len as ::core::primitive::u32,
            ) + encoded_len
        }
        fn encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::EncodeError,
        > {
            let encoded_len = <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encoded_len(self, ctx);
            let buf_len = buf.len();
            let offset = ::grost::__private::varing::encode_u32_varint_to(
                    encoded_len as ::core::primitive::u32,
                    buf,
                )
                .map_err(|e| {
                    ::grost::__private::flavors::network::EncodeError::from_varint_error(
                            e,
                        )
                        .update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_length_delimited_len(self, ctx),
                            buf_len,
                        )
                })?;
            if offset >= buf_len {
                return ::core::result::Result::Err(
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <Self as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            ::grost::__private::flavors::network::LengthDelimited,
                        >>::encoded_length_delimited_len(self, ctx),
                        buf_len,
                    ),
                );
            }
            <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Network,
                ::grost::__private::flavors::network::LengthDelimited,
            >>::encode(self, ctx, &mut buf[offset..])
                .map(|write| {
                    #[cfg(debug_assertions)]
                    {
                        ::grost::__private::debug_assert_write_eq::<
                            Self,
                        >(write, encoded_len);
                    }
                    write + offset
                })
                .map_err(|e| {
                    e
                        .update(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_length_delimited_len(self, ctx),
                            buf_len,
                        )
                })
        }
    }
    fn insufficient_buffer_error<T, W>(
        f: &T,
        ctx: &<::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::Context,
        selector: ::core::option::Option<
            &<T as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                W,
            >>::Selector,
        >,
        buf_len: ::core::primitive::usize,
    ) -> ::grost::__private::flavors::network::EncodeError
    where
        T: ::grost::__private::PartialEncode<::grost::__private::flavors::Network, W>
            + ::grost::__private::Encode<::grost::__private::flavors::Network, W>
            + ::grost::__private::Selectable<::grost::__private::flavors::Network, W>
            + ?::core::marker::Sized,
        W: ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
    {
        match selector {
            ::core::option::Option::Some(selector) => {
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    <T as ::grost::__private::PartialEncode<
                        ::grost::__private::flavors::Network,
                        W,
                    >>::partial_encoded_len(f, ctx, selector),
                    buf_len,
                )
            }
            ::core::option::Option::None => {
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    <T as ::grost::__private::Encode<
                        ::grost::__private::flavors::Network,
                        W,
                    >>::encoded_length_delimited_len(f, ctx),
                    buf_len,
                )
            }
        }
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::PartialEncodeField,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &User,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
            &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &User,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<User as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                const ENCODED_LEN_FN: CommentFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::PartialEncodeField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    1u32,
                > = <Comment>::reflection::<::grost::__private::flavors::Network>()
                    .user()
                    .partial_encoded_len();
                let identifier_len = *<Comment>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .user()
                    .encoded_identifier_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .user()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                <User as ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::partial_encode_length_delimited(f, ctx, &mut buf[offset..], selector)
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::PartialEncodeField,
            >,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &User,
            &::grost::__private::flavors::network::Context,
            &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &User,
                ctx: &::grost::__private::flavors::network::Context,
                selector: &<User as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::primitive::usize {
                (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .user()
                    .encoded_identifier_len())
                    + <User as ::grost::__private::PartialEncode<
                        ::grost::__private::flavors::Network,
                        <User as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::partial_encoded_length_delimited_len(f, ctx, selector)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::EncodeField,
            >,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &User,
            &::grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &User,
                ctx: &::grost::__private::flavors::network::Context,
            ) -> ::core::primitive::usize {
                (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .user()
                    .encoded_identifier_len())
                    + <User as ::grost::__private::Encode<
                        ::grost::__private::flavors::Network,
                        <User as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::encoded_length_delimited_len(f, ctx)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::EncodeField,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &User,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &User,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                const ENCODED_LEN_FN: CommentFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::EncodeField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    1u32,
                > = <Comment>::reflection::<::grost::__private::flavors::Network>()
                    .user()
                    .encoded_len();
                let identifier_len = *<Comment>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .user()
                    .encoded_identifier_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .user()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                <User as ::grost::__private::Encode<
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::encode_length_delimited(f, ctx, &mut buf[offset..])
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::PartialEncodeRefField,
            >,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &<User as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
            &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &<User as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <User as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
                selector: &<User as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::primitive::usize {
                (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .user()
                    .encoded_identifier_len())
                    + <<User as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <User as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output as ::grost::__private::PartialEncode<
                        ::grost::__private::flavors::Network,
                        <User as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::partial_encoded_length_delimited_len(f, ctx, selector)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::PartialEncodeRefField,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &<User as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
            &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &<User as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <User as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<User as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                let identifier_len = *<Comment>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .user()
                    .encoded_identifier_len();
                const ENCODED_LEN_FN: CommentFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::PartialEncodeRefField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    1u32,
                > = <Comment>::reflection::<::grost::__private::flavors::Network>()
                    .user()
                    .partial_encoded_ref_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .user()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                <<User as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <User as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output as ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::partial_encode_length_delimited(f, ctx, buf, selector)
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::EncodeRefField,
            >,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &<User as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &<User as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <User as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
            ) -> ::core::primitive::usize {
                (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .user()
                    .encoded_identifier_len())
                    + <<User as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <User as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output as ::grost::__private::Encode<
                        ::grost::__private::flavors::Network,
                        <User as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::encoded_length_delimited_len(f, ctx)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::EncodeRefField,
        >,
        ::grost::__private::flavors::Network,
        1u32,
    > {
        type Reflection = fn(
            &<User as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &<User as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <User as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                let identifier_len = *<Comment>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .user()
                    .encoded_identifier_len();
                const ENCODED_LEN_FN: CommentFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::EncodeRefField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    1u32,
                > = <Comment>::reflection::<::grost::__private::flavors::Network>()
                    .user()
                    .encoded_ref_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .user()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                <<User as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <User as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output as ::grost::__private::Encode<
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::encode_length_delimited(f, ctx, buf)
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::PartialEncodeField,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &::core::option::Option<User>,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
            &<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &::core::option::Option<User>,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        User,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                const ENCODED_LEN_FN: CommentFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::PartialEncodeField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    2u32,
                > = <Comment>::reflection::<::grost::__private::flavors::Network>()
                    .replyer()
                    .partial_encoded_len();
                let identifier_len = *<Comment>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .replyer()
                    .encoded_identifier_len();
                match f {
                    ::core::option::Option::None => ::core::result::Result::Ok(0),
                    ::core::option::Option::Some(field) => {
                        let buf_len = buf.len();
                        let mut offset = 0;
                        if offset > buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(f, ctx, selector),
                                    buf_len,
                                ),
                            );
                        }
                        buf[offset..offset + identifier_len]
                            .copy_from_slice(
                                &<Comment>::reflection::<
                                    ::grost::__private::flavors::Network,
                                >()
                                    .replyer()
                                    .encoded_identifier(),
                            );
                        offset += identifier_len;
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(f, ctx, selector),
                                    buf_len,
                                ),
                            );
                        }
                        <User as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                User,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::partial_encode_length_delimited(
                                field,
                                ctx,
                                &mut buf[offset..],
                                selector,
                            )
                            .map(|len| offset + len)
                            .map_err(|e| {
                                e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len)
                            })
                    }
                }
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::PartialEncodeField,
            >,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &::core::option::Option<User>,
            &::grost::__private::flavors::network::Context,
            &<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::core::option::Option<User>,
                ctx: &::grost::__private::flavors::network::Context,
                selector: &<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        User,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::primitive::usize {
                match f {
                    ::core::option::Option::None => 0,
                    ::core::option::Option::Some(f) => {
                        (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .replyer()
                            .encoded_identifier_len())
                            + <User as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    User,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >>::partial_encoded_length_delimited_len(f, ctx, selector)
                    }
                }
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::EncodeField,
            >,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &::core::option::Option<User>,
            &::grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::core::option::Option<User>,
                ctx: &::grost::__private::flavors::network::Context,
            ) -> ::core::primitive::usize {
                match f {
                    ::core::option::Option::None => 0,
                    ::core::option::Option::Some(f) => {
                        (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .replyer()
                            .encoded_identifier_len())
                            + <User as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    User,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >>::encoded_length_delimited_len(f, ctx)
                    }
                }
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::EncodeField,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &::core::option::Option<User>,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &::core::option::Option<User>,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                const ENCODED_LEN_FN: CommentFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::EncodeField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    2u32,
                > = <Comment>::reflection::<::grost::__private::flavors::Network>()
                    .replyer()
                    .encoded_len();
                let identifier_len = *<Comment>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .replyer()
                    .encoded_identifier_len();
                match f {
                    ::core::option::Option::None => ::core::result::Result::Ok(0),
                    ::core::option::Option::Some(field) => {
                        let buf_len = buf.len();
                        let mut offset = 0;
                        if offset > buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(f, ctx),
                                    buf_len,
                                ),
                            );
                        }
                        buf[offset..offset + identifier_len]
                            .copy_from_slice(
                                &<Comment>::reflection::<
                                    ::grost::__private::flavors::Network,
                                >()
                                    .replyer()
                                    .encoded_identifier(),
                            );
                        offset += identifier_len;
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(f, ctx),
                                    buf_len,
                                ),
                            );
                        }
                        <User as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                User,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::encode_length_delimited(field, ctx, &mut buf[offset..])
                            .map(|len| offset + len)
                            .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
                    }
                }
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::PartialEncodeRefField,
            >,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &::core::option::Option<
                <::core::option::Option<
                    User,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            User,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
            >,
            &::grost::__private::flavors::network::Context,
            &<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::core::option::Option<
                    <::core::option::Option<
                        User,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                User,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output,
                >,
                ctx: &::grost::__private::flavors::network::Context,
                selector: &<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        User,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::primitive::usize {
                match f {
                    ::core::option::Option::None => 0,
                    ::core::option::Option::Some(f) => {
                        (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .replyer()
                            .encoded_identifier_len())
                            + <<::core::option::Option<
                                User,
                            > as ::grost::__private::convert::State<
                                ::grost::__private::convert::Encoded<
                                    '_,
                                    ::grost::__private::flavors::Network,
                                    <::core::option::Option<
                                        User,
                                    > as ::grost::__private::flavors::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                >,
                            >>::Output as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    User,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >>::partial_encoded_length_delimited_len(f, ctx, selector)
                    }
                }
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::PartialEncodeRefField,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &::core::option::Option<
                <::core::option::Option<
                    User,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            User,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
            >,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
            &<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                field: &::core::option::Option<
                    <::core::option::Option<
                        User,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                User,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output,
                >,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        User,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                match field {
                    ::core::option::Option::None => ::core::result::Result::Ok(0),
                    ::core::option::Option::Some(f) => {
                        let identifier_len = *<Comment>::reflection::<
                            ::grost::__private::flavors::Network,
                        >()
                            .replyer()
                            .encoded_identifier_len();
                        const ENCODED_LEN_FN: CommentFieldReflection<
                            ::grost::__private::reflection::encode::EncodeReflection<
                                ::grost::__private::reflection::Len<
                                    ::grost::__private::reflection::encode::PartialEncodeRefField,
                                >,
                            >,
                            ::grost::__private::flavors::Network,
                            2u32,
                        > = <Comment>::reflection::<
                            ::grost::__private::flavors::Network,
                        >()
                            .replyer()
                            .partial_encoded_ref_len();
                        let buf_len = buf.len();
                        let mut offset = 0;
                        if offset > buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(field, ctx, selector),
                                    buf_len,
                                ),
                            );
                        }
                        buf[offset..offset + identifier_len]
                            .copy_from_slice(
                                &<Comment>::reflection::<
                                    ::grost::__private::flavors::Network,
                                >()
                                    .replyer()
                                    .encoded_identifier(),
                            );
                        offset += identifier_len;
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(field, ctx, selector),
                                    buf_len,
                                ),
                            );
                        }
                        <<::core::option::Option<
                            User,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Encoded<
                                '_,
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    User,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >,
                        >>::Output as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                User,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::partial_encode_length_delimited(f, ctx, buf, selector)
                            .map(|len| offset + len)
                            .map_err(|e| {
                                e.update((ENCODED_LEN_FN)(field, ctx, selector), buf_len)
                            })
                    }
                }
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::EncodeRefField,
            >,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &::core::option::Option<
                <::core::option::Option<
                    User,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            User,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
            >,
            &::grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::core::option::Option<
                    <::core::option::Option<
                        User,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                User,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output,
                >,
                ctx: &::grost::__private::flavors::network::Context,
            ) -> ::core::primitive::usize {
                match f {
                    ::core::option::Option::None => 0,
                    ::core::option::Option::Some(f) => {
                        (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .replyer()
                            .encoded_identifier_len())
                            + <<::core::option::Option<
                                User,
                            > as ::grost::__private::convert::State<
                                ::grost::__private::convert::Encoded<
                                    '_,
                                    ::grost::__private::flavors::Network,
                                    <::core::option::Option<
                                        User,
                                    > as ::grost::__private::flavors::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                >,
                            >>::Output as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    User,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >>::encoded_length_delimited_len(f, ctx)
                    }
                }
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::EncodeRefField,
        >,
        ::grost::__private::flavors::Network,
        2u32,
    > {
        type Reflection = fn(
            &::core::option::Option<
                <::core::option::Option<
                    User,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            User,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
            >,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                field: &::core::option::Option<
                    <::core::option::Option<
                        User,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                User,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output,
                >,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                match field {
                    ::core::option::Option::Some(f) => {
                        let identifier_len = *<Comment>::reflection::<
                            ::grost::__private::flavors::Network,
                        >()
                            .replyer()
                            .encoded_identifier_len();
                        const ENCODED_LEN_FN: CommentFieldReflection<
                            ::grost::__private::reflection::encode::EncodeReflection<
                                ::grost::__private::reflection::Len<
                                    ::grost::__private::reflection::encode::EncodeRefField,
                                >,
                            >,
                            ::grost::__private::flavors::Network,
                            2u32,
                        > = <Comment>::reflection::<
                            ::grost::__private::flavors::Network,
                        >()
                            .replyer()
                            .encoded_ref_len();
                        let buf_len = buf.len();
                        let mut offset = 0;
                        if offset > buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(field, ctx),
                                    buf_len,
                                ),
                            );
                        }
                        buf[offset..offset + identifier_len]
                            .copy_from_slice(
                                &<Comment>::reflection::<
                                    ::grost::__private::flavors::Network,
                                >()
                                    .replyer()
                                    .encoded_identifier(),
                            );
                        offset += identifier_len;
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(field, ctx),
                                    buf_len,
                                ),
                            );
                        }
                        <<::core::option::Option<
                            User,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Encoded<
                                '_,
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    User,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >,
                        >>::Output as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                User,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::encode_length_delimited(f, ctx, buf)
                            .map(|len| offset + len)
                            .map_err(|e| e.update((ENCODED_LEN_FN)(field, ctx), buf_len))
                    }
                    ::core::option::Option::None => ::core::result::Result::Ok(0),
                }
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::PartialEncodeField,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &::std::string::String,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
            &<::std::string::String as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &::std::string::String,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                const ENCODED_LEN_FN: CommentFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::PartialEncodeField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    3u32,
                > = <Comment>::reflection::<::grost::__private::flavors::Network>()
                    .title()
                    .partial_encoded_len();
                let identifier_len = *<Comment>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .title()
                    .encoded_identifier_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .title()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                <::std::string::String as ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::partial_encode_length_delimited(f, ctx, &mut buf[offset..], selector)
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::PartialEncodeField,
            >,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &::std::string::String,
            &::grost::__private::flavors::network::Context,
            &<::std::string::String as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::std::string::String,
                ctx: &::grost::__private::flavors::network::Context,
                selector: &<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::primitive::usize {
                (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .title()
                    .encoded_identifier_len())
                    + <::std::string::String as ::grost::__private::PartialEncode<
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::partial_encoded_length_delimited_len(f, ctx, selector)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::EncodeField,
            >,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &::std::string::String,
            &::grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::std::string::String,
                ctx: &::grost::__private::flavors::network::Context,
            ) -> ::core::primitive::usize {
                (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .title()
                    .encoded_identifier_len())
                    + <::std::string::String as ::grost::__private::Encode<
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::encoded_length_delimited_len(f, ctx)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::EncodeField,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &::std::string::String,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &::std::string::String,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                const ENCODED_LEN_FN: CommentFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::EncodeField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    3u32,
                > = <Comment>::reflection::<::grost::__private::flavors::Network>()
                    .title()
                    .encoded_len();
                let identifier_len = *<Comment>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .title()
                    .encoded_identifier_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .title()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                <::std::string::String as ::grost::__private::Encode<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::encode_length_delimited(f, ctx, &mut buf[offset..])
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::PartialEncodeRefField,
            >,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &<::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
            &<::std::string::String as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &<::std::string::String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
                selector: &<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::primitive::usize {
                (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .title()
                    .encoded_identifier_len())
                    + <<::std::string::String as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output as ::grost::__private::PartialEncode<
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::partial_encoded_length_delimited_len(f, ctx, selector)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::PartialEncodeRefField,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &<::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
            &<::std::string::String as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &<::std::string::String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                let identifier_len = *<Comment>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .title()
                    .encoded_identifier_len();
                const ENCODED_LEN_FN: CommentFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::PartialEncodeRefField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    3u32,
                > = <Comment>::reflection::<::grost::__private::flavors::Network>()
                    .title()
                    .partial_encoded_ref_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .title()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                <<::std::string::String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output as ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::partial_encode_length_delimited(f, ctx, buf, selector)
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::EncodeRefField,
            >,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &<::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &<::std::string::String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
            ) -> ::core::primitive::usize {
                (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                    .title()
                    .encoded_identifier_len())
                    + <<::std::string::String as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output as ::grost::__private::Encode<
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::encoded_length_delimited_len(f, ctx)
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::EncodeRefField,
        >,
        ::grost::__private::flavors::Network,
        3u32,
    > {
        type Reflection = fn(
            &<::std::string::String as ::grost::__private::convert::State<
                ::grost::__private::convert::Encoded<
                    '_,
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >,
            >>::Output,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &<::std::string::String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                let identifier_len = *<Comment>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .title()
                    .encoded_identifier_len();
                const ENCODED_LEN_FN: CommentFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::EncodeRefField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    3u32,
                > = <Comment>::reflection::<::grost::__private::flavors::Network>()
                    .title()
                    .encoded_ref_len();
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + identifier_len]
                    .copy_from_slice(
                        &<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .title()
                            .encoded_identifier(),
                    );
                offset += identifier_len;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            (ENCODED_LEN_FN)(f, ctx),
                            buf_len,
                        ),
                    );
                }
                <<::std::string::String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output as ::grost::__private::Encode<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::encode_length_delimited(f, ctx, buf)
                    .map(|len| offset + len)
                    .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::PartialEncodeField,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = fn(
            &::core::option::Option<::std::string::String>,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
            &<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &::core::option::Option<::std::string::String>,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                const ENCODED_LEN_FN: CommentFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::PartialEncodeField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    4u32,
                > = <Comment>::reflection::<::grost::__private::flavors::Network>()
                    .content()
                    .partial_encoded_len();
                let identifier_len = *<Comment>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .content()
                    .encoded_identifier_len();
                match f {
                    ::core::option::Option::None => ::core::result::Result::Ok(0),
                    ::core::option::Option::Some(field) => {
                        let buf_len = buf.len();
                        let mut offset = 0;
                        if offset > buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(f, ctx, selector),
                                    buf_len,
                                ),
                            );
                        }
                        buf[offset..offset + identifier_len]
                            .copy_from_slice(
                                &<Comment>::reflection::<
                                    ::grost::__private::flavors::Network,
                                >()
                                    .content()
                                    .encoded_identifier(),
                            );
                        offset += identifier_len;
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(f, ctx, selector),
                                    buf_len,
                                ),
                            );
                        }
                        <::std::string::String as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::partial_encode_length_delimited(
                                field,
                                ctx,
                                &mut buf[offset..],
                                selector,
                            )
                            .map(|len| offset + len)
                            .map_err(|e| {
                                e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len)
                            })
                    }
                }
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::PartialEncodeField,
            >,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = fn(
            &::core::option::Option<::std::string::String>,
            &::grost::__private::flavors::network::Context,
            &<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::core::option::Option<::std::string::String>,
                ctx: &::grost::__private::flavors::network::Context,
                selector: &<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::primitive::usize {
                match f {
                    ::core::option::Option::None => 0,
                    ::core::option::Option::Some(f) => {
                        (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .content()
                            .encoded_identifier_len())
                            + <::std::string::String as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >>::partial_encoded_length_delimited_len(f, ctx, selector)
                    }
                }
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::EncodeField,
            >,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = fn(
            &::core::option::Option<::std::string::String>,
            &::grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::core::option::Option<::std::string::String>,
                ctx: &::grost::__private::flavors::network::Context,
            ) -> ::core::primitive::usize {
                match f {
                    ::core::option::Option::None => 0,
                    ::core::option::Option::Some(f) => {
                        (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .content()
                            .encoded_identifier_len())
                            + <::std::string::String as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >>::encoded_length_delimited_len(f, ctx)
                    }
                }
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::EncodeField,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = fn(
            &::core::option::Option<::std::string::String>,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                f: &::core::option::Option<::std::string::String>,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                const ENCODED_LEN_FN: CommentFieldReflection<
                    ::grost::__private::reflection::encode::EncodeReflection<
                        ::grost::__private::reflection::Len<
                            ::grost::__private::reflection::encode::EncodeField,
                        >,
                    >,
                    ::grost::__private::flavors::Network,
                    4u32,
                > = <Comment>::reflection::<::grost::__private::flavors::Network>()
                    .content()
                    .encoded_len();
                let identifier_len = *<Comment>::reflection::<
                    ::grost::__private::flavors::Network,
                >()
                    .content()
                    .encoded_identifier_len();
                match f {
                    ::core::option::Option::None => ::core::result::Result::Ok(0),
                    ::core::option::Option::Some(field) => {
                        let buf_len = buf.len();
                        let mut offset = 0;
                        if offset > buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(f, ctx),
                                    buf_len,
                                ),
                            );
                        }
                        buf[offset..offset + identifier_len]
                            .copy_from_slice(
                                &<Comment>::reflection::<
                                    ::grost::__private::flavors::Network,
                                >()
                                    .content()
                                    .encoded_identifier(),
                            );
                        offset += identifier_len;
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(f, ctx),
                                    buf_len,
                                ),
                            );
                        }
                        <::std::string::String as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::encode_length_delimited(field, ctx, &mut buf[offset..])
                            .map(|len| offset + len)
                            .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
                    }
                }
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::PartialEncodeRefField,
            >,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = fn(
            &::core::option::Option<
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
            >,
            &::grost::__private::flavors::network::Context,
            &<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::core::option::Option<
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output,
                >,
                ctx: &::grost::__private::flavors::network::Context,
                selector: &<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::primitive::usize {
                match f {
                    ::core::option::Option::None => 0,
                    ::core::option::Option::Some(f) => {
                        (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .content()
                            .encoded_identifier_len())
                            + <<::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::convert::State<
                                ::grost::__private::convert::Encoded<
                                    '_,
                                    ::grost::__private::flavors::Network,
                                    <::core::option::Option<
                                        ::std::string::String,
                                    > as ::grost::__private::flavors::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                >,
                            >>::Output as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >>::partial_encoded_length_delimited_len(f, ctx, selector)
                    }
                }
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::PartialEncodeRefField,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = fn(
            &::core::option::Option<
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
            >,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
            &<::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                field: &::core::option::Option<
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output,
                >,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                match field {
                    ::core::option::Option::None => ::core::result::Result::Ok(0),
                    ::core::option::Option::Some(f) => {
                        let identifier_len = *<Comment>::reflection::<
                            ::grost::__private::flavors::Network,
                        >()
                            .content()
                            .encoded_identifier_len();
                        const ENCODED_LEN_FN: CommentFieldReflection<
                            ::grost::__private::reflection::encode::EncodeReflection<
                                ::grost::__private::reflection::Len<
                                    ::grost::__private::reflection::encode::PartialEncodeRefField,
                                >,
                            >,
                            ::grost::__private::flavors::Network,
                            4u32,
                        > = <Comment>::reflection::<
                            ::grost::__private::flavors::Network,
                        >()
                            .content()
                            .partial_encoded_ref_len();
                        let buf_len = buf.len();
                        let mut offset = 0;
                        if offset > buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(field, ctx, selector),
                                    buf_len,
                                ),
                            );
                        }
                        buf[offset..offset + identifier_len]
                            .copy_from_slice(
                                &<Comment>::reflection::<
                                    ::grost::__private::flavors::Network,
                                >()
                                    .content()
                                    .encoded_identifier(),
                            );
                        offset += identifier_len;
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(field, ctx, selector),
                                    buf_len,
                                ),
                            );
                        }
                        <<::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Encoded<
                                '_,
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >,
                        >>::Output as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::partial_encode_length_delimited(f, ctx, buf, selector)
                            .map(|len| offset + len)
                            .map_err(|e| {
                                e.update((ENCODED_LEN_FN)(field, ctx, selector), buf_len)
                            })
                    }
                }
            }
            encode
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::Len<
                ::grost::__private::reflection::encode::EncodeRefField,
            >,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = fn(
            &::core::option::Option<
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
            >,
            &::grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize;
        const REFLECTION: &Self::Reflection = &{
            fn encoded_len(
                f: &::core::option::Option<
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output,
                >,
                ctx: &::grost::__private::flavors::network::Context,
            ) -> ::core::primitive::usize {
                match f {
                    ::core::option::Option::None => 0,
                    ::core::option::Option::Some(f) => {
                        (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                            .content()
                            .encoded_identifier_len())
                            + <<::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::convert::State<
                                ::grost::__private::convert::Encoded<
                                    '_,
                                    ::grost::__private::flavors::Network,
                                    <::core::option::Option<
                                        ::std::string::String,
                                    > as ::grost::__private::flavors::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                >,
                            >>::Output as ::grost::__private::Encode<
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >>::encoded_length_delimited_len(f, ctx)
                    }
                }
            }
            encoded_len
        };
    }
    #[allow(clippy::type_complexity)]
    #[automatically_derived]
    impl ::grost::__private::reflection::Reflectable<
        ::grost::__private::flavors::Network,
    >
    for CommentFieldReflection<
        ::grost::__private::reflection::encode::EncodeReflection<
            ::grost::__private::reflection::encode::EncodeRefField,
        >,
        ::grost::__private::flavors::Network,
        4u32,
    > {
        type Reflection = fn(
            &::core::option::Option<
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Encoded<
                        '_,
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >,
                >>::Output,
            >,
            &::grost::__private::flavors::network::Context,
            &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::flavors::network::EncodeError,
        >;
        const REFLECTION: &Self::Reflection = &{
            fn encode(
                field: &::core::option::Option<
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Encoded<
                            '_,
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >,
                    >>::Output,
                >,
                ctx: &::grost::__private::flavors::network::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                ::grost::__private::flavors::network::EncodeError,
            > {
                match field {
                    ::core::option::Option::Some(f) => {
                        let identifier_len = *<Comment>::reflection::<
                            ::grost::__private::flavors::Network,
                        >()
                            .content()
                            .encoded_identifier_len();
                        const ENCODED_LEN_FN: CommentFieldReflection<
                            ::grost::__private::reflection::encode::EncodeReflection<
                                ::grost::__private::reflection::Len<
                                    ::grost::__private::reflection::encode::EncodeRefField,
                                >,
                            >,
                            ::grost::__private::flavors::Network,
                            4u32,
                        > = <Comment>::reflection::<
                            ::grost::__private::flavors::Network,
                        >()
                            .content()
                            .encoded_ref_len();
                        let buf_len = buf.len();
                        let mut offset = 0;
                        if offset > buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(field, ctx),
                                    buf_len,
                                ),
                            );
                        }
                        buf[offset..offset + identifier_len]
                            .copy_from_slice(
                                &<Comment>::reflection::<
                                    ::grost::__private::flavors::Network,
                                >()
                                    .content()
                                    .encoded_identifier(),
                            );
                        offset += identifier_len;
                        if offset >= buf_len {
                            return ::core::result::Result::Err(
                                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                                    (ENCODED_LEN_FN)(field, ctx),
                                    buf_len,
                                ),
                            );
                        }
                        <<::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Encoded<
                                '_,
                                ::grost::__private::flavors::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Network,
                                >>::Format,
                            >,
                        >>::Output as ::grost::__private::Encode<
                            ::grost::__private::flavors::Network,
                            <::core::option::Option<
                                ::std::string::String,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::encode_length_delimited(f, ctx, buf)
                            .map(|len| offset + len)
                            .map_err(|e| e.update((ENCODED_LEN_FN)(field, ctx), buf_len))
                    }
                    ::core::option::Option::None => ::core::result::Result::Ok(0),
                }
            }
            encode
        };
    }
};
