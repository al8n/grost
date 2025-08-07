use crate::{
  buffer::Buf, convert::Extracted, flavors::Flavor, selection::Selectable, state::State,
};

/// The decoded type for `str`
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Str<RB: ?Sized>(RB);

impl<RB> Default for Str<RB>
where
  RB: Buf,
{
  fn default() -> Self {
    Self(RB::empty())
  }
}

impl<RB: ?Sized, F: ?Sized + Flavor> Selectable<F> for Str<RB> {
  type Selector = bool;
}

impl<RB: ?Sized, O> State<Extracted<O>> for Str<RB> {
  type Output = Self;
}

impl<RB> core::borrow::Borrow<str> for Str<RB>
where
  RB: Buf,
{
  fn borrow(&self) -> &str {
    self
  }
}

impl<RB> core::hash::Hash for Str<RB>
where
  RB: Buf,
{
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.as_ref().hash(state);
  }
}

impl<RB> core::fmt::Debug for Str<RB>
where
  RB: Buf,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    self.as_ref().fmt(f)
  }
}

impl<RB> core::fmt::Display for Str<RB>
where
  RB: Buf,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    self.as_ref().fmt(f)
  }
}

impl<RB> core::ops::Deref for Str<RB>
where
  RB: Buf,
{
  type Target = str;

  fn deref(&self) -> &Self::Target {
    // SAFETY: We guarantee that the bytes is valid UTF-8 when constructing this type,
    // so we can safely convert it to a `&str` without checking.
    unsafe { core::str::from_utf8_unchecked(self.0.remaining_slice()) }
  }
}

impl<RB> AsRef<str> for Str<RB>
where
  RB: Buf,
{
  fn as_ref(&self) -> &str {
    self
  }
}

impl<RB> Str<RB> {
  /// Returns the read buffer.
  #[inline]
  pub fn into_inner(self) -> RB
  where
    RB: Sized,
  {
    self.0
  }

  /// Creates a new `Str` from the given read buffer.
  ///
  /// If the buffer is not valid UTF-8, it will return the buffer back as an error.
  #[inline]
  pub fn try_new(buf: RB) -> Result<Self, RB>
  where
    RB: Sized + Buf,
  {
    if core::str::from_utf8(buf.remaining_slice()).is_ok() {
      Ok(Self(buf))
    } else {
      Err(buf)
    }
  }

  /// Creates an empty `Str`.
  #[inline]
  pub fn empty() -> Self
  where
    RB: Sized + Buf,
  {
    Self(RB::empty())
  }
}
