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
    const ALL_TAG: ::core::primitive::u8 = 1;
    const NONE_TAG: ::core::primitive::u8 = 2;
    const SELECT_TAG: ::core::primitive::u8 = 3;
    const UNSELECT_TAG: ::core::primitive::u8 = 4;
    const SELECT_ONE_TAG: ::core::primitive::u8 = 5;
    const UNSELECT_ONE_TAG: ::core::primitive::u8 = 6;
    impl UserSelection {
        /// Encode the selection into a buffer.
        ///
        /// Returns the number of bytes written to the buffer.
        #[inline]
        pub fn encode(
            &self,
            buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<
            ::core::primitive::usize,
            ::grost::__private::EncodeError,
        > {
            use ::core::iter::Iterator;
            macro_rules! encode {
                (@ many $buf_len:ident, $fn:ident, $tag:ident) => {
                    { buf[0] = $tag; let mut offset = 1; let data_size = self. $fn :: <
                    ::grost::__private::flavors::Network > ().map(| f |
                    ::grost::__private::varing::encoded_u32_varint_len(f.tag().get()))
                    .sum:: < ::core::primitive::usize > (); let data_size_len =
                    ::grost::__private::varing::encoded_u32_varint_len(data_size as
                    ::core::primitive::u32); let total_len = offset + data_size_len +
                    data_size; if $buf_len < total_len { return
                    ::core::result::Result::Err(::grost::__private::EncodeError::insufficient_buffer(total_len,
                    $buf_len)); } offset +=
                    ::grost::__private::varing::encode_u32_varint_to(data_size as
                    ::core::primitive::u32, & mut buf[offset..]).map_err(| e |
                    ::grost::__private::EncodeError::from_varint_error(e)
                    .update(total_len, $buf_len)) ?; for tag in self. $fn :: <
                    ::grost::__private::flavors::Network > ().map(| f | f.tag().get()) {
                    offset += ::grost::__private::varing::encode_u32_varint_to(tag, & mut
                    buf[offset..]).map_err(| e |
                    ::grost::__private::EncodeError::from_varint_error(e)
                    .update(total_len, $buf_len)) ?; } ::core::result::Result::Ok(offset)
                    }
                };
                (@ single $buf_len:ident, $fn:ident, $tag:ident) => {
                    { buf[0] = $tag; let selected = self. $fn :: <
                    ::grost::__private::flavors::Network > ().next().unwrap().tag()
                    .get(); ::grost::__private::varing::encode_u32_varint_to(selected, &
                    mut buf[1..]).map_err(| e |
                    ::grost::__private::EncodeError::from_varint_error(e).update(self
                    .encoded_len(), $buf_len)) }
                };
            }
            if self.is_empty() {
                if buf.is_empty() {
                    return ::core::result::Result::Err(
                        ::grost::__private::EncodeError::insufficient_buffer(
                            1,
                            buf.len(),
                        ),
                    );
                }
                buf[0] = NONE_TAG;
                return ::core::result::Result::Ok(1);
            }
            if self.is_all() {
                if buf.is_empty() {
                    return ::core::result::Result::Err(
                        ::grost::__private::EncodeError::insufficient_buffer(
                            1,
                            buf.len(),
                        ),
                    );
                }
                buf[0] = ALL_TAG;
                return ::core::result::Result::Ok(1);
            }
            let num_selected = self.num_selected();
            let num_unselected = self.num_unselected();
            let buf_len = buf.len();
            if buf_len < 2 {
                return ::core::result::Result::Err(
                    ::grost::__private::EncodeError::insufficient_buffer(
                        self.encoded_len(),
                        buf_len,
                    ),
                );
            }
            if num_selected >= num_unselected {
                if num_selected == 1 {
                    encode! {
                        @ single buf_len, iter_selected, SELECT_ONE_TAG
                    }
                } else {
                    encode! {
                        @ many buf_len, iter_selected, SELECT_TAG
                    }
                }
            } else if num_unselected == 1 {
                encode! {
                    @ single buf_len, iter_unselected, UNSELECT_ONE_TAG
                }
            } else {
                encode! {
                    @ many buf_len, iter_unselected, UNSELECT_TAG
                }
            }
        }
        /// Returns the length of the encoded selection.
        #[inline]
        pub fn encoded_len(&self) -> ::core::primitive::usize {
            use ::core::iter::Iterator;
            macro_rules! len {
                (@ many $fn:ident) => {
                    { let data_size = self. $fn :: < ::grost::__private::flavors::Network
                    > ().map(| f | ::grost::__private::varing::encoded_u32_varint_len(f
                    .tag().get())).sum:: < ::core::primitive::usize > (); let
                    data_size_len =
                    ::grost::__private::varing::encoded_u32_varint_len(data_size as
                    ::core::primitive::u32); 1 + data_size_len + data_size }
                };
                (@ single $fn:ident) => {
                    { let selected = self. $fn :: < ::grost::__private::flavors::Network
                    > ().next().unwrap().tag().get(); 1 +
                    ::grost::__private::varing::encoded_u32_varint_len(selected) }
                };
            }
            if self.is_empty() {
                return 1;
            }
            if self.is_all() {
                return 1;
            }
            let num_selected = self.num_selected();
            let num_unselected = self.num_unselected();
            if num_selected >= num_unselected {
                if num_selected == 1 {
                    len! {
                        @ single iter_selected
                    }
                } else {
                    len! {
                        @ many iter_selected
                    }
                }
            } else if num_unselected == 1 {
                len! {
                    @ single iter_unselected
                }
            } else {
                len! {
                    @ many iter_unselected
                }
            }
        }
        /// Decodes the selection from a buffer.
        pub fn decode<'a, F, UB>(
            src: &'a [u8],
        ) -> ::core::result::Result<
            (::core::primitive::usize, ::grost::__private::SelectionSet<Self, UB>),
            ::grost::__private::DecodeError<F>,
        >
        where
            F: ::grost::__private::Flavor + ?::core::marker::Sized,
            UB: ::grost::__private::Buffer<
                    ::grost::__private::Unknown<F, &'a [::core::primitive::u8]>,
                > + 'a,
        {
            if src.is_empty() {
                return ::core::result::Result::Err(
                    ::grost::__private::DecodeError::buffer_underflow(),
                );
            }
            let tag = src[0];
            match tag {
                NONE_TAG => {
                    ::core::result::Result::Ok((
                        1,
                        ::grost::__private::SelectionSet::new(
                            Self::empty(),
                            ::core::option::Option::None,
                        ),
                    ))
                }
                ALL_TAG => {
                    ::core::result::Result::Ok((
                        1,
                        ::grost::__private::SelectionSet::new(
                            Self::all(),
                            ::core::option::Option::None,
                        ),
                    ))
                }
                SELECT_TAG => {
                    let (read, data_size) = ::grost::__private::varing::decode_u32_varint(
                        &src[1..],
                    )?;
                    let mut offset = 1 + read;
                    let total = offset + data_size as usize;
                    if total > src.len() {
                        return ::core::result::Result::Err(
                            ::grost::__private::DecodeError::buffer_underflow(),
                        );
                    }
                    let mut selection = Self::empty();
                    while offset < total {
                        let (read, tag) = ::grost::__private::varing::decode_u32_varint(
                            &src[offset..],
                        )?;
                        offset += read;
                    }
                    ::core::result::Result::Ok((
                        total,
                        ::grost::__private::SelectionSet::new(
                            selection,
                            ::core::option::Option::None,
                        ),
                    ))
                }
                UNSELECT_TAG => {
                    let (read, data_size) = ::grost::__private::varing::decode_u32_varint(
                        &src[1..],
                    )?;
                    let mut offset = 1 + read;
                    if offset + data_size as usize > src.len() {
                        return ::core::result::Result::Err(
                            ::grost::__private::DecodeError::buffer_underflow(),
                        );
                    }
                    ::core::todo!()
                }
                SELECT_ONE_TAG => {
                    let (read, tag) = ::grost::__private::varing::decode_u32_varint(
                        &src[1..],
                    )?;
                    ::core::todo!()
                }
                UNSELECT_ONE_TAG => {
                    let (read, tag) = ::grost::__private::varing::decode_u32_varint(
                        &src[1..],
                    )?;
                    ::core::todo!()
                }
                _ => {}
            }
        }
    }
};
