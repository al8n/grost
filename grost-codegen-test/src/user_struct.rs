#![no_implicit_prelude]

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
pub enum UserFieldIndexer {
    /// The field indexer for the field `name`
    Name = 0u32,
    /// The field indexer for the field `age`
    Age = 1u32,
    /// The field indexer for the field `email`
    Email = 2u32,
}
impl UserFieldIndexer {
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
impl ::core::iter::Iterator for UserFieldIndexer {
    type Item = Self;
    fn next(&mut self) -> ::core::option::Option<Self> {
        Self::next(self)
    }
    fn size_hint(
        &self,
    ) -> (::core::primitive::usize, ::core::option::Option<::core::primitive::usize>) {
        let remaining = self.remaining();
        (remaining, ::core::option::Option::Some(remaining))
    }
}
impl ::core::iter::DoubleEndedIterator for UserFieldIndexer {
    fn next_back(&mut self) -> ::core::option::Option<Self> {
        Self::prev(self)
    }
}
impl ::core::iter::FusedIterator for UserFieldIndexer {}
impl ::core::iter::ExactSizeIterator for UserFieldIndexer {
    fn len(&self) -> ::core::primitive::usize {
        self.remaining()
    }
}
#[derive(::core::fmt::Debug, ::core::clone::Clone)]
///A user struct
pub struct User {
    name: ::std::string::String,
    age: u32,
    email: ::core::option::Option<::std::string::String>,
}
impl User {
    /// The reflection information of the `name` field for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_NAME_REFLECTION: ::grost::__private::reflection::FieldReflection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::FieldReflectionBuilder::<
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
    /// The reflection information of the `age` field for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_AGE_REFLECTION: ::grost::__private::reflection::FieldReflection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::FieldReflectionBuilder::<
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
    /// The reflection information of the `email` field for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_EMAIL_REFLECTION: ::grost::__private::reflection::FieldReflection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::FieldReflectionBuilder::<
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
    /// The reflection of the struct `User` for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_REFLECTION: ::grost::__private::reflection::StructReflection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::StructReflectionBuilder::<
        ::grost::__private::flavors::Network,
    > {
        name: "User",
        schema_name: "User",
        fields: &[
            Self::NETWORK_FLAVOR_NAME_REFLECTION,
            Self::NETWORK_FLAVOR_AGE_REFLECTION,
            Self::NETWORK_FLAVOR_EMAIL_REFLECTION,
        ],
    }
        .build();
}
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
    pub fn name(&self) -> &::std::string::String {
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
    pub const fn age(&self) -> u32 {
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
    pub fn email(&self) -> ::core::option::Option<&::std::string::String> {
        ::core::option::Option::as_ref(&self.email)
    }
    /// Gets the mutable reference of the field `email`.
    #[inline]
    pub fn email_mut(&mut self) -> ::core::option::Option<&mut ::std::string::String> {
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
impl ::grost::__private::flavors::DefaultWireFormat<
    ::grost::__private::flavors::network::Network,
> for User {
    type Format = ::grost::__private::flavors::network::LengthDelimited;
}
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
        ::core::todo!()
    }
    fn encoded_len(
        &self,
        ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
    ) -> ::core::primitive::usize {
        ::core::todo!()
    }
    fn encoded_length_delimited_len(
        &self,
        ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
    ) -> ::core::primitive::usize {
        ::core::todo!()
    }
    fn encode_length_delimited(
        &self,
        ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
        buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<
        ::core::primitive::usize,
        <::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::EncodeError,
    > {
        ::core::todo!()
    }
}
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
        ::core::todo!()
    }
    fn partial_encoded_len(
        &self,
        ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
        selector: &<User as ::grost::__private::Selectable<
            ::grost::__private::flavors::network::Network,
        >>::Selector,
    ) -> ::core::primitive::usize {
        ::core::todo!()
    }
    fn partial_encoded_length_delimited_len(
        &self,
        ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
        selector: &<User as ::grost::__private::Selectable<
            ::grost::__private::flavors::network::Network,
        >>::Selector,
    ) -> ::core::primitive::usize {
        ::core::todo!()
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
        ::core::todo!()
    }
}
/// An iterator over the selected fields of the [`UserSelector`]
pub struct UserSelectorIter<
    'a,
    F: ?::core::marker::Sized,
    const N: ::core::primitive::bool = true,
> {
    selector: &'a UserSelector<F>,
    index: ::core::option::Option<UserFieldIndexer>,
    num: ::core::primitive::usize,
    yielded: ::core::primitive::usize,
}
impl<
    'a,
    F: ?::core::marker::Sized,
    const N: ::core::primitive::bool,
> UserSelectorIter<'a, F, N> {
    #[inline]
    const fn new(selector: &'a UserSelector<F>, num: ::core::primitive::usize) -> Self {
        Self {
            selector,
            index: ::core::option::Option::Some(UserFieldIndexer::FIRST),
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
/// The selection type for User
pub struct UserSelector<F: ?::core::marker::Sized> {
    _m: ::core::marker::PhantomData<F>,
    name: ::core::primitive::bool,
    age: ::core::primitive::bool,
    email: ::core::primitive::bool,
}
#[automatically_derived]
impl<F: ?::core::marker::Sized> UserSelector<F> {
    fn debug_helper(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::result::Result<(), ::core::fmt::Error> {
        let num_selected = self.num_selected();
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
impl<F: ?::core::marker::Sized> ::grost::__private::Selector for UserSelector<F> {
    const ALL: Self = Self::all();
    const NONE: Self = Self::empty();
    fn selected(&self) -> ::core::primitive::usize {
        self.num_selected()
    }
    fn unselected(&self) -> ::core::primitive::usize {
        self.num_unselected()
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
    pub const fn num_selected(&self) -> ::core::primitive::usize {
        let mut num = 0;
        if self.name {
            num += 1;
        }
        if self.age {
            num += 1;
        }
        if self.email {
            num += 1;
        }
        num
    }
    /// Returns the number of unselected fields.
    #[inline]
    pub const fn num_unselected(&self) -> ::core::primitive::usize {
        let mut num = 0;
        if !self.name {
            num += 1;
        }
        if !self.age {
            num += 1;
        }
        if !self.email {
            num += 1;
        }
        num
    }
    /// Returns an iterator over the selected fields.
    #[inline]
    pub const fn iter_selected(&self) -> UserSelectorIter<F, true> {
        UserSelectorIter::new(self, self.num_selected())
    }
    /// Returns an iterator over the unselected fields.
    #[inline]
    pub const fn iter_unselected(&self) -> UserSelectorIter<F, false> {
        UserSelectorIter::new(self, self.num_unselected())
    }
    /// Returns `true` if such field is selected.
    #[inline]
    pub const fn contains(&self, idx: UserFieldIndexer) -> ::core::primitive::bool {
        match idx {
            UserFieldIndexer::Name => self.name,
            UserFieldIndexer::Age => self.age,
            UserFieldIndexer::Email => self.email,
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
    pub const fn update_name(&mut self, value: ::core::primitive::bool) -> &mut Self {
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
    /// Check if the `User.name` field is set
    #[inline]
    pub const fn contains_name(&self) -> ::core::primitive::bool {
        self.name
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
    /// Check if the `User.age` field is set
    #[inline]
    pub const fn contains_age(&self) -> ::core::primitive::bool {
        self.age
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
    pub const fn update_email(&mut self, value: ::core::primitive::bool) -> &mut Self {
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
    /// Check if the `User.email` field is set
    #[inline]
    pub const fn contains_email(&self) -> ::core::primitive::bool {
        self.email
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
const _: () = {
    #[automatically_derived]
    impl UserSelector<::grost::__private::flavors::Network> {
        fn __field_reflection_by_index_network_flavor(
            &self,
            idx: UserFieldIndexer,
            select: ::core::primitive::bool,
        ) -> ::core::option::Option<
            &'static ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >,
        > {
            match idx {
                UserFieldIndexer::Name => {
                    match (select, self.name) {
                        (true, true) => {
                            ::core::option::Option::Some(
                                &User::NETWORK_FLAVOR_NAME_REFLECTION,
                            )
                        }
                        (true, false) => ::core::option::Option::None,
                        (false, true) => ::core::option::Option::None,
                        (false, false) => {
                            ::core::option::Option::Some(
                                &User::NETWORK_FLAVOR_NAME_REFLECTION,
                            )
                        }
                    }
                }
                UserFieldIndexer::Age => {
                    match (select, self.age) {
                        (true, true) => {
                            ::core::option::Option::Some(
                                &User::NETWORK_FLAVOR_AGE_REFLECTION,
                            )
                        }
                        (true, false) => ::core::option::Option::None,
                        (false, true) => ::core::option::Option::None,
                        (false, false) => {
                            ::core::option::Option::Some(
                                &User::NETWORK_FLAVOR_AGE_REFLECTION,
                            )
                        }
                    }
                }
                UserFieldIndexer::Email => {
                    match (select, self.email) {
                        (true, true) => {
                            ::core::option::Option::Some(
                                &User::NETWORK_FLAVOR_EMAIL_REFLECTION,
                            )
                        }
                        (true, false) => ::core::option::Option::None,
                        (false, true) => ::core::option::Option::None,
                        (false, false) => {
                            ::core::option::Option::Some(
                                &User::NETWORK_FLAVOR_EMAIL_REFLECTION,
                            )
                        }
                    }
                }
            }
        }
    }
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
                        match self
                            .selector
                            .__field_reflection_by_index_network_flavor(index, N)
                        {
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
pub enum CommentFieldIndexer {
    /// The field indexer for the field `user`
    User = 0u32,
    /// The field indexer for the field `title`
    Title = 1u32,
    /// The field indexer for the field `content`
    Content = 2u32,
}
impl CommentFieldIndexer {
    /// The number of variants of this field indexer.
    pub const VARIANTS: ::core::primitive::usize = 3usize;
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
            Self::User => ::core::option::Option::Some(Self::Title),
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
            Self::Title => ::core::option::Option::Some(Self::User),
        }
    }
    /// Returns the remaining number of fields.
    #[inline]
    pub const fn remaining(&self) -> ::core::primitive::usize {
        Self::LAST as ::core::primitive::usize
            - *self as ::core::primitive::u32 as ::core::primitive::usize
    }
}
impl ::core::iter::Iterator for CommentFieldIndexer {
    type Item = Self;
    fn next(&mut self) -> ::core::option::Option<Self> {
        Self::next(self)
    }
    fn size_hint(
        &self,
    ) -> (::core::primitive::usize, ::core::option::Option<::core::primitive::usize>) {
        let remaining = self.remaining();
        (remaining, ::core::option::Option::Some(remaining))
    }
}
impl ::core::iter::DoubleEndedIterator for CommentFieldIndexer {
    fn next_back(&mut self) -> ::core::option::Option<Self> {
        Self::prev(self)
    }
}
impl ::core::iter::FusedIterator for CommentFieldIndexer {}
impl ::core::iter::ExactSizeIterator for CommentFieldIndexer {
    fn len(&self) -> ::core::primitive::usize {
        self.remaining()
    }
}
#[derive(::core::fmt::Debug, ::core::clone::Clone)]
///A comment struct
pub struct Comment {
    user: User,
    title: ::std::string::String,
    content: ::core::option::Option<::std::string::String>,
}
impl Comment {
    /// The reflection information of the `user` field for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_USER_REFLECTION: ::grost::__private::reflection::FieldReflection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::FieldReflectionBuilder::<
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
        >::Struct(<User>::NETWORK_FLAVOR_REFLECTION),
    }
        .build();
    /// The reflection information of the `title` field for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_TITLE_REFLECTION: ::grost::__private::reflection::FieldReflection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::FieldReflectionBuilder::<
        ::grost::__private::flavors::Network,
    > {
        identifier: ::grost::__private::flavors::network::Identifier::new(
            <<::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::WIRE_TYPE,
            ::grost::__private::flavors::network::Tag::new(2u32),
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
    /// The reflection information of the `content` field for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_CONTENT_REFLECTION: ::grost::__private::reflection::FieldReflection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::FieldReflectionBuilder::<
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
    /// The reflection of the struct `Comment` for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_REFLECTION: ::grost::__private::reflection::StructReflection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::StructReflectionBuilder::<
        ::grost::__private::flavors::Network,
    > {
        name: "Comment",
        schema_name: "Comment",
        fields: &[
            Self::NETWORK_FLAVOR_USER_REFLECTION,
            Self::NETWORK_FLAVOR_TITLE_REFLECTION,
            Self::NETWORK_FLAVOR_CONTENT_REFLECTION,
        ],
    }
        .build();
}
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
            title: ::core::default::Default::default(),
            content: ::core::default::Default::default(),
        }
    }
    /// Gets the reference of the field `user`.
    #[inline]
    pub fn user(&self) -> &User {
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
    /// Gets the reference of the field `title`.
    #[inline]
    pub fn title(&self) -> &::std::string::String {
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
    pub fn content(&self) -> ::core::option::Option<&::std::string::String> {
        ::core::option::Option::as_ref(&self.content)
    }
    /// Gets the mutable reference of the field `content`.
    #[inline]
    pub fn content_mut(&mut self) -> ::core::option::Option<&mut ::std::string::String> {
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
impl ::grost::__private::flavors::DefaultWireFormat<
    ::grost::__private::flavors::network::Network,
> for Comment {
    type Format = ::grost::__private::flavors::network::LengthDelimited;
}
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
        ::core::todo!()
    }
    fn encoded_len(
        &self,
        ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
    ) -> ::core::primitive::usize {
        ::core::todo!()
    }
    fn encoded_length_delimited_len(
        &self,
        ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
    ) -> ::core::primitive::usize {
        ::core::todo!()
    }
    fn encode_length_delimited(
        &self,
        ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
        buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<
        ::core::primitive::usize,
        <::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::EncodeError,
    > {
        ::core::todo!()
    }
}
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
        ::core::todo!()
    }
    fn partial_encoded_len(
        &self,
        ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
        selector: &<Comment as ::grost::__private::Selectable<
            ::grost::__private::flavors::network::Network,
        >>::Selector,
    ) -> ::core::primitive::usize {
        ::core::todo!()
    }
    fn partial_encoded_length_delimited_len(
        &self,
        ctx: &<::grost::__private::flavors::network::Network as ::grost::__private::flavors::Flavor>::Context,
        selector: &<Comment as ::grost::__private::Selectable<
            ::grost::__private::flavors::network::Network,
        >>::Selector,
    ) -> ::core::primitive::usize {
        ::core::todo!()
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
        ::core::todo!()
    }
}
/// An iterator over the selected fields of the [`CommentSelector`]
pub struct CommentSelectorIter<
    'a,
    F: ?::core::marker::Sized,
    const N: ::core::primitive::bool = true,
> {
    selector: &'a CommentSelector<F>,
    index: ::core::option::Option<CommentFieldIndexer>,
    num: ::core::primitive::usize,
    yielded: ::core::primitive::usize,
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
            index: ::core::option::Option::Some(CommentFieldIndexer::FIRST),
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
/// The selection type for Comment
pub struct CommentSelector<F: ?::core::marker::Sized> {
    _m: ::core::marker::PhantomData<F>,
    user: <User as ::grost::__private::Selectable<F>>::Selector,
    title: ::core::primitive::bool,
    content: ::core::primitive::bool,
}
#[automatically_derived]
impl<F: ?::core::marker::Sized> CommentSelector<F> {
    fn debug_helper(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::result::Result<(), ::core::fmt::Error> {
        let num_selected = self.num_selected();
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
        self.user == other.user && self.title == other.title
            && self.content == other.content
    }
}
#[automatically_derived]
impl<F: ?::core::marker::Sized> ::core::cmp::Eq for CommentSelector<F> {}
#[automatically_derived]
impl<F: ?::core::marker::Sized> ::core::hash::Hash for CommentSelector<F> {
    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.user.hash(state);
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
impl<F: ?::core::marker::Sized> ::grost::__private::Selector for CommentSelector<F> {
    const ALL: Self = Self::all();
    const NONE: Self = Self::empty();
    fn selected(&self) -> ::core::primitive::usize {
        self.num_selected()
    }
    fn unselected(&self) -> ::core::primitive::usize {
        self.num_unselected()
    }
    fn flip(&mut self) -> &mut Self {
        <<User as ::grost::__private::Selectable<
            F,
        >>::Selector as ::grost::__private::Selector>::flip(&mut self.user);
        self.title = !self.title;
        self.content = !self.content;
        self
    }
    fn merge(&mut self, other: Self) -> &mut Self {
        <<User as ::grost::__private::Selectable<
            F,
        >>::Selector as ::grost::__private::Selector>::merge(&mut self.user, other.user);
        self.title |= other.title;
        self.content |= other.content;
        self
    }
}
#[automatically_derived]
impl<F: ?::core::marker::Sized> CommentSelector<F> {
    /// The number of options in this selection type.
    pub const OPTIONS: ::core::primitive::usize = 3usize;
    /// Returns a selector which selects nothing.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            _m: ::core::marker::PhantomData,
            user: <<User as ::grost::__private::Selectable<
                F,
            >>::Selector as ::grost::__private::Selector>::NONE,
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
            >>::Selector as ::grost::__private::Selector>::ALL,
            title: true,
            content: true,
        }
    }
    /// Returns `true` if the selector selects nothing.
    #[inline]
    pub const fn is_empty(&self) -> ::core::primitive::bool {
        self.user.is_empty() && !self.title && !self.content
    }
    /// Returns `true` if the selector selects all.
    #[inline]
    pub const fn is_all(&self) -> ::core::primitive::bool {
        self.user.is_all() && self.title && self.content
    }
    /// Returns the number of selected fields.
    #[inline]
    pub const fn num_selected(&self) -> ::core::primitive::usize {
        let mut num = 0;
        if !self.user.is_empty() {
            num += 1;
        }
        if self.title {
            num += 1;
        }
        if self.content {
            num += 1;
        }
        num
    }
    /// Returns the number of unselected fields.
    #[inline]
    pub const fn num_unselected(&self) -> ::core::primitive::usize {
        let mut num = 0;
        if self.user.is_empty() {
            num += 1;
        }
        if !self.title {
            num += 1;
        }
        if !self.content {
            num += 1;
        }
        num
    }
    /// Returns an iterator over the selected fields.
    #[inline]
    pub const fn iter_selected(&self) -> CommentSelectorIter<F, true> {
        CommentSelectorIter::new(self, self.num_selected())
    }
    /// Returns an iterator over the unselected fields.
    #[inline]
    pub const fn iter_unselected(&self) -> CommentSelectorIter<F, false> {
        CommentSelectorIter::new(self, self.num_unselected())
    }
    /// Returns `true` if such field is selected.
    #[inline]
    pub const fn contains(&self, idx: CommentFieldIndexer) -> ::core::primitive::bool {
        match idx {
            CommentFieldIndexer::User => !self.user.is_empty(),
            CommentFieldIndexer::Title => self.title,
            CommentFieldIndexer::Content => self.content,
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
        >>::Selector as ::grost::__private::Selector>::NONE;
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
    pub const fn update_title(&mut self, value: ::core::primitive::bool) -> &mut Self {
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
    /// Check if the `Comment.title` field is set
    #[inline]
    pub const fn contains_title(&self) -> ::core::primitive::bool {
        self.title
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
    pub const fn update_content(&mut self, value: ::core::primitive::bool) -> &mut Self {
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
    /// Check if the `Comment.content` field is set
    #[inline]
    pub const fn contains_content(&self) -> ::core::primitive::bool {
        self.content
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
const _: () = {
    #[automatically_derived]
    impl CommentSelector<::grost::__private::flavors::Network> {
        fn __field_reflection_by_index_network_flavor(
            &self,
            idx: CommentFieldIndexer,
            select: ::core::primitive::bool,
        ) -> ::core::option::Option<
            &'static ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >,
        > {
            match idx {
                CommentFieldIndexer::User => {
                    match (select, self.user.is_empty()) {
                        (true, false) => {
                            ::core::option::Option::Some(
                                &Comment::NETWORK_FLAVOR_USER_REFLECTION,
                            )
                        }
                        (true, true) => ::core::option::Option::None,
                        (false, false) => ::core::option::Option::None,
                        (false, true) => {
                            ::core::option::Option::Some(
                                &Comment::NETWORK_FLAVOR_USER_REFLECTION,
                            )
                        }
                    }
                }
                CommentFieldIndexer::Title => {
                    match (select, self.title) {
                        (true, true) => {
                            ::core::option::Option::Some(
                                &Comment::NETWORK_FLAVOR_TITLE_REFLECTION,
                            )
                        }
                        (true, false) => ::core::option::Option::None,
                        (false, true) => ::core::option::Option::None,
                        (false, false) => {
                            ::core::option::Option::Some(
                                &Comment::NETWORK_FLAVOR_TITLE_REFLECTION,
                            )
                        }
                    }
                }
                CommentFieldIndexer::Content => {
                    match (select, self.content) {
                        (true, true) => {
                            ::core::option::Option::Some(
                                &Comment::NETWORK_FLAVOR_CONTENT_REFLECTION,
                            )
                        }
                        (true, false) => ::core::option::Option::None,
                        (false, true) => ::core::option::Option::None,
                        (false, false) => {
                            ::core::option::Option::Some(
                                &Comment::NETWORK_FLAVOR_CONTENT_REFLECTION,
                            )
                        }
                    }
                }
            }
        }
    }
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
                        match self
                            .selector
                            .__field_reflection_by_index_network_flavor(index, N)
                        {
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
        const __NETWORK_FLAVOR_ENCODED_LEN_TITLE_TAG__: ::core::primitive::usize = ::grost::__private::varing::encoded_u32_varint_len(
            2u32,
        );
        const __NETWORK_FLAVOR_ENCODED_TITLE_TAG__: &[::core::primitive::u8] = ::grost::__private::varing::encode_u32_varint(
                2u32,
            )
            .as_slice();
        const __NETWORK_FLAVOR_ENCODED_LEN_CONTENT_TAG__: ::core::primitive::usize = ::grost::__private::varing::encoded_u32_varint_len(
            3u32,
        );
        const __NETWORK_FLAVOR_ENCODED_CONTENT_TAG__: &[::core::primitive::u8] = ::grost::__private::varing::encode_u32_varint(
                3u32,
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
