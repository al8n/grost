#![allow(clippy::type_complexity)]
use ghost::phantom;

use crate::{
  flavors::Flavor,
  reflection::{
    EncodeReflection, IdentifierReflection, Len, TagReflection, WireFormatReflection,
    WireSchemaTypeReflection,
  },
};

use super::{
  super::{super::Reflectable, SchemaType},
  Object, ObjectReflection,
};

#[phantom]
pub struct ObjectFieldReflection<O: ?Sized, T: ?Sized, F: ?Sized, const TAG: u32>;

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
    ObjectFieldReflection
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

impl<O, T, F, const TAG: u32> Reflectable<O> for ObjectFieldReflection<&O, T, F, TAG>
where
  O: ?Sized,
  F: ?Sized,
  T: ?Sized,
  ObjectFieldReflection<O, T, F, TAG>: Reflectable<O, Reflection = ObjectField>,
  ObjectReflection<O, Object, F>: Reflectable<O, Reflection = Object>,
{
  type Reflection = ObjectField;

  const REFLECTION: &'static Self::Reflection =
    <ObjectFieldReflection<O, T, F, TAG> as Reflectable<O>>::REFLECTION;
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
  pub const fn wire_format(&self) -> WireFormatReflection<O, F, TAG>
  where
    WireFormatReflection<O, F, TAG>: Reflectable<O>,
  {
    WireFormatReflection::new()
  }

  /// Returns the reflection to the tag of the field.
  #[inline]
  pub const fn tag(&self) -> TagReflection<ObjectFieldReflection<O, F::Tag, F, TAG>>
  where
    TagReflection<ObjectFieldReflection<O, F::Tag, F, TAG>>: Reflectable<O, Reflection = F::Tag>,
  {
    TagReflection::new()
  }

  /// Returns the reflection to the encoded tag of the field.
  #[inline]
  pub const fn encoded_tag(
    &self,
  ) -> EncodeReflection<TagReflection<ObjectFieldReflection<O, F::Identifier, F, TAG>>>
  where
    EncodeReflection<TagReflection<ObjectFieldReflection<O, F::Identifier, F, TAG>>>:
      Reflectable<O, Reflection = [u8]>,
  {
    EncodeReflection::new()
  }

  /// Returns the reflection to the length of encoded tag of the field.
  #[inline]
  pub const fn encoded_tag_len(
    &self,
  ) -> Len<EncodeReflection<TagReflection<ObjectFieldReflection<O, F::Identifier, F, TAG>>>>
  where
    Len<EncodeReflection<TagReflection<ObjectFieldReflection<O, F::Identifier, F, TAG>>>>:
      Reflectable<O, Reflection = usize>,
  {
    Len::new()
  }

  /// Returns the reflection to the wire type of the field.
  #[inline]
  pub const fn wire_type(
    &self,
  ) -> WireSchemaTypeReflection<ObjectFieldReflection<O, F::WireType, F, TAG>>
  where
    WireSchemaTypeReflection<ObjectFieldReflection<O, F::WireType, F, TAG>>:
      Reflectable<O, Reflection = F::WireType>,
  {
    WireSchemaTypeReflection::new()
  }

  /// Returns the reflection to the identifier of the field.
  #[inline]
  pub const fn identifier(
    &self,
  ) -> IdentifierReflection<ObjectFieldReflection<O, F::Identifier, F, TAG>>
  where
    IdentifierReflection<ObjectFieldReflection<O, F::Identifier, F, TAG>>:
      Reflectable<O, Reflection = F::Identifier>,
  {
    IdentifierReflection::new()
  }

  /// Returns the reflection to the encoded identifier of the field.
  #[inline]
  pub const fn encoded_identifier(
    &self,
  ) -> EncodeReflection<IdentifierReflection<ObjectFieldReflection<O, F::Identifier, F, TAG>>>
  where
    EncodeReflection<IdentifierReflection<ObjectFieldReflection<O, F::Identifier, F, TAG>>>:
      Reflectable<O, Reflection = [u8]>,
  {
    EncodeReflection::new()
  }

  /// Returns the reflection to the length of encoded identifier of the field.
  #[inline]
  pub const fn encoded_identifier_len(
    &self,
  ) -> Len<EncodeReflection<IdentifierReflection<ObjectFieldReflection<O, F::Identifier, F, TAG>>>>
  where
    Len<EncodeReflection<IdentifierReflection<ObjectFieldReflection<O, F::Identifier, F, TAG>>>>:
      Reflectable<O, Reflection = usize>,
  {
    Len::new()
  }
}

#[doc(hidden)]
pub struct ObjectFieldBuilder {
  pub name: &'static str,
  pub description: &'static str,
  pub ty: &'static SchemaType,
}

impl ObjectFieldBuilder {
  #[inline]
  pub const fn build(self) -> ObjectField {
    ObjectField {
      name: self.name,
      ty: self.ty,
      description: self.description,
    }
  }
}

/// The information of a field in the Graph protocol buffer
#[derive(derive_more::Debug)]
pub struct ObjectField {
  name: &'static str,
  description: &'static str,
  ty: &'static SchemaType,
}

impl Clone for ObjectField {
  fn clone(&self) -> Self {
    *self
  }
}

impl Copy for ObjectField {}

impl ObjectField {
  /// Get the schema name of the field.
  ///
  /// This will returns the name in the Graph protocol buffer schema file.
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  /// Get the schema description of the field.
  ///
  /// This will returns the description in the Graph protocol buffer schema file.
  #[inline]
  pub const fn description(&self) -> &'static str {
    self.description
  }

  /// Get the schema type of the field.
  ///
  /// This will returns the type in the Graph protocol buffer schema file.
  #[inline]
  pub const fn ty(&self) -> &'static SchemaType {
    self.ty
  }
}
