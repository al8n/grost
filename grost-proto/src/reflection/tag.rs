use crate::flavors::Flavor;

use super::{EncodeReflection, Len, ObjectFieldReflection, Reflectable, TagReflection};

impl<O, F, const TAG: u32> core::fmt::Debug
  for TagReflection<ObjectFieldReflection<O, F::Tag, F, TAG>>
where
  O: ?Sized,
  F: ?Sized + Flavor,
  Self: Reflectable<O, Reflection = F::Tag>,
  F::Tag: core::fmt::Debug,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::ops::Deref::deref(self).fmt(f)
  }
}

impl<O, F, const TAG: u32> core::fmt::Display
  for TagReflection<ObjectFieldReflection<O, F::Tag, F, TAG>>
where
  O: ?Sized,
  F: ?Sized + Flavor,
  Self: Reflectable<O, Reflection = F::Tag>,
  F::Tag: core::fmt::Display,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::ops::Deref::deref(self).fmt(f)
  }
}

impl<O, F, const TAG: u32> core::ops::Deref
  for TagReflection<ObjectFieldReflection<O, F::Tag, F, TAG>>
where
  O: ?Sized,
  F: ?Sized + Flavor,
  Self: Reflectable<O, Reflection = F::Tag>,
{
  type Target = F::Tag;

  #[inline]
  fn deref(&self) -> &Self::Target {
    <Self as Reflectable<O>>::REFLECTION
  }
}

impl<O, F, const TAG: u32> core::fmt::Debug
  for EncodeReflection<TagReflection<ObjectFieldReflection<O, F::Tag, F, TAG>>>
where
  O: ?Sized,
  F: ?Sized + Flavor,
  Self: Reflectable<O, Reflection = [u8]>,
  F::Tag: core::fmt::Debug,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::ops::Deref::deref(self).fmt(f)
  }
}

impl<O, F, const TAG: u32> core::ops::Deref
  for EncodeReflection<TagReflection<ObjectFieldReflection<O, F::Tag, F, TAG>>>
where
  O: ?Sized,
  F: ?Sized + Flavor,
  Self: Reflectable<O, Reflection = [u8]>,
{
  type Target = [u8];

  #[inline]
  fn deref(&self) -> &Self::Target {
    <Self as Reflectable<O>>::REFLECTION
  }
}

impl<O, F, const TAG: u32> core::fmt::Debug
  for Len<EncodeReflection<TagReflection<ObjectFieldReflection<O, F::Identifier, F, TAG>>>>
where
  O: ?Sized,
  F: ?Sized + Flavor,
  Self: Reflectable<O, Reflection = usize>,
  F::Identifier: core::fmt::Debug,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::ops::Deref::deref(self).fmt(f)
  }
}

impl<O, F, const TAG: u32> core::fmt::Display
  for Len<EncodeReflection<TagReflection<ObjectFieldReflection<O, F::Identifier, F, TAG>>>>
where
  O: ?Sized,
  F: ?Sized + Flavor,
  Self: Reflectable<O, Reflection = usize>,
  F::Identifier: core::fmt::Display,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::ops::Deref::deref(self).fmt(f)
  }
}

impl<O, F, const TAG: u32> core::ops::Deref
  for Len<EncodeReflection<TagReflection<ObjectFieldReflection<O, F::Identifier, F, TAG>>>>
where
  O: ?Sized,
  F: ?Sized + Flavor,
  Self: Reflectable<O, Reflection = usize>,
{
  type Target = usize;

  #[inline]
  fn deref(&self) -> &Self::Target {
    <Self as Reflectable<O>>::REFLECTION
  }
}

impl<R> TagReflection<R>
where
  R: ?Sized,
{
  /// Returns the reflection to the encoded tag.
  #[inline]
  pub const fn encoded<T>(&self) -> &'static [u8]
  where
    T: ?Sized,
    EncodeReflection<Self>: Reflectable<T, Reflection = [u8]>,
  {
    <EncodeReflection<Self> as Reflectable<T>>::REFLECTION
  }

  /// Returns the reflection to the length of the encoded tag.
  #[inline]
  pub const fn encoded_len<T>(&self) -> usize
  where
    T: ?Sized,
    Len<EncodeReflection<Self>>: Reflectable<T, Reflection = usize>,
  {
    *<Len<EncodeReflection<Self>> as Reflectable<T>>::REFLECTION
  }
}
