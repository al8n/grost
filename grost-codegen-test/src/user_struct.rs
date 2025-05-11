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
    #[allow(clippy::type_complexity)]
    pub struct PartialUserRef<'__grost_flavor__> {
        name: ::core::option::Option<
            <::std::string::String as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        >,
        age: ::core::option::Option<
            <u32 as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <u32 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        >,
        email: ::core::option::Option<
            <::std::string::String as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        >,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::grost::__private::Referenceable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for User {
        type Ref<'__grost_flavor__> = PartialUserRef<'__grost_flavor__>
        where
            Self: '__grost_flavor__;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<'__grost_flavor__> ::core::default::Default
    for PartialUserRef<'__grost_flavor__> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_flavor__> PartialUserRef<'__grost_flavor__> {
        /// Creates an empty instance.
        #[inline]
        pub const fn new() -> Self {
            Self {
                name: ::core::option::Option::None,
                age: ::core::option::Option::None,
                email: ::core::option::Option::None,
            }
        }
        #[inline]
        pub const fn name_ref(
            &self,
        ) -> ::core::option::Option<
            &<::std::string::String as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        > {
            self.name.as_ref()
        }
        #[inline]
        pub const fn name_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <::std::string::String as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        > {
            self.name.as_mut()
        }
        #[inline]
        pub const fn take_name(
            &mut self,
        ) -> ::core::option::Option<
            <::std::string::String as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
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
            value: <::std::string::String as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        ) -> &mut Self {
            self.name = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_name(
            mut self,
            value: <::std::string::String as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
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
            &<u32 as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <u32 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        > {
            self.age.as_ref()
        }
        #[inline]
        pub const fn age_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <u32 as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <u32 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        > {
            self.age.as_mut()
        }
        #[inline]
        pub const fn take_age(
            &mut self,
        ) -> ::core::option::Option<
            <u32 as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <u32 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
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
            value: <u32 as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <u32 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        ) -> &mut Self {
            self.age = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_age(
            mut self,
            value: <u32 as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <u32 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
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
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        > {
            self.email.as_ref()
        }
        #[inline]
        pub const fn email_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        > {
            self.email.as_mut()
        }
        #[inline]
        pub const fn take_email(
            &mut self,
        ) -> ::core::option::Option<
            <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
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
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        ) -> &mut Self {
            self.email = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_email(
            mut self,
            value: <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
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
        pub const fn wire_format<W>(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::WireFormatReflection<W>,
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
    > UserFieldReflection<::grost::__private::reflection::FieldReflection<F>, F, TAG>
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
            ::grost::__private::reflection::FieldReflection<F>,
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
            ::grost::__private::reflection::FieldReflection<F>,
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
            ::grost::__private::reflection::FieldReflection<F>,
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
    const _: () = {
        /// The selection type for User
        #[allow(non_camel_case_types)]
        pub struct UserSelector {
            name: <::std::string::String as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
            age: <u32 as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <u32 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
            email: <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl UserSelector {
            fn debug_helper(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                let num_selected = self.selected();
                let mut idx = 0;
                ::core::write!(f, ::core::concat!("("))?;
                if self.name {
                    if idx != num_selected - 1 {
                        ::core::write!(f, ::core::concat!("name", " & "))?;
                    } else {
                        ::core::write!(f, "name")?;
                    }
                    idx += 1;
                }
                if self.age {
                    if idx != num_selected - 1 {
                        ::core::write!(f, ::core::concat!("age", " & "))?;
                    } else {
                        ::core::write!(f, "age")?;
                    }
                    idx += 1;
                }
                if self.email {
                    if idx != num_selected - 1 {
                        ::core::write!(f, ::core::concat!("email", " & "))?;
                    } else {
                        ::core::write!(f, "email")?;
                    }
                    idx += 1;
                }
                ::core::write!(f, ")")
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for UserSelector {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                ::core::write!(f, "UserSelector")?;
                self.debug_helper(f)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::PartialEq for UserSelector {
            fn eq(&self, other: &Self) -> ::core::primitive::bool {
                self.name == other.name && self.age == other.age
                    && self.email == other.email
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::Eq for UserSelector {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::hash::Hash for UserSelector {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.name.hash(state);
                self.age.hash(state);
                self.email.hash(state);
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for UserSelector {
            fn clone(&self) -> Self {
                *self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for UserSelector {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::grost::__private::Selector<::grost::__private::flavors::Network>
        for UserSelector {
            const ALL: Self = Self::all();
            const NONE: Self = Self::empty();
            fn selected(&self) -> ::core::primitive::usize {
                Self::selected(self)
            }
            fn unselected(&self) -> ::core::primitive::usize {
                Self::unselected(self)
            }
            fn flip(&mut self) -> &mut Self {
                <<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::flip(&mut self.name);
                <<u32 as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <u32 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::flip(&mut self.age);
                <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::flip(&mut self.email);
                self
            }
            fn merge(&mut self, other: Self) -> &mut Self {
                <<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::merge(&mut self.name, other.name);
                <<u32 as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <u32 as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::merge(&mut self.age, other.age);
                <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::merge(&mut self.email, other.email);
                self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl UserSelector {
            /// The number of options in this selection type.
            pub const OPTIONS: ::core::primitive::usize = 3usize;
            /// Returns a selector which selects nothing.
            #[inline]
            pub const fn empty() -> Self {
                Self {
                    name: <<::std::string::String as ::grost::__private::Selectable<
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::Selector as ::grost::__private::Selector<
                        ::grost::__private::flavors::Network,
                    >>::NONE,
                    age: <<u32 as ::grost::__private::Selectable<
                        ::grost::__private::flavors::Network,
                        <u32 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::Selector as ::grost::__private::Selector<
                        ::grost::__private::flavors::Network,
                    >>::NONE,
                    email: <<::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::Selectable<
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::Selector as ::grost::__private::Selector<
                        ::grost::__private::flavors::Network,
                    >>::NONE,
                }
            }
            /// Returns a selector which selects all.
            #[inline]
            pub const fn all() -> Self {
                Self {
                    name: <<::std::string::String as ::grost::__private::Selectable<
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::Selector as ::grost::__private::Selector<
                        ::grost::__private::flavors::Network,
                    >>::ALL,
                    age: <<u32 as ::grost::__private::Selectable<
                        ::grost::__private::flavors::Network,
                        <u32 as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::Selector as ::grost::__private::Selector<
                        ::grost::__private::flavors::Network,
                    >>::ALL,
                    email: <<::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::Selectable<
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::Selector as ::grost::__private::Selector<
                        ::grost::__private::flavors::Network,
                    >>::ALL,
                }
            }
            /// Returns `true` if the selector selects nothing.
            #[inline]
            pub const fn is_empty(&self) -> ::core::primitive::bool {
                !self.name && !self.age && !self.email
            }
            /// Returns `true` if the selector selects all.
            #[inline]
            pub const fn is_all(&self) -> ::core::primitive::bool {
                self.name && self.age && self.email
            }
            /// Returns the number of selected fields.
            #[inline]
            pub const fn selected(&self) -> ::core::primitive::usize {
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
            pub const fn unselected(&self) -> ::core::primitive::usize {
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
            pub const fn iter_selected(&self) -> UserSelectorIter<true> {
                UserSelectorIter::new(self, self.selected())
            }
            /// Returns an iterator over the unselected fields.
            #[inline]
            pub const fn iter_unselected(&self) -> UserSelectorIter<false> {
                UserSelectorIter::new(self, self.unselected())
            }
            /// Returns `true` if such field is selected.
            #[inline]
            pub const fn is_selected(
                &self,
                idx: UserFieldIndex,
            ) -> ::core::primitive::bool {
                match idx {
                    UserFieldIndex::Name => self.is_name_selected(),
                    UserFieldIndex::Age => self.is_age_selected(),
                    UserFieldIndex::Email => self.is_email_selected(),
                }
            }
            /// Returns `true` if such field is unselected.
            #[inline]
            pub const fn is_unselected(
                &self,
                idx: UserFieldIndex,
            ) -> ::core::primitive::bool {
                match idx {
                    UserFieldIndex::Name => self.is_name_unselected(),
                    UserFieldIndex::Age => self.is_age_unselected(),
                    UserFieldIndex::Email => self.is_email_unselected(),
                }
            }
            /// Select the `User.name` field
            #[inline]
            pub const fn select_name(&mut self) -> &mut Self {
                self.name = true;
                self
            }
            /// Unselect the `User.name` field
            #[inline]
            pub const fn unselect_name(&mut self) -> &mut Self {
                self.name = false;
                self
            }
            /// Update the `User.name` field
            #[inline]
            pub const fn update_name(
                &mut self,
                value: ::core::primitive::bool,
            ) -> &mut Self {
                self.name = value;
                self
            }
            /// Toggle the `User.name` field
            #[inline]
            pub const fn toggle_name(&mut self) -> &mut Self {
                self.name = !self.name;
                self
            }
            /// Set the `User.name` field
            #[inline]
            pub const fn with_name(mut self) -> Self {
                self.name = true;
                self
            }
            /// Unset the `User.name` field
            #[inline]
            pub const fn without_name(mut self) -> Self {
                self.name = false;
                self
            }
            /// Set or unset the `User.name` field
            #[inline]
            pub const fn maybe_name(mut self, val: ::core::primitive::bool) -> Self {
                self.name = val;
                self
            }
            /// Returns `true` if the `User.name` field is selected
            #[inline]
            pub const fn is_name_selected(&self) -> ::core::primitive::bool {
                self.name
            }
            /// Returns `true` if the `User.name` field is unselected
            #[inline]
            pub const fn is_name_unselected(&self) -> ::core::primitive::bool {
                !self.name
            }
            /// Select the `User.age` field
            #[inline]
            pub const fn select_age(&mut self) -> &mut Self {
                self.age = true;
                self
            }
            /// Unselect the `User.age` field
            #[inline]
            pub const fn unselect_age(&mut self) -> &mut Self {
                self.age = false;
                self
            }
            /// Update the `User.age` field
            #[inline]
            pub const fn update_age(
                &mut self,
                value: ::core::primitive::bool,
            ) -> &mut Self {
                self.age = value;
                self
            }
            /// Toggle the `User.age` field
            #[inline]
            pub const fn toggle_age(&mut self) -> &mut Self {
                self.age = !self.age;
                self
            }
            /// Set the `User.age` field
            #[inline]
            pub const fn with_age(mut self) -> Self {
                self.age = true;
                self
            }
            /// Unset the `User.age` field
            #[inline]
            pub const fn without_age(mut self) -> Self {
                self.age = false;
                self
            }
            /// Set or unset the `User.age` field
            #[inline]
            pub const fn maybe_age(mut self, val: ::core::primitive::bool) -> Self {
                self.age = val;
                self
            }
            /// Returns `true` if the `User.age` field is selected
            #[inline]
            pub const fn is_age_selected(&self) -> ::core::primitive::bool {
                self.age
            }
            /// Returns `true` if the `User.age` field is unselected
            #[inline]
            pub const fn is_age_unselected(&self) -> ::core::primitive::bool {
                !self.age
            }
            /// Select the `User.email` field
            #[inline]
            pub const fn select_email(&mut self) -> &mut Self {
                self.email = true;
                self
            }
            /// Unselect the `User.email` field
            #[inline]
            pub const fn unselect_email(&mut self) -> &mut Self {
                self.email = false;
                self
            }
            /// Update the `User.email` field
            #[inline]
            pub const fn update_email(
                &mut self,
                value: ::core::primitive::bool,
            ) -> &mut Self {
                self.email = value;
                self
            }
            /// Toggle the `User.email` field
            #[inline]
            pub const fn toggle_email(&mut self) -> &mut Self {
                self.email = !self.email;
                self
            }
            /// Set the `User.email` field
            #[inline]
            pub const fn with_email(mut self) -> Self {
                self.email = true;
                self
            }
            /// Unset the `User.email` field
            #[inline]
            pub const fn without_email(mut self) -> Self {
                self.email = false;
                self
            }
            /// Set or unset the `User.email` field
            #[inline]
            pub const fn maybe_email(mut self, val: ::core::primitive::bool) -> Self {
                self.email = val;
                self
            }
            /// Returns `true` if the `User.email` field is selected
            #[inline]
            pub const fn is_email_selected(&self) -> ::core::primitive::bool {
                self.email
            }
            /// Returns `true` if the `User.email` field is unselected
            #[inline]
            pub const fn is_email_unselected(&self) -> ::core::primitive::bool {
                !self.email
            }
        }
        /// An iterator over the selected fields of the [`UserSelector`]
        #[allow(non_camel_case_types)]
        pub struct UserSelectorIter<
            '__grost_selector_iter__,
            const N: ::core::primitive::bool = true,
        > {
            selector: &'__grost_selector_iter__ UserSelector,
            index: ::core::option::Option<UserFieldIndex>,
            num: ::core::primitive::usize,
            yielded: ::core::primitive::usize,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<
            '__grost_selector_iter__,
            const N: ::core::primitive::bool,
        > UserSelectorIter<'__grost_selector_iter__, N> {
            #[inline]
            const fn new(
                selector: &'__grost_selector_iter__ UserSelector,
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
        ::grost::__private::encoded_state!(
            & '__grost_encoded_state__::grost::__private::flavors::Network : User as
            ::grost::__private::flavors::network::LengthDelimited => PartialUserRef <
            '__grost_encoded_state__ >
        );
        ::grost::__private::encoded_state!(
            & '__grost_encoded_state__::grost::__private::flavors::Network : PartialUser
            as ::grost::__private::flavors::network::LengthDelimited => PartialUserRef <
            '__grost_encoded_state__ >
        );
        ::grost::__private::encoded_state!(
            & '__grost_encoded_state__::grost::__private::flavors::Network :
            PartialUserRef < '__grost_encoded_state__ > as
            ::grost::__private::flavors::network::LengthDelimited => PartialUserRef <
            '__grost_encoded_state__ >
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
            type Selector = UserSelector;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::grost::__private::Selectable<
            ::grost::__private::flavors::Network,
            ::grost::__private::flavors::network::Repeated<
                ::grost::__private::flavors::network::LengthDelimited,
            >,
        > for User {
            type Selector = UserSelector;
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
            type Selector = UserSelector;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::grost::__private::Selectable<
            ::grost::__private::flavors::Network,
            ::grost::__private::flavors::network::LengthDelimited,
        > for PartialUser {
            type Selector = UserSelector;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::grost::__private::Selectable<
            ::grost::__private::flavors::Network,
            ::grost::__private::flavors::network::Repeated<
                ::grost::__private::flavors::network::LengthDelimited,
            >,
        > for PartialUser {
            type Selector = UserSelector;
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
            type Selector = UserSelector;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<
            '__grost_lifetime__,
        > ::grost::__private::Selectable<
            ::grost::__private::flavors::Network,
            ::grost::__private::flavors::network::LengthDelimited,
        > for PartialUserRef<'__grost_lifetime__> {
            type Selector = UserSelector;
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
        > for PartialUserRef<'__grost_lifetime__> {
            type Selector = UserSelector;
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
        > for PartialUserRef<'__grost_lifetime__> {
            type Selector = UserSelector;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<
            ::grost::__private::flavors::Network,
        >
        for UserFieldReflection<
            ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >,
            ::grost::__private::flavors::Network,
            1u32,
        > {
            type Reflection = ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >;
            const REFLECTION: &Self::Reflection = &::grost::__private::reflection::FieldReflectionBuilder::<
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
            ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >,
            ::grost::__private::flavors::Network,
            2u32,
        > {
            type Reflection = ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >;
            const REFLECTION: &Self::Reflection = &::grost::__private::reflection::FieldReflectionBuilder::<
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
            ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >,
            ::grost::__private::flavors::Network,
            3u32,
        > {
            type Reflection = ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >;
            const REFLECTION: &Self::Reflection = &::grost::__private::reflection::FieldReflectionBuilder::<
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
                ty: ::core::any::type_name::<
                    ::core::option::Option<::std::string::String>,
                >,
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
                    ::grost::__private::reflection::FieldReflection<
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
                    ::grost::__private::reflection::FieldReflection<
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
                    ::grost::__private::reflection::FieldReflection<
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
            ::grost::__private::reflection::WireFormatReflection<
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >,
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
            ::grost::__private::reflection::WireFormatReflection<
                <u32 as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >,
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
            ::grost::__private::reflection::WireFormatReflection<
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >,
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
                        ::grost::__private::reflection::FieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                        ::grost::__private::flavors::Network,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        ::grost::__private::flavors::Network,
                    >>::REFLECTION,
                    <UserFieldReflection<
                        ::grost::__private::reflection::FieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                        ::grost::__private::flavors::Network,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        ::grost::__private::flavors::Network,
                    >>::REFLECTION,
                    <UserFieldReflection<
                        ::grost::__private::reflection::FieldReflection<
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
    };
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
    #[allow(clippy::type_complexity)]
    pub struct PartialCommentRef<'__grost_flavor__> {
        user: ::core::option::Option<
            <User as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        >,
        replyer: ::core::option::Option<
            <User as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        >,
        title: ::core::option::Option<
            <::std::string::String as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        >,
        content: ::core::option::Option<
            <::std::string::String as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        >,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::grost::__private::Referenceable<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for Comment {
        type Ref<'__grost_flavor__> = PartialCommentRef<'__grost_flavor__>
        where
            Self: '__grost_flavor__;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<'__grost_flavor__> ::core::default::Default
    for PartialCommentRef<'__grost_flavor__> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_flavor__> PartialCommentRef<'__grost_flavor__> {
        /// Creates an empty instance.
        #[inline]
        pub const fn new() -> Self {
            Self {
                user: ::core::option::Option::None,
                replyer: ::core::option::Option::None,
                title: ::core::option::Option::None,
                content: ::core::option::Option::None,
            }
        }
        #[inline]
        pub const fn user_ref(
            &self,
        ) -> ::core::option::Option<
            &<User as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        > {
            self.user.as_ref()
        }
        #[inline]
        pub const fn user_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <User as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        > {
            self.user.as_mut()
        }
        #[inline]
        pub const fn take_user(
            &mut self,
        ) -> ::core::option::Option<
            <User as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
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
            value: <User as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        ) -> &mut Self {
            self.user = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_user(
            mut self,
            value: <User as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
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
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        > {
            self.replyer.as_ref()
        }
        #[inline]
        pub const fn replyer_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <::core::option::Option<
                User,
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        > {
            self.replyer.as_mut()
        }
        #[inline]
        pub const fn take_replyer(
            &mut self,
        ) -> ::core::option::Option<
            <::core::option::Option<
                User,
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
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
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        ) -> &mut Self {
            self.replyer = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_replyer(
            mut self,
            value: <::core::option::Option<
                User,
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
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
            &<::std::string::String as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        > {
            self.title.as_ref()
        }
        #[inline]
        pub const fn title_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <::std::string::String as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        > {
            self.title.as_mut()
        }
        #[inline]
        pub const fn take_title(
            &mut self,
        ) -> ::core::option::Option<
            <::std::string::String as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
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
            value: <::std::string::String as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        ) -> &mut Self {
            self.title = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_title(
            mut self,
            value: <::std::string::String as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
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
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        > {
            self.content.as_ref()
        }
        #[inline]
        pub const fn content_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        > {
            self.content.as_mut()
        }
        #[inline]
        pub const fn take_content(
            &mut self,
        ) -> ::core::option::Option<
            <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
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
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
        ) -> &mut Self {
            self.content = ::core::option::Option::Some(value);
            self
        }
        #[inline]
        pub fn with_content(
            mut self,
            value: <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Referenceable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Ref<'__grost_flavor__>,
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
        pub const fn wire_format<W>(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::WireFormatReflection<W>,
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
    > CommentFieldReflection<::grost::__private::reflection::FieldReflection<F>, F, TAG>
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
            ::grost::__private::reflection::FieldReflection<F>,
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
            ::grost::__private::reflection::FieldReflection<F>,
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
            ::grost::__private::reflection::FieldReflection<F>,
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
            ::grost::__private::reflection::FieldReflection<F>,
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
    const _: () = {
        /// The selection type for Comment
        #[allow(non_camel_case_types)]
        pub struct CommentSelector {
            user: <User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
            replyer: <::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
            title: <::std::string::String as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
            content: <::core::option::Option<
                ::std::string::String,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl CommentSelector {
            fn debug_helper(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                let num_selected = self.selected();
                let mut idx = 0;
                ::core::write!(f, ::core::concat!("("))?;
                if !self.user.is_empty() {
                    if idx != num_selected - 1 {
                        ::core::write!(f, "user")?;
                        self.user.debug_helper(f)?;
                        ::core::write!(f, " & ")?;
                    } else {
                        ::core::write!(f, "user")?;
                        self.user.debug_helper(f)?;
                    }
                    idx += 1;
                }
                if !self.replyer.is_empty() {
                    if idx != num_selected - 1 {
                        ::core::write!(f, "replyer")?;
                        self.replyer.debug_helper(f)?;
                        ::core::write!(f, " & ")?;
                    } else {
                        ::core::write!(f, "replyer")?;
                        self.replyer.debug_helper(f)?;
                    }
                    idx += 1;
                }
                if self.title {
                    if idx != num_selected - 1 {
                        ::core::write!(f, ::core::concat!("title", " & "))?;
                    } else {
                        ::core::write!(f, "title")?;
                    }
                    idx += 1;
                }
                if self.content {
                    if idx != num_selected - 1 {
                        ::core::write!(f, ::core::concat!("content", " & "))?;
                    } else {
                        ::core::write!(f, "content")?;
                    }
                    idx += 1;
                }
                ::core::write!(f, ")")
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for CommentSelector {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                ::core::write!(f, "CommentSelector")?;
                self.debug_helper(f)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::PartialEq for CommentSelector {
            fn eq(&self, other: &Self) -> ::core::primitive::bool {
                self.user == other.user && self.replyer == other.replyer
                    && self.title == other.title && self.content == other.content
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::Eq for CommentSelector {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::hash::Hash for CommentSelector {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.user.hash(state);
                self.replyer.hash(state);
                self.title.hash(state);
                self.content.hash(state);
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for CommentSelector {
            fn clone(&self) -> Self {
                *self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for CommentSelector {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::grost::__private::Selector<::grost::__private::flavors::Network>
        for CommentSelector {
            const ALL: Self = Self::all();
            const NONE: Self = Self::empty();
            fn selected(&self) -> ::core::primitive::usize {
                Self::selected(self)
            }
            fn unselected(&self) -> ::core::primitive::usize {
                Self::unselected(self)
            }
            fn flip(&mut self) -> &mut Self {
                <<User as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::flip(&mut self.user);
                <<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        User,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::flip(&mut self.replyer);
                <<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::flip(&mut self.title);
                <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::flip(&mut self.content);
                self
            }
            fn merge(&mut self, other: Self) -> &mut Self {
                <<User as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::merge(&mut self.user, other.user);
                <<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        User,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::merge(&mut self.replyer, other.replyer);
                <<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::merge(&mut self.title, other.title);
                <<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::merge(&mut self.content, other.content);
                self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl CommentSelector {
            /// The number of options in this selection type.
            pub const OPTIONS: ::core::primitive::usize = 4usize;
            /// Returns a selector which selects nothing.
            #[inline]
            pub const fn empty() -> Self {
                Self {
                    user: <<User as ::grost::__private::Selectable<
                        ::grost::__private::flavors::Network,
                        <User as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::Selector as ::grost::__private::Selector<
                        ::grost::__private::flavors::Network,
                    >>::NONE,
                    replyer: <<::core::option::Option<
                        User,
                    > as ::grost::__private::Selectable<
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            User,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::Selector as ::grost::__private::Selector<
                        ::grost::__private::flavors::Network,
                    >>::NONE,
                    title: <<::std::string::String as ::grost::__private::Selectable<
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::Selector as ::grost::__private::Selector<
                        ::grost::__private::flavors::Network,
                    >>::NONE,
                    content: <<::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::Selectable<
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::Selector as ::grost::__private::Selector<
                        ::grost::__private::flavors::Network,
                    >>::NONE,
                }
            }
            /// Returns a selector which selects all.
            #[inline]
            pub const fn all() -> Self {
                Self {
                    user: <<User as ::grost::__private::Selectable<
                        ::grost::__private::flavors::Network,
                        <User as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::Selector as ::grost::__private::Selector<
                        ::grost::__private::flavors::Network,
                    >>::ALL,
                    replyer: <<::core::option::Option<
                        User,
                    > as ::grost::__private::Selectable<
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            User,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::Selector as ::grost::__private::Selector<
                        ::grost::__private::flavors::Network,
                    >>::ALL,
                    title: <<::std::string::String as ::grost::__private::Selectable<
                        ::grost::__private::flavors::Network,
                        <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::Selector as ::grost::__private::Selector<
                        ::grost::__private::flavors::Network,
                    >>::ALL,
                    content: <<::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::Selectable<
                        ::grost::__private::flavors::Network,
                        <::core::option::Option<
                            ::std::string::String,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::Selector as ::grost::__private::Selector<
                        ::grost::__private::flavors::Network,
                    >>::ALL,
                }
            }
            /// Returns `true` if the selector selects nothing.
            #[inline]
            pub const fn is_empty(&self) -> ::core::primitive::bool {
                self.user.is_empty() && self.replyer.is_empty() && !self.title
                    && !self.content
            }
            /// Returns `true` if the selector selects all.
            #[inline]
            pub const fn is_all(&self) -> ::core::primitive::bool {
                self.user.is_all() && self.replyer.is_all() && self.title && self.content
            }
            /// Returns the number of selected fields.
            #[inline]
            pub const fn selected(&self) -> ::core::primitive::usize {
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
            pub const fn unselected(&self) -> ::core::primitive::usize {
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
            pub const fn iter_selected(&self) -> CommentSelectorIter<true> {
                CommentSelectorIter::new(self, self.selected())
            }
            /// Returns an iterator over the unselected fields.
            #[inline]
            pub const fn iter_unselected(&self) -> CommentSelectorIter<false> {
                CommentSelectorIter::new(self, self.unselected())
            }
            /// Returns `true` if such field is selected.
            #[inline]
            pub const fn is_selected(
                &self,
                idx: CommentFieldIndex,
            ) -> ::core::primitive::bool {
                match idx {
                    CommentFieldIndex::User => self.is_user_selected(),
                    CommentFieldIndex::Replyer => self.is_replyer_selected(),
                    CommentFieldIndex::Title => self.is_title_selected(),
                    CommentFieldIndex::Content => self.is_content_selected(),
                }
            }
            /// Returns `true` if such field is unselected.
            #[inline]
            pub const fn is_unselected(
                &self,
                idx: CommentFieldIndex,
            ) -> ::core::primitive::bool {
                match idx {
                    CommentFieldIndex::User => self.is_user_unselected(),
                    CommentFieldIndex::Replyer => self.is_replyer_unselected(),
                    CommentFieldIndex::Title => self.is_title_unselected(),
                    CommentFieldIndex::Content => self.is_content_unselected(),
                }
            }
            /// Select the `Comment.user` field
            #[inline]
            pub fn select_user(
                &mut self,
                val: <User as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> &mut Self {
                self.user = val;
                self
            }
            /// Unselect the `Comment.user` field
            #[inline]
            pub fn unselect_user(&mut self) -> &mut Self {
                self.user = <<User as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <User as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::NONE;
                self
            }
            /// Get a reference to the selector of `Comment.user` field
            #[inline]
            pub const fn user_ref(
                &self,
            ) -> &<User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector {
                &self.user
            }
            /// Get a mutable reference to the selector of `Comment.user` field
            #[inline]
            pub const fn user_mut(
                &mut self,
            ) -> &mut <User as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector {
                &mut self.user
            }
            /// Returns `true` if the `Comment.user` field is selected
            #[inline]
            pub const fn is_user_selected(&self) -> ::core::primitive::bool {
                !self.user.is_empty()
            }
            /// Returns `true` if the `Comment.user` field is unselected
            #[inline]
            pub const fn is_user_unselected(&self) -> ::core::primitive::bool {
                self.user.is_empty()
            }
            /// Select the `Comment.replyer` field
            #[inline]
            pub fn select_replyer(
                &mut self,
                val: <::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        User,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector,
            ) -> &mut Self {
                self.replyer = val;
                self
            }
            /// Unselect the `Comment.replyer` field
            #[inline]
            pub fn unselect_replyer(&mut self) -> &mut Self {
                self.replyer = <<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::Network,
                    <::core::option::Option<
                        User,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Network,
                    >>::Format,
                >>::Selector as ::grost::__private::Selector<
                    ::grost::__private::flavors::Network,
                >>::NONE;
                self
            }
            /// Get a reference to the selector of `Comment.replyer` field
            #[inline]
            pub const fn replyer_ref(
                &self,
            ) -> &<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
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
                ::grost::__private::flavors::Network,
                <::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::Selector {
                &mut self.replyer
            }
            /// Returns `true` if the `Comment.replyer` field is selected
            #[inline]
            pub const fn is_replyer_selected(&self) -> ::core::primitive::bool {
                !self.replyer.is_empty()
            }
            /// Returns `true` if the `Comment.replyer` field is unselected
            #[inline]
            pub const fn is_replyer_unselected(&self) -> ::core::primitive::bool {
                self.replyer.is_empty()
            }
            /// Select the `Comment.title` field
            #[inline]
            pub const fn select_title(&mut self) -> &mut Self {
                self.title = true;
                self
            }
            /// Unselect the `Comment.title` field
            #[inline]
            pub const fn unselect_title(&mut self) -> &mut Self {
                self.title = false;
                self
            }
            /// Update the `Comment.title` field
            #[inline]
            pub const fn update_title(
                &mut self,
                value: ::core::primitive::bool,
            ) -> &mut Self {
                self.title = value;
                self
            }
            /// Toggle the `Comment.title` field
            #[inline]
            pub const fn toggle_title(&mut self) -> &mut Self {
                self.title = !self.title;
                self
            }
            /// Set the `Comment.title` field
            #[inline]
            pub const fn with_title(mut self) -> Self {
                self.title = true;
                self
            }
            /// Unset the `Comment.title` field
            #[inline]
            pub const fn without_title(mut self) -> Self {
                self.title = false;
                self
            }
            /// Set or unset the `Comment.title` field
            #[inline]
            pub const fn maybe_title(mut self, val: ::core::primitive::bool) -> Self {
                self.title = val;
                self
            }
            /// Returns `true` if the `Comment.title` field is selected
            #[inline]
            pub const fn is_title_selected(&self) -> ::core::primitive::bool {
                self.title
            }
            /// Returns `true` if the `Comment.title` field is unselected
            #[inline]
            pub const fn is_title_unselected(&self) -> ::core::primitive::bool {
                !self.title
            }
            /// Select the `Comment.content` field
            #[inline]
            pub const fn select_content(&mut self) -> &mut Self {
                self.content = true;
                self
            }
            /// Unselect the `Comment.content` field
            #[inline]
            pub const fn unselect_content(&mut self) -> &mut Self {
                self.content = false;
                self
            }
            /// Update the `Comment.content` field
            #[inline]
            pub const fn update_content(
                &mut self,
                value: ::core::primitive::bool,
            ) -> &mut Self {
                self.content = value;
                self
            }
            /// Toggle the `Comment.content` field
            #[inline]
            pub const fn toggle_content(&mut self) -> &mut Self {
                self.content = !self.content;
                self
            }
            /// Set the `Comment.content` field
            #[inline]
            pub const fn with_content(mut self) -> Self {
                self.content = true;
                self
            }
            /// Unset the `Comment.content` field
            #[inline]
            pub const fn without_content(mut self) -> Self {
                self.content = false;
                self
            }
            /// Set or unset the `Comment.content` field
            #[inline]
            pub const fn maybe_content(mut self, val: ::core::primitive::bool) -> Self {
                self.content = val;
                self
            }
            /// Returns `true` if the `Comment.content` field is selected
            #[inline]
            pub const fn is_content_selected(&self) -> ::core::primitive::bool {
                self.content
            }
            /// Returns `true` if the `Comment.content` field is unselected
            #[inline]
            pub const fn is_content_unselected(&self) -> ::core::primitive::bool {
                !self.content
            }
        }
        /// An iterator over the selected fields of the [`CommentSelector`]
        #[allow(non_camel_case_types)]
        pub struct CommentSelectorIter<
            '__grost_selector_iter__,
            const N: ::core::primitive::bool = true,
        > {
            selector: &'__grost_selector_iter__ CommentSelector,
            index: ::core::option::Option<CommentFieldIndex>,
            num: ::core::primitive::usize,
            yielded: ::core::primitive::usize,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<
            '__grost_selector_iter__,
            const N: ::core::primitive::bool,
        > CommentSelectorIter<'__grost_selector_iter__, N> {
            #[inline]
            const fn new(
                selector: &'__grost_selector_iter__ CommentSelector,
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
        ::grost::__private::encoded_state!(
            & '__grost_encoded_state__::grost::__private::flavors::Network : Comment as
            ::grost::__private::flavors::network::LengthDelimited => PartialCommentRef <
            '__grost_encoded_state__ >
        );
        ::grost::__private::encoded_state!(
            & '__grost_encoded_state__::grost::__private::flavors::Network :
            PartialComment as ::grost::__private::flavors::network::LengthDelimited =>
            PartialCommentRef < '__grost_encoded_state__ >
        );
        ::grost::__private::encoded_state!(
            & '__grost_encoded_state__::grost::__private::flavors::Network :
            PartialCommentRef < '__grost_encoded_state__ > as
            ::grost::__private::flavors::network::LengthDelimited => PartialCommentRef <
            '__grost_encoded_state__ >
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
            type Selector = CommentSelector;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::grost::__private::Selectable<
            ::grost::__private::flavors::Network,
            ::grost::__private::flavors::network::Repeated<
                ::grost::__private::flavors::network::LengthDelimited,
            >,
        > for Comment {
            type Selector = CommentSelector;
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
            type Selector = CommentSelector;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::grost::__private::Selectable<
            ::grost::__private::flavors::Network,
            ::grost::__private::flavors::network::LengthDelimited,
        > for PartialComment {
            type Selector = CommentSelector;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::grost::__private::Selectable<
            ::grost::__private::flavors::Network,
            ::grost::__private::flavors::network::Repeated<
                ::grost::__private::flavors::network::LengthDelimited,
            >,
        > for PartialComment {
            type Selector = CommentSelector;
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
            type Selector = CommentSelector;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<
            '__grost_lifetime__,
        > ::grost::__private::Selectable<
            ::grost::__private::flavors::Network,
            ::grost::__private::flavors::network::LengthDelimited,
        > for PartialCommentRef<'__grost_lifetime__> {
            type Selector = CommentSelector;
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
        > for PartialCommentRef<'__grost_lifetime__> {
            type Selector = CommentSelector;
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
        > for PartialCommentRef<'__grost_lifetime__> {
            type Selector = CommentSelector;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<
            ::grost::__private::flavors::Network,
        >
        for CommentFieldReflection<
            ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >,
            ::grost::__private::flavors::Network,
            1u32,
        > {
            type Reflection = ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >;
            const REFLECTION: &Self::Reflection = &::grost::__private::reflection::FieldReflectionBuilder::<
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
            ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >,
            ::grost::__private::flavors::Network,
            2u32,
        > {
            type Reflection = ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >;
            const REFLECTION: &Self::Reflection = &::grost::__private::reflection::FieldReflectionBuilder::<
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
            ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >,
            ::grost::__private::flavors::Network,
            3u32,
        > {
            type Reflection = ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >;
            const REFLECTION: &Self::Reflection = &::grost::__private::reflection::FieldReflectionBuilder::<
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
            ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >,
            ::grost::__private::flavors::Network,
            4u32,
        > {
            type Reflection = ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >;
            const REFLECTION: &Self::Reflection = &::grost::__private::reflection::FieldReflectionBuilder::<
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
                ty: ::core::any::type_name::<
                    ::core::option::Option<::std::string::String>,
                >,
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
                    ::grost::__private::reflection::FieldReflection<
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
                    ::grost::__private::reflection::FieldReflection<
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
                    ::grost::__private::reflection::FieldReflection<
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
                    ::grost::__private::reflection::FieldReflection<
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
            ::grost::__private::reflection::WireFormatReflection<
                <User as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >,
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
            ::grost::__private::reflection::WireFormatReflection<
                <::core::option::Option<
                    User,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >,
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
            ::grost::__private::reflection::WireFormatReflection<
                <::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >,
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
            ::grost::__private::reflection::WireFormatReflection<
                <::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >,
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
                        ::grost::__private::reflection::FieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                        ::grost::__private::flavors::Network,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        ::grost::__private::flavors::Network,
                    >>::REFLECTION,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::FieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                        ::grost::__private::flavors::Network,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        ::grost::__private::flavors::Network,
                    >>::REFLECTION,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::FieldReflection<
                            ::grost::__private::flavors::Network,
                        >,
                        ::grost::__private::flavors::Network,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        ::grost::__private::flavors::Network,
                    >>::REFLECTION,
                    <CommentFieldReflection<
                        ::grost::__private::reflection::FieldReflection<
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
    };
};
