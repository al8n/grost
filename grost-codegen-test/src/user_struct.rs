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
        impl ::grost::__private::Selectable<
            ::grost::__private::flavors::Network,
            ::grost::__private::flavors::network::LengthDelimited,
        > for User {
            type Selector = UserSelector;
        }
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
                self.name = !self.name;
                self.age = !self.age;
                self.email = !self.email;
                self
            }
            fn merge(&mut self, other: Self) -> &mut Self {
                self.name |= other.name;
                self.age |= other.age;
                self.email |= other.email;
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
        pub struct UserSelectorIter<'a, const N: ::core::primitive::bool = true> {
            selector: &'a UserSelector,
            index: ::core::option::Option<UserFieldIndex>,
            num: ::core::primitive::usize,
            yielded: ::core::primitive::usize,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<'a, const N: ::core::primitive::bool> UserSelectorIter<'a, N> {
            #[inline]
            const fn new(
                selector: &'a UserSelector,
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
        #[automatically_derived]
        impl ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        > for User {
            type Format = ::grost::__private::flavors::network::LengthDelimited;
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
        impl ::grost::__private::Selectable<
            ::grost::__private::flavors::Network,
            ::grost::__private::flavors::network::LengthDelimited,
        > for Comment {
            type Selector = CommentSelector;
        }
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
                self.title = !self.title;
                self.content = !self.content;
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
                self.title |= other.title;
                self.content |= other.content;
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
        pub struct CommentSelectorIter<'a, const N: ::core::primitive::bool = true> {
            selector: &'a CommentSelector,
            index: ::core::option::Option<CommentFieldIndex>,
            num: ::core::primitive::usize,
            yielded: ::core::primitive::usize,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<'a, const N: ::core::primitive::bool> CommentSelectorIter<'a, N> {
            #[inline]
            const fn new(
                selector: &'a CommentSelector,
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
        #[automatically_derived]
        impl ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        > for Comment {
            type Format = ::grost::__private::flavors::network::LengthDelimited;
        }
    };
};
