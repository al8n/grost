use core::marker::PhantomData;

use super::Reflectable;

/// Reflection to the wire format of a field
pub struct WireFormatReflection<O: ?Sized, F: ?Sized, const TAG: u32> {
  _object: PhantomData<O>,
  _flavor: PhantomData<F>,
}

impl<O, F, const TAG: u32> Default for WireFormatReflection<O, F, TAG>
where
  O: ?Sized,
  F: ?Sized,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<O, F, const TAG: u32> WireFormatReflection<O, F, TAG>
where
  O: ?Sized,
  F: ?Sized,
{
  /// Creates a new [`WireFormatReflection`].
  #[inline]
  pub const fn new() -> Self {
    Self {
      _object: PhantomData,
      _flavor: PhantomData,
    }
  }
}

impl<O, F, const TAG: u32> core::fmt::Debug for WireFormatReflection<O, F, TAG>
where
  O: ?Sized,
  F: ?Sized,
  Self: Reflectable<O>,
  <Self as Reflectable<O>>::Reflection: core::fmt::Debug,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::ops::Deref::deref(self).fmt(f)
  }
}

impl<O, F, const TAG: u32> core::fmt::Display for WireFormatReflection<O, F, TAG>
where
  O: ?Sized,
  F: ?Sized,
  Self: Reflectable<O>,
  <Self as Reflectable<O>>::Reflection: core::fmt::Display,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::ops::Deref::deref(self).fmt(f)
  }
}

impl<O, F, const TAG: u32> core::ops::Deref for WireFormatReflection<O, F, TAG>
where
  O: ?Sized,
  F: ?Sized,
  Self: Reflectable<O>,
{
  type Target = <Self as Reflectable<O>>::Reflection;

  fn deref(&self) -> &Self::Target {
    Self::REFLECTION
  }
}
