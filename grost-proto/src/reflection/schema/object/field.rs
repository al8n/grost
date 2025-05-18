use core::marker::PhantomData;

use crate::{flavors::Flavor, reflection::WireFormatReflection};

use super::{super::{Type, super::Reflectable}, Object, ObjectReflection};

pub struct ObjectFieldReflection<O: ?Sized, T: ?Sized, F: ?Sized, const TAG: u32> {
  _object: PhantomData<O>,
  _flavor: PhantomData<F>,
  _target: PhantomData<T>,
}

impl<O, T, F, const TAG: u32> Default for ObjectFieldReflection<O, T, F, TAG>
where
  O: ?Sized,
  F: ?Sized,
  T: ?Sized,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<O, T, F, const TAG: u32> ObjectFieldReflection<O, T, F, TAG>
where
  O: ?Sized,
  F: ?Sized,
  T: ?Sized,
{
  /// Creates a new [`ObjectFieldReflection`].
  #[inline]
  pub const fn new() -> Self {
    Self {
      _object: PhantomData,
      _flavor: PhantomData,
      _target: PhantomData,
    }
  }
}

impl<O, T, F, const TAG: u32> Reflectable<O> for ObjectFieldReflection<&O, T, F, TAG>
where
  O: ?Sized,
  F: ?Sized,
  T: ?Sized,
  ObjectFieldReflection<O, T, F, TAG>: Reflectable<O, Reflection = ObjectField>,
  ObjectReflection<O, Object, F>: Reflectable<O, Reflection = Object>,
{
  type Reflection = ObjectField;

  const REFLECTION: &'static Self::Reflection = <ObjectFieldReflection<O, T, F, TAG> as Reflectable<O>>::REFLECTION;
}

impl<O, F, const TAG: u32> ObjectFieldReflection<O, ObjectField, F, TAG>
where
  O: ?Sized,
  F: ?Sized + Flavor,
  Self: Reflectable<O, Reflection = ObjectField>,
  ObjectReflection<O, Object, F>: Reflectable<O, Reflection = Object>,
{
  /// Returns the relection to the wire format of the field.
  #[inline]
  pub const fn wire_format(&self) -> ObjectFieldReflection<O, WireFormatReflection, F, TAG>
  where
    ObjectFieldReflection<O, WireFormatReflection, F, TAG>: Reflectable<O>,
  {
    ObjectFieldReflection::new()
  }

  /// Returns the reflection to the tag of the field.
  #[inline]
  pub const fn tag(&self) -> ObjectFieldReflection<O, F::Tag, F, TAG>
  where
    ObjectFieldReflection<O, F::Tag, F, TAG>: Reflectable<O, Reflection = F::Tag>,
  {
    ObjectFieldReflection::new()
  }

  /// Returns the reflection to the wire type of the field.
  #[inline]
  pub const fn wire_type(
    &self,
  ) -> ObjectFieldReflection<O, F::WireType, F, TAG>
  where
    ObjectFieldReflection<O, F::WireType, F, TAG>: Reflectable<O, Reflection = F::WireType>,
  {
    ObjectFieldReflection::new()
  }

  /// Returns the reflection to the identifier of the field.
  #[inline]
  pub const fn identifier(
    &self,
  ) -> ObjectFieldReflection<O, F::Identifier, F, TAG>
  where
    ObjectFieldReflection<O, F::Identifier, F, TAG>: Reflectable<O, Reflection = F::Identifier>,
  {
    ObjectFieldReflection::new()
  }
}

impl<O, T, F, const TAG: u32> core::fmt::Debug for ObjectFieldReflection<O, T, F, TAG>
where
  T: ?Sized,
  O: ?Sized,
  F: ?Sized,
  Self: Reflectable<O>,
  <Self as Reflectable<O>>::Reflection: core::fmt::Debug,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::ops::Deref::deref(self).fmt(f)
  }
}

impl<O, T, F, const TAG: u32> core::fmt::Display for ObjectFieldReflection<O, T, F, TAG>
where
  T: ?Sized,
  O: ?Sized,
  F: ?Sized,
  Self: Reflectable<O>,
  <Self as Reflectable<O>>::Reflection: core::fmt::Display,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::ops::Deref::deref(self).fmt(f)
  }
}

impl<O, T, F, const TAG: u32> core::ops::Deref for ObjectFieldReflection<O, T, F, TAG>
where
  T: ?Sized,
  O: ?Sized,
  F: ?Sized,
  Self: Reflectable<O>,
{
  type Target = <Self as Reflectable<O>>::Reflection;

  fn deref(&self) -> &Self::Target {
    Self::REFLECTION
  }
}

#[doc(hidden)]
pub struct ObjectFieldBuilder {
  pub name: &'static str,
  /// A hack to avoid https://github.com/rust-lang/rust/issues/63084
  pub ty: fn() -> &'static str,
  pub schema_name: &'static str,
  pub schema_description: &'static str,
  pub schema_type: &'static Type,
}

impl ObjectFieldBuilder {
  #[inline]
  pub const fn build(self) -> ObjectField {
    ObjectField {
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
pub struct ObjectField {
  name: &'static str,
  /// A hack to avoid https://github.com/rust-lang/rust/issues/63084
  #[debug("{}", (self.ty)())]
  ty: fn() -> &'static str,
  schema_name: &'static str,
  schema_description: &'static str,
  schema_type: &'static Type,
}

impl Clone for ObjectField {
  fn clone(&self) -> Self {
    *self
  }
}

impl Copy for ObjectField {}

impl ObjectField {
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

