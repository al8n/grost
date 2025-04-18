#[derive(::core::fmt::Debug, ::core::clone::Clone)]
///A user struct
pub struct User {
    name: String,
    age: u32,
    email: Option<String>,
}
impl ::core::default::Default for User {
    fn default() -> Self {
        Self::new()
    }
}
impl User {
    const __NAME_TAG__: ::grost::__private::Tag = ::grost::__private::Tag::new(1u32);
    const __NAME_IDENTIFIER__: ::grost::__private::Identifier = ::grost::__private::Identifier::new(
        ::grost::__private::WireType::LengthDelimited,
        Self::__NAME_TAG__,
    );
    const __NAME_IDENTIFIER_ENCODED_LEN__: ::core::primitive::usize = Self::__NAME_IDENTIFIER__
        .encoded_len();
    const __ENCODED_NAME_IDENTIFIER__: &[::core::primitive::u8] = Self::__NAME_IDENTIFIER__
        .encode()
        .as_slice();
    /// The reflection information of the `name` field
    pub const NAME_REFLECTION: ::grost::__private::FieldRelection = ::grost::__private::FieldRelectionBuilder {
        identifier: Self::__NAME_IDENTIFIER__,
        encoded_identifier_len: Self::__NAME_IDENTIFIER_ENCODED_LEN__,
        encoded_identifier: Self::__ENCODED_NAME_IDENTIFIER__,
        name: "name",
        ty: "String",
        schema_name: "name",
        schema_type: "String!",
    }
        .build();
    const __AGE_TAG__: ::grost::__private::Tag = ::grost::__private::Tag::new(2u32);
    const __AGE_IDENTIFIER__: ::grost::__private::Identifier = ::grost::__private::Identifier::new(
        ::grost::__private::WireType::Varint,
        Self::__AGE_TAG__,
    );
    const __AGE_IDENTIFIER_ENCODED_LEN__: ::core::primitive::usize = Self::__AGE_IDENTIFIER__
        .encoded_len();
    const __ENCODED_AGE_IDENTIFIER__: &[::core::primitive::u8] = Self::__AGE_IDENTIFIER__
        .encode()
        .as_slice();
    /// The reflection information of the `age` field
    pub const AGE_REFLECTION: ::grost::__private::FieldRelection = ::grost::__private::FieldRelectionBuilder {
        identifier: Self::__AGE_IDENTIFIER__,
        encoded_identifier_len: Self::__AGE_IDENTIFIER_ENCODED_LEN__,
        encoded_identifier: Self::__ENCODED_AGE_IDENTIFIER__,
        name: "age",
        ty: "u32",
        schema_name: "age",
        schema_type: "u32!",
    }
        .build();
    const __EMAIL_TAG__: ::grost::__private::Tag = ::grost::__private::Tag::new(3u32);
    const __EMAIL_IDENTIFIER__: ::grost::__private::Identifier = ::grost::__private::Identifier::new(
        ::grost::__private::WireType::LengthDelimited,
        Self::__EMAIL_TAG__,
    );
    const __EMAIL_IDENTIFIER_ENCODED_LEN__: ::core::primitive::usize = Self::__EMAIL_IDENTIFIER__
        .encoded_len();
    const __ENCODED_EMAIL_IDENTIFIER__: &[::core::primitive::u8] = Self::__EMAIL_IDENTIFIER__
        .encode()
        .as_slice();
    /// The reflection information of the `email` field
    pub const EMAIL_REFLECTION: ::grost::__private::FieldRelection = ::grost::__private::FieldRelectionBuilder {
        identifier: Self::__EMAIL_IDENTIFIER__,
        encoded_identifier_len: Self::__EMAIL_IDENTIFIER_ENCODED_LEN__,
        encoded_identifier: Self::__ENCODED_EMAIL_IDENTIFIER__,
        name: "email",
        ty: "Option<String>",
        schema_name: "email",
        schema_type: "String",
    }
        .build();
    /// The reflection of the struct
    pub const REFLECTION: ::grost::__private::StructReflection = ::grost::__private::StructReflectionBuilder {
        name: "User",
        schema_name: "User",
        fields: &[Self::NAME_REFLECTION, Self::AGE_REFLECTION, Self::EMAIL_REFLECTION],
    }
        .build();
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
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Gets the mutable reference of the field `name`.
    #[inline]
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    /// Sets the `name`.
    #[inline]
    pub fn set_name(&mut self, name: String) -> &Self {
        self.name = name;
        self
    }
    /// Sets the `name`.
    #[inline]
    pub fn with_name(mut self, name: String) -> Self {
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
    pub const fn set_age(&mut self, age: u32) -> &Self {
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
    pub fn email(&self) -> Option<&str> {
        core::option::Option::as_deref(&self.email)
    }
    /// Gets the mutable reference of the field `email`.
    #[inline]
    pub fn email_mut(&mut self) -> &mut Option<String> {
        &mut self.email
    }
    /// Sets the `email`.
    #[inline]
    pub fn set_email(&mut self, email: Option<String>) -> &Self {
        self.email = email;
        self
    }
    /// Sets the `email`.
    #[inline]
    pub fn with_email(mut self, email: Option<String>) -> Self {
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
    const fn select_field_reflection_iter(&self) -> UserSelectionFlagsIter {
        UserSelectionFlagsIter::new(self.iter())
    }
}
/// Yield a set of selected fields.
pub struct UserSelectionFlagsIter {
    iter: ::grost::__private::bitflags::iter::Iter<UserSelectionFlags>,
}
impl UserSelectionFlagsIter {
    #[inline]
    const fn new(
        iter: ::grost::__private::bitflags::iter::Iter<UserSelectionFlags>,
    ) -> Self {
        Self { iter }
    }
}
impl ::core::iter::Iterator for UserSelectionFlagsIter {
    type Item = ::grost::__private::FieldRelection;
    #[inline]
    fn next(&mut self) -> ::core::option::Option<Self::Item> {
        for f in ::core::iter::Iterator::by_ref(&mut self.iter) {
            if let ::core::option::Option::Some(val) = User::REFLECTION
                .fields()
                .get(f.bits().trailing_zeros() as ::core::primitive::usize)
            {
                return ::core::option::Option::Some(*val);
            }
        }
        ::core::option::Option::None
    }
}
impl ::core::iter::FusedIterator for UserSelectionFlagsIter {}
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
    pub const fn iter(&self) -> UserSelectionFlagsIter {
        self.flags.select_field_reflection_iter()
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
