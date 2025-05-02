#![no_implicit_prelude]

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
///A user struct
pub struct User {
    name: ::std::string::String,
    age: u32,
    email: ::core::option::Option<::std::string::String>,
}
impl User {
    /// The reflection information of the `name` field for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_NAME_REFLECTION: ::grost::__private::reflection::FieldRelection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::FieldRelectionBuilder::<
        ::grost::__private::flavors::Network,
    > {
        identifier: ::grost::__private::flavors::network::Identifier::new(
            <<::std::string::String as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::WIRE_TYPE,
            ::grost::__private::Tag::new(1u32),
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
    pub const NETWORK_FLAVOR_AGE_REFLECTION: ::grost::__private::reflection::FieldRelection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::FieldRelectionBuilder::<
        ::grost::__private::flavors::Network,
    > {
        identifier: ::grost::__private::flavors::network::Identifier::new(
            <<u32 as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Network,
            >>::Format as ::grost::__private::flavors::WireFormat<
                ::grost::__private::flavors::Network,
            >>::WIRE_TYPE,
            ::grost::__private::Tag::new(2u32),
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
    pub const NETWORK_FLAVOR_EMAIL_REFLECTION: ::grost::__private::reflection::FieldRelection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::FieldRelectionBuilder::<
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
            ::grost::__private::Tag::new(3u32),
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
/// The selection type for User
#[derive(
    ::core::fmt::Debug,
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::hash::Hash,
)]
pub struct UserSelection {
    name: ::core::primitive::bool,
    age: ::core::primitive::bool,
    email: ::core::primitive::bool,
}
impl ::grost::__private::Selectable for User {
    type Selector = UserSelection;
}
impl ::grost::__private::Selector for UserSelection {
    const ALL: Self = Self::all();
    const NONE: Self = Self::empty();
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
impl UserSelection {
    /// The number of options in this selection type.
    pub const OPTIONS: ::core::primitive::usize = 3usize;
    /// Returns a selector which selects nothing.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            name: false,
            age: false,
            email: false,
        }
    }
    /// Returns a selector which selects all.
    #[inline]
    pub const fn all() -> Self {
        Self {
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
const _: () = {
    const ALL_TAG: ::grost::__private::Tag = ::grost::__private::Tag::new(1);
    const NONE_TAG: ::grost::__private::Tag = ::grost::__private::Tag::new(2);
    const SELECT_TAG: ::grost::__private::Tag = ::grost::__private::Tag::new(3);
    const UNSELECT_TAG: ::grost::__private::Tag = ::grost::__private::Tag::new(4);
    const SELECT_ONE_TAG: ::grost::__private::Tag = ::grost::__private::Tag::new(5);
    const UNSELECT_ONE_TAG: ::grost::__private::Tag = ::grost::__private::Tag::new(6);
    const ALL_IDENTIFIER: ::grost::__private::flavors::network::Identifier = ::grost::__private::flavors::network::Identifier::new(
        ::grost::__private::flavors::network::WireType::Zst,
        ALL_TAG,
    );
    const NONE_IDENTIFIER: ::grost::__private::flavors::network::Identifier = ::grost::__private::flavors::network::Identifier::new(
        ::grost::__private::flavors::network::WireType::Zst,
        NONE_TAG,
    );
    const SELECT_IDENTIFIER: ::grost::__private::flavors::network::Identifier = ::grost::__private::flavors::network::Identifier::new(
        ::grost::__private::flavors::network::WireType::LengthDelimited,
        SELECT_TAG,
    );
    const UNSELECT_IDENTIFIER: ::grost::__private::flavors::network::Identifier = ::grost::__private::flavors::network::Identifier::new(
        ::grost::__private::flavors::network::WireType::LengthDelimited,
        UNSELECT_TAG,
    );
    const SELECT_ONE_IDENTIFIER: ::grost::__private::flavors::network::Identifier = ::grost::__private::flavors::network::Identifier::new(
        ::grost::__private::flavors::network::WireType::Varint,
        SELECT_ONE_TAG,
    );
    const UNSELECT_ONE_IDENTIFIER: ::grost::__private::flavors::network::Identifier = ::grost::__private::flavors::network::Identifier::new(
        ::grost::__private::flavors::network::WireType::Varint,
        UNSELECT_ONE_TAG,
    );
    const ALL_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize = ALL_IDENTIFIER
        .encoded_len();
    const NONE_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize = NONE_IDENTIFIER
        .encoded_len();
    const SELECT_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize = SELECT_IDENTIFIER
        .encoded_len();
    const UNSELECT_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize = UNSELECT_IDENTIFIER
        .encoded_len();
    const SELECT_ONE_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize = SELECT_ONE_IDENTIFIER
        .encoded_len();
    const UNSELECT_ONE_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize = UNSELECT_ONE_IDENTIFIER
        .encoded_len();
    const ALL_IDENTIFIER_ENCODED: &[::core::primitive::u8] = ALL_IDENTIFIER
        .encode()
        .as_slice();
    const NONE_IDENTIFIER_ENCODED: &[::core::primitive::u8] = NONE_IDENTIFIER
        .encode()
        .as_slice();
    const SELECT_IDENTIFIER_ENCODED: &[::core::primitive::u8] = SELECT_IDENTIFIER
        .encode()
        .as_slice();
    const UNSELECT_IDENTIFIER_ENCODED: &[::core::primitive::u8] = UNSELECT_IDENTIFIER
        .encode()
        .as_slice();
    const SELECT_ONE_IDENTIFIER_ENCODED: &[::core::primitive::u8] = SELECT_ONE_IDENTIFIER
        .encode()
        .as_slice();
    const UNSELECT_ONE_IDENTIFIER_ENCODED: &[::core::primitive::u8] = UNSELECT_ONE_IDENTIFIER
        .encode()
        .as_slice();
    impl ::grost::__private::DefaultWireFormat<::grost::__private::flavors::Network>
    for UserSelection {
        type Format = ::grost::__private::flavors::network::LengthDelimited;
    }
    ::grost::__private::selectable_scalar!(UserSelection);
    ::grost::__private::partial_encode_scalar!(
        ::grost::__private::flavors::Network : UserSelection as
        ::grost::__private::flavors::network::LengthDelimited
    );
    ::grost::__private::decode_owned_scalar!(
        ::grost::__private::flavors::Network : UserSelection as
        ::grost::__private::flavors::network::LengthDelimited
    );
    impl ::grost::__private::Encode<
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
    > for UserSelection {
        #[inline]
        fn encode(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::Flavor>::EncodeError,
        > {
            ::core::todo!()
        }
        #[inline]
        fn encoded_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::Flavor>::Context,
        ) -> ::core::primitive::usize {
            ::core::todo!()
        }
        #[inline]
        fn encoded_length_delimited_len(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::Flavor>::Context,
        ) -> ::core::primitive::usize {
            ::core::todo!()
        }
        #[inline]
        fn encode_length_delimited(
            &self,
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::Flavor>::Context,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            <::grost::__private::flavors::Network as ::grost::__private::Flavor>::EncodeError,
        > {
            ::core::todo!()
        }
    }
    impl<
        'de,
    > ::grost::__private::Decode<
        'de,
        ::grost::__private::flavors::Network,
        ::grost::__private::flavors::network::LengthDelimited,
        Self,
    > for UserSelection {
        #[inline]
        fn decode<UB>(
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::Flavor>::Context,
            src: &'de [::core::primitive::u8],
        ) -> ::core::result::Result<
            (::core::primitive::usize, Self),
            <::grost::__private::flavors::Network as ::grost::__private::Flavor>::DecodeError,
        >
        where
            UB: ::grost::__private::Buffer<
                    ::grost::__private::flavors::network::Unknown<
                        &'de [::core::primitive::u8],
                    >,
                > + 'de,
        {
            ::core::todo!()
        }
        #[inline]
        fn decode_length_delimited<UB>(
            ctx: &<::grost::__private::flavors::Network as ::grost::__private::Flavor>::Context,
            src: &'de [::core::primitive::u8],
        ) -> ::core::result::Result<
            (::core::primitive::usize, Self),
            <::grost::__private::flavors::Network as ::grost::__private::Flavor>::DecodeError,
        >
        where
            UB: ::grost::__private::Buffer<
                    ::grost::__private::flavors::network::Unknown<
                        &'de [::core::primitive::u8],
                    >,
                > + 'de,
        {
            ::core::todo!()
        }
    }
};
