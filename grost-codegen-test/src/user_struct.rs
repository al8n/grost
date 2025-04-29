#![no_implicit_prelude]

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
///A user struct
pub struct User {
    name: ::std::string::String,
    age: u32,
    email: ::core::option::Option<::std::string::String>,
}
impl User {
    /// The reflection information of the `name` field for [`GrostCodegenNetworkNetwork`](::grost::__private::flavors::Network) flavor.
    pub const GROST_CODEGEN_NETWORK_NETWORK_FLAVOR_NAME_REFLECTION: ::grost::__private::reflection::FieldRelection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::FieldRelectionBuilder::<
        ::grost::__private::flavors::Network,
    > {
        tag: ::grost::__private::Tag::new(1u32),
        wire_type: <::std::string::String as ::grost::__private::Wirable<
            ::grost::__private::flavors::Network,
        >>::WIRE_TYPE,
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
    /// The reflection information of the `age` field for [`GrostCodegenNetworkNetwork`](::grost::__private::flavors::Network) flavor.
    pub const GROST_CODEGEN_NETWORK_NETWORK_FLAVOR_AGE_REFLECTION: ::grost::__private::reflection::FieldRelection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::FieldRelectionBuilder::<
        ::grost::__private::flavors::Network,
    > {
        tag: ::grost::__private::Tag::new(2u32),
        wire_type: <u32 as ::grost::__private::Wirable<
            ::grost::__private::flavors::Network,
        >>::WIRE_TYPE,
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
    /// The reflection information of the `email` field for [`GrostCodegenNetworkNetwork`](::grost::__private::flavors::Network) flavor.
    pub const GROST_CODEGEN_NETWORK_NETWORK_FLAVOR_EMAIL_REFLECTION: ::grost::__private::reflection::FieldRelection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::FieldRelectionBuilder::<
        ::grost::__private::flavors::Network,
    > {
        tag: ::grost::__private::Tag::new(3u32),
        wire_type: <::core::option::Option<
            ::std::string::String,
        > as ::grost::__private::Wirable<
            ::grost::__private::flavors::Network,
        >>::WIRE_TYPE,
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
    /// The reflection of the struct `User` for [`GrostCodegenNetworkNetwork`](::grost::__private::flavors::Network) flavor.
    pub const GROST_CODEGEN_NETWORK_NETWORK_FLAVOR_REFLECTION: ::grost::__private::reflection::StructReflection<
        ::grost::__private::flavors::Network,
    > = ::grost::__private::reflection::StructReflectionBuilder::<
        ::grost::__private::flavors::Network,
    > {
        name: "User",
        schema_name: "User",
        fields: &[
            Self::GROST_CODEGEN_NETWORK_NETWORK_FLAVOR_NAME_REFLECTION,
            Self::GROST_CODEGEN_NETWORK_NETWORK_FLAVOR_AGE_REFLECTION,
            Self::GROST_CODEGEN_NETWORK_NETWORK_FLAVOR_EMAIL_REFLECTION,
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
::grost::__private::bitflags::bitflags! {
    #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy,
    ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::cmp::PartialOrd, ::core::cmp::Ord,
    ::core::hash::Hash,)] struct UserSelectionFlags : ::core::primitive::u8 { const NAME
    = 1u8; const AGE = 2u8; const EMAIL = 4u8; }
}
impl UserSelectionFlags {
    #[inline]
    const fn select_name(&mut self) -> &mut Self {
        *self = self.union(Self::NAME);
        self
    }
    #[inline]
    const fn unselect_name(&mut self) -> &mut Self {
        *self = self.difference(Self::NAME);
        self
    }
    #[inline]
    const fn update_name(&mut self, value: ::core::primitive::bool) -> &mut Self {
        if value { self.select_name() } else { self.unselect_name() }
    }
    #[inline]
    const fn toggle_name(&mut self) -> &mut Self {
        *self = self.symmetric_difference(Self::NAME);
        self
    }
    #[inline]
    const fn with_name(self) -> Self {
        self.union(Self::NAME)
    }
    #[inline]
    const fn without_name(self) -> Self {
        self.difference(Self::NAME)
    }
    #[inline]
    const fn maybe_name(self, val: ::core::primitive::bool) -> Self {
        if val { self.with_name() } else { self.without_name() }
    }
    #[inline]
    const fn contains_name(&self) -> ::core::primitive::bool {
        self.contains(Self::NAME)
    }
    #[inline]
    const fn select_age(&mut self) -> &mut Self {
        *self = self.union(Self::AGE);
        self
    }
    #[inline]
    const fn unselect_age(&mut self) -> &mut Self {
        *self = self.difference(Self::AGE);
        self
    }
    #[inline]
    const fn update_age(&mut self, value: ::core::primitive::bool) -> &mut Self {
        if value { self.select_age() } else { self.unselect_age() }
    }
    #[inline]
    const fn toggle_age(&mut self) -> &mut Self {
        *self = self.symmetric_difference(Self::AGE);
        self
    }
    #[inline]
    const fn with_age(self) -> Self {
        self.union(Self::AGE)
    }
    #[inline]
    const fn without_age(self) -> Self {
        self.difference(Self::AGE)
    }
    #[inline]
    const fn maybe_age(self, val: ::core::primitive::bool) -> Self {
        if val { self.with_age() } else { self.without_age() }
    }
    #[inline]
    const fn contains_age(&self) -> ::core::primitive::bool {
        self.contains(Self::AGE)
    }
    #[inline]
    const fn select_email(&mut self) -> &mut Self {
        *self = self.union(Self::EMAIL);
        self
    }
    #[inline]
    const fn unselect_email(&mut self) -> &mut Self {
        *self = self.difference(Self::EMAIL);
        self
    }
    #[inline]
    const fn update_email(&mut self, value: ::core::primitive::bool) -> &mut Self {
        if value { self.select_email() } else { self.unselect_email() }
    }
    #[inline]
    const fn toggle_email(&mut self) -> &mut Self {
        *self = self.symmetric_difference(Self::EMAIL);
        self
    }
    #[inline]
    const fn with_email(self) -> Self {
        self.union(Self::EMAIL)
    }
    #[inline]
    const fn without_email(self) -> Self {
        self.difference(Self::EMAIL)
    }
    #[inline]
    const fn maybe_email(self, val: ::core::primitive::bool) -> Self {
        if val { self.with_email() } else { self.without_email() }
    }
    #[inline]
    const fn contains_email(&self) -> ::core::primitive::bool {
        self.contains(Self::EMAIL)
    }
    #[inline]
    const fn merge(&self, other: Self) -> Self {
        Self(self.0.union(other.0))
    }
    #[inline]
    const fn select_field_reflection_iter_selected<F>(
        &self,
    ) -> UserSelectionFlagsIter<F, true>
    where
        F: ?::core::marker::Sized,
    {
        UserSelectionFlagsIter::new(*self)
    }
    #[inline]
    const fn select_field_reflection_iter_unselected<F>(
        &self,
    ) -> UserSelectionFlagsIter<F, false>
    where
        F: ?::core::marker::Sized,
    {
        UserSelectionFlagsIter::new(*self)
    }
    #[inline]
    const fn num_selected(&self) -> ::core::primitive::usize {
        self.bits().count_ones() as ::core::primitive::usize
    }
    #[inline]
    const fn num_unselected(&self) -> ::core::primitive::usize {
        self.bits().count_zeros() as ::core::primitive::usize
    }
    const OPTIONS: ::core::primitive::usize = 3usize;
}
/// Yield a set of selected fields.
pub struct UserSelectionFlagsIter<
    F: ?::core::marker::Sized,
    const S: ::core::primitive::bool = true,
> {
    iter: ::grost::__private::bitflags::iter::Iter<UserSelectionFlags>,
    yielded: ::core::primitive::usize,
    len: ::core::primitive::usize,
    _m: ::core::marker::PhantomData<F>,
}
impl<F, const S: ::core::primitive::bool> UserSelectionFlagsIter<F, S>
where
    F: ?::core::marker::Sized,
{
    #[inline]
    const fn new(mut selection: UserSelectionFlags) -> Self {
        if !S {
            selection = selection.complement();
        }
        Self {
            iter: selection.iter(),
            yielded: 0,
            len: selection.num_selected(),
            _m: ::core::marker::PhantomData,
        }
    }
    /// Returns the exact remaining length of the iterator.
    #[inline]
    pub const fn remaining(&self) -> ::core::primitive::usize {
        self.len - self.yielded
    }
    /// Returns `true` if the iterator is empty.
    #[inline]
    pub const fn is_empty(&self) -> ::core::primitive::bool {
        self.remaining() == 0
    }
}
impl<const S: ::core::primitive::bool> ::core::iter::Iterator
for UserSelectionFlagsIter<::grost::__private::flavors::Network, S> {
    type Item = ::grost::__private::reflection::FieldRelection<
        ::grost::__private::flavors::Network,
    >;
    #[inline]
    fn next(&mut self) -> ::core::option::Option<Self::Item> {
        for f in ::core::iter::Iterator::by_ref(&mut self.iter) {
            if let ::core::option::Option::Some(val) = User::GROST_CODEGEN_NETWORK_NETWORK_FLAVOR_REFLECTION
                .fields()
                .get(f.bits().trailing_zeros() as ::core::primitive::usize)
            {
                return ::core::option::Option::Some(*val);
            }
        }
        ::core::option::Option::None
    }
}
impl<const S: ::core::primitive::bool> ::core::iter::FusedIterator
for UserSelectionFlagsIter<::grost::__private::flavors::Network, S> {}
impl<const S: ::core::primitive::bool> ::core::iter::ExactSizeIterator
for UserSelectionFlagsIter<::grost::__private::flavors::Network, S> {
    #[inline]
    fn len(&self) -> ::core::primitive::usize {
        self.remaining()
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
    flags: UserSelectionFlags,
}
impl UserSelection {
    /// The number of options in this selection type.
    pub const OPTIONS: ::core::primitive::usize = UserSelectionFlags::OPTIONS;
    /// Get a flags value with all bits unset.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            flags: UserSelectionFlags::empty(),
        }
    }
    /// Get a flags value with all known bits set.
    #[inline]
    pub const fn all() -> Self {
        Self {
            flags: UserSelectionFlags::all(),
        }
    }
    /// Whether all bits in this flags value are unset.
    #[inline]
    pub const fn is_empty(&self) -> ::core::primitive::bool {
        self.flags.is_empty()
    }
    /// Whether all bits in this flags value are set.
    #[inline]
    pub const fn is_all(&self) -> ::core::primitive::bool {
        self.flags.is_all()
    }
    /// Returns an iterator over the selected fields, the iterator will yield the `FieldRelection` of the selected fields.
    #[inline]
    pub const fn iter_selected<F: ?::core::marker::Sized>(
        &self,
    ) -> UserSelectionFlagsIter<F, true> {
        self.flags.select_field_reflection_iter_selected()
    }
    /// Returns an iterator over the unselected fields, the iterator will yield the `FieldRelection` of the unselected fields.
    #[inline]
    pub const fn iter_unselected<F: ?::core::marker::Sized>(
        &self,
    ) -> UserSelectionFlagsIter<F, false> {
        self.flags.select_field_reflection_iter_unselected()
    }
    /// Merge another selection set into this one.
    #[inline]
    pub const fn merge(&mut self, other: Self) -> &mut Self {
        self.flags = self.flags.merge(other.flags);
        self
    }
    /// Merge another selection set into a new one.
    #[inline]
    pub const fn merge_into(mut self, other: Self) -> Self {
        self.flags = self.flags.merge(other.flags);
        self
    }
    /// Returns the number of selected fields.
    #[inline]
    pub const fn num_selected(&self) -> ::core::primitive::usize {
        self.flags.num_selected()
    }
    /// Returns the number of unselected fields.
    #[inline]
    pub const fn num_unselected(&self) -> ::core::primitive::usize {
        self.flags.num_unselected()
    }
    /// Select the `User.name` field
    #[inline]
    pub const fn select_name(&mut self) -> &mut Self {
        self.flags.select_name();
        self
    }
    /// Unselect the `User.name` field
    #[inline]
    pub const fn unselect_name(&mut self) -> &mut Self {
        self.flags.unselect_name();
        self
    }
    /// Update the `User.name` field
    #[inline]
    pub const fn update_name(&mut self, value: ::core::primitive::bool) -> &mut Self {
        self.flags.update_name(value);
        self
    }
    /// Toggle the `User.name` field
    #[inline]
    pub const fn toggle_name(&mut self) -> &mut Self {
        self.flags.toggle_name();
        self
    }
    /// Set the `User.name` field
    #[inline]
    pub const fn with_name(mut self) -> Self {
        self.flags = self.flags.with_name();
        self
    }
    /// Unset the `User.name` field
    #[inline]
    pub const fn without_name(mut self) -> Self {
        self.flags = self.flags.without_name();
        self
    }
    /// Set or unset the `User.name` field
    #[inline]
    pub const fn maybe_name(mut self, val: ::core::primitive::bool) -> Self {
        self.flags = self.flags.maybe_name(val);
        self
    }
    /// Check if the `User.name` field is set
    #[inline]
    pub const fn contains_name(&self) -> ::core::primitive::bool {
        self.flags.contains_name()
    }
    /// Select the `User.age` field
    #[inline]
    pub const fn select_age(&mut self) -> &mut Self {
        self.flags.select_age();
        self
    }
    /// Unselect the `User.age` field
    #[inline]
    pub const fn unselect_age(&mut self) -> &mut Self {
        self.flags.unselect_age();
        self
    }
    /// Update the `User.age` field
    #[inline]
    pub const fn update_age(&mut self, value: ::core::primitive::bool) -> &mut Self {
        self.flags.update_age(value);
        self
    }
    /// Toggle the `User.age` field
    #[inline]
    pub const fn toggle_age(&mut self) -> &mut Self {
        self.flags.toggle_age();
        self
    }
    /// Set the `User.age` field
    #[inline]
    pub const fn with_age(mut self) -> Self {
        self.flags = self.flags.with_age();
        self
    }
    /// Unset the `User.age` field
    #[inline]
    pub const fn without_age(mut self) -> Self {
        self.flags = self.flags.without_age();
        self
    }
    /// Set or unset the `User.age` field
    #[inline]
    pub const fn maybe_age(mut self, val: ::core::primitive::bool) -> Self {
        self.flags = self.flags.maybe_age(val);
        self
    }
    /// Check if the `User.age` field is set
    #[inline]
    pub const fn contains_age(&self) -> ::core::primitive::bool {
        self.flags.contains_age()
    }
    /// Select the `User.email` field
    #[inline]
    pub const fn select_email(&mut self) -> &mut Self {
        self.flags.select_email();
        self
    }
    /// Unselect the `User.email` field
    #[inline]
    pub const fn unselect_email(&mut self) -> &mut Self {
        self.flags.unselect_email();
        self
    }
    /// Update the `User.email` field
    #[inline]
    pub const fn update_email(&mut self, value: ::core::primitive::bool) -> &mut Self {
        self.flags.update_email(value);
        self
    }
    /// Toggle the `User.email` field
    #[inline]
    pub const fn toggle_email(&mut self) -> &mut Self {
        self.flags.toggle_email();
        self
    }
    /// Set the `User.email` field
    #[inline]
    pub const fn with_email(mut self) -> Self {
        self.flags = self.flags.with_email();
        self
    }
    /// Unset the `User.email` field
    #[inline]
    pub const fn without_email(mut self) -> Self {
        self.flags = self.flags.without_email();
        self
    }
    /// Set or unset the `User.email` field
    #[inline]
    pub const fn maybe_email(mut self, val: ::core::primitive::bool) -> Self {
        self.flags = self.flags.maybe_email(val);
        self
    }
    /// Check if the `User.email` field is set
    #[inline]
    pub const fn contains_email(&self) -> ::core::primitive::bool {
        self.flags.contains_email()
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
