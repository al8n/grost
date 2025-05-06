#![no_implicit_prelude]

impl<F: ?::core::marker::Sized> ::grost::__private::indexer::Indexable<F> for User {
    type Indexer = UserFieldIndexer;
}
/// The concrete field index for the struct [`User`]
pub struct UserFieldIndex<O: ?::core::marker::Sized, F: ?::core::marker::Sized> {
    variant: UserFieldIndexer,
    _flavor: ::core::marker::PhantomData<F>,
    _output: ::core::marker::PhantomData<O>,
}
impl<
    O: ?::core::marker::Sized,
    F: ?::core::marker::Sized,
> ::core::convert::AsRef<UserFieldIndexer> for UserFieldIndex<O, F> {
    fn as_ref(&self) -> &UserFieldIndexer {
        &self.variant
    }
}
impl<O: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::clone::Clone
for UserFieldIndex<O, F> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<O: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::marker::Copy
for UserFieldIndex<O, F> {}
impl<O: ?::core::marker::Sized, F: ?::core::marker::Sized> UserFieldIndex<O, F> {
    /// Create a new field index.
    #[inline]
    pub const fn new(variant: UserFieldIndexer) -> Self {
        Self {
            variant,
            _flavor: ::core::marker::PhantomData,
            _output: ::core::marker::PhantomData,
        }
    }
    /// Returns the indexer which creates this index.
    #[inline]
    pub const fn indexer(&self) -> UserFieldIndexer {
        self.variant
    }
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
pub enum UserFieldIndexer {
    /// The field indexer for the field `name`
    Name = 0u32,
    /// The field indexer for the field `age`
    Age = 1u32,
    /// The field indexer for the field `email`
    Email = 2u32,
}
#[automatically_derived]
impl UserFieldIndexer {
    /// The number of variants of this field indexer.
    pub const VARIANTS: ::core::primitive::usize = 3usize;
    /// The first field indexer.
    pub const FIRST: Self = Self::Name;
    /// The last field indexer.
    pub const LAST: Self = Self::Email;
    /// Returns the field reflection index, which can be used to index the field reflection.
    #[inline]
    pub const fn field_reflection<F>(
        &self,
    ) -> UserFieldIndex<::grost::__private::reflection::FieldReflection<F>, F>
    where
        F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
        UserFieldIndex::new(*self)
    }
    /// Returns the tag index, which can be used to index the tag of the field.
    #[inline]
    pub const fn tag<F>(&self) -> UserFieldIndex<F::Tag, F>
    where
        F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
        UserFieldIndex::new(*self)
    }
    /// Returns the identifier index, which can be used to index the identifier of the field.
    #[inline]
    pub const fn identifier<F>(&self) -> UserFieldIndex<F::Identifier, F>
    where
        F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
        UserFieldIndex::new(*self)
    }
    /// Returns the wire type index, which can be used to index the wire type of the field.
    #[inline]
    pub const fn wire_type<F>(&self) -> UserFieldIndex<F::WireType, F>
    where
        F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
        UserFieldIndex::new(*self)
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
#[automatically_derived]
impl ::core::iter::DoubleEndedIterator for UserFieldIndexer {
    fn next_back(&mut self) -> ::core::option::Option<Self> {
        Self::prev(self)
    }
}
#[automatically_derived]
impl ::core::iter::FusedIterator for UserFieldIndexer {}
#[automatically_derived]
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
    pub const NETWORK_FLAVOR_NAME_REFLECTION: &::grost::__private::reflection::FieldReflection<
        ::grost::__private::flavors::Network,
    > = &::grost::__private::reflection::FieldReflectionBuilder::<
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
    const __NETWORK_FLAVOR_NAME_REFLECTION_OPTIONAL__: ::core::option::Option<
        &::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >,
    > = ::core::option::Option::Some(Self::NETWORK_FLAVOR_NAME_REFLECTION);
    /// The reflection information of the `age` field for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_AGE_REFLECTION: &::grost::__private::reflection::FieldReflection<
        ::grost::__private::flavors::Network,
    > = &::grost::__private::reflection::FieldReflectionBuilder::<
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
    const __NETWORK_FLAVOR_AGE_REFLECTION_OPTIONAL__: ::core::option::Option<
        &::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >,
    > = ::core::option::Option::Some(Self::NETWORK_FLAVOR_AGE_REFLECTION);
    /// The reflection information of the `email` field for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_EMAIL_REFLECTION: &::grost::__private::reflection::FieldReflection<
        ::grost::__private::flavors::Network,
    > = &::grost::__private::reflection::FieldReflectionBuilder::<
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
    const __NETWORK_FLAVOR_EMAIL_REFLECTION_OPTIONAL__: ::core::option::Option<
        &::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >,
    > = ::core::option::Option::Some(Self::NETWORK_FLAVOR_EMAIL_REFLECTION);
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
const _: () = {
    const NETWORK_FLAVOR_NAME_IDENTIFIER: ::grost::__private::flavors::network::Identifier = User::NETWORK_FLAVOR_NAME_REFLECTION
        .identifier();
    const NETWORK_FLAVOR_NAME_TAG: ::grost::__private::flavors::network::Tag = NETWORK_FLAVOR_NAME_IDENTIFIER
        .tag();
    const NETWORK_FLAVOR_NAME_WIRE_TYPE: ::grost::__private::flavors::network::WireType = NETWORK_FLAVOR_NAME_IDENTIFIER
        .wire_type();
    const NETWORK_FLAVOR_ENCODED_NAME_IDENTIFIER_LEN: ::core::primitive::usize = NETWORK_FLAVOR_NAME_IDENTIFIER
        .encoded_len();
    const NETWORK_FLAVOR_ENCODED_NAME_IDENTIFIER: &[::core::primitive::u8] = NETWORK_FLAVOR_NAME_IDENTIFIER
        .encode()
        .as_slice();
    const NETWORK_FLAVOR_AGE_IDENTIFIER: ::grost::__private::flavors::network::Identifier = User::NETWORK_FLAVOR_AGE_REFLECTION
        .identifier();
    const NETWORK_FLAVOR_AGE_TAG: ::grost::__private::flavors::network::Tag = NETWORK_FLAVOR_AGE_IDENTIFIER
        .tag();
    const NETWORK_FLAVOR_AGE_WIRE_TYPE: ::grost::__private::flavors::network::WireType = NETWORK_FLAVOR_AGE_IDENTIFIER
        .wire_type();
    const NETWORK_FLAVOR_ENCODED_AGE_IDENTIFIER_LEN: ::core::primitive::usize = NETWORK_FLAVOR_AGE_IDENTIFIER
        .encoded_len();
    const NETWORK_FLAVOR_ENCODED_AGE_IDENTIFIER: &[::core::primitive::u8] = NETWORK_FLAVOR_AGE_IDENTIFIER
        .encode()
        .as_slice();
    const NETWORK_FLAVOR_EMAIL_IDENTIFIER: ::grost::__private::flavors::network::Identifier = User::NETWORK_FLAVOR_EMAIL_REFLECTION
        .identifier();
    const NETWORK_FLAVOR_EMAIL_TAG: ::grost::__private::flavors::network::Tag = NETWORK_FLAVOR_EMAIL_IDENTIFIER
        .tag();
    const NETWORK_FLAVOR_EMAIL_WIRE_TYPE: ::grost::__private::flavors::network::WireType = NETWORK_FLAVOR_EMAIL_IDENTIFIER
        .wire_type();
    const NETWORK_FLAVOR_ENCODED_EMAIL_IDENTIFIER_LEN: ::core::primitive::usize = NETWORK_FLAVOR_EMAIL_IDENTIFIER
        .encoded_len();
    const NETWORK_FLAVOR_ENCODED_EMAIL_IDENTIFIER: &[::core::primitive::u8] = NETWORK_FLAVOR_EMAIL_IDENTIFIER
        .encode()
        .as_slice();
    #[automatically_derived]
    impl ::core::ops::Index<(UserFieldIndexer, ::core::primitive::bool)>
    for UserSelector<::grost::__private::flavors::Network> {
        type Output = ::core::option::Option<
            &'static ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >,
        >;
        fn index(
            &self,
            (indexer, select): (UserFieldIndexer, ::core::primitive::bool),
        ) -> &Self::Output {
            const NONE: &::core::option::Option<
                &'static ::grost::__private::reflection::FieldReflection<
                    ::grost::__private::flavors::Network,
                >,
            > = &::core::option::Option::None;
            match indexer {
                UserFieldIndexer::Name => {
                    match (select, self.name) {
                        (true, true) => {
                            &User::__NETWORK_FLAVOR_NAME_REFLECTION_OPTIONAL__
                        }
                        (true, false) => NONE,
                        (false, true) => NONE,
                        (false, false) => {
                            &User::__NETWORK_FLAVOR_NAME_REFLECTION_OPTIONAL__
                        }
                    }
                }
                UserFieldIndexer::Age => {
                    match (select, self.age) {
                        (true, true) => &User::__NETWORK_FLAVOR_AGE_REFLECTION_OPTIONAL__,
                        (true, false) => NONE,
                        (false, true) => NONE,
                        (false, false) => {
                            &User::__NETWORK_FLAVOR_AGE_REFLECTION_OPTIONAL__
                        }
                    }
                }
                UserFieldIndexer::Email => {
                    match (select, self.email) {
                        (true, true) => {
                            &User::__NETWORK_FLAVOR_EMAIL_REFLECTION_OPTIONAL__
                        }
                        (true, false) => NONE,
                        (false, true) => NONE,
                        (false, false) => {
                            &User::__NETWORK_FLAVOR_EMAIL_REFLECTION_OPTIONAL__
                        }
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::ops::Index<UserFieldIndexer>
    for UserSelector<::grost::__private::flavors::Network> {
        type Output = ::core::option::Option<
            &'static ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >,
        >;
        fn index(&self, indexer: UserFieldIndexer) -> &Self::Output {
            const NONE: &::core::option::Option<
                &::grost::__private::reflection::FieldReflection<
                    ::grost::__private::flavors::Network,
                >,
            > = &::core::option::Option::None;
            match indexer {
                UserFieldIndexer::Name => {
                    if self.name {
                        &User::__NETWORK_FLAVOR_NAME_REFLECTION_OPTIONAL__
                    } else {
                        NONE
                    }
                }
                UserFieldIndexer::Age => {
                    if self.age {
                        &User::__NETWORK_FLAVOR_AGE_REFLECTION_OPTIONAL__
                    } else {
                        NONE
                    }
                }
                UserFieldIndexer::Email => {
                    if self.email {
                        &User::__NETWORK_FLAVOR_EMAIL_REFLECTION_OPTIONAL__
                    } else {
                        NONE
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::ops::Index<()>
    for UserFieldIndex<
        ::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
    > {
        type Output = &'static ::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >;
        fn index(&self, indexer: ()) -> &Self::Output {
            match ::core::convert::AsRef::<UserFieldIndexer>::as_ref(self) {
                UserFieldIndexer::Name => &User::NETWORK_FLAVOR_NAME_REFLECTION,
                UserFieldIndexer::Age => &User::NETWORK_FLAVOR_AGE_REFLECTION,
                UserFieldIndexer::Email => &User::NETWORK_FLAVOR_EMAIL_REFLECTION,
            }
        }
    }
    #[automatically_derived]
    impl ::core::ops::Deref
    for UserFieldIndex<
        ::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
    > {
        type Target = ::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >;
        fn deref(&self) -> &Self::Target {
            self[()]
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug
    for UserFieldIndex<
        ::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
    > {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::fmt::Debug::fmt(&self[()], f)
        }
    }
    #[automatically_derived]
    impl ::core::ops::Index<()>
    for UserFieldIndex<
        ::grost::__private::flavors::network::Tag,
        ::grost::__private::flavors::Network,
    > {
        type Output = ::grost::__private::network::Tag;
        fn index(&self, _: ()) -> &Self::Output {
            match ::core::convert::AsRef::<UserFieldIndexer>::as_ref(self) {
                UserFieldIndexer::Name => &NETWORK_FLAVOR_NAME_TAG,
                UserFieldIndexer::Age => &NETWORK_FLAVOR_AGE_TAG,
                UserFieldIndexer::Email => &NETWORK_FLAVOR_EMAIL_TAG,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug
    for UserFieldIndex<
        ::grost::__private::flavors::network::Tag,
        ::grost::__private::flavors::Network,
    > {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::fmt::Debug::fmt(&self[()], f)
        }
    }
    #[automatically_derived]
    impl ::core::ops::Deref
    for UserFieldIndex<
        ::grost::__private::flavors::network::Tag,
        ::grost::__private::flavors::Network,
    > {
        type Target = ::grost::__private::flavors::network::Tag;
        fn deref(&self) -> &Self::Target {
            &self[()]
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Display
    for UserFieldIndex<
        ::grost::__private::flavors::network::Tag,
        ::grost::__private::flavors::Network,
    > {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::fmt::Display::fmt(&self[()], f)
        }
    }
    #[automatically_derived]
    impl ::core::ops::Index<()>
    for UserFieldIndex<
        ::grost::__private::flavors::network::WireType,
        ::grost::__private::flavors::Network,
    > {
        type Output = ::grost::__private::network::WireType;
        fn index(&self, _: ()) -> &Self::Output {
            match ::core::convert::AsRef::<UserFieldIndexer>::as_ref(self) {
                UserFieldIndexer::Name => &NETWORK_FLAVOR_NAME_WIRE_TYPE,
                UserFieldIndexer::Age => &NETWORK_FLAVOR_AGE_WIRE_TYPE,
                UserFieldIndexer::Email => &NETWORK_FLAVOR_EMAIL_WIRE_TYPE,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug
    for UserFieldIndex<
        ::grost::__private::flavors::network::WireType,
        ::grost::__private::flavors::Network,
    > {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::fmt::Debug::fmt(&self[()], f)
        }
    }
    #[automatically_derived]
    impl ::core::ops::Deref
    for UserFieldIndex<
        ::grost::__private::flavors::network::WireType,
        ::grost::__private::flavors::Network,
    > {
        type Target = ::grost::__private::flavors::network::WireType;
        fn deref(&self) -> &Self::Target {
            &self[()]
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Display
    for UserFieldIndex<
        ::grost::__private::flavors::network::WireType,
        ::grost::__private::flavors::Network,
    > {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::fmt::Display::fmt(&self[()], f)
        }
    }
    #[automatically_derived]
    impl ::core::ops::Index<()>
    for UserFieldIndex<
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
    > {
        type Output = ::grost::__private::network::Identifier;
        fn index(&self, _: ()) -> &Self::Output {
            match ::core::convert::AsRef::<UserFieldIndexer>::as_ref(self) {
                UserFieldIndexer::Name => &NETWORK_FLAVOR_NAME_IDENTIFIER,
                UserFieldIndexer::Age => &NETWORK_FLAVOR_AGE_IDENTIFIER,
                UserFieldIndexer::Email => &NETWORK_FLAVOR_EMAIL_IDENTIFIER,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug
    for UserFieldIndex<
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
    > {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::fmt::Debug::fmt(&self[()], f)
        }
    }
    #[automatically_derived]
    impl ::core::ops::Deref
    for UserFieldIndex<
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
    > {
        type Target = ::grost::__private::flavors::network::Identifier;
        fn deref(&self) -> &Self::Target {
            &self[()]
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Display
    for UserFieldIndex<
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
    > {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::fmt::Display::fmt(&self[()], f)
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
        W: ::grost::__private::WireFormat<::grost::__private::flavors::network::Network>,
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
    fn partial_encoded_name_len(
        f: &::std::string::String,
        ctx: &::grost::__private::flavors::network::Context,
        selector: &<::std::string::String as ::grost::__private::Selectable<
            ::grost::__private::flavors::network::Network,
        >>::Selector,
    ) -> ::core::primitive::usize {
        NETWORK_FLAVOR_ENCODED_NAME_IDENTIFIER_LEN
            + <::std::string::String as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::network::Network,
                <::std::string::String as ::grost::__private::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::partial_encoded_length_delimited_len(f, ctx, selector)
    }
    fn encoded_name_len(
        f: &::std::string::String,
        ctx: &::grost::__private::flavors::network::Context,
    ) -> ::core::primitive::usize {
        NETWORK_FLAVOR_ENCODED_NAME_IDENTIFIER_LEN
            + <::std::string::String as ::grost::__private::Encode<
                ::grost::__private::flavors::network::Network,
                <::std::string::String as ::grost::__private::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::encoded_length_delimited_len(f, ctx)
    }
    fn partial_encode_name(
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
        let buf_len = buf.len();
        let mut offset = 0;
        if offset > buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    partial_encoded_name_len(f, ctx, selector),
                    buf_len,
                ),
            );
        }
        buf[offset..offset + NETWORK_FLAVOR_ENCODED_NAME_IDENTIFIER_LEN]
            .copy_from_slice(NETWORK_FLAVOR_ENCODED_NAME_IDENTIFIER);
        offset += NETWORK_FLAVOR_ENCODED_NAME_IDENTIFIER_LEN;
        if offset >= buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    partial_encoded_name_len(f, ctx, selector),
                    buf_len,
                ),
            );
        }
        <::std::string::String as ::grost::__private::PartialEncode<
            ::grost::__private::flavors::network::Network,
            <::std::string::String as ::grost::__private::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format,
        >>::partial_encode_length_delimited(f, ctx, &mut buf[offset..], selector)
            .map(|len| offset + len)
            .map_err(|e| e.update(partial_encoded_name_len(f, ctx, selector), buf_len))
    }
    fn encode_name(
        f: &::std::string::String,
        ctx: &::grost::__private::flavors::network::Context,
        buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<
        ::core::primitive::usize,
        ::grost::__private::flavors::network::EncodeError,
    > {
        let buf_len = buf.len();
        let mut offset = 0;
        if offset > buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    encoded_name_len(f, ctx),
                    buf_len,
                ),
            );
        }
        buf[offset..offset + NETWORK_FLAVOR_ENCODED_NAME_IDENTIFIER_LEN]
            .copy_from_slice(NETWORK_FLAVOR_ENCODED_NAME_IDENTIFIER);
        offset += NETWORK_FLAVOR_ENCODED_NAME_IDENTIFIER_LEN;
        if offset >= buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    encoded_name_len(f, ctx),
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
            .map_err(|e| e.update(encoded_name_len(f, ctx), buf_len))
    }
    fn partial_encoded_age_len(
        f: &u32,
        ctx: &::grost::__private::flavors::network::Context,
        selector: &<u32 as ::grost::__private::Selectable<
            ::grost::__private::flavors::network::Network,
        >>::Selector,
    ) -> ::core::primitive::usize {
        NETWORK_FLAVOR_ENCODED_AGE_IDENTIFIER_LEN
            + <u32 as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::network::Network,
                <u32 as ::grost::__private::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::partial_encoded_length_delimited_len(f, ctx, selector)
    }
    fn encoded_age_len(
        f: &u32,
        ctx: &::grost::__private::flavors::network::Context,
    ) -> ::core::primitive::usize {
        NETWORK_FLAVOR_ENCODED_AGE_IDENTIFIER_LEN
            + <u32 as ::grost::__private::Encode<
                ::grost::__private::flavors::network::Network,
                <u32 as ::grost::__private::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::encoded_length_delimited_len(f, ctx)
    }
    fn partial_encode_age(
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
        let buf_len = buf.len();
        let mut offset = 0;
        if offset > buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    partial_encoded_age_len(f, ctx, selector),
                    buf_len,
                ),
            );
        }
        buf[offset..offset + NETWORK_FLAVOR_ENCODED_AGE_IDENTIFIER_LEN]
            .copy_from_slice(NETWORK_FLAVOR_ENCODED_AGE_IDENTIFIER);
        offset += NETWORK_FLAVOR_ENCODED_AGE_IDENTIFIER_LEN;
        if offset >= buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    partial_encoded_age_len(f, ctx, selector),
                    buf_len,
                ),
            );
        }
        <u32 as ::grost::__private::PartialEncode<
            ::grost::__private::flavors::network::Network,
            <u32 as ::grost::__private::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format,
        >>::partial_encode_length_delimited(f, ctx, &mut buf[offset..], selector)
            .map(|len| offset + len)
            .map_err(|e| e.update(partial_encoded_age_len(f, ctx, selector), buf_len))
    }
    fn encode_age(
        f: &u32,
        ctx: &::grost::__private::flavors::network::Context,
        buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<
        ::core::primitive::usize,
        ::grost::__private::flavors::network::EncodeError,
    > {
        let buf_len = buf.len();
        let mut offset = 0;
        if offset > buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    encoded_age_len(f, ctx),
                    buf_len,
                ),
            );
        }
        buf[offset..offset + NETWORK_FLAVOR_ENCODED_AGE_IDENTIFIER_LEN]
            .copy_from_slice(NETWORK_FLAVOR_ENCODED_AGE_IDENTIFIER);
        offset += NETWORK_FLAVOR_ENCODED_AGE_IDENTIFIER_LEN;
        if offset >= buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    encoded_age_len(f, ctx),
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
            .map_err(|e| e.update(encoded_age_len(f, ctx), buf_len))
    }
    fn partial_encoded_email_len(
        f: &::core::option::Option<::std::string::String>,
        ctx: &::grost::__private::flavors::network::Context,
        selector: &<::std::string::String as ::grost::__private::Selectable<
            ::grost::__private::flavors::network::Network,
        >>::Selector,
    ) -> ::core::primitive::usize {
        match f {
            ::core::option::Option::Some(f) => {
                NETWORK_FLAVOR_ENCODED_EMAIL_IDENTIFIER_LEN
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
    fn encoded_email_len(
        f: &::core::option::Option<::std::string::String>,
        ctx: &::grost::__private::flavors::network::Context,
    ) -> ::core::primitive::usize {
        match f {
            ::core::option::Option::Some(f) => {
                NETWORK_FLAVOR_ENCODED_EMAIL_IDENTIFIER_LEN
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
    fn partial_encode_email(
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
        match f {
            ::core::option::Option::None => ::core::result::Result::Ok(0),
            ::core::option::Option::Some(field) => {
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            partial_encoded_email_len(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + NETWORK_FLAVOR_ENCODED_EMAIL_IDENTIFIER_LEN]
                    .copy_from_slice(NETWORK_FLAVOR_ENCODED_EMAIL_IDENTIFIER);
                offset += NETWORK_FLAVOR_ENCODED_EMAIL_IDENTIFIER_LEN;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            partial_encoded_email_len(f, ctx, selector),
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
                        e.update(partial_encoded_email_len(f, ctx, selector), buf_len)
                    })
            }
        }
    }
    fn encode_email(
        f: &::core::option::Option<::std::string::String>,
        ctx: &::grost::__private::flavors::network::Context,
        buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<
        ::core::primitive::usize,
        ::grost::__private::flavors::network::EncodeError,
    > {
        match f {
            ::core::option::Option::None => ::core::result::Result::Ok(0),
            ::core::option::Option::Some(field) => {
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            encoded_email_len(f, ctx),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + NETWORK_FLAVOR_ENCODED_EMAIL_IDENTIFIER_LEN]
                    .copy_from_slice(NETWORK_FLAVOR_ENCODED_EMAIL_IDENTIFIER);
                offset += NETWORK_FLAVOR_ENCODED_EMAIL_IDENTIFIER_LEN;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            encoded_email_len(f, ctx),
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
                    .map_err(|e| e.update(encoded_email_len(f, ctx), buf_len))
            }
        }
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
                += encode_name(&self.name, ctx, &mut buf[offset..])
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
                += encode_age(&self.age, ctx, &mut buf[offset..])
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
                += encode_email(&self.email, ctx, &mut buf[offset..])
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
            len += encoded_name_len(&self.name, ctx);
            len += encoded_age_len(&self.age, ctx);
            len += encoded_email_len(&self.email, ctx);
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
                += partial_encode_name(
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
                += partial_encode_age(&self.age, ctx, &mut buf[offset..], &selector.age)
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
                += partial_encode_email(
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
            len += partial_encoded_name_len(&self.name, ctx, &selector.name);
            len += partial_encoded_age_len(&self.age, ctx, &selector.age);
            len += partial_encoded_email_len(&self.email, ctx, &selector.email);
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
                        >>::partial_encoded_length_delimited_len(self, ctx, selector),
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
    pub const fn is_selected(&self, idx: UserFieldIndexer) -> ::core::primitive::bool {
        match idx {
            UserFieldIndexer::Name => self.is_name_selected(),
            UserFieldIndexer::Age => self.is_age_selected(),
            UserFieldIndexer::Email => self.is_email_selected(),
        }
    }
    /// Returns `true` if such field is unselected.
    #[inline]
    pub const fn is_unselected(&self, idx: UserFieldIndexer) -> ::core::primitive::bool {
        match idx {
            UserFieldIndexer::Name => self.is_name_unselected(),
            UserFieldIndexer::Age => self.is_age_unselected(),
            UserFieldIndexer::Email => self.is_email_unselected(),
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
impl<F: ?::core::marker::Sized> ::grost::__private::indexer::Indexable<F> for Comment {
    type Indexer = CommentFieldIndexer;
}
/// The concrete field index for the struct [`Comment`]
pub struct CommentFieldIndex<O: ?::core::marker::Sized, F: ?::core::marker::Sized> {
    variant: CommentFieldIndexer,
    _flavor: ::core::marker::PhantomData<F>,
    _output: ::core::marker::PhantomData<O>,
}
impl<
    O: ?::core::marker::Sized,
    F: ?::core::marker::Sized,
> ::core::convert::AsRef<CommentFieldIndexer> for CommentFieldIndex<O, F> {
    fn as_ref(&self) -> &CommentFieldIndexer {
        &self.variant
    }
}
impl<O: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::clone::Clone
for CommentFieldIndex<O, F> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<O: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::marker::Copy
for CommentFieldIndex<O, F> {}
impl<O: ?::core::marker::Sized, F: ?::core::marker::Sized> CommentFieldIndex<O, F> {
    /// Create a new field index.
    #[inline]
    pub const fn new(variant: CommentFieldIndexer) -> Self {
        Self {
            variant,
            _flavor: ::core::marker::PhantomData,
            _output: ::core::marker::PhantomData,
        }
    }
    /// Returns the indexer which creates this index.
    #[inline]
    pub const fn indexer(&self) -> CommentFieldIndexer {
        self.variant
    }
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
pub enum CommentFieldIndexer {
    /// The field indexer for the field `user`
    User = 0u32,
    /// The field indexer for the field `replyer`
    Replyer = 1u32,
    /// The field indexer for the field `title`
    Title = 2u32,
    /// The field indexer for the field `content`
    Content = 3u32,
}
#[automatically_derived]
impl CommentFieldIndexer {
    /// The number of variants of this field indexer.
    pub const VARIANTS: ::core::primitive::usize = 4usize;
    /// The first field indexer.
    pub const FIRST: Self = Self::User;
    /// The last field indexer.
    pub const LAST: Self = Self::Content;
    /// Returns the field reflection index, which can be used to index the field reflection.
    #[inline]
    pub const fn field_reflection<F>(
        &self,
    ) -> CommentFieldIndex<::grost::__private::reflection::FieldReflection<F>, F>
    where
        F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
        CommentFieldIndex::new(*self)
    }
    /// Returns the tag index, which can be used to index the tag of the field.
    #[inline]
    pub const fn tag<F>(&self) -> CommentFieldIndex<F::Tag, F>
    where
        F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
        CommentFieldIndex::new(*self)
    }
    /// Returns the identifier index, which can be used to index the identifier of the field.
    #[inline]
    pub const fn identifier<F>(&self) -> CommentFieldIndex<F::Identifier, F>
    where
        F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
        CommentFieldIndex::new(*self)
    }
    /// Returns the wire type index, which can be used to index the wire type of the field.
    #[inline]
    pub const fn wire_type<F>(&self) -> CommentFieldIndex<F::WireType, F>
    where
        F: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    {
        CommentFieldIndex::new(*self)
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
#[automatically_derived]
impl ::core::iter::DoubleEndedIterator for CommentFieldIndexer {
    fn next_back(&mut self) -> ::core::option::Option<Self> {
        Self::prev(self)
    }
}
#[automatically_derived]
impl ::core::iter::FusedIterator for CommentFieldIndexer {}
#[automatically_derived]
impl ::core::iter::ExactSizeIterator for CommentFieldIndexer {
    fn len(&self) -> ::core::primitive::usize {
        self.remaining()
    }
}
#[derive(::core::fmt::Debug, ::core::clone::Clone)]
///A comment struct
pub struct Comment {
    user: User,
    replyer: ::core::option::Option<User>,
    title: ::std::string::String,
    content: ::core::option::Option<::std::string::String>,
}
impl Comment {
    /// The reflection information of the `user` field for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_USER_REFLECTION: &::grost::__private::reflection::FieldReflection<
        ::grost::__private::flavors::Network,
    > = &::grost::__private::reflection::FieldReflectionBuilder::<
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
    const __NETWORK_FLAVOR_USER_REFLECTION_OPTIONAL__: ::core::option::Option<
        &::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >,
    > = ::core::option::Option::Some(Self::NETWORK_FLAVOR_USER_REFLECTION);
    /// The reflection information of the `replyer` field for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_REPLYER_REFLECTION: &::grost::__private::reflection::FieldReflection<
        ::grost::__private::flavors::Network,
    > = &::grost::__private::reflection::FieldReflectionBuilder::<
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
            >::Struct(<User>::NETWORK_FLAVOR_REFLECTION),
        ),
    }
        .build();
    const __NETWORK_FLAVOR_REPLYER_REFLECTION_OPTIONAL__: ::core::option::Option<
        &::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >,
    > = ::core::option::Option::Some(Self::NETWORK_FLAVOR_REPLYER_REFLECTION);
    /// The reflection information of the `title` field for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_TITLE_REFLECTION: &::grost::__private::reflection::FieldReflection<
        ::grost::__private::flavors::Network,
    > = &::grost::__private::reflection::FieldReflectionBuilder::<
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
    const __NETWORK_FLAVOR_TITLE_REFLECTION_OPTIONAL__: ::core::option::Option<
        &::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >,
    > = ::core::option::Option::Some(Self::NETWORK_FLAVOR_TITLE_REFLECTION);
    /// The reflection information of the `content` field for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_CONTENT_REFLECTION: &::grost::__private::reflection::FieldReflection<
        ::grost::__private::flavors::Network,
    > = &::grost::__private::reflection::FieldReflectionBuilder::<
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
    const __NETWORK_FLAVOR_CONTENT_REFLECTION_OPTIONAL__: ::core::option::Option<
        &::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >,
    > = ::core::option::Option::Some(Self::NETWORK_FLAVOR_CONTENT_REFLECTION);
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
            Self::NETWORK_FLAVOR_REPLYER_REFLECTION,
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
            replyer: ::core::default::Default::default(),
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
    /// Gets the reference of the field `replyer`.
    #[inline]
    pub fn replyer(&self) -> ::core::option::Option<&User> {
        ::core::option::Option::as_ref(&self.replyer)
    }
    /// Gets the mutable reference of the field `replyer`.
    #[inline]
    pub fn replyer_mut(&mut self) -> ::core::option::Option<&mut User> {
        ::core::option::Option::as_mut(&mut self.replyer)
    }
    /// Sets the `replyer`.
    #[inline]
    pub fn set_replyer(&mut self, replyer: ::core::option::Option<User>) -> &mut Self {
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
const _: () = {
    const NETWORK_FLAVOR_USER_IDENTIFIER: ::grost::__private::flavors::network::Identifier = Comment::NETWORK_FLAVOR_USER_REFLECTION
        .identifier();
    const NETWORK_FLAVOR_USER_TAG: ::grost::__private::flavors::network::Tag = NETWORK_FLAVOR_USER_IDENTIFIER
        .tag();
    const NETWORK_FLAVOR_USER_WIRE_TYPE: ::grost::__private::flavors::network::WireType = NETWORK_FLAVOR_USER_IDENTIFIER
        .wire_type();
    const NETWORK_FLAVOR_ENCODED_USER_IDENTIFIER_LEN: ::core::primitive::usize = NETWORK_FLAVOR_USER_IDENTIFIER
        .encoded_len();
    const NETWORK_FLAVOR_ENCODED_USER_IDENTIFIER: &[::core::primitive::u8] = NETWORK_FLAVOR_USER_IDENTIFIER
        .encode()
        .as_slice();
    const NETWORK_FLAVOR_REPLYER_IDENTIFIER: ::grost::__private::flavors::network::Identifier = Comment::NETWORK_FLAVOR_REPLYER_REFLECTION
        .identifier();
    const NETWORK_FLAVOR_REPLYER_TAG: ::grost::__private::flavors::network::Tag = NETWORK_FLAVOR_REPLYER_IDENTIFIER
        .tag();
    const NETWORK_FLAVOR_REPLYER_WIRE_TYPE: ::grost::__private::flavors::network::WireType = NETWORK_FLAVOR_REPLYER_IDENTIFIER
        .wire_type();
    const NETWORK_FLAVOR_ENCODED_REPLYER_IDENTIFIER_LEN: ::core::primitive::usize = NETWORK_FLAVOR_REPLYER_IDENTIFIER
        .encoded_len();
    const NETWORK_FLAVOR_ENCODED_REPLYER_IDENTIFIER: &[::core::primitive::u8] = NETWORK_FLAVOR_REPLYER_IDENTIFIER
        .encode()
        .as_slice();
    const NETWORK_FLAVOR_TITLE_IDENTIFIER: ::grost::__private::flavors::network::Identifier = Comment::NETWORK_FLAVOR_TITLE_REFLECTION
        .identifier();
    const NETWORK_FLAVOR_TITLE_TAG: ::grost::__private::flavors::network::Tag = NETWORK_FLAVOR_TITLE_IDENTIFIER
        .tag();
    const NETWORK_FLAVOR_TITLE_WIRE_TYPE: ::grost::__private::flavors::network::WireType = NETWORK_FLAVOR_TITLE_IDENTIFIER
        .wire_type();
    const NETWORK_FLAVOR_ENCODED_TITLE_IDENTIFIER_LEN: ::core::primitive::usize = NETWORK_FLAVOR_TITLE_IDENTIFIER
        .encoded_len();
    const NETWORK_FLAVOR_ENCODED_TITLE_IDENTIFIER: &[::core::primitive::u8] = NETWORK_FLAVOR_TITLE_IDENTIFIER
        .encode()
        .as_slice();
    const NETWORK_FLAVOR_CONTENT_IDENTIFIER: ::grost::__private::flavors::network::Identifier = Comment::NETWORK_FLAVOR_CONTENT_REFLECTION
        .identifier();
    const NETWORK_FLAVOR_CONTENT_TAG: ::grost::__private::flavors::network::Tag = NETWORK_FLAVOR_CONTENT_IDENTIFIER
        .tag();
    const NETWORK_FLAVOR_CONTENT_WIRE_TYPE: ::grost::__private::flavors::network::WireType = NETWORK_FLAVOR_CONTENT_IDENTIFIER
        .wire_type();
    const NETWORK_FLAVOR_ENCODED_CONTENT_IDENTIFIER_LEN: ::core::primitive::usize = NETWORK_FLAVOR_CONTENT_IDENTIFIER
        .encoded_len();
    const NETWORK_FLAVOR_ENCODED_CONTENT_IDENTIFIER: &[::core::primitive::u8] = NETWORK_FLAVOR_CONTENT_IDENTIFIER
        .encode()
        .as_slice();
    #[automatically_derived]
    impl ::core::ops::Index<(CommentFieldIndexer, ::core::primitive::bool)>
    for CommentSelector<::grost::__private::flavors::Network> {
        type Output = ::core::option::Option<
            &'static ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >,
        >;
        fn index(
            &self,
            (indexer, select): (CommentFieldIndexer, ::core::primitive::bool),
        ) -> &Self::Output {
            const NONE: &::core::option::Option<
                &'static ::grost::__private::reflection::FieldReflection<
                    ::grost::__private::flavors::Network,
                >,
            > = &::core::option::Option::None;
            match indexer {
                CommentFieldIndexer::User => {
                    match (select, self.user.is_empty()) {
                        (true, false) => {
                            &Comment::__NETWORK_FLAVOR_USER_REFLECTION_OPTIONAL__
                        }
                        (true, true) => NONE,
                        (false, false) => NONE,
                        (false, true) => {
                            &Comment::__NETWORK_FLAVOR_USER_REFLECTION_OPTIONAL__
                        }
                    }
                }
                CommentFieldIndexer::Replyer => {
                    match (select, self.replyer.is_empty()) {
                        (true, false) => {
                            &Comment::__NETWORK_FLAVOR_REPLYER_REFLECTION_OPTIONAL__
                        }
                        (true, true) => NONE,
                        (false, false) => NONE,
                        (false, true) => {
                            &Comment::__NETWORK_FLAVOR_REPLYER_REFLECTION_OPTIONAL__
                        }
                    }
                }
                CommentFieldIndexer::Title => {
                    match (select, self.title) {
                        (true, true) => {
                            &Comment::__NETWORK_FLAVOR_TITLE_REFLECTION_OPTIONAL__
                        }
                        (true, false) => NONE,
                        (false, true) => NONE,
                        (false, false) => {
                            &Comment::__NETWORK_FLAVOR_TITLE_REFLECTION_OPTIONAL__
                        }
                    }
                }
                CommentFieldIndexer::Content => {
                    match (select, self.content) {
                        (true, true) => {
                            &Comment::__NETWORK_FLAVOR_CONTENT_REFLECTION_OPTIONAL__
                        }
                        (true, false) => NONE,
                        (false, true) => NONE,
                        (false, false) => {
                            &Comment::__NETWORK_FLAVOR_CONTENT_REFLECTION_OPTIONAL__
                        }
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::ops::Index<CommentFieldIndexer>
    for CommentSelector<::grost::__private::flavors::Network> {
        type Output = ::core::option::Option<
            &'static ::grost::__private::reflection::FieldReflection<
                ::grost::__private::flavors::Network,
            >,
        >;
        fn index(&self, indexer: CommentFieldIndexer) -> &Self::Output {
            const NONE: &::core::option::Option<
                &::grost::__private::reflection::FieldReflection<
                    ::grost::__private::flavors::Network,
                >,
            > = &::core::option::Option::None;
            match indexer {
                CommentFieldIndexer::User => {
                    if self.user.is_empty() {
                        NONE
                    } else {
                        &Comment::__NETWORK_FLAVOR_USER_REFLECTION_OPTIONAL__
                    }
                }
                CommentFieldIndexer::Replyer => {
                    if self.replyer.is_empty() {
                        NONE
                    } else {
                        &Comment::__NETWORK_FLAVOR_REPLYER_REFLECTION_OPTIONAL__
                    }
                }
                CommentFieldIndexer::Title => {
                    if self.title {
                        &Comment::__NETWORK_FLAVOR_TITLE_REFLECTION_OPTIONAL__
                    } else {
                        NONE
                    }
                }
                CommentFieldIndexer::Content => {
                    if self.content {
                        &Comment::__NETWORK_FLAVOR_CONTENT_REFLECTION_OPTIONAL__
                    } else {
                        NONE
                    }
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::ops::Index<()>
    for CommentFieldIndex<
        ::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
    > {
        type Output = &'static ::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >;
        fn index(&self, indexer: ()) -> &Self::Output {
            match ::core::convert::AsRef::<CommentFieldIndexer>::as_ref(self) {
                CommentFieldIndexer::User => &Comment::NETWORK_FLAVOR_USER_REFLECTION,
                CommentFieldIndexer::Replyer => {
                    &Comment::NETWORK_FLAVOR_REPLYER_REFLECTION
                }
                CommentFieldIndexer::Title => &Comment::NETWORK_FLAVOR_TITLE_REFLECTION,
                CommentFieldIndexer::Content => {
                    &Comment::NETWORK_FLAVOR_CONTENT_REFLECTION
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::ops::Deref
    for CommentFieldIndex<
        ::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
    > {
        type Target = ::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >;
        fn deref(&self) -> &Self::Target {
            self[()]
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug
    for CommentFieldIndex<
        ::grost::__private::reflection::FieldReflection<
            ::grost::__private::flavors::Network,
        >,
        ::grost::__private::flavors::Network,
    > {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::fmt::Debug::fmt(&self[()], f)
        }
    }
    #[automatically_derived]
    impl ::core::ops::Index<()>
    for CommentFieldIndex<
        ::grost::__private::flavors::network::Tag,
        ::grost::__private::flavors::Network,
    > {
        type Output = ::grost::__private::network::Tag;
        fn index(&self, _: ()) -> &Self::Output {
            match ::core::convert::AsRef::<CommentFieldIndexer>::as_ref(self) {
                CommentFieldIndexer::User => &NETWORK_FLAVOR_USER_TAG,
                CommentFieldIndexer::Replyer => &NETWORK_FLAVOR_REPLYER_TAG,
                CommentFieldIndexer::Title => &NETWORK_FLAVOR_TITLE_TAG,
                CommentFieldIndexer::Content => &NETWORK_FLAVOR_CONTENT_TAG,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug
    for CommentFieldIndex<
        ::grost::__private::flavors::network::Tag,
        ::grost::__private::flavors::Network,
    > {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::fmt::Debug::fmt(&self[()], f)
        }
    }
    #[automatically_derived]
    impl ::core::ops::Deref
    for CommentFieldIndex<
        ::grost::__private::flavors::network::Tag,
        ::grost::__private::flavors::Network,
    > {
        type Target = ::grost::__private::flavors::network::Tag;
        fn deref(&self) -> &Self::Target {
            &self[()]
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Display
    for CommentFieldIndex<
        ::grost::__private::flavors::network::Tag,
        ::grost::__private::flavors::Network,
    > {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::fmt::Display::fmt(&self[()], f)
        }
    }
    #[automatically_derived]
    impl ::core::ops::Index<()>
    for CommentFieldIndex<
        ::grost::__private::flavors::network::WireType,
        ::grost::__private::flavors::Network,
    > {
        type Output = ::grost::__private::network::WireType;
        fn index(&self, _: ()) -> &Self::Output {
            match ::core::convert::AsRef::<CommentFieldIndexer>::as_ref(self) {
                CommentFieldIndexer::User => &NETWORK_FLAVOR_USER_WIRE_TYPE,
                CommentFieldIndexer::Replyer => &NETWORK_FLAVOR_REPLYER_WIRE_TYPE,
                CommentFieldIndexer::Title => &NETWORK_FLAVOR_TITLE_WIRE_TYPE,
                CommentFieldIndexer::Content => &NETWORK_FLAVOR_CONTENT_WIRE_TYPE,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug
    for CommentFieldIndex<
        ::grost::__private::flavors::network::WireType,
        ::grost::__private::flavors::Network,
    > {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::fmt::Debug::fmt(&self[()], f)
        }
    }
    #[automatically_derived]
    impl ::core::ops::Deref
    for CommentFieldIndex<
        ::grost::__private::flavors::network::WireType,
        ::grost::__private::flavors::Network,
    > {
        type Target = ::grost::__private::flavors::network::WireType;
        fn deref(&self) -> &Self::Target {
            &self[()]
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Display
    for CommentFieldIndex<
        ::grost::__private::flavors::network::WireType,
        ::grost::__private::flavors::Network,
    > {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::fmt::Display::fmt(&self[()], f)
        }
    }
    #[automatically_derived]
    impl ::core::ops::Index<()>
    for CommentFieldIndex<
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
    > {
        type Output = ::grost::__private::network::Identifier;
        fn index(&self, _: ()) -> &Self::Output {
            match ::core::convert::AsRef::<CommentFieldIndexer>::as_ref(self) {
                CommentFieldIndexer::User => &NETWORK_FLAVOR_USER_IDENTIFIER,
                CommentFieldIndexer::Replyer => &NETWORK_FLAVOR_REPLYER_IDENTIFIER,
                CommentFieldIndexer::Title => &NETWORK_FLAVOR_TITLE_IDENTIFIER,
                CommentFieldIndexer::Content => &NETWORK_FLAVOR_CONTENT_IDENTIFIER,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug
    for CommentFieldIndex<
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
    > {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::fmt::Debug::fmt(&self[()], f)
        }
    }
    #[automatically_derived]
    impl ::core::ops::Deref
    for CommentFieldIndex<
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
    > {
        type Target = ::grost::__private::flavors::network::Identifier;
        fn deref(&self) -> &Self::Target {
            &self[()]
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Display
    for CommentFieldIndex<
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
    > {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            ::core::fmt::Display::fmt(&self[()], f)
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
        W: ::grost::__private::WireFormat<::grost::__private::flavors::network::Network>,
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
    fn partial_encoded_user_len(
        f: &User,
        ctx: &::grost::__private::flavors::network::Context,
        selector: &<User as ::grost::__private::Selectable<
            ::grost::__private::flavors::network::Network,
        >>::Selector,
    ) -> ::core::primitive::usize {
        NETWORK_FLAVOR_ENCODED_USER_IDENTIFIER_LEN
            + <User as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::network::Network,
                <User as ::grost::__private::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::partial_encoded_length_delimited_len(f, ctx, selector)
    }
    fn encoded_user_len(
        f: &User,
        ctx: &::grost::__private::flavors::network::Context,
    ) -> ::core::primitive::usize {
        NETWORK_FLAVOR_ENCODED_USER_IDENTIFIER_LEN
            + <User as ::grost::__private::Encode<
                ::grost::__private::flavors::network::Network,
                <User as ::grost::__private::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::encoded_length_delimited_len(f, ctx)
    }
    fn partial_encode_user(
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
        let buf_len = buf.len();
        let mut offset = 0;
        if offset > buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    partial_encoded_user_len(f, ctx, selector),
                    buf_len,
                ),
            );
        }
        buf[offset..offset + NETWORK_FLAVOR_ENCODED_USER_IDENTIFIER_LEN]
            .copy_from_slice(NETWORK_FLAVOR_ENCODED_USER_IDENTIFIER);
        offset += NETWORK_FLAVOR_ENCODED_USER_IDENTIFIER_LEN;
        if offset >= buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    partial_encoded_user_len(f, ctx, selector),
                    buf_len,
                ),
            );
        }
        <User as ::grost::__private::PartialEncode<
            ::grost::__private::flavors::network::Network,
            <User as ::grost::__private::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format,
        >>::partial_encode_length_delimited(f, ctx, &mut buf[offset..], selector)
            .map(|len| offset + len)
            .map_err(|e| e.update(partial_encoded_user_len(f, ctx, selector), buf_len))
    }
    fn encode_user(
        f: &User,
        ctx: &::grost::__private::flavors::network::Context,
        buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<
        ::core::primitive::usize,
        ::grost::__private::flavors::network::EncodeError,
    > {
        let buf_len = buf.len();
        let mut offset = 0;
        if offset > buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    encoded_user_len(f, ctx),
                    buf_len,
                ),
            );
        }
        buf[offset..offset + NETWORK_FLAVOR_ENCODED_USER_IDENTIFIER_LEN]
            .copy_from_slice(NETWORK_FLAVOR_ENCODED_USER_IDENTIFIER);
        offset += NETWORK_FLAVOR_ENCODED_USER_IDENTIFIER_LEN;
        if offset >= buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    encoded_user_len(f, ctx),
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
            .map_err(|e| e.update(encoded_user_len(f, ctx), buf_len))
    }
    fn partial_encoded_replyer_len(
        f: &::core::option::Option<User>,
        ctx: &::grost::__private::flavors::network::Context,
        selector: &<User as ::grost::__private::Selectable<
            ::grost::__private::flavors::network::Network,
        >>::Selector,
    ) -> ::core::primitive::usize {
        match f {
            ::core::option::Option::Some(f) => {
                NETWORK_FLAVOR_ENCODED_REPLYER_IDENTIFIER_LEN
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
    fn encoded_replyer_len(
        f: &::core::option::Option<User>,
        ctx: &::grost::__private::flavors::network::Context,
    ) -> ::core::primitive::usize {
        match f {
            ::core::option::Option::Some(f) => {
                NETWORK_FLAVOR_ENCODED_REPLYER_IDENTIFIER_LEN
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
    fn partial_encode_replyer(
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
        match f {
            ::core::option::Option::None => ::core::result::Result::Ok(0),
            ::core::option::Option::Some(field) => {
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            partial_encoded_replyer_len(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + NETWORK_FLAVOR_ENCODED_REPLYER_IDENTIFIER_LEN]
                    .copy_from_slice(NETWORK_FLAVOR_ENCODED_REPLYER_IDENTIFIER);
                offset += NETWORK_FLAVOR_ENCODED_REPLYER_IDENTIFIER_LEN;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            partial_encoded_replyer_len(f, ctx, selector),
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
                        e.update(partial_encoded_replyer_len(f, ctx, selector), buf_len)
                    })
            }
        }
    }
    fn encode_replyer(
        f: &::core::option::Option<User>,
        ctx: &::grost::__private::flavors::network::Context,
        buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<
        ::core::primitive::usize,
        ::grost::__private::flavors::network::EncodeError,
    > {
        match f {
            ::core::option::Option::None => ::core::result::Result::Ok(0),
            ::core::option::Option::Some(field) => {
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            encoded_replyer_len(f, ctx),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + NETWORK_FLAVOR_ENCODED_REPLYER_IDENTIFIER_LEN]
                    .copy_from_slice(NETWORK_FLAVOR_ENCODED_REPLYER_IDENTIFIER);
                offset += NETWORK_FLAVOR_ENCODED_REPLYER_IDENTIFIER_LEN;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            encoded_replyer_len(f, ctx),
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
                    .map_err(|e| e.update(encoded_replyer_len(f, ctx), buf_len))
            }
        }
    }
    fn partial_encoded_title_len(
        f: &::std::string::String,
        ctx: &::grost::__private::flavors::network::Context,
        selector: &<::std::string::String as ::grost::__private::Selectable<
            ::grost::__private::flavors::network::Network,
        >>::Selector,
    ) -> ::core::primitive::usize {
        NETWORK_FLAVOR_ENCODED_TITLE_IDENTIFIER_LEN
            + <::std::string::String as ::grost::__private::PartialEncode<
                ::grost::__private::flavors::network::Network,
                <::std::string::String as ::grost::__private::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::partial_encoded_length_delimited_len(f, ctx, selector)
    }
    fn encoded_title_len(
        f: &::std::string::String,
        ctx: &::grost::__private::flavors::network::Context,
    ) -> ::core::primitive::usize {
        NETWORK_FLAVOR_ENCODED_TITLE_IDENTIFIER_LEN
            + <::std::string::String as ::grost::__private::Encode<
                ::grost::__private::flavors::network::Network,
                <::std::string::String as ::grost::__private::DefaultWireFormat<
                    ::grost::__private::flavors::Network,
                >>::Format,
            >>::encoded_length_delimited_len(f, ctx)
    }
    fn partial_encode_title(
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
        let buf_len = buf.len();
        let mut offset = 0;
        if offset > buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    partial_encoded_title_len(f, ctx, selector),
                    buf_len,
                ),
            );
        }
        buf[offset..offset + NETWORK_FLAVOR_ENCODED_TITLE_IDENTIFIER_LEN]
            .copy_from_slice(NETWORK_FLAVOR_ENCODED_TITLE_IDENTIFIER);
        offset += NETWORK_FLAVOR_ENCODED_TITLE_IDENTIFIER_LEN;
        if offset >= buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    partial_encoded_title_len(f, ctx, selector),
                    buf_len,
                ),
            );
        }
        <::std::string::String as ::grost::__private::PartialEncode<
            ::grost::__private::flavors::network::Network,
            <::std::string::String as ::grost::__private::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format,
        >>::partial_encode_length_delimited(f, ctx, &mut buf[offset..], selector)
            .map(|len| offset + len)
            .map_err(|e| e.update(partial_encoded_title_len(f, ctx, selector), buf_len))
    }
    fn encode_title(
        f: &::std::string::String,
        ctx: &::grost::__private::flavors::network::Context,
        buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<
        ::core::primitive::usize,
        ::grost::__private::flavors::network::EncodeError,
    > {
        let buf_len = buf.len();
        let mut offset = 0;
        if offset > buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    encoded_title_len(f, ctx),
                    buf_len,
                ),
            );
        }
        buf[offset..offset + NETWORK_FLAVOR_ENCODED_TITLE_IDENTIFIER_LEN]
            .copy_from_slice(NETWORK_FLAVOR_ENCODED_TITLE_IDENTIFIER);
        offset += NETWORK_FLAVOR_ENCODED_TITLE_IDENTIFIER_LEN;
        if offset >= buf_len {
            return ::core::result::Result::Err(
                ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    encoded_title_len(f, ctx),
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
            .map_err(|e| e.update(encoded_title_len(f, ctx), buf_len))
    }
    fn partial_encoded_content_len(
        f: &::core::option::Option<::std::string::String>,
        ctx: &::grost::__private::flavors::network::Context,
        selector: &<::std::string::String as ::grost::__private::Selectable<
            ::grost::__private::flavors::network::Network,
        >>::Selector,
    ) -> ::core::primitive::usize {
        match f {
            ::core::option::Option::Some(f) => {
                NETWORK_FLAVOR_ENCODED_CONTENT_IDENTIFIER_LEN
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
    fn encoded_content_len(
        f: &::core::option::Option<::std::string::String>,
        ctx: &::grost::__private::flavors::network::Context,
    ) -> ::core::primitive::usize {
        match f {
            ::core::option::Option::Some(f) => {
                NETWORK_FLAVOR_ENCODED_CONTENT_IDENTIFIER_LEN
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
    fn partial_encode_content(
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
        match f {
            ::core::option::Option::None => ::core::result::Result::Ok(0),
            ::core::option::Option::Some(field) => {
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            partial_encoded_content_len(f, ctx, selector),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + NETWORK_FLAVOR_ENCODED_CONTENT_IDENTIFIER_LEN]
                    .copy_from_slice(NETWORK_FLAVOR_ENCODED_CONTENT_IDENTIFIER);
                offset += NETWORK_FLAVOR_ENCODED_CONTENT_IDENTIFIER_LEN;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            partial_encoded_content_len(f, ctx, selector),
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
                        e.update(partial_encoded_content_len(f, ctx, selector), buf_len)
                    })
            }
        }
    }
    fn encode_content(
        f: &::core::option::Option<::std::string::String>,
        ctx: &::grost::__private::flavors::network::Context,
        buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<
        ::core::primitive::usize,
        ::grost::__private::flavors::network::EncodeError,
    > {
        match f {
            ::core::option::Option::None => ::core::result::Result::Ok(0),
            ::core::option::Option::Some(field) => {
                let buf_len = buf.len();
                let mut offset = 0;
                if offset > buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            encoded_content_len(f, ctx),
                            buf_len,
                        ),
                    );
                }
                buf[offset..offset + NETWORK_FLAVOR_ENCODED_CONTENT_IDENTIFIER_LEN]
                    .copy_from_slice(NETWORK_FLAVOR_ENCODED_CONTENT_IDENTIFIER);
                offset += NETWORK_FLAVOR_ENCODED_CONTENT_IDENTIFIER_LEN;
                if offset >= buf_len {
                    return ::core::result::Result::Err(
                        ::grost::__private::flavors::network::EncodeError::insufficient_buffer(
                            encoded_content_len(f, ctx),
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
                    .map_err(|e| e.update(encoded_content_len(f, ctx), buf_len))
            }
        }
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
                += encode_user(&self.user, ctx, &mut buf[offset..])
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
                += encode_replyer(&self.replyer, ctx, &mut buf[offset..])
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
                += encode_title(&self.title, ctx, &mut buf[offset..])
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
                += encode_content(&self.content, ctx, &mut buf[offset..])
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
            len += encoded_user_len(&self.user, ctx);
            len += encoded_replyer_len(&self.replyer, ctx);
            len += encoded_title_len(&self.title, ctx);
            len += encoded_content_len(&self.content, ctx);
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
                += partial_encode_user(
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
                += partial_encode_replyer(
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
                += partial_encode_title(
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
                += partial_encode_content(
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
            len += partial_encoded_user_len(&self.user, ctx, &selector.user);
            len += partial_encoded_replyer_len(&self.replyer, ctx, &selector.replyer);
            len += partial_encoded_title_len(&self.title, ctx, &selector.title);
            len += partial_encoded_content_len(&self.content, ctx, &selector.content);
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
                        >>::partial_encoded_length_delimited_len(self, ctx, selector),
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
    replyer: <::core::option::Option<
        User,
    > as ::grost::__private::Selectable<F>>::Selector,
    title: ::core::primitive::bool,
    content: ::core::primitive::bool,
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
impl<F: ?::core::marker::Sized> ::grost::__private::Selector<F> for CommentSelector<F> {
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
        self.user.is_empty() && self.replyer.is_empty() && !self.title && !self.content
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
        idx: CommentFieldIndexer,
    ) -> ::core::primitive::bool {
        match idx {
            CommentFieldIndexer::User => self.is_user_selected(),
            CommentFieldIndexer::Replyer => self.is_replyer_selected(),
            CommentFieldIndexer::Title => self.is_title_selected(),
            CommentFieldIndexer::Content => self.is_content_selected(),
        }
    }
    /// Returns `true` if such field is unselected.
    #[inline]
    pub const fn is_unselected(
        &self,
        idx: CommentFieldIndexer,
    ) -> ::core::primitive::bool {
        match idx {
            CommentFieldIndexer::User => self.is_user_unselected(),
            CommentFieldIndexer::Replyer => self.is_replyer_unselected(),
            CommentFieldIndexer::Title => self.is_title_unselected(),
            CommentFieldIndexer::Content => self.is_content_unselected(),
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
    ) -> &<::core::option::Option<User> as ::grost::__private::Selectable<F>>::Selector {
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
