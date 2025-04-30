use crate::Tag;

use super::{Flavor, Type};

#[doc(hidden)]
pub struct StructReflectionBuilder<F: Flavor + ?Sized> {
  pub name: &'static str,
  pub schema_name: &'static str,
  pub fields: &'static [FieldRelection<F>],
}

impl<F: Flavor + ?Sized> StructReflectionBuilder<F> {
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
#[derive(Debug)]
pub struct StructReflection<F: Flavor + ?Sized> {
  name: &'static str,
  schema_name: &'static str,
  fields: &'static [FieldRelection<F>],
}

impl<F: Flavor + ?Sized> Clone for StructReflection<F> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<F: Flavor + ?Sized> Copy for StructReflection<F> {}

impl<F: Flavor + ?Sized> StructReflection<F> {
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
pub struct FieldRelectionBuilder<F: Flavor + ?Sized> {
  pub name: &'static str,
  /// A hack to avoid https://github.com/rust-lang/rust/issues/63084
  pub ty: fn() -> &'static str,
  pub schema_name: &'static str,
  pub schema_type: Type<F>,
  pub tag: Tag,
  pub wire_type: F::WireType,
  // pub encoded_identifier: &'static [u8],
  // pub encoded_identifier_len: usize,
}

impl<F: Flavor + ?Sized> FieldRelectionBuilder<F> {
  #[inline]
  pub const fn build(self) -> FieldRelection<F> {
    FieldRelection {
      name: self.name,
      ty: self.ty,
      schema_name: self.schema_name,
      schema_type: self.schema_type,
      tag: self.tag,
      wire_type: self.wire_type,
    }
  }
}

/// The information of a field in the Graph protocol buffer
#[derive(Debug)]
pub struct FieldRelection<F: Flavor + ?Sized> {
  name: &'static str,
  /// A hack to avoid https://github.com/rust-lang/rust/issues/63084
  ty: fn() -> &'static str,
  schema_name: &'static str,
  schema_type: Type<F>,
  tag: Tag,
  wire_type: F::WireType,
}

impl<F: Flavor> Clone for FieldRelection<F> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<F: Flavor> Copy for FieldRelection<F> {}

impl<F: Flavor> FieldRelection<F> {
  /// Get the name of the field
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  /// Returns the name of the field type
  // TODO(al8n): make this const if https://github.com/rust-lang/rust/issues/63084 const stable
  pub fn ty(&self) -> &'static str {
    (self.ty)()
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
  pub const fn schema_type(&self) -> &Type<F> {
    &self.schema_type
  }

  /// Get the tag of the field
  #[inline]
  pub const fn tag(&self) -> Tag {
    self.tag
  }

  /// Get the wire type of the field
  #[inline]
  pub const fn wire_type(&self) -> F::WireType {
    self.wire_type
  }
}
