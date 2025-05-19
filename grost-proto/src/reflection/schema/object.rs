use super::super::Reflectable;
use core::marker::PhantomData;

pub use field::*;

mod field;

pub struct ObjectReflection<O: ?Sized, T: ?Sized, F: ?Sized> {
  _object: PhantomData<O>,
  _flavor: PhantomData<F>,
  _target: PhantomData<T>,
}

impl<O, T, F> Default for ObjectReflection<O, T, F>
where
  O: ?Sized,
  F: ?Sized,
  T: ?Sized,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<O, T, F> ObjectReflection<O, T, F>
where
  O: ?Sized,
  F: ?Sized,
  T: ?Sized,
{
  /// Creates a new [`ObjectReflection`].
  #[inline]
  pub const fn new() -> Self {
    Self {
      _object: PhantomData,
      _flavor: PhantomData,
      _target: PhantomData,
    }
  }
}

impl<O, T, F> Reflectable<O> for ObjectReflection<&O, T, F>
where
  O: ?Sized,
  F: ?Sized,
  T: ?Sized,
  ObjectReflection<O, T, F>: Reflectable<O, Reflection = Object>,
{
  type Reflection = Object;

  const REFLECTION: &'static Self::Reflection =
    <ObjectReflection<O, T, F> as Reflectable<O>>::REFLECTION;
}

#[doc(hidden)]
pub struct ObjectBuilder {
  pub name: &'static str,
  pub description: &'static str,
  pub fields: &'static [&'static ObjectField],
}

impl ObjectBuilder {
  #[inline]
  pub const fn build(self) -> Object {
    Object {
      name: self.name,
      description: self.description,
      fields: self.fields,
    }
  }
}

/// The struct information of an object in the Graph protocol buffer
#[derive(Debug)]
pub struct Object {
  name: &'static str,
  description: &'static str,
  fields: &'static [&'static ObjectField],
}

impl Clone for Object {
  fn clone(&self) -> Self {
    *self
  }
}

impl Copy for Object {}

impl Object {
  /// Get the schema name of the struct.
  ///
  /// This will returns the name in the Graph protocol buffer schema file.
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  /// Get the schema description of the struct.
  ///
  /// This will returns the description in the Graph protocol buffer schema file.
  #[inline]
  pub const fn description(&self) -> &'static str {
    self.description
  }

  /// Get the fields of this struct
  #[inline]
  pub const fn fields(&self) -> &'static [&'static ObjectField] {
    self.fields
  }
}
