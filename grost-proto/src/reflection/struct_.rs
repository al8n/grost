use super::{Flavor, Type};

#[doc(hidden)]
pub struct ObjectReflectionBuilder<F: Flavor + ?Sized> {
  pub name: &'static str,
  pub schema_name: &'static str,
  pub fields: &'static [&'static ObjectFieldReflection<F>],
}

impl<F: Flavor + ?Sized> ObjectReflectionBuilder<F> {
  #[inline]
  pub const fn build(self) -> ObjectReflection<F> {
    ObjectReflection {
      name: self.name,
      schema_name: self.schema_name,
      fields: self.fields,
    }
  }
}

/// The struct information of an object in the Graph protocol buffer
#[derive(Debug)]
pub struct ObjectReflection<F: Flavor + ?Sized> {
  name: &'static str,
  schema_name: &'static str,
  fields: &'static [&'static ObjectFieldReflection<F>],
}

impl<F: Flavor + ?Sized> Clone for ObjectReflection<F> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<F: Flavor + ?Sized> Copy for ObjectReflection<F> {}

impl<F: Flavor + ?Sized> ObjectReflection<F> {
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
  pub const fn fields(&self) -> &'static [&'static ObjectFieldReflection<F>] {
    self.fields
  }
}

#[doc(hidden)]
pub struct ObjectFieldReflectionBuilder<F: Flavor + ?Sized> {
  pub name: &'static str,
  /// A hack to avoid https://github.com/rust-lang/rust/issues/63084
  pub ty: fn() -> &'static str,
  pub schema_name: &'static str,
  pub schema_type: Type<F>,
  pub identifier: F::Identifier,
}

impl<F: Flavor + ?Sized> ObjectFieldReflectionBuilder<F> {
  #[inline]
  pub const fn build(self) -> ObjectFieldReflection<F> {
    ObjectFieldReflection {
      name: self.name,
      ty: self.ty,
      schema_name: self.schema_name,
      schema_type: self.schema_type,
      identifier: self.identifier,
    }
  }
}

/// The information of a field in the Graph protocol buffer
#[derive(derive_more::Debug)]
pub struct ObjectFieldReflection<F: Flavor + ?Sized> {
  name: &'static str,
  /// A hack to avoid https://github.com/rust-lang/rust/issues/63084
  #[debug("{}", (self.ty)())]
  ty: fn() -> &'static str,
  schema_name: &'static str,
  schema_type: Type<F>,
  identifier: F::Identifier,
}

impl<F: Flavor> Clone for ObjectFieldReflection<F> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<F: Flavor> Copy for ObjectFieldReflection<F> {}

impl<F: Flavor> ObjectFieldReflection<F> {
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

  /// Get the identifier of the field
  #[inline]
  pub const fn identifier(&self) -> F::Identifier {
    self.identifier
  }
}
