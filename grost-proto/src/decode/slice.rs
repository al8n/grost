use crate::{
  buffer::ReadBuf,
  convert::{Flattened, State},
  flavors::Flavor,
  selection::Selectable,
};

/// The decoded type for `[u8]`
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BytesSlice<RB: ?Sized>(RB);

impl<RB: ReadBuf> Default for BytesSlice<RB> {
  fn default() -> Self {
    Self::empty()
  }
}

impl<RB: ?Sized, F: ?Sized + Flavor> Selectable<F> for BytesSlice<RB> {
  type Selector = bool;

  fn is_empty(&self) -> bool {
    false
  }
}

impl<RB: ?Sized, O> State<Flattened<O>> for BytesSlice<RB> {
  type Output = Self;
}

impl<RB> core::borrow::Borrow<[u8]> for BytesSlice<RB>
where
  RB: ReadBuf,
{
  fn borrow(&self) -> &[u8] {
    self
  }
}

impl<RB> core::hash::Hash for BytesSlice<RB>
where
  RB: ReadBuf,
{
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.as_ref().hash(state);
  }
}

impl<RB> core::fmt::Debug for BytesSlice<RB>
where
  RB: ReadBuf,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    self.as_ref().fmt(f)
  }
}

impl<RB> core::ops::Deref for BytesSlice<RB>
where
  RB: ReadBuf,
{
  type Target = [u8];

  fn deref(&self) -> &Self::Target {
    self.0.as_bytes()
  }
}

impl<RB> AsRef<[u8]> for BytesSlice<RB>
where
  RB: ReadBuf,
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
    RB: ReadBuf,
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
  pub fn as_slice(&self) -> &[u8]
  where
    RB: ReadBuf,
  {
    self
  }
}
