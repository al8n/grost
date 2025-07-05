use ghost::phantom;

use crate::flavors::WireFormat;

use super::Groto;

wire_format!(
  WireType<Groto>
  /// The varint encoding/decoding wire format
  "varint",
  /// The length-delimited encoding/decoding wire format
  "length-delimited",
  /// The fixed 8-bit length encoding/decoding wire format
  "fixed8",
  /// The fixed 16-bit length encoding/decoding wire format
  "fixed16",
  /// The fixed 32-bit length encoding/decoding wire format
  "fixed32",
  /// The fixed 64-bit length encoding/decoding wire format
  "fixed64",
  /// The fixed 128-bit length encoding/decoding wire format
  "fixed128",
);

wire_type!(
  enum WireType<Groto> {
    /// The nullable encoding/decoding wire format
    "nullable" = 0,
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

/// A wire format for borrowed data.
///
/// ## Why we need `Borrowed` wire format wrapper?
///
/// e.g. when implementing `Encode` for `[T]`, we know that if `[T]` is encodable,
/// then `[&T]` must be encodable. But we cannot write the following code,
/// as `T` must be `Sized`, requried by `[T]`:
///
/// ```ignore
/// // We cannot write this code, as `T` must be `Sized`
/// impl<T: ?Sized> Encode<LengthDelimited, Groto> for [T] { ... }
/// ```
///
/// We cannot write another impl for `[&T]`, it will reports conflicting implementations.
///
/// ```ignore
/// // This implementation will conflict with the above one.
/// impl<T: ?Sized> Encode<LengthDelimited, Groto> for [&T] { ... }
/// ```
///
/// Hence, we need this wrapper type to indicate that the data is in borrowed state,
/// and we can implement `Encode` for `[&T]` like this:
///
/// ```ignore
/// impl<'a, T: ?Sized> Encode<Groto, Borrowed<'a, LengthDelimited>> for [&'a T] { ... }
/// ```
///
/// This is quite useful, when you have the following struct:
///
/// ```ignore
/// struct Friends(Vec<String>);
/// ```
///
/// And somehow, in a gossip groto, you receive three friends names from different peers.
///
/// ```ignore
/// let peer1_subscriber = subscribe_peer(1);
/// let peer2_subscriber = subscribe_peer(2);
/// let peer3_subscriber = subscribe_peer(3);
///
/// let alice_from_peer1 = peer1_subscriber.next().await;
/// let bob_from_peer2 = peer2_subscriber.next().await;
/// let charlie_from_peer3 = peer3_subscriber.next().await;
///
/// let borrowed_friends: &[&[&str]] = &[&alice_from_peer1, &bob_from_peer2, &charlie_from_peer3];
/// ```
///
/// Tranditional serialization frameworks would require you to create a new `Vec<String>`
/// and encode `&[String]`. With the `Borrowed` wrapper, you do not need to create a new `&[String]`
/// to encode it, you can just encode the borrowed slice directly as there is a blanket
/// implementation in this crate.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[phantom]
pub struct Borrowed<'a, W: ?Sized>;

impl<'a, W: ?Sized> core::fmt::Display for Borrowed<'a, W> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("borrowed")
  }
}

impl<'a, W: WireFormat<Groto>> From<Borrowed<'a, W>> for WireType {
  fn from(_: Borrowed<'a, W>) -> Self {
    W::WIRE_TYPE
  }
}

impl<'a, W: WireFormat<Groto>> WireFormat<Groto> for Borrowed<'a, W> {
  const NAME: &'static str = "borrowed";
  const WIRE_TYPE: WireType = W::WIRE_TYPE;
  const SELF: Self = Borrowed;
}

/// A wire format for nested data to encode it as a flattened structure.
///
/// ## Why we need `Flatten` wire format wrapper?
///
/// e.g. when implementing `Encode` for `Vec<Vec<T>>`, in some case, we want it to be encoded as `Vec<T>`:
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[phantom]
pub struct Flatten<W: ?Sized, I: ?Sized>;

impl<W: ?Sized, I: ?Sized> core::fmt::Display for Flatten<W, I> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("flatten")
  }
}

impl<W: WireFormat<Groto>, I: WireFormat<Groto>> From<Flatten<W, I>> for WireType {
  fn from(_: Flatten<W, I>) -> Self {
    W::WIRE_TYPE
  }
}

impl<W: WireFormat<Groto>, I: WireFormat<Groto>> WireFormat<Groto> for Flatten<W, I> {
  const NAME: &'static str = "flatten";
  const WIRE_TYPE: WireType = W::WIRE_TYPE;
  const SELF: Self = Flatten;
}

/// A wire format for packed repeated fields.
///
/// This is used to encode repeated fields in a more compact form,
/// a length prefix is used to indicate the number of bytes that the total packed data occupies.
///
/// For example, if you have a repeated field of integers, the packed format will encode them as:
///
/// ```text
/// | identifier | total_length | elem1 | elem2 | elem3 | ...
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[phantom]
pub struct Packed<W: ?Sized>;

impl<W: ?Sized> core::fmt::Display for Packed<W> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("packed")
  }
}

impl<W: WireFormat<Groto>> From<Packed<W>> for WireType {
  fn from(_: Packed<W>) -> Self {
    WireType::LengthDelimited
  }
}

impl<W: WireFormat<Groto>> WireFormat<Groto> for Packed<W> {
  const NAME: &'static str = "packed";
  const WIRE_TYPE: WireType = WireType::LengthDelimited;
  const SELF: Self = Packed;
}

/// A wire format for packed entry repeated fields.
///
/// This is used to encode repeated fields in a more compact form,
/// a length prefix is used to indicate the number of bytes that the total packed data occupies.
///
/// For example, if you have a repeated field of integers, the packed format will encode them as:
///
/// ```text
/// | identifier | total_length | ent1 | ent2 | ent3 | ...
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[phantom]
pub struct PackedEntry<KW: ?Sized, VW: ?Sized>;

impl<KW: ?Sized, VW: ?Sized> core::fmt::Display for PackedEntry<KW, VW> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("packed_entry")
  }
}

impl<KW: WireFormat<Groto>, VW: WireFormat<Groto>> From<PackedEntry<KW, VW>> for WireType {
  fn from(_: PackedEntry<KW, VW>) -> Self {
    WireType::LengthDelimited
  }
}

impl<KW: WireFormat<Groto>, VW: WireFormat<Groto>> WireFormat<Groto> for PackedEntry<KW, VW> {
  const NAME: &'static str = "packed_entry";
  const WIRE_TYPE: WireType = WireType::LengthDelimited;
  const SELF: Self = PackedEntry;
}

/// A wire format for nullable fields, in Rust it is used to represent `Option<T>` fields,
/// in schema, it corresponds to fields without `!`.
///
/// This wire format will make it have different encoding behavior for repeated fields.
/// For non-repeated fields, its encoding behavior just the same as inner wire format.
///
/// But, for `Packed<Nullable<W>>`, or `Repeated<Nullable<W>>`, it will be encoded differently.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[phantom]
pub struct Nullable<W: ?Sized>;

impl<W: ?Sized> core::fmt::Display for Nullable<W> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("nullable")
  }
}

impl<W: WireFormat<Groto>> From<Nullable<W>> for WireType {
  fn from(_: Nullable<W>) -> Self {
    W::WIRE_TYPE
  }
}

impl<W: WireFormat<Groto>> WireFormat<Groto> for Nullable<W> {
  const NAME: &'static str = "nullable";
  const WIRE_TYPE: WireType = W::WIRE_TYPE;
  const SELF: Self = Nullable;
}

/// The repeated wire format for element encoding within repeated fields.
///
/// When used as `Repeated<Repeated>`, this changes the encoding strategy
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
/// `Repeated` encoding repeats the field identifier for each element:
///
/// ```text
/// | identifier | elem1 | identifier | elem2 | identifier | elem3 | ... |
/// ```
///
/// This format is useful for:
/// - Streaming scenarios where you need to process elements before receiving the entire collection
/// - Compatibility with Protocol Buffer's default repeated field encoding
/// - Cases where elements may be added incrementally

// TODO(al8n): change const `I: u32` to `I: Identifier` wihen `feature(adt_const_params)` is stable
#[phantom]
pub struct Repeated<W: ?Sized, B: ?Sized, const I: u32>;

impl<W: ?Sized, B: ?Sized, const I: u32> Clone for Repeated<W, B, I> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<W: ?Sized, B: ?Sized, const I: u32> Copy for Repeated<W, B, I> {}

impl<W: ?Sized, B: ?Sized, const I: u32> core::hash::Hash for Repeated<W, B, I> {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    I.hash(state);
  }
}

impl<W: ?Sized, B: ?Sized, const I: u32> PartialEq for Repeated<W, B, I> {
  fn eq(&self, _: &Self) -> bool {
    true
  }
}

impl<W: ?Sized, B: ?Sized, const I: u32> Eq for Repeated<W, B, I> {}

impl<W: ?Sized, B: ?Sized, const I: u32> core::fmt::Display for Repeated<W, B, I> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("repeated")
  }
}

impl<W: ?Sized, B: ?Sized, const I: u32> core::fmt::Debug for Repeated<W, B, I> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("Repeated")
  }
}

impl<W, B, const I: u32> WireFormat<Groto> for Repeated<W, B, I>
where
  W: WireFormat<Groto>,
  B: ?Sized + core::hash::Hash + Eq,
{
  const NAME: &'static str = "repeated";
  const WIRE_TYPE: WireType = W::WIRE_TYPE;
  const SELF: Self = Repeated;
}

impl<const I: u32, W, B> From<Repeated<W, B, I>> for WireType
where
  W: WireFormat<Groto>,
  B: ?Sized,
{
  fn from(_: Repeated<W, B, I>) -> Self {
    W::WIRE_TYPE
  }
}
