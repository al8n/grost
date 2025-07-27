use crate::reflection::{Reflectable, SchemaType, SchemaTypeReflection};

impl<T> Reflectable<Self> for SchemaTypeReflection<[T]>
where
  T: Reflectable<T, Reflection = SchemaType>,
{
  type Reflection = SchemaType;

  const REFLECTION: &Self::Reflection = &SchemaType::List(T::REFLECTION);
}

impl<T> Reflectable<Self> for SchemaTypeReflection<&[T]>
where
  T: Reflectable<T, Reflection = SchemaType>,
{
  type Reflection = SchemaType;

  const REFLECTION: &'static Self::Reflection = &SchemaType::List(T::REFLECTION);
}
