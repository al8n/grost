use crate::{
  buffer::ReadBuf,
  convert::{Flattened, State},
  encode::{Encode, PartialEncode},
  flavors::{Flavor, Groto, groto::LengthDelimited},
  selection::Selectable,
};

/// The decoded type for `str`
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Str<RB: ?Sized>(RB);

impl<RB> Default for Str<RB>
where
  RB: ReadBuf,
{
  fn default() -> Self {
    Self(RB::empty())
  }
}

impl<RB: ?Sized, F: ?Sized + Flavor> Selectable<F> for Str<RB> {
  type Selector = bool;
}

impl<RB: ?Sized, O> State<Flattened<O>> for Str<RB> {
  type Output = Self;
}

impl<RB> core::borrow::Borrow<str> for Str<RB>
where
  RB: ReadBuf,
{
  fn borrow(&self) -> &str {
    self
  }
}

impl<RB> core::hash::Hash for Str<RB>
where
  RB: ReadBuf,
{
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.as_ref().hash(state);
  }
}

impl<RB> core::fmt::Debug for Str<RB>
where
  RB: ReadBuf,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    self.as_ref().fmt(f)
  }
}

impl<RB> core::fmt::Display for Str<RB>
where
  RB: ReadBuf,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    self.as_ref().fmt(f)
  }
}

impl<RB> core::ops::Deref for Str<RB>
where
  RB: ReadBuf,
{
  type Target = str;

  fn deref(&self) -> &Self::Target {
    // SAFETY: We guarantee that the bytes is valid UTF-8 when constructing this type,
    // so we can safely convert it to a `&str` without checking.
    unsafe { core::str::from_utf8_unchecked(self.0.as_bytes()) }
  }
}

impl<RB> AsRef<str> for Str<RB>
where
  RB: ReadBuf,
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
    RB: Sized + ReadBuf,
  {
    if core::str::from_utf8(buf.as_bytes()).is_ok() {
      Ok(Self(buf))
    } else {
      Err(buf)
    }
  }

  /// Creates an empty `Str`.
  #[inline]
  pub fn empty() -> Self
  where
    RB: Sized + ReadBuf,
  {
    Self(RB::empty())
  }
}

impl<RB> Encode<LengthDelimited, Groto> for Str<RB>
where
  RB: ReadBuf,
{
  fn encode_raw(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
  ) -> Result<usize, <Groto as Flavor>::Error> {
    <str as Encode<LengthDelimited, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_raw_len(&self, context: &<Groto as Flavor>::Context) -> usize {
    <str as Encode<LengthDelimited, Groto>>::encoded_raw_len(self, context)
  }

  fn encode(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
  ) -> Result<usize, <Groto as Flavor>::Error> {
    <str as Encode<LengthDelimited, Groto>>::encode(self, context, buf)
  }

  fn encoded_len(&self, context: &<Groto as Flavor>::Context) -> usize {
    <str as Encode<LengthDelimited, Groto>>::encoded_len(self, context)
  }
}

impl<RB> PartialEncode<LengthDelimited, Groto> for Str<RB>
where
  RB: ReadBuf,
{
  fn partial_encode_raw(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, <Groto as Flavor>::Error> {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encode_raw(self, context, buf, selector)
  }

  fn partial_encoded_raw_len(
    &self,
    context: &<Groto as Flavor>::Context,
    selector: &Self::Selector,
  ) -> usize {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encoded_raw_len(self, context, selector)
  }

  fn partial_encode(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, <Groto as Flavor>::Error> {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encode(self, context, buf, selector)
  }

  fn partial_encoded_len(
    &self,
    context: &<Groto as Flavor>::Context,
    selector: &Self::Selector,
  ) -> usize {
    <str as PartialEncode<LengthDelimited, Groto>>::partial_encoded_len(self, context, selector)
  }
}
