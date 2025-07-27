use crate::flavors::Flavor;

use super::{EncodeReflection, Len, ObjectFieldReflection, Reflectable, WireSchemaTypeReflection};

impl<O, F, const TAG: u32> core::fmt::Debug
  for WireSchemaTypeReflection<ObjectFieldReflection<O, F::WireType, F, TAG>>
where
  O: ?Sized,
  F: ?Sized + Flavor,
  Self: Reflectable<O, Reflection = F::WireType>,
  F::WireType: core::fmt::Debug,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::ops::Deref::deref(self).fmt(f)
  }
}

impl<O, F, const TAG: u32> core::fmt::Display
  for WireSchemaTypeReflection<ObjectFieldReflection<O, F::WireType, F, TAG>>
where
  O: ?Sized,
  F: ?Sized + Flavor,
  Self: Reflectable<O, Reflection = F::WireType>,
  F::WireType: core::fmt::Display,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::ops::Deref::deref(self).fmt(f)
  }
}

impl<O, F, const TAG: u32> core::ops::Deref
  for WireSchemaTypeReflection<ObjectFieldReflection<O, F::WireType, F, TAG>>
where
  O: ?Sized,
  F: ?Sized + Flavor,
  Self: Reflectable<O, Reflection = F::WireType>,
{
  type Target = F::WireType;

  #[inline]
  fn deref(&self) -> &Self::Target {
    <Self as Reflectable<O>>::REFLECTION
  }
}

impl<R> WireSchemaTypeReflection<R>
where
  R: ?Sized,
{
  /// Returns the reflection to the encoded wire type.
  #[inline]
  pub const fn encoded<T>(&self) -> &'static [u8]
  where
    T: ?Sized,
    EncodeReflection<Self>: Reflectable<T, Reflection = [u8]>,
  {
    <EncodeReflection<Self> as Reflectable<T>>::REFLECTION
  }

  /// Returns the reflection to the length of the encoded wire type.
  #[inline]
  pub const fn encoded_len<T>(&self) -> usize
  where
    T: ?Sized,
    Len<EncodeReflection<Self>>: Reflectable<T, Reflection = usize>,
  {
    *<Len<EncodeReflection<Self>> as Reflectable<T>>::REFLECTION
  }
}
