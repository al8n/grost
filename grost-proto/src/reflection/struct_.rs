use super::{Flavor, TypeReflection};

#[doc(hidden)]
pub struct StructReflectionBuilder<F: Flavor> {
  pub name: &'static str,
  pub schema_name: &'static str,
  pub fields: &'static [FieldRelection<F>],
}

impl<F: Flavor> StructReflectionBuilder<F> {
  #[inline]
  pub const fn build(self) -> StructReflection<F> {
    StructReflection {
      name: self.name,
      schema_name: self.schema_name,
      fields: self.fields,
    }
  }
}

/// The struct information of an object in the Graph protocol buffer
#[derive(Clone, Copy, Debug)]
pub struct StructReflection<F: Flavor> {
  name: &'static str,
  schema_name: &'static str,
  fields: &'static [FieldRelection<F>],
}

impl<F: Flavor> StructReflection<F> {
  /// Get the name of the struct
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  /// Get the schema name of the struct.
  ///
  /// This will returns the name in the Graph protocol buffer schema file.
  #[inline]
  pub const fn schema_name(&self) -> &'static str {
    self.schema_name
  }

  /// Get the fields of this struct
  #[inline]
  pub const fn fields(&self) -> &'static [FieldRelection<F>] {
    self.fields
  }
}

#[doc(hidden)]
pub struct FieldRelectionBuilder<F: Flavor> {
  pub name: &'static str,
  pub ty: TypeReflection<F>,
  pub schema_name: &'static str,
  pub schema_type: &'static str,
  pub tag: F::Tag,
  pub wire_type: F::WireType,
  pub encoded_identifier: &'static [u8],
  pub encoded_identifier_len: usize,
}

impl<F: Flavor> FieldRelectionBuilder<F> {
  #[inline]
  pub const fn build(self) -> FieldRelection<F> {
    FieldRelection {
      name: self.name,
      ty: self.ty,
      schema_name: self.schema_name,
      schema_type: self.schema_type,
      tag: self.tag,
      wire_type: self.wire_type,
      encoded_identifier: self.encoded_identifier,
      encoded_identifier_len: self.encoded_identifier_len,
    }
  }
}

/// The information of a field in the Graph protocol buffer
#[derive(Clone, Copy, Debug)]
pub struct FieldRelection<F: Flavor> {
  name: &'static str,
  ty: TypeReflection<F>,
  schema_name: &'static str,
  schema_type: &'static str,
  tag: F::Tag,
  wire_type: F::WireType,
  encoded_identifier: &'static [u8],
  encoded_identifier_len: usize,
}

impl<F: Flavor> FieldRelection<F> {
  /// Get the name of the field
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  /// Get the rust type of the field, the type must be a full quailified.
  ///
  /// See [`schema_type`](FieldRelection::schema_type) for the type in the Graph protocol buffer file.
  #[inline]
  pub const fn ty(&self) -> &TypeReflection<F> {
    &self.ty
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

  /// Get the tag of the field
  #[inline]
  pub const fn tag(&self) -> F::Tag {
    self.tag
  }

  /// Get the wire type of the field
  #[inline]
  pub const fn wire_type(&self) -> F::WireType {
    self.wire_type
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
