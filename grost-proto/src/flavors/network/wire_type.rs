use core::marker::PhantomData;

use crate::flavors::WireFormat;

use super::Network;

wire_type!(
  enum WireType<Network> {
    /// The zero-sized type wire format, this wire format requires no information included.
    "zst" = 0,
    /// The varint encoding/decoding wire format
    "varint" = 1,
    /// The length-delimited encoding/decoding wire format
    "length-delimited" = 2,
    /// The fixed 8-bit length encoding/decoding wire format
    "fixed8" = 3,
    /// The fixed 16-bit length encoding/decoding wire format
    "fixed16" = 4,
    /// The fixed 32-bit length encoding/decoding wire format
    "fixed32" = 5,
    /// The fixed 64-bit length encoding/decoding wire format
    "fixed64" = 6,
    /// The fixed 128-bit length encoding/decoding wire format
    "fixed128" = 7,
  }
);

/// The wire format for repeated.
#[derive(PartialEq, Eq, Hash)]
pub struct Repeated<W: ?Sized>(PhantomData<W>);

impl<W: ?Sized> From<Repeated<W>> for WireType {
  fn from(_: Repeated<W>) -> Self {
    WireType::LengthDelimited
  }
}

impl<W> WireFormat<Network> for Repeated<W>
where
  W: WireFormat<Network>,
{
  const NAME: &'static str = "repeated";
  const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

impl<W> core::fmt::Debug for Repeated<W>
where
  W: core::fmt::Debug,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_tuple("Repeated").finish()
  }
}

impl<W> core::fmt::Display for Repeated<W>
where
  W: ?Sized + core::fmt::Display,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "repeated")
  }
}

impl<W: ?Sized> Default for Repeated<W> {
  /// Returns the wire format for repeated.
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl<W: ?Sized> Repeated<W> {
  /// Returns the wire format for repeated.
  #[inline]
  pub const fn new() -> Self {
    Self(PhantomData)
  }
}

impl<W: ?Sized> Clone for Repeated<W> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<W: ?Sized> Copy for Repeated<W> {}
