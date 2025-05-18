use super::Type;

#[doc(hidden)]
pub struct ObjectReflectionBuilder {
  pub name: &'static str,
  pub schema_name: &'static str,
  pub fields: &'static [&'static ObjectFieldReflection],
}

impl ObjectReflectionBuilder {
  #[inline]
  pub const fn build(self) -> ObjectReflection {
    ObjectReflection {
      name: self.name,
      schema_name: self.schema_name,
      fields: self.fields,
    }
  }
}

/// The struct information of an object in the Graph protocol buffer
#[derive(Debug)]
pub struct ObjectReflection {
  name: &'static str,
  schema_name: &'static str,
  fields: &'static [&'static ObjectFieldReflection],
}

impl Clone for ObjectReflection {
  fn clone(&self) -> Self {
    *self
  }
}

impl Copy for ObjectReflection {}

impl ObjectReflection {
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
  pub const fn fields(&self) -> &'static [&'static ObjectFieldReflection] {
    self.fields
  }
}

#[doc(hidden)]
pub struct ObjectFieldReflectionBuilder {
  pub name: &'static str,
  /// A hack to avoid https://github.com/rust-lang/rust/issues/63084
  pub ty: fn() -> &'static str,
  pub schema_name: &'static str,
  pub schema_description: &'static str,
  pub schema_type: &'static Type,
}

impl ObjectFieldReflectionBuilder {
  #[inline]
  pub const fn build(self) -> ObjectFieldReflection {
    ObjectFieldReflection {
      name: self.name,
      ty: self.ty,
      schema_name: self.schema_name,
      schema_type: self.schema_type,
      schema_description: self.schema_description,
    }
  }
}

/// The information of a field in the Graph protocol buffer
#[derive(derive_more::Debug)]
pub struct ObjectFieldReflection {
  name: &'static str,
  /// A hack to avoid https://github.com/rust-lang/rust/issues/63084
  #[debug("{}", (self.ty)())]
  ty: fn() -> &'static str,
  schema_name: &'static str,
  schema_description: &'static str,
  schema_type: &'static Type,
}

impl Clone for ObjectFieldReflection {
  fn clone(&self) -> Self {
    *self
  }
}

impl Copy for ObjectFieldReflection {}

impl ObjectFieldReflection {
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

  /// Get the schema description of the field.
  ///
  /// This will returns the description in the Graph protocol buffer schema file.
  #[inline]
  pub const fn schema_description(&self) -> &'static str {
    self.schema_description
  }

  /// Get the schema type of the field.
  ///
  /// This will returns the type in the Graph protocol buffer schema file.
  #[inline]
  pub const fn schema_type(&self) -> &'static Type {
    self.schema_type
  }
}
