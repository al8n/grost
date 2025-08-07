use crate::{
  buffer::Buf, convert::Extracted, flavors::Flavor, selection::Selectable, state::State,
};

/// The decoded type for `[u8]`
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BytesSlice<RB: ?Sized>(RB);

impl<RB: Buf> Default for BytesSlice<RB> {
  fn default() -> Self {
    Self::empty()
  }
}

impl<RB: ?Sized, F: ?Sized + Flavor> Selectable<F> for BytesSlice<RB> {
  type Selector = bool;
}

impl<RB: ?Sized, O> State<Extracted<O>> for BytesSlice<RB> {
  type Output = Self;
}

impl<RB> core::borrow::Borrow<[u8]> for BytesSlice<RB>
where
  RB: Buf,
{
  fn borrow(&self) -> &[u8] {
    self
  }
}

impl<RB> core::hash::Hash for BytesSlice<RB>
where
  RB: Buf,
{
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.as_ref().hash(state);
  }
}

impl<RB> core::fmt::Debug for BytesSlice<RB>
where
  RB: Buf,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    self.as_ref().fmt(f)
  }
}

impl<RB> core::ops::Deref for BytesSlice<RB>
where
  RB: Buf,
{
  type Target = [u8];

  fn deref(&self) -> &Self::Target {
    self.0.remaining_slice()
  }
}

impl<RB> AsRef<[u8]> for BytesSlice<RB>
where
  RB: Buf,
{
  fn as_ref(&self) -> &[u8] {
    self
  }
}

impl<RB> BytesSlice<RB> {
  /// Creates a new `BytesSlice` from the given read buffer.
  #[inline]
  pub const fn new(buf: RB) -> Self
  where
    RB: Sized,
  {
    Self(buf)
  }

  /// Creates an empty `BytesSlice`.
  #[inline]
  pub fn empty() -> Self
  where
    RB: Buf,
  {
    Self(RB::empty())
  }

  /// Returns the read buffer.
  #[inline]
  pub fn into_inner(self) -> RB
  where
    RB: Sized,
  {
    self.0
  }

  /// Returns a reference to the bytes in the slice.
  #[inline]
  pub fn remaining_slice(&self) -> &[u8]
  where
    RB: Buf,
  {
    self
  }
}
