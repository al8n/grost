use std::num::NonZeroUsize;

use grost::{Buffer, Decode, Encode, Identifier, Unknown, Wirable};

pub struct PartialUserRef<'a, UB> {
  name: Option<&'a str>,
  age: Option<u32>,
  email: Option<&'a str>,
  unknown_buffer: UB,
}



impl Wirable for User {}

impl<'de, UB> Decode<'de, PartialUserRef<'de, UB>> for User {
  type UnknownBuffer<B> = UB;

  fn decode<B>(context: &grost::Context, src: B) -> Result<(usize, PartialUserRef<'de, Self::UnknownBuffer<B>>), grost::error::DecodeError>
  where
    Self: Sized + 'de,
    B: grost::Buffer + 'de,
    Self::UnknownBuffer<B>: grost::UnknownBuffer<B>,
  {
    let buf = src.as_bytes();
    let buf_len = buf.len();
    let mut offset = 0;

    let mut name: Option<&str> = None;
    let mut age: Option<u32> = None;
    let mut email: Option<&str> = None;
    let mut unknowns = Self::UnknownBuffer::<B>::new();

    while offset < buf_len {
      let (readed, identifier) = Identifier::decode(&buf[offset..])?;

      match identifier {
        Self::AGE_IDENTIFIER => {
          let (readed, val) = u8::decode::<B>(context, src.slice(offset + readed..))?;
          offset += readed;
          age = Some(val as _);
        },
        Self::EMAIL_IDENTIFIER => {
        
        },
        Self::NAME_IDENTIFIER => {
        
        }
        _ => {
          let (readed, u) = Unknown::decode(identifier, src.slice(offset..))?;
          offset += readed;
          if unknowns.push(u).is_some() {
            return Err(grost::error::DecodeError::unknown_buffer_overflow(unknowns.len(), NonZeroUsize::new(unknowns.len() + 1).unwrap()));
          }
        }
      }
    }

    Ok((offset, PartialUserRef {
      name,
      age,
      email,
      unknown_buffer: unknowns,
    }))
  }
}

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
  /// The tag of the `name` field
  pub const NAME_TAG: ::grost::__private::Tag = ::grost::__private::Tag::new(1u32);
  /// The identifier of the `name` field
  pub const NAME_IDENTIFIER: ::grost::__private::Identifier = ::grost::__private::Identifier::new(
    ::grost::__private::WireType::LengthDelimited,
    Self::NAME_TAG,
  );
  /// The encoded length of the identifier of the `name` field
  pub const NAME_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize =
    Self::NAME_IDENTIFIER.encoded_len();
  /// The encoded identifier of the `name` field
  pub const ENCODED_NAME_IDENTIFIER: &[::core::primitive::u8] =
    Self::NAME_IDENTIFIER.encode().as_slice();
  /// The reflection information of the `name` field
  pub const NAME_INFO: ::grost::__private::FieldInfo = ::grost::__private::FieldInfoBuilder {
    identifier: Self::NAME_IDENTIFIER,
    encoded_identifier_len: Self::NAME_IDENTIFIER_ENCODED_LEN,
    encoded_identifier: Self::ENCODED_NAME_IDENTIFIER,
    name: "name",
    ty: "String",
    schema_name: "name",
    schema_type: "String!",
  }
  .build();
  /// The tag of the `age` field
  pub const AGE_TAG: ::grost::__private::Tag = ::grost::__private::Tag::new(2u32);
  /// The identifier of the `age` field
  pub const AGE_IDENTIFIER: ::grost::__private::Identifier =
    ::grost::__private::Identifier::new(::grost::__private::WireType::Varint, Self::AGE_TAG);
  /// The encoded length of the identifier of the `age` field
  pub const AGE_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize =
    Self::AGE_IDENTIFIER.encoded_len();
  /// The encoded identifier of the `age` field
  pub const ENCODED_AGE_IDENTIFIER: &[::core::primitive::u8] =
    Self::AGE_IDENTIFIER.encode().as_slice();
  /// The reflection information of the `age` field
  pub const AGE_INFO: ::grost::__private::FieldInfo = ::grost::__private::FieldInfoBuilder {
    identifier: Self::AGE_IDENTIFIER,
    encoded_identifier_len: Self::AGE_IDENTIFIER_ENCODED_LEN,
    encoded_identifier: Self::ENCODED_AGE_IDENTIFIER,
    name: "age",
    ty: "u32",
    schema_name: "age",
    schema_type: "u32!",
  }
  .build();
  /// The tag of the `email` field
  pub const EMAIL_TAG: ::grost::__private::Tag = ::grost::__private::Tag::new(3u32);
  /// The identifier of the `email` field
  pub const EMAIL_IDENTIFIER: ::grost::__private::Identifier = ::grost::__private::Identifier::new(
    ::grost::__private::WireType::LengthDelimited,
    Self::EMAIL_TAG,
  );
  /// The encoded length of the identifier of the `email` field
  pub const EMAIL_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize =
    Self::EMAIL_IDENTIFIER.encoded_len();
  /// The encoded identifier of the `email` field
  pub const ENCODED_EMAIL_IDENTIFIER: &[::core::primitive::u8] =
    Self::EMAIL_IDENTIFIER.encode().as_slice();
  /// The reflection information of the `email` field
  pub const EMAIL_INFO: ::grost::__private::FieldInfo = ::grost::__private::FieldInfoBuilder {
    identifier: Self::EMAIL_IDENTIFIER,
    encoded_identifier_len: Self::EMAIL_IDENTIFIER_ENCODED_LEN,
    encoded_identifier: Self::ENCODED_EMAIL_IDENTIFIER,
    name: "email",
    ty: "Option<String>",
    schema_name: "email",
    schema_type: "String",
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
