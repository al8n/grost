#![no_implicit_prelude]

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
///A user struct
pub struct User {
    name: ::std::string::String,
    age: u32,
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
/// The selection type for User
pub struct UserSelector<F: ?::core::marker::Sized> {
    _m: ::core::marker::PhantomData<F>,
    name: ::core::primitive::bool,
    age: ::core::primitive::bool,
    email: ::core::primitive::bool,
}
/// An iterator over the selected fields of the [`UserSelector`]
pub struct UserSelectorIter<
    'a,
    F: ?::core::marker::Sized,
    const N: ::core::primitive::bool = true,
> {
    selector: &'a UserSelector<F>,
    index: ::core::option::Option<UserFieldIndex>,
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
    impl<
        'a,
        F: ?::core::marker::Sized,
        const N: ::core::primitive::bool,
    > UserSelectorIter<'a, F, N> {
        #[inline]
        const fn new(
            selector: &'a UserSelector<F>,
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
    impl<F: ?::core::marker::Sized> UserSelector<F> {
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
    impl<F: ?::core::marker::Sized> ::core::fmt::Debug for UserSelector<F> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::write!(f, "UserSelector")?;
            self.debug_helper(f)
        }
    }
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::core::cmp::PartialEq for UserSelector<F> {
        fn eq(&self, other: &Self) -> ::core::primitive::bool {
            self.name == other.name && self.age == other.age && self.email == other.email
        }
    }
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::core::cmp::Eq for UserSelector<F> {}
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::core::hash::Hash for UserSelector<F> {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
            self.name.hash(state);
            self.age.hash(state);
            self.email.hash(state);
        }
    }
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::core::clone::Clone for UserSelector<F> {
        fn clone(&self) -> Self {
            *self
        }
    }
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::core::marker::Copy for UserSelector<F> {}
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::grost::__private::Selectable<F> for User {
        type Selector = UserSelector<F>;
    }
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::grost::__private::Selector<F> for UserSelector<F> {
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
    impl<F: ?::core::marker::Sized> UserSelector<F> {
        /// The number of options in this selection type.
        pub const OPTIONS: ::core::primitive::usize = 3usize;
        /// Returns a selector which selects nothing.
        #[inline]
        pub const fn empty() -> Self {
            Self {
                _m: ::core::marker::PhantomData,
                name: false,
                age: false,
                email: false,
            }
        }
        /// Returns a selector which selects all.
        #[inline]
        pub const fn all() -> Self {
            Self {
                _m: ::core::marker::PhantomData,
                name: true,
                age: true,
                email: true,
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
        pub const fn iter_selected(&self) -> UserSelectorIter<F, true> {
            UserSelectorIter::new(self, self.selected())
        }
        /// Returns an iterator over the unselected fields.
        #[inline]
        pub const fn iter_unselected(&self) -> UserSelectorIter<F, false> {
            UserSelectorIter::new(self, self.unselected())
        }
        /// Returns `true` if such field is selected.
        #[inline]
        pub const fn is_selected(&self, idx: UserFieldIndex) -> ::core::primitive::bool {
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
        pub const fn update_age(&mut self, value: ::core::primitive::bool) -> &mut Self {
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
    #[automatically_derived]
    impl<
        F: ?::core::marker::Sized,
    > ::grost::__private::Encode<
        ::grost::__private::flavors::Select,
        ::grost::__private::flavors::selector::Zst,
    > for UserSelector<F> {
        fn encode(
            &self,
            _: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Select as ::grost::__private::Flavor>::EncodeError,
        > {
            const SELECT_NONE: ::core::primitive::u8 = ::grost::__private::flavors::selector::SelectorIdentifier::none()
                .as_u8();
            const SELECT_ALL: ::core::primitive::u8 = ::grost::__private::flavors::selector::SelectorIdentifier::all()
                .as_u8();
            if self.is_empty() {
                if buf.is_empty() {
                    return ::core::result::Result::Err(
                        ::grost::__private::EncodeError::insufficient_buffer(1, 0),
                    );
                }
                buf[0] = SELECT_NONE;
                return ::core::result::Result::Ok(1);
            }
            if self.is_all() {
                if buf.is_empty() {
                    return ::core::result::Result::Err(
                        ::grost::__private::EncodeError::insufficient_buffer(1, 0),
                    );
                }
                buf[0] = SELECT_ALL;
                return ::core::result::Result::Ok(1);
            }
            ::core::result::Result::Err(
                ::grost::__private::EncodeError::custom(
                    "only select all fields or no fields can be encoded as zst",
                ),
            )
        }
        fn encoded_len(
            &self,
            _: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
        ) -> ::core::primitive::usize {
            1
        }
        fn encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
        ) -> ::core::primitive::usize {
            <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Select,
                ::grost::__private::flavors::selector::Zst,
            >>::encoded_len(self, ctx)
        }
        fn encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Select as ::grost::__private::Flavor>::EncodeError,
        > {
            <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Select,
                ::grost::__private::flavors::selector::Zst,
            >>::encode(self, ctx, buf)
        }
        fn encode_identified(
            &self,
            ctx: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
            identifier: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Identifier,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Select as ::grost::__private::Flavor>::EncodeError,
        > {
            if identifier.wire_type()
                != ::grost::__private::flavors::selector::SelectorWireType::Zst
            {
                return ::core::result::Result::Err(
                    ::grost::__private::EncodeError::unsupported_wire_type(
                        ::core::any::type_name::<Self>(),
                        identifier.wire_type(),
                    ),
                );
            }
            <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Select,
                ::grost::__private::flavors::selector::Zst,
            >>::encode(self, ctx, buf)
        }
        fn encoded_identified_len(
            &self,
            ctx: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
            identifier: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Identifier,
        ) -> ::core::primitive::usize {
            <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Select,
                ::grost::__private::flavors::selector::Zst,
            >>::encoded_len(self, ctx)
        }
    }
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::grost::__private::indexer::Indexable<F> for User {
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
    /// The field reflection of the struct.
    pub struct UserFieldReflection<
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized,
        const TAG: ::core::primitive::u32,
    > {
        _reflect: ::core::marker::PhantomData<R>,
        _flavor: ::core::marker::PhantomData<F>,
    }
    #[automatically_derived]
    impl<R, F, const TAG: ::core::primitive::u32> ::core::ops::Deref
    for UserFieldReflection<R, F, TAG>
    where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
    impl<R, F, const TAG: ::core::primitive::u32> UserFieldReflection<R, F, TAG>
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
    impl<
        F,
        const TAG: ::core::primitive::u32,
    > UserFieldReflection<::grost::__private::reflection::FieldReflection<F>, F, TAG>
    where
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
    {
        /// Returns the reflection of the field.
        #[inline]
        const fn new() -> Self {
            Self::new_in()
        }
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
        /// Returns the relection to a tag of the field.
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
        pub const fn encode_ref<'a>(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::encode::EncodeRefField<'a>,
            >,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the reflection to the reference encode fn which will give the length of the encoded data.
        #[inline]
        pub const fn encoded_ref_len<'a>(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::Len<
                    ::grost::__private::reflection::encode::EncodeRefField<'a>,
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
        pub const fn partial_encode_ref<'a>(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::encode::PartialEncodeRefField<'a>,
            >,
            F,
            TAG,
        > {
            UserFieldReflection::new_in()
        }
        /// Returns the reflection to the partial reference encode fn which will give the length of the encoded data.
        #[inline]
        pub const fn partial_encoded_ref_len<'a>(
            &self,
        ) -> UserFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::Len<
                    ::grost::__private::reflection::encode::PartialEncodeRefField<'a>,
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
    /// The reflection bridge type.
    pub struct UserReflection<R: ?::core::marker::Sized, F: ?::core::marker::Sized> {
        _reflect: ::core::marker::PhantomData<R>,
        _flavor: ::core::marker::PhantomData<F>,
    }
    #[automatically_derived]
    impl<R, F> ::core::ops::Deref for UserReflection<R, F>
    where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
    impl<F> UserReflection<::grost::__private::reflection::StructReflection<F>, F>
    where
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
            ::grost::__private::reflection::StructReflection<__GROST_FLAVOR__>,
            __GROST_FLAVOR__,
        >
        where
            __GROST_FLAVOR__: ?::core::marker::Sized + ::grost::__private::Flavor,
        {
            UserReflection::new()
        }
    }
    const _: () = {
        const _: () = {
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
            impl ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::network::Network,
            > for User {
                type Reflection = ::grost::__private::reflection::StructReflection<
                    ::grost::__private::flavors::network::Network,
                >;
                const REFLECTION: &Self::Reflection = &::grost::__private::reflection::StructReflectionBuilder::<
                    ::grost::__private::flavors::network::Network,
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
            impl ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::network::Network,
            >
            for UserReflection<
                ::grost::__private::reflection::StructReflection<
                    ::grost::__private::flavors::network::Network,
                >,
                ::grost::__private::flavors::network::Network,
            > {
                type Reflection = ::grost::__private::reflection::StructReflection<
                    ::grost::__private::flavors::network::Network,
                >;
                const REFLECTION: &Self::Reflection = <User as ::grost::__private::reflection::Reflectable<
                    ::grost::__private::flavors::network::Network,
                >>::REFLECTION;
            }
        };
    };
    const _: () = {
        #[automatically_derived]
        impl ::core::ops::Index<(UserFieldIndex, ::core::primitive::bool)>
        for UserSelector<::grost::__private::flavors::Network> {
            type Output = ::core::option::Option<
                &'static ::grost::__private::reflection::FieldReflection<
                    ::grost::__private::flavors::Network,
                >,
            >;
            fn index(
                &self,
                (indexer, select): (UserFieldIndex, ::core::primitive::bool),
            ) -> &Self::Output {
                const NONE: &::core::option::Option<
                    &'static ::grost::__private::reflection::FieldReflection<
                        ::grost::__private::flavors::Network,
                    >,
                > = &::core::option::Option::None;
                match indexer {
                    UserFieldIndex::Name => {
                        const REFLECTION: ::core::option::Option<
                            &::grost::__private::reflection::FieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                        > = ::core::option::Option::Some(
                            <UserFieldReflection<
                                ::grost::__private::reflection::FieldReflection<
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
                            &::grost::__private::reflection::FieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                        > = ::core::option::Option::Some(
                            <UserFieldReflection<
                                ::grost::__private::reflection::FieldReflection<
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
                            &::grost::__private::reflection::FieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                        > = ::core::option::Option::Some(
                            <UserFieldReflection<
                                ::grost::__private::reflection::FieldReflection<
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
                &'static ::grost::__private::reflection::FieldReflection<
                    ::grost::__private::flavors::Network,
                >,
            >;
            fn index(&self, indexer: UserFieldIndex) -> &Self::Output {
                const NONE: &::core::option::Option<
                    &::grost::__private::reflection::FieldReflection<
                        ::grost::__private::flavors::Network,
                    >,
                > = &::core::option::Option::None;
                match indexer {
                    UserFieldIndex::Name => {
                        const REFLECTION: ::core::option::Option<
                            &::grost::__private::reflection::FieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                        > = ::core::option::Option::Some(
                            <UserFieldReflection<
                                ::grost::__private::reflection::FieldReflection<
                                    ::grost::__private::flavors::Network,
                                >,
                                ::grost::__private::flavors::Network,
                                1u32,
                            > as ::grost::__private::reflection::Reflectable<
                                ::grost::__private::flavors::Network,
                            >>::REFLECTION,
                        );
                        if self.name { &REFLECTION } else { NONE }
                    }
                    UserFieldIndex::Age => {
                        const REFLECTION: ::core::option::Option<
                            &::grost::__private::reflection::FieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                        > = ::core::option::Option::Some(
                            <UserFieldReflection<
                                ::grost::__private::reflection::FieldReflection<
                                    ::grost::__private::flavors::Network,
                                >,
                                ::grost::__private::flavors::Network,
                                2u32,
                            > as ::grost::__private::reflection::Reflectable<
                                ::grost::__private::flavors::Network,
                            >>::REFLECTION,
                        );
                        if self.age { &REFLECTION } else { NONE }
                    }
                    UserFieldIndex::Email => {
                        const REFLECTION: ::core::option::Option<
                            &::grost::__private::reflection::FieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                        > = ::core::option::Option::Some(
                            <UserFieldReflection<
                                ::grost::__private::reflection::FieldReflection<
                                    ::grost::__private::flavors::Network,
                                >,
                                ::grost::__private::flavors::Network,
                                3u32,
                            > as ::grost::__private::reflection::Reflectable<
                                ::grost::__private::flavors::Network,
                            >>::REFLECTION,
                        );
                        if self.email { &REFLECTION } else { NONE }
                    }
                }
            }
        }
        fn insufficient_buffer_error<T, W>(
            f: &T,
            ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
            selector: ::core::option::Option<
                &<T as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            >,
            buf_len: ::core::primitive::usize,
        ) -> ::grost::__private::flavors::network::EncodeError
        where
            T: ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::network::Network,
                    W,
                >
                + ::grost::__private::Encode<
                    ::grost::__private::flavors::network::Network,
                    W,
                >
                + ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                > + ?::core::marker::Sized,
            W: ::grost::__private::WireFormat<
                ::grost::__private::flavors::network::Network,
            >,
        {
            match selector {
                ::core::option::Option::Some(selector) => {
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <T as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::network::Network,
                            W,
                        >>::partial_encoded_len(f, ctx, selector),
                        buf_len,
                    )
                }
                ::core::option::Option::None => {
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <T as ::grost::__private::Encode<
                            ::grost::__private::flavors::network::Network,
                            W,
                        >>::encoded_length_delimited_len(f, ctx),
                        buf_len,
                    )
                }
            }
        }
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
                &::grost::__private::network::Context,
                &<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            ) -> ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = &{
                fn encoded_len(
                    f: &::std::string::String,
                    ctx: &::grost::__private::flavors::network::Context,
                    selector: &<::std::string::String as ::grost::__private::Selectable<
                        ::grost::__private::flavors::network::Network,
                    >>::Selector,
                ) -> ::core::primitive::usize {
                    (*<User>::reflection::<::grost::__private::flavors::Network>()
                        .name()
                        .encoded_identifier_len())
                        + <::std::string::String as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::network::Network,
                            <::std::string::String as ::grost::__private::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::partial_encoded_length_delimited_len(f, ctx, selector)
                }
                encoded_len
            };
        }
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
                &::grost::__private::network::Context,
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
                            ::grost::__private::flavors::network::Network,
                            <::std::string::String as ::grost::__private::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::encoded_length_delimited_len(f, ctx)
                }
                encoded_len
            };
        }
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
                &::grost::__private::network::Context,
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
                        ::grost::__private::flavors::network::Network,
                        <::std::string::String as ::grost::__private::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::encode_length_delimited(f, ctx, &mut buf[offset..])
                        .map(|len| offset + len)
                        .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
                }
                encode
            };
        }
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
                &::grost::__private::network::Context,
                &mut [::core::primitive::u8],
                &<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
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
                        ::grost::__private::flavors::network::Network,
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
                        ::grost::__private::flavors::network::Network,
                        <::std::string::String as ::grost::__private::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::partial_encode_length_delimited(
                            f,
                            ctx,
                            &mut buf[offset..],
                            selector,
                        )
                        .map(|len| offset + len)
                        .map_err(|e| {
                            e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len)
                        })
                }
                encode
            };
        }
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
                &::grost::__private::network::Context,
                &<u32 as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            ) -> ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = &{
                fn encoded_len(
                    f: &u32,
                    ctx: &::grost::__private::flavors::network::Context,
                    selector: &<u32 as ::grost::__private::Selectable<
                        ::grost::__private::flavors::network::Network,
                    >>::Selector,
                ) -> ::core::primitive::usize {
                    (*<User>::reflection::<::grost::__private::flavors::Network>()
                        .age()
                        .encoded_identifier_len())
                        + <u32 as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::network::Network,
                            <u32 as ::grost::__private::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::partial_encoded_length_delimited_len(f, ctx, selector)
                }
                encoded_len
            };
        }
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
                &::grost::__private::network::Context,
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
                            ::grost::__private::flavors::network::Network,
                            <u32 as ::grost::__private::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::encoded_length_delimited_len(f, ctx)
                }
                encoded_len
            };
        }
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
                &::grost::__private::network::Context,
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
                        ::grost::__private::flavors::network::Network,
                        <u32 as ::grost::__private::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::encode_length_delimited(f, ctx, &mut buf[offset..])
                        .map(|len| offset + len)
                        .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
                }
                encode
            };
        }
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
                &::grost::__private::network::Context,
                &mut [::core::primitive::u8],
                &<u32 as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
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
                        ::grost::__private::flavors::network::Network,
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
                        ::grost::__private::flavors::network::Network,
                        <u32 as ::grost::__private::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::partial_encode_length_delimited(
                            f,
                            ctx,
                            &mut buf[offset..],
                            selector,
                        )
                        .map(|len| offset + len)
                        .map_err(|e| {
                            e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len)
                        })
                }
                encode
            };
        }
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
                &::grost::__private::network::Context,
                &<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            ) -> ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = &{
                fn encoded_len(
                    f: &::core::option::Option<::std::string::String>,
                    ctx: &::grost::__private::flavors::network::Context,
                    selector: &<::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::Selectable<
                        ::grost::__private::flavors::network::Network,
                    >>::Selector,
                ) -> ::core::primitive::usize {
                    match f {
                        ::core::option::Option::Some(f) => {
                            (*<User>::reflection::<
                                ::grost::__private::flavors::Network,
                            >()
                                .email()
                                .encoded_identifier_len())
                                + <::std::string::String as ::grost::__private::PartialEncode<
                                    ::grost::__private::flavors::network::Network,
                                    <::core::option::Option<
                                        ::std::string::String,
                                    > as ::grost::__private::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                >>::partial_encoded_length_delimited_len(f, ctx, selector)
                        }
                        ::core::option::Option::None => 0,
                    }
                }
                encoded_len
            };
        }
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
                &::grost::__private::network::Context,
            ) -> ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = &{
                fn encoded_len(
                    f: &::core::option::Option<::std::string::String>,
                    ctx: &::grost::__private::flavors::network::Context,
                ) -> ::core::primitive::usize {
                    match f {
                        ::core::option::Option::Some(f) => {
                            (*<User>::reflection::<
                                ::grost::__private::flavors::Network,
                            >()
                                .email()
                                .encoded_identifier_len())
                                + <::std::string::String as ::grost::__private::Encode<
                                    ::grost::__private::flavors::network::Network,
                                    <::core::option::Option<
                                        ::std::string::String,
                                    > as ::grost::__private::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                >>::encoded_length_delimited_len(f, ctx)
                        }
                        ::core::option::Option::None => 0,
                    }
                }
                encoded_len
            };
        }
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
                &::grost::__private::network::Context,
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
                                ::grost::__private::flavors::network::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::DefaultWireFormat<
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
                &::grost::__private::network::Context,
                &mut [::core::primitive::u8],
                &<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
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
                    selector: &<::std::string::String as ::grost::__private::Selectable<
                        ::grost::__private::flavors::network::Network,
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
                                ::grost::__private::flavors::network::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::DefaultWireFormat<
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
        #[automatically_derived]
        impl ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::network::Network,
        > for User {
            type Format = ::grost::__private::flavors::network::LengthDelimited;
        }
        #[automatically_derived]
        impl ::grost::__private::Encode<
            ::grost::__private::flavors::network::Network,
            ::grost::__private::flavors::network::LengthDelimited,
        > for User {
            fn encode(
                &self,
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                <::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::EncodeError,
            > {
                let mut offset = 0;
                let buf_len = buf.len();
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
                ::core::result::Result::Ok(offset)
            }
            fn encoded_len(
                &self,
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
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
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
            ) -> ::core::primitive::usize {
                let encoded_len = <Self as ::grost::__private::Encode<
                    ::grost::__private::flavors::network::Network,
                    ::grost::__private::flavors::network::LengthDelimited,
                >>::encoded_len(self, ctx);
                ::grost::__private::varing::encoded_u32_varint_len(
                    encoded_len as ::core::primitive::u32,
                ) + encoded_len
            }
            fn encode_length_delimited(
                &self,
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                <::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::EncodeError,
            > {
                let encoded_len = <Self as ::grost::__private::Encode<
                    ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_length_delimited_len(self, ctx),
                                buf_len,
                            )
                    })?;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::network::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_length_delimited_len(self, ctx),
                            buf_len,
                        ),
                    );
                }
                <Self as ::grost::__private::Encode<
                    ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_length_delimited_len(self, ctx),
                                buf_len,
                            )
                    })
            }
        }
        #[automatically_derived]
        impl ::grost::__private::PartialEncode<
            ::grost::__private::flavors::network::Network,
            ::grost::__private::flavors::network::LengthDelimited,
        > for User {
            fn partial_encode(
                &self,
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<User as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                <::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::EncodeError,
            > {
                let mut offset = 0;
                let buf_len = buf.len();
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::network::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_len(self, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                offset
                    += (<User>::reflection::<::grost::__private::flavors::Network>()
                        .age()
                        .partial_encode())(
                            &self.age,
                            ctx,
                            &mut buf[offset..],
                            &selector.age,
                        )
                        .map_err(|e| {
                            e.update(
                                <Self as ::grost::__private::PartialEncode<
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
                ::core::result::Result::Ok(offset)
            }
            fn partial_encoded_len(
                &self,
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
                selector: &<User as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
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
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
                selector: &<User as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            ) -> ::core::primitive::usize {
                let encoded_len = <Self as ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::network::Network,
                    ::grost::__private::flavors::network::LengthDelimited,
                >>::partial_encoded_len(self, ctx, selector);
                ::grost::__private::varing::encoded_u32_varint_len(
                    encoded_len as ::core::primitive::u32,
                ) + encoded_len
            }
            fn partial_encode_length_delimited(
                &self,
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<User as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                <::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::EncodeError,
            > {
                let encoded_len = <Self as ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
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
                                ::grost::__private::flavors::network::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_length_delimited_len(
                                self,
                                ctx,
                                selector,
                            ),
                            buf_len,
                        ),
                    );
                }
                <Self as ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
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
    };
};
const _: () = {
    #[automatically_derived]
    impl<'a, const N: ::core::primitive::bool> ::core::iter::Iterator
    for UserSelectorIter<'a, ::grost::__private::flavors::Network, N> {
        type Item = &'static ::grost::__private::reflection::FieldReflection<
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
    impl<'a, const N: ::core::primitive::bool> ::core::iter::FusedIterator
    for UserSelectorIter<'a, ::grost::__private::flavors::Network, N> {}
    #[automatically_derived]
    impl<'a, const N: ::core::primitive::bool> ::core::iter::ExactSizeIterator
    for UserSelectorIter<'a, ::grost::__private::flavors::Network, N> {
        #[inline]
        fn len(&self) -> ::core::primitive::usize {
            self.remaining()
        }
    }
    #[automatically_derived]
    impl UserSelector<::grost::__private::flavors::Network> {
        const __NETWORK_FLAVOR_ENCODED_LEN_NAME_TAG__: ::core::primitive::usize = ::grost::__private::varing::encoded_u32_varint_len(
            1u32,
        );
        const __NETWORK_FLAVOR_ENCODED_NAME_TAG__: &[::core::primitive::u8] = ::grost::__private::varing::encode_u32_varint(
                1u32,
            )
            .as_slice();
        const __NETWORK_FLAVOR_ENCODED_LEN_AGE_TAG__: ::core::primitive::usize = ::grost::__private::varing::encoded_u32_varint_len(
            2u32,
        );
        const __NETWORK_FLAVOR_ENCODED_AGE_TAG__: &[::core::primitive::u8] = ::grost::__private::varing::encode_u32_varint(
                2u32,
            )
            .as_slice();
        const __NETWORK_FLAVOR_ENCODED_LEN_EMAIL_TAG__: ::core::primitive::usize = ::grost::__private::varing::encoded_u32_varint_len(
            3u32,
        );
        const __NETWORK_FLAVOR_ENCODED_EMAIL_TAG__: &[::core::primitive::u8] = ::grost::__private::varing::encode_u32_varint(
                3u32,
            )
            .as_slice();
    }
    #[automatically_derived]
    impl ::grost::__private::DefaultWireFormat<::grost::__private::flavors::Select>
    for UserSelector<::grost::__private::flavors::Network> {
        type Format = ::grost::__private::flavors::selector::LengthDelimited;
    }
    impl ::grost::__private::Encode<
        ::grost::__private::flavors::Select,
        ::grost::__private::flavors::selector::LengthDelimited,
    > for UserSelector<::grost::__private::flavors::Network> {
        fn encode(
            &self,
            _: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Select as ::grost::__private::Flavor>::EncodeError,
        > {
            ::core::todo!()
        }
        fn encoded_len(
            &self,
            _: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
        ) -> ::core::primitive::usize {
            ::core::todo!()
        }
        fn encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
        ) -> ::core::primitive::usize {
            ::core::todo!()
        }
        fn encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Select as ::grost::__private::Flavor>::EncodeError,
        > {
            ::core::todo!()
        }
    }
};
#[derive(::core::fmt::Debug, ::core::clone::Clone)]
///A comment struct
pub struct Comment {
    user: User,
    replyer: ::core::option::Option<User>,
    title: ::std::string::String,
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
/// The selection type for Comment
pub struct CommentSelector<F: ?::core::marker::Sized> {
    _m: ::core::marker::PhantomData<F>,
    user: <User as ::grost::__private::Selectable<F>>::Selector,
    replyer: <::core::option::Option<
        User,
    > as ::grost::__private::Selectable<F>>::Selector,
    title: ::core::primitive::bool,
    content: ::core::primitive::bool,
}
/// An iterator over the selected fields of the [`CommentSelector`]
pub struct CommentSelectorIter<
    'a,
    F: ?::core::marker::Sized,
    const N: ::core::primitive::bool = true,
> {
    selector: &'a CommentSelector<F>,
    index: ::core::option::Option<CommentFieldIndex>,
    num: ::core::primitive::usize,
    yielded: ::core::primitive::usize,
}
const _: () = {
    impl ::core::default::Default for Comment {
        fn default() -> Self {
            Self::new()
        }
    }
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
    impl<
        'a,
        F: ?::core::marker::Sized,
        const N: ::core::primitive::bool,
    > CommentSelectorIter<'a, F, N> {
        #[inline]
        const fn new(
            selector: &'a CommentSelector<F>,
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
    impl<F: ?::core::marker::Sized> CommentSelector<F> {
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
    impl<F: ?::core::marker::Sized> ::core::fmt::Debug for CommentSelector<F> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::write!(f, "CommentSelector")?;
            self.debug_helper(f)
        }
    }
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::core::cmp::PartialEq for CommentSelector<F> {
        fn eq(&self, other: &Self) -> ::core::primitive::bool {
            self.user == other.user && self.replyer == other.replyer
                && self.title == other.title && self.content == other.content
        }
    }
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::core::cmp::Eq for CommentSelector<F> {}
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::core::hash::Hash for CommentSelector<F> {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
            self.user.hash(state);
            self.replyer.hash(state);
            self.title.hash(state);
            self.content.hash(state);
        }
    }
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::core::clone::Clone for CommentSelector<F> {
        fn clone(&self) -> Self {
            *self
        }
    }
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::core::marker::Copy for CommentSelector<F> {}
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::grost::__private::Selectable<F> for Comment {
        type Selector = CommentSelector<F>;
    }
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::grost::__private::Selector<F>
    for CommentSelector<F> {
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
                F,
            >>::Selector as ::grost::__private::Selector<F>>::flip(&mut self.user);
            <<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                F,
            >>::Selector as ::grost::__private::Selector<F>>::flip(&mut self.replyer);
            self.title = !self.title;
            self.content = !self.content;
            self
        }
        fn merge(&mut self, other: Self) -> &mut Self {
            <<User as ::grost::__private::Selectable<
                F,
            >>::Selector as ::grost::__private::Selector<
                F,
            >>::merge(&mut self.user, other.user);
            <<::core::option::Option<
                User,
            > as ::grost::__private::Selectable<
                F,
            >>::Selector as ::grost::__private::Selector<
                F,
            >>::merge(&mut self.replyer, other.replyer);
            self.title |= other.title;
            self.content |= other.content;
            self
        }
    }
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> CommentSelector<F> {
        /// The number of options in this selection type.
        pub const OPTIONS: ::core::primitive::usize = 4usize;
        /// Returns a selector which selects nothing.
        #[inline]
        pub const fn empty() -> Self {
            Self {
                _m: ::core::marker::PhantomData,
                user: <<User as ::grost::__private::Selectable<
                    F,
                >>::Selector as ::grost::__private::Selector<F>>::NONE,
                replyer: <<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    F,
                >>::Selector as ::grost::__private::Selector<F>>::NONE,
                title: false,
                content: false,
            }
        }
        /// Returns a selector which selects all.
        #[inline]
        pub const fn all() -> Self {
            Self {
                _m: ::core::marker::PhantomData,
                user: <<User as ::grost::__private::Selectable<
                    F,
                >>::Selector as ::grost::__private::Selector<F>>::ALL,
                replyer: <<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    F,
                >>::Selector as ::grost::__private::Selector<F>>::ALL,
                title: true,
                content: true,
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
        pub const fn iter_selected(&self) -> CommentSelectorIter<F, true> {
            CommentSelectorIter::new(self, self.selected())
        }
        /// Returns an iterator over the unselected fields.
        #[inline]
        pub const fn iter_unselected(&self) -> CommentSelectorIter<F, false> {
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
            val: <User as ::grost::__private::Selectable<F>>::Selector,
        ) -> &mut Self {
            self.user = val;
            self
        }
        /// Unselect the `Comment.user` field
        #[inline]
        pub fn unselect_user(&mut self) -> &mut Self {
            self.user = <<User as ::grost::__private::Selectable<
                F,
            >>::Selector as ::grost::__private::Selector<F>>::NONE;
            self
        }
        /// Get a reference to the selector of `Comment.user` field
        #[inline]
        pub const fn user_ref(
            &self,
        ) -> &<User as ::grost::__private::Selectable<F>>::Selector {
            &self.user
        }
        /// Get a mutable reference to the selector of `Comment.user` field
        #[inline]
        pub const fn user_mut(
            &mut self,
        ) -> &mut <User as ::grost::__private::Selectable<F>>::Selector {
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
            > as ::grost::__private::Selectable<F>>::Selector,
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
                F,
            >>::Selector as ::grost::__private::Selector<F>>::NONE;
            self
        }
        /// Get a reference to the selector of `Comment.replyer` field
        #[inline]
        pub const fn replyer_ref(
            &self,
        ) -> &<::core::option::Option<
            User,
        > as ::grost::__private::Selectable<F>>::Selector {
            &self.replyer
        }
        /// Get a mutable reference to the selector of `Comment.replyer` field
        #[inline]
        pub const fn replyer_mut(
            &mut self,
        ) -> &mut <::core::option::Option<
            User,
        > as ::grost::__private::Selectable<F>>::Selector {
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
    #[automatically_derived]
    impl<
        F: ?::core::marker::Sized,
    > ::grost::__private::Encode<
        ::grost::__private::flavors::Select,
        ::grost::__private::flavors::selector::Zst,
    > for CommentSelector<F> {
        fn encode(
            &self,
            _: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Select as ::grost::__private::Flavor>::EncodeError,
        > {
            const SELECT_NONE: ::core::primitive::u8 = ::grost::__private::flavors::selector::SelectorIdentifier::none()
                .as_u8();
            const SELECT_ALL: ::core::primitive::u8 = ::grost::__private::flavors::selector::SelectorIdentifier::all()
                .as_u8();
            if self.is_empty() {
                if buf.is_empty() {
                    return ::core::result::Result::Err(
                        ::grost::__private::EncodeError::insufficient_buffer(1, 0),
                    );
                }
                buf[0] = SELECT_NONE;
                return ::core::result::Result::Ok(1);
            }
            if self.is_all() {
                if buf.is_empty() {
                    return ::core::result::Result::Err(
                        ::grost::__private::EncodeError::insufficient_buffer(1, 0),
                    );
                }
                buf[0] = SELECT_ALL;
                return ::core::result::Result::Ok(1);
            }
            ::core::result::Result::Err(
                ::grost::__private::EncodeError::custom(
                    "only select all fields or no fields can be encoded as zst",
                ),
            )
        }
        fn encoded_len(
            &self,
            _: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
        ) -> ::core::primitive::usize {
            1
        }
        fn encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
        ) -> ::core::primitive::usize {
            <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Select,
                ::grost::__private::flavors::selector::Zst,
            >>::encoded_len(self, ctx)
        }
        fn encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Select as ::grost::__private::Flavor>::EncodeError,
        > {
            <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Select,
                ::grost::__private::flavors::selector::Zst,
            >>::encode(self, ctx, buf)
        }
        fn encode_identified(
            &self,
            ctx: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
            identifier: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Identifier,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Select as ::grost::__private::Flavor>::EncodeError,
        > {
            if identifier.wire_type()
                != ::grost::__private::flavors::selector::SelectorWireType::Zst
            {
                return ::core::result::Result::Err(
                    ::grost::__private::EncodeError::unsupported_wire_type(
                        ::core::any::type_name::<Self>(),
                        identifier.wire_type(),
                    ),
                );
            }
            <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Select,
                ::grost::__private::flavors::selector::Zst,
            >>::encode(self, ctx, buf)
        }
        fn encoded_identified_len(
            &self,
            ctx: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
            identifier: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Identifier,
        ) -> ::core::primitive::usize {
            <Self as ::grost::__private::Encode<
                ::grost::__private::flavors::Select,
                ::grost::__private::flavors::selector::Zst,
            >>::encoded_len(self, ctx)
        }
    }
    #[automatically_derived]
    impl<F: ?::core::marker::Sized> ::grost::__private::indexer::Indexable<F>
    for Comment {
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
    /// The field reflection of the struct.
    pub struct CommentFieldReflection<
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized,
        const TAG: ::core::primitive::u32,
    > {
        _reflect: ::core::marker::PhantomData<R>,
        _flavor: ::core::marker::PhantomData<F>,
    }
    #[automatically_derived]
    impl<R, F, const TAG: ::core::primitive::u32> ::core::ops::Deref
    for CommentFieldReflection<R, F, TAG>
    where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
    impl<R, F, const TAG: ::core::primitive::u32> CommentFieldReflection<R, F, TAG>
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
    impl<
        F,
        const TAG: ::core::primitive::u32,
    > CommentFieldReflection<::grost::__private::reflection::FieldReflection<F>, F, TAG>
    where
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
    {
        /// Returns the reflection of the field.
        #[inline]
        const fn new() -> Self {
            Self::new_in()
        }
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
        /// Returns the relection to a tag of the field.
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
        pub const fn encode_ref<'a>(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::encode::EncodeRefField<'a>,
            >,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the reflection to the reference encode fn which will give the length of the encoded data.
        #[inline]
        pub const fn encoded_ref_len<'a>(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::Len<
                    ::grost::__private::reflection::encode::EncodeRefField<'a>,
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
        pub const fn partial_encode_ref<'a>(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::encode::PartialEncodeRefField<'a>,
            >,
            F,
            TAG,
        > {
            CommentFieldReflection::new_in()
        }
        /// Returns the reflection to the partial reference encode fn which will give the length of the encoded data.
        #[inline]
        pub const fn partial_encoded_ref_len<'a>(
            &self,
        ) -> CommentFieldReflection<
            ::grost::__private::reflection::encode::EncodeReflection<
                ::grost::__private::reflection::Len<
                    ::grost::__private::reflection::encode::PartialEncodeRefField<'a>,
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
    /// The reflection bridge type.
    pub struct CommentReflection<R: ?::core::marker::Sized, F: ?::core::marker::Sized> {
        _reflect: ::core::marker::PhantomData<R>,
        _flavor: ::core::marker::PhantomData<F>,
    }
    #[automatically_derived]
    impl<R, F> ::core::ops::Deref for CommentReflection<R, F>
    where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
    impl<F> CommentReflection<::grost::__private::reflection::StructReflection<F>, F>
    where
        F: ?::core::marker::Sized + ::grost::__private::Flavor,
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
            ::grost::__private::reflection::StructReflection<__GROST_FLAVOR__>,
            __GROST_FLAVOR__,
        >
        where
            __GROST_FLAVOR__: ?::core::marker::Sized + ::grost::__private::Flavor,
        {
            CommentReflection::new()
        }
    }
    const _: () = {
        const _: () = {
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
                    >::Struct(
                        <User as ::grost::__private::reflection::Reflectable<
                            ::grost::__private::flavors::Network,
                        >>::REFLECTION,
                    ),
                }
                    .build();
            }
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
                        >::Struct(
                            <User as ::grost::__private::reflection::Reflectable<
                                ::grost::__private::flavors::Network,
                            >>::REFLECTION,
                        ),
                    ),
                }
                    .build();
            }
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
            impl ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::network::Network,
            > for Comment {
                type Reflection = ::grost::__private::reflection::StructReflection<
                    ::grost::__private::flavors::network::Network,
                >;
                const REFLECTION: &Self::Reflection = &::grost::__private::reflection::StructReflectionBuilder::<
                    ::grost::__private::flavors::network::Network,
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
            impl ::grost::__private::reflection::Reflectable<
                ::grost::__private::flavors::network::Network,
            >
            for CommentReflection<
                ::grost::__private::reflection::StructReflection<
                    ::grost::__private::flavors::network::Network,
                >,
                ::grost::__private::flavors::network::Network,
            > {
                type Reflection = ::grost::__private::reflection::StructReflection<
                    ::grost::__private::flavors::network::Network,
                >;
                const REFLECTION: &Self::Reflection = <Comment as ::grost::__private::reflection::Reflectable<
                    ::grost::__private::flavors::network::Network,
                >>::REFLECTION;
            }
        };
    };
    const _: () = {
        #[automatically_derived]
        impl ::core::ops::Index<(CommentFieldIndex, ::core::primitive::bool)>
        for CommentSelector<::grost::__private::flavors::Network> {
            type Output = ::core::option::Option<
                &'static ::grost::__private::reflection::FieldReflection<
                    ::grost::__private::flavors::Network,
                >,
            >;
            fn index(
                &self,
                (indexer, select): (CommentFieldIndex, ::core::primitive::bool),
            ) -> &Self::Output {
                const NONE: &::core::option::Option<
                    &'static ::grost::__private::reflection::FieldReflection<
                        ::grost::__private::flavors::Network,
                    >,
                > = &::core::option::Option::None;
                match indexer {
                    CommentFieldIndex::User => {
                        const REFLECTION: ::core::option::Option<
                            &::grost::__private::reflection::FieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                        > = ::core::option::Option::Some(
                            <CommentFieldReflection<
                                ::grost::__private::reflection::FieldReflection<
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
                            &::grost::__private::reflection::FieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                        > = ::core::option::Option::Some(
                            <CommentFieldReflection<
                                ::grost::__private::reflection::FieldReflection<
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
                            &::grost::__private::reflection::FieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                        > = ::core::option::Option::Some(
                            <CommentFieldReflection<
                                ::grost::__private::reflection::FieldReflection<
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
                            &::grost::__private::reflection::FieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                        > = ::core::option::Option::Some(
                            <CommentFieldReflection<
                                ::grost::__private::reflection::FieldReflection<
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
                &'static ::grost::__private::reflection::FieldReflection<
                    ::grost::__private::flavors::Network,
                >,
            >;
            fn index(&self, indexer: CommentFieldIndex) -> &Self::Output {
                const NONE: &::core::option::Option<
                    &::grost::__private::reflection::FieldReflection<
                        ::grost::__private::flavors::Network,
                    >,
                > = &::core::option::Option::None;
                match indexer {
                    CommentFieldIndex::User => {
                        const REFLECTION: ::core::option::Option<
                            &::grost::__private::reflection::FieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                        > = ::core::option::Option::Some(
                            <CommentFieldReflection<
                                ::grost::__private::reflection::FieldReflection<
                                    ::grost::__private::flavors::Network,
                                >,
                                ::grost::__private::flavors::Network,
                                1u32,
                            > as ::grost::__private::reflection::Reflectable<
                                ::grost::__private::flavors::Network,
                            >>::REFLECTION,
                        );
                        if self.user.is_empty() { NONE } else { &REFLECTION }
                    }
                    CommentFieldIndex::Replyer => {
                        const REFLECTION: ::core::option::Option<
                            &::grost::__private::reflection::FieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                        > = ::core::option::Option::Some(
                            <CommentFieldReflection<
                                ::grost::__private::reflection::FieldReflection<
                                    ::grost::__private::flavors::Network,
                                >,
                                ::grost::__private::flavors::Network,
                                2u32,
                            > as ::grost::__private::reflection::Reflectable<
                                ::grost::__private::flavors::Network,
                            >>::REFLECTION,
                        );
                        if self.replyer.is_empty() { NONE } else { &REFLECTION }
                    }
                    CommentFieldIndex::Title => {
                        const REFLECTION: ::core::option::Option<
                            &::grost::__private::reflection::FieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                        > = ::core::option::Option::Some(
                            <CommentFieldReflection<
                                ::grost::__private::reflection::FieldReflection<
                                    ::grost::__private::flavors::Network,
                                >,
                                ::grost::__private::flavors::Network,
                                3u32,
                            > as ::grost::__private::reflection::Reflectable<
                                ::grost::__private::flavors::Network,
                            >>::REFLECTION,
                        );
                        if self.title { &REFLECTION } else { NONE }
                    }
                    CommentFieldIndex::Content => {
                        const REFLECTION: ::core::option::Option<
                            &::grost::__private::reflection::FieldReflection<
                                ::grost::__private::flavors::Network,
                            >,
                        > = ::core::option::Option::Some(
                            <CommentFieldReflection<
                                ::grost::__private::reflection::FieldReflection<
                                    ::grost::__private::flavors::Network,
                                >,
                                ::grost::__private::flavors::Network,
                                4u32,
                            > as ::grost::__private::reflection::Reflectable<
                                ::grost::__private::flavors::Network,
                            >>::REFLECTION,
                        );
                        if self.content { &REFLECTION } else { NONE }
                    }
                }
            }
        }
        fn insufficient_buffer_error<T, W>(
            f: &T,
            ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
            selector: ::core::option::Option<
                &<T as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            >,
            buf_len: ::core::primitive::usize,
        ) -> ::grost::__private::flavors::network::EncodeError
        where
            T: ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::network::Network,
                    W,
                >
                + ::grost::__private::Encode<
                    ::grost::__private::flavors::network::Network,
                    W,
                >
                + ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                > + ?::core::marker::Sized,
            W: ::grost::__private::WireFormat<
                ::grost::__private::flavors::network::Network,
            >,
        {
            match selector {
                ::core::option::Option::Some(selector) => {
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <T as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::network::Network,
                            W,
                        >>::partial_encoded_len(f, ctx, selector),
                        buf_len,
                    )
                }
                ::core::option::Option::None => {
                    ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                        <T as ::grost::__private::Encode<
                            ::grost::__private::flavors::network::Network,
                            W,
                        >>::encoded_length_delimited_len(f, ctx),
                        buf_len,
                    )
                }
            }
        }
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
                &::grost::__private::network::Context,
                &<User as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            ) -> ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = &{
                fn encoded_len(
                    f: &User,
                    ctx: &::grost::__private::flavors::network::Context,
                    selector: &<User as ::grost::__private::Selectable<
                        ::grost::__private::flavors::network::Network,
                    >>::Selector,
                ) -> ::core::primitive::usize {
                    (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .user()
                        .encoded_identifier_len())
                        + <User as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::network::Network,
                            <User as ::grost::__private::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::partial_encoded_length_delimited_len(f, ctx, selector)
                }
                encoded_len
            };
        }
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
                &::grost::__private::network::Context,
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
                            ::grost::__private::flavors::network::Network,
                            <User as ::grost::__private::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::encoded_length_delimited_len(f, ctx)
                }
                encoded_len
            };
        }
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
                &::grost::__private::network::Context,
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
                            &<Comment>::reflection::<
                                ::grost::__private::flavors::Network,
                            >()
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
                        ::grost::__private::flavors::network::Network,
                        <User as ::grost::__private::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::encode_length_delimited(f, ctx, &mut buf[offset..])
                        .map(|len| offset + len)
                        .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
                }
                encode
            };
        }
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
                &::grost::__private::network::Context,
                &mut [::core::primitive::u8],
                &<User as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
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
                        ::grost::__private::flavors::network::Network,
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
                            &<Comment>::reflection::<
                                ::grost::__private::flavors::Network,
                            >()
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
                        ::grost::__private::flavors::network::Network,
                        <User as ::grost::__private::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::partial_encode_length_delimited(
                            f,
                            ctx,
                            &mut buf[offset..],
                            selector,
                        )
                        .map(|len| offset + len)
                        .map_err(|e| {
                            e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len)
                        })
                }
                encode
            };
        }
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
                &::grost::__private::network::Context,
                &<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            ) -> ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = &{
                fn encoded_len(
                    f: &::core::option::Option<User>,
                    ctx: &::grost::__private::flavors::network::Context,
                    selector: &<::core::option::Option<
                        User,
                    > as ::grost::__private::Selectable<
                        ::grost::__private::flavors::network::Network,
                    >>::Selector,
                ) -> ::core::primitive::usize {
                    match f {
                        ::core::option::Option::Some(f) => {
                            (*<Comment>::reflection::<
                                ::grost::__private::flavors::Network,
                            >()
                                .replyer()
                                .encoded_identifier_len())
                                + <User as ::grost::__private::PartialEncode<
                                    ::grost::__private::flavors::network::Network,
                                    <::core::option::Option<
                                        User,
                                    > as ::grost::__private::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                >>::partial_encoded_length_delimited_len(f, ctx, selector)
                        }
                        ::core::option::Option::None => 0,
                    }
                }
                encoded_len
            };
        }
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
                &::grost::__private::network::Context,
            ) -> ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = &{
                fn encoded_len(
                    f: &::core::option::Option<User>,
                    ctx: &::grost::__private::flavors::network::Context,
                ) -> ::core::primitive::usize {
                    match f {
                        ::core::option::Option::Some(f) => {
                            (*<Comment>::reflection::<
                                ::grost::__private::flavors::Network,
                            >()
                                .replyer()
                                .encoded_identifier_len())
                                + <User as ::grost::__private::Encode<
                                    ::grost::__private::flavors::network::Network,
                                    <::core::option::Option<
                                        User,
                                    > as ::grost::__private::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                >>::encoded_length_delimited_len(f, ctx)
                        }
                        ::core::option::Option::None => 0,
                    }
                }
                encoded_len
            };
        }
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
                &::grost::__private::network::Context,
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
                                ::grost::__private::flavors::network::Network,
                                <::core::option::Option<
                                    User,
                                > as ::grost::__private::DefaultWireFormat<
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
                &::grost::__private::network::Context,
                &mut [::core::primitive::u8],
                &<::core::option::Option<
                    User,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
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
                    selector: &<User as ::grost::__private::Selectable<
                        ::grost::__private::flavors::network::Network,
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
                                ::grost::__private::flavors::network::Network,
                                <::core::option::Option<
                                    User,
                                > as ::grost::__private::DefaultWireFormat<
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
                &::grost::__private::network::Context,
                &<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            ) -> ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = &{
                fn encoded_len(
                    f: &::std::string::String,
                    ctx: &::grost::__private::flavors::network::Context,
                    selector: &<::std::string::String as ::grost::__private::Selectable<
                        ::grost::__private::flavors::network::Network,
                    >>::Selector,
                ) -> ::core::primitive::usize {
                    (*<Comment>::reflection::<::grost::__private::flavors::Network>()
                        .title()
                        .encoded_identifier_len())
                        + <::std::string::String as ::grost::__private::PartialEncode<
                            ::grost::__private::flavors::network::Network,
                            <::std::string::String as ::grost::__private::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::partial_encoded_length_delimited_len(f, ctx, selector)
                }
                encoded_len
            };
        }
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
                &::grost::__private::network::Context,
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
                            ::grost::__private::flavors::network::Network,
                            <::std::string::String as ::grost::__private::DefaultWireFormat<
                                ::grost::__private::flavors::Network,
                            >>::Format,
                        >>::encoded_length_delimited_len(f, ctx)
                }
                encoded_len
            };
        }
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
                &::grost::__private::network::Context,
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
                            &<Comment>::reflection::<
                                ::grost::__private::flavors::Network,
                            >()
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
                        ::grost::__private::flavors::network::Network,
                        <::std::string::String as ::grost::__private::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::encode_length_delimited(f, ctx, &mut buf[offset..])
                        .map(|len| offset + len)
                        .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
                }
                encode
            };
        }
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
                &::grost::__private::network::Context,
                &mut [::core::primitive::u8],
                &<::std::string::String as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
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
                        ::grost::__private::flavors::network::Network,
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
                            &<Comment>::reflection::<
                                ::grost::__private::flavors::Network,
                            >()
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
                        ::grost::__private::flavors::network::Network,
                        <::std::string::String as ::grost::__private::DefaultWireFormat<
                            ::grost::__private::flavors::Network,
                        >>::Format,
                    >>::partial_encode_length_delimited(
                            f,
                            ctx,
                            &mut buf[offset..],
                            selector,
                        )
                        .map(|len| offset + len)
                        .map_err(|e| {
                            e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len)
                        })
                }
                encode
            };
        }
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
                &::grost::__private::network::Context,
                &<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            ) -> ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = &{
                fn encoded_len(
                    f: &::core::option::Option<::std::string::String>,
                    ctx: &::grost::__private::flavors::network::Context,
                    selector: &<::core::option::Option<
                        ::std::string::String,
                    > as ::grost::__private::Selectable<
                        ::grost::__private::flavors::network::Network,
                    >>::Selector,
                ) -> ::core::primitive::usize {
                    match f {
                        ::core::option::Option::Some(f) => {
                            (*<Comment>::reflection::<
                                ::grost::__private::flavors::Network,
                            >()
                                .content()
                                .encoded_identifier_len())
                                + <::std::string::String as ::grost::__private::PartialEncode<
                                    ::grost::__private::flavors::network::Network,
                                    <::core::option::Option<
                                        ::std::string::String,
                                    > as ::grost::__private::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                >>::partial_encoded_length_delimited_len(f, ctx, selector)
                        }
                        ::core::option::Option::None => 0,
                    }
                }
                encoded_len
            };
        }
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
                &::grost::__private::network::Context,
            ) -> ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = &{
                fn encoded_len(
                    f: &::core::option::Option<::std::string::String>,
                    ctx: &::grost::__private::flavors::network::Context,
                ) -> ::core::primitive::usize {
                    match f {
                        ::core::option::Option::Some(f) => {
                            (*<Comment>::reflection::<
                                ::grost::__private::flavors::Network,
                            >()
                                .content()
                                .encoded_identifier_len())
                                + <::std::string::String as ::grost::__private::Encode<
                                    ::grost::__private::flavors::network::Network,
                                    <::core::option::Option<
                                        ::std::string::String,
                                    > as ::grost::__private::DefaultWireFormat<
                                        ::grost::__private::flavors::Network,
                                    >>::Format,
                                >>::encoded_length_delimited_len(f, ctx)
                        }
                        ::core::option::Option::None => 0,
                    }
                }
                encoded_len
            };
        }
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
                &::grost::__private::network::Context,
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
                                ::grost::__private::flavors::network::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::DefaultWireFormat<
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
                &::grost::__private::network::Context,
                &mut [::core::primitive::u8],
                &<::core::option::Option<
                    ::std::string::String,
                > as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
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
                    selector: &<::std::string::String as ::grost::__private::Selectable<
                        ::grost::__private::flavors::network::Network,
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
                                ::grost::__private::flavors::network::Network,
                                <::core::option::Option<
                                    ::std::string::String,
                                > as ::grost::__private::DefaultWireFormat<
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
        #[automatically_derived]
        impl ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::network::Network,
        > for Comment {
            type Format = ::grost::__private::flavors::network::LengthDelimited;
        }
        #[automatically_derived]
        impl ::grost::__private::Encode<
            ::grost::__private::flavors::network::Network,
            ::grost::__private::flavors::network::LengthDelimited,
        > for Comment {
            fn encode(
                &self,
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                <::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::EncodeError,
            > {
                let mut offset = 0;
                let buf_len = buf.len();
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_len(self, ctx),
                                buf_len,
                            )
                        })?;
                ::core::result::Result::Ok(offset)
            }
            fn encoded_len(
                &self,
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
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
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
            ) -> ::core::primitive::usize {
                let encoded_len = <Self as ::grost::__private::Encode<
                    ::grost::__private::flavors::network::Network,
                    ::grost::__private::flavors::network::LengthDelimited,
                >>::encoded_len(self, ctx);
                ::grost::__private::varing::encoded_u32_varint_len(
                    encoded_len as ::core::primitive::u32,
                ) + encoded_len
            }
            fn encode_length_delimited(
                &self,
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
                buf: &mut [::core::primitive::u8],
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                <::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::EncodeError,
            > {
                let encoded_len = <Self as ::grost::__private::Encode<
                    ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_length_delimited_len(self, ctx),
                                buf_len,
                            )
                    })?;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::Encode<
                                ::grost::__private::flavors::network::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::encoded_length_delimited_len(self, ctx),
                            buf_len,
                        ),
                    );
                }
                <Self as ::grost::__private::Encode<
                    ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::encoded_length_delimited_len(self, ctx),
                                buf_len,
                            )
                    })
            }
        }
        #[automatically_derived]
        impl ::grost::__private::PartialEncode<
            ::grost::__private::flavors::network::Network,
            ::grost::__private::flavors::network::LengthDelimited,
        > for Comment {
            fn partial_encode(
                &self,
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<Comment as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                <::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::EncodeError,
            > {
                let mut offset = 0;
                let buf_len = buf.len();
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            <Self as ::grost::__private::PartialEncode<
                                ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
                                    ::grost::__private::flavors::network::LengthDelimited,
                                >>::partial_encoded_len(self, ctx, selector),
                                buf_len,
                            )
                        })?;
                ::core::result::Result::Ok(offset)
            }
            fn partial_encoded_len(
                &self,
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
                selector: &<Comment as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
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
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
                selector: &<Comment as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            ) -> ::core::primitive::usize {
                let encoded_len = <Self as ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::network::Network,
                    ::grost::__private::flavors::network::LengthDelimited,
                >>::partial_encoded_len(self, ctx, selector);
                ::grost::__private::varing::encoded_u32_varint_len(
                    encoded_len as ::core::primitive::u32,
                ) + encoded_len
            }
            fn partial_encode_length_delimited(
                &self,
                ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
                buf: &mut [::core::primitive::u8],
                selector: &<Comment as ::grost::__private::Selectable<
                    ::grost::__private::flavors::network::Network,
                >>::Selector,
            ) -> ::core::result::Result<
                ::core::primitive::usize,
                <::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::EncodeError,
            > {
                let encoded_len = <Self as ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
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
                                ::grost::__private::flavors::network::Network,
                                ::grost::__private::flavors::network::LengthDelimited,
                            >>::partial_encoded_length_delimited_len(
                                self,
                                ctx,
                                selector,
                            ),
                            buf_len,
                        ),
                    );
                }
                <Self as ::grost::__private::PartialEncode<
                    ::grost::__private::flavors::network::Network,
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
                                    ::grost::__private::flavors::network::Network,
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
    };
};
const _: () = {
    #[automatically_derived]
    impl<'a, const N: ::core::primitive::bool> ::core::iter::Iterator
    for CommentSelectorIter<'a, ::grost::__private::flavors::Network, N> {
        type Item = &'static ::grost::__private::reflection::FieldReflection<
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
    impl<'a, const N: ::core::primitive::bool> ::core::iter::FusedIterator
    for CommentSelectorIter<'a, ::grost::__private::flavors::Network, N> {}
    #[automatically_derived]
    impl<'a, const N: ::core::primitive::bool> ::core::iter::ExactSizeIterator
    for CommentSelectorIter<'a, ::grost::__private::flavors::Network, N> {
        #[inline]
        fn len(&self) -> ::core::primitive::usize {
            self.remaining()
        }
    }
    #[automatically_derived]
    impl CommentSelector<::grost::__private::flavors::Network> {
        const __NETWORK_FLAVOR_ENCODED_LEN_USER_TAG__: ::core::primitive::usize = ::grost::__private::varing::encoded_u32_varint_len(
            1u32,
        );
        const __NETWORK_FLAVOR_ENCODED_USER_TAG__: &[::core::primitive::u8] = ::grost::__private::varing::encode_u32_varint(
                1u32,
            )
            .as_slice();
        const __NETWORK_FLAVOR_ENCODED_LEN_REPLYER_TAG__: ::core::primitive::usize = ::grost::__private::varing::encoded_u32_varint_len(
            2u32,
        );
        const __NETWORK_FLAVOR_ENCODED_REPLYER_TAG__: &[::core::primitive::u8] = ::grost::__private::varing::encode_u32_varint(
                2u32,
            )
            .as_slice();
        const __NETWORK_FLAVOR_ENCODED_LEN_TITLE_TAG__: ::core::primitive::usize = ::grost::__private::varing::encoded_u32_varint_len(
            3u32,
        );
        const __NETWORK_FLAVOR_ENCODED_TITLE_TAG__: &[::core::primitive::u8] = ::grost::__private::varing::encode_u32_varint(
                3u32,
            )
            .as_slice();
        const __NETWORK_FLAVOR_ENCODED_LEN_CONTENT_TAG__: ::core::primitive::usize = ::grost::__private::varing::encoded_u32_varint_len(
            4u32,
        );
        const __NETWORK_FLAVOR_ENCODED_CONTENT_TAG__: &[::core::primitive::u8] = ::grost::__private::varing::encode_u32_varint(
                4u32,
            )
            .as_slice();
    }
    #[automatically_derived]
    impl ::grost::__private::DefaultWireFormat<::grost::__private::flavors::Select>
    for CommentSelector<::grost::__private::flavors::Network> {
        type Format = ::grost::__private::flavors::selector::LengthDelimited;
    }
    impl ::grost::__private::Encode<
        ::grost::__private::flavors::Select,
        ::grost::__private::flavors::selector::LengthDelimited,
    > for CommentSelector<::grost::__private::flavors::Network> {
        fn encode(
            &self,
            _: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Select as ::grost::__private::Flavor>::EncodeError,
        > {
            ::core::todo!()
        }
        fn encoded_len(
            &self,
            _: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
        ) -> ::core::primitive::usize {
            ::core::todo!()
        }
        fn encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
        ) -> ::core::primitive::usize {
            ::core::todo!()
        }
        fn encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Select as ::grost::__private::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Select as ::grost::__private::Flavor>::EncodeError,
        > {
            ::core::todo!()
        }
    }
};
