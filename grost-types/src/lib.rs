#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;

pub use error::*;
pub use identifier::Identifier;
pub use tag::Tag;
pub use wire_type::WireType;

mod error;
mod identifier;
mod tag;
mod wire_type;

#[doc(hidden)]
pub struct StructInfoBuilder {
  pub name: &'static str,
  pub ty: &'static str,
  pub schema_name: &'static str,
  pub schema_type: &'static str,
  pub fields: &'static [FieldInfo],
}

impl StructInfoBuilder {
  #[inline]
  pub const fn build(self) -> StructInfo {
    StructInfo {
      name: self.name,
      ty: self.ty,
      schema_name: self.schema_name,
      schema_type: self.schema_type,
      fields: self.fields,
    }
  }
}

/// The struct information of an object in the Graph protocol buffer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StructInfo {
  name: &'static str,
  ty: &'static str,
  schema_name: &'static str,
  schema_type: &'static str,
  fields: &'static [FieldInfo],
}

impl StructInfo {
  /// Get the name of the struct
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  /// Get the rust type of the struct, the type must be a full quailified.
  ///
  /// See [`schema_type`](StructInfo::schema_type) for the type in the Graph protocol buffer file.
  #[inline]
  pub const fn ty(&self) -> &'static str {
    self.ty
  }

  /// Get the schema name of the struct.
  ///
  /// This will returns the name in the Graph protocol buffer schema file.
  #[inline]
  pub const fn schema_name(&self) -> &'static str {
    self.schema_name
  }

  /// Get the schema type of the struct.
  ///
  /// This will returns the type in the Graph protocol buffer schema file.
  #[inline]
  pub const fn schema_type(&self) -> &'static str {
    self.schema_type
  }

  /// Get the fields of this struct
  #[inline]
  pub const fn fields(&self) -> &'static [FieldInfo] {
    self.fields
  }
}

#[doc(hidden)]
pub struct FieldInfoBuilder {
  pub name: &'static str,
  pub ty: &'static str,
  pub schema_name: &'static str,
  pub schema_type: &'static str,
  pub identifier: Identifier,
  pub encoded_identifier: &'static [u8],
  pub encoded_identifier_len: usize,
}

impl FieldInfoBuilder {
  #[inline]
  pub const fn build(self) -> FieldInfo {
    FieldInfo {
      name: self.name,
      ty: self.ty,
      schema_name: self.schema_name,
      schema_type: self.schema_type,
      identifier: self.identifier,
      encoded_identifier: self.encoded_identifier,
      encoded_identifier_len: self.encoded_identifier_len,
    }
  }
}

/// The information of a field in the Graph protocol buffer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FieldInfo {
  name: &'static str,
  ty: &'static str,
  schema_name: &'static str,
  schema_type: &'static str,
  identifier: Identifier,
  encoded_identifier: &'static [u8],
  encoded_identifier_len: usize,
}

impl FieldInfo {
  #[doc(hidden)]
  pub const fn __init__(
    name: &'static str,
    ty: &'static str,
    schema_name: &'static str,
    schema_type: &'static str,
    identifier: Identifier,
    encoded_identifier: &'static [u8],
    encoded_identifier_len: usize,
  ) -> Self {
    Self {
      name,
      ty,
      schema_name,
      schema_type,
      identifier,
      encoded_identifier,
      encoded_identifier_len,
    }
  }

  /// Get the name of the field
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  /// Get the rust type of the field, the type must be a full quailified.
  ///
  /// See [`schema_type`](FieldInfo::schema_type) for the type in the Graph protocol buffer file.
  #[inline]
  pub const fn ty(&self) -> &'static str {
    self.ty
  }

  /// Get the schema name of the field.
  ///
  /// This will returns the name in the Graph protocol buffer schema file.
  #[inline]
  pub const fn schema_name(&self) -> &'static str {
    self.schema_name
  }

  /// Get the schema type of the field.
  ///
  /// This will returns the type in the Graph protocol buffer schema file.
  #[inline]
  pub const fn schema_type(&self) -> &'static str {
    self.schema_type
  }

  /// Get the identifier of the field
  #[inline]
  pub const fn identifier(&self) -> Identifier {
    self.identifier
  }

  /// Get the encoded identifier of the field
  #[inline]
  pub const fn encoded_identifier(&self) -> &'static [u8] {
    self.encoded_identifier
  }

  /// Get the encoded identifier length of the field
  #[inline]
  pub const fn encoded_identifier_len(&self) -> usize {
    self.encoded_identifier_len
  }
}

#[doc(hidden)]
pub struct EnumVariantInfoBuilder<T: 'static> {
  pub name: &'static str,
  pub schema_name: &'static str,
  pub description: &'static str,
  pub value: T,
}

impl<T: Copy + 'static> EnumVariantInfoBuilder<T> {
  #[inline]
  pub const fn build(self) -> EnumVariantInfo<T> {
    EnumVariantInfo {
      name: self.name,
      schema_name: self.schema_name,
      description: self.description,
      value: self.value,
    }
  }
}

/// The information of a variant of enum in the Graph protocol buffer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnumVariantInfo<T: 'static> {
  name: &'static str,
  schema_name: &'static str,
  description: &'static str,
  value: T,
}

impl<T: 'static> EnumVariantInfo<T> {
  /// Get the name of the variant
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  /// Get the schema name of the variant
  #[inline]
  pub const fn schema_name(&self) -> &'static str {
    self.schema_name
  }

  /// Get the value of the variant
  #[inline]
  pub const fn value(&self) -> T
  where
    T: Copy,
  {
    self.value
  }

  /// Get the description of the variant
  #[inline]
  pub const fn description(&self) -> &'static str {
    self.description
  }
}

#[doc(hidden)]
pub struct EnumInfoBuilder<T: 'static> {
  pub name: &'static str,
  pub schema_name: &'static str,
  pub variants: &'static [EnumVariantInfo<T>],
  pub description: &'static str,
}

impl<T: 'static> EnumInfoBuilder<T> {
  #[inline]
  pub const fn build(self) -> EnumInfo<T> {
    EnumInfo {
      name: self.name,
      schema_name: self.schema_name,
      variants: self.variants,
      description: self.description,
    }
  }
}

/// The information of an enum in the Graph protocol buffer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnumInfo<T: 'static> {
  name: &'static str,
  schema_name: &'static str,
  description: &'static str,
  variants: &'static [EnumVariantInfo<T>],
}

impl<T: 'static> EnumInfo<T> {
  /// Get the name of the enum
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  /// Get the schema name of the enum
  #[inline]
  pub const fn schema_name(&self) -> &'static str {
    self.schema_name
  }

  /// Get the variants of the enum
  #[inline]
  pub const fn variants(&self) -> &'static [EnumVariantInfo<T>] {
    self.variants
  }

  /// Get the description of the enum
  #[inline]
  pub const fn description(&self) -> &'static str {
    self.description
  }
}

/// Skip a field from the Graph protocol buffer
#[inline]
pub const fn skip(src: &[u8]) -> Result<usize, DecodeError> {
  let buf_len = src.len();
  if buf_len == 0 {
    return Ok(0);
  }

  let mut offset = 0;
  let (bytes_read, identifier) = match Identifier::decode_inner(src) {
    Ok((bytes_read, identifier)) => (bytes_read, identifier),
    Err(e) => return Err(DecodeError::from_varint_error(e)),
  };
  offset += bytes_read;

  let (_, src) = src.split_at(offset);
  let val = match identifier.wire_type() {
    WireType::Varint => match varing::consume_varint(src) {
      Ok(bytes_read) => offset + bytes_read,
      Err(e) => return Err(DecodeError::from_varint_error(e)),
    },
    WireType::LengthDelimited => {
      // Skip length-delimited field by reading the length and skipping the payload
      if src.is_empty() {
        return Err(DecodeError::buffer_underflow());
      }

      match varing::decode_u32_varint(src) {
        Ok((bytes_read, length)) => offset + bytes_read + length as usize,
        Err(e) => return Err(DecodeError::from_varint_error(e)),
      }
    }
    WireType::Byte => offset + 1,
    WireType::Fixed16 => offset + 2,
    WireType::Fixed32 => offset + 4,
    WireType::Fixed64 => offset + 8,
    WireType::Fixed128 => offset + 16,
    WireType::Zst => offset,
  };

  if val > buf_len {
    return Ok(buf_len);
  }

  Ok(val)
}
