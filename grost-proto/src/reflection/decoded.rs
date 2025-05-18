use core::marker::PhantomData;

use crate::flavors::Flavor;

use super::{Identified, ObjectField, Reflectable, Reflection};

/// A phantom relection type which can be dereferenced to [`Reflectable::REFLECTION`].
#[repr(transparent)]
pub struct DecodedReflection<T: ?Sized, R: ?Sized, F: ?Sized> {
  _r: PhantomData<R>,
  _f: PhantomData<F>,
  _t: PhantomData<T>,
}

impl<T, R, F> Default for DecodedReflection<T, R, F>
where
  T: ?Sized,
  R: ?Sized,
  F: ?Sized,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<T, R, F> DecodedReflection<T, R, F>
where
  T: ?Sized,
  R: ?Sized,
  F: ?Sized,
{
  /// Creates a new [`DecodedReflection`].
  #[inline]
  pub const fn new() -> Self {
    Self {
      _r: PhantomData,
      _f: PhantomData,
      _t: PhantomData,
    }
  }
}

#[allow(clippy::type_complexity)]
impl<T, F, const TAG: u32> DecodedReflection<T, Identified<ObjectField, TAG>, F>
where
  T: ?Sized,
  F: ?Sized + Flavor,
  Reflection<T, Identified<ObjectField, TAG>, F>: Reflectable<T, Reflection = ObjectField>,
{

}
