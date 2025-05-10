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

macro_rules! fixed_size {
  ($($ty:ty:$size:literal),+$(,)?) => {
    $(
      impl $ty {
        /// The size of the corresponding fixed-size type.
        pub const SIZE: usize = $size;
      }
    )*
  };
}

fixed_size!(
  Fixed8: 1,
  Fixed16: 2,
  Fixed32: 4,
  Fixed64: 8,
  Fixed128: 16,
);

/// The stream wire format for element encoding within repeated fields.
///
/// When used as `Repeated<Stream>`, this changes the encoding strategy
/// to a more stream-friendly format where each element is individually tagged.
/// This is similar to how Protocol Buffers encodes repeated fields when the `packed=false`
/// option is used.
///
/// Instead of the default length-prefixed encoding:
///
/// ```text
/// | identifier | total_length | elem1 | elem2 | elem3 | ... |
/// ```
///
/// `Repeated<Stream>` encoding repeats the field identifier for each element:
///
/// ```text
/// | identifier | elem1 | identifier | elem2 | identifier | elem3 | ... |
/// ```
///
/// This format is useful for:
/// - Streaming scenarios where you need to process elements before receiving the entire collection
/// - Compatibility with Protocol Buffer's default repeated field encoding
/// - Cases where elements may be added incrementally
#[derive(Debug, PartialEq, Eq, Hash, derive_more::Display)]
#[display("stream")]
// TODO(al8n): change const `I: u32` to `I: Identifier` wihen `feature(adt_const_params)` is stable
pub struct Stream<W: ?Sized, const I: u32>(PhantomData<W>);

/// Represents the wire format for repeated fields.
///
/// `Repeated<W>` is a marker type that indicates a field should be encoded as a repeated
/// field, where `W` is the wire format of the contained elements.
///
/// By default, repeated fields are encoded using length-prefixed encoding:
///
/// ```text
/// | identifier | total_length | elem1 | elem2 | elem3 | ... |
/// ```
///
/// However, for a streaming encoding format where each element is individually tagged with
/// the field identifier, use `Repeated<Stream>` instead. See the [`Stream`] type
/// documentation for more details.
#[derive(PartialEq, Eq, Hash)]
pub struct Repeated<W: ?Sized>(PhantomData<W>);

impl<W: ?Sized> From<Repeated<W>> for WireType {
  fn from(_: Repeated<W>) -> Self {
    WireType::LengthDelimited
  }
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

macro_rules! impl_wire_format_for_repeated {
  ($($wf:ty),+$(,)?) => {
    $(
      impl WireFormat<Network> for Repeated<$wf> {
        const NAME: &'static str = "repeated";
        const WIRE_TYPE: WireType = WireType::LengthDelimited;
      }

      impl<const I: u32> WireFormat<Network> for Repeated<Stream<$wf, I>> {
        const NAME: &'static str = "repeated";
        const WIRE_TYPE: WireType = <$wf>::WIRE_TYPE;
      }
    )*
  };
}

impl_wire_format_for_repeated!(
  Fixed8,
  Fixed16,
  Fixed32,
  Fixed64,
  Fixed128,
  Varint,
  LengthDelimited,
);
