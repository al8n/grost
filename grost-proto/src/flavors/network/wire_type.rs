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
/// impl<T: ?Sized> Encode<Network, LengthDelimited> for [T] { ... }
/// ```
///
/// We cannot write another impl for `[&T]`, it will reports conflicting implementations.
///
/// ```ignore
/// // This implementation will conflict with the above one.
/// impl<T: ?Sized> Encode<Network, LengthDelimited> for [&T] { ... }
/// ```
///
/// Hence, we need this wrapper type to indicate that the data is in borrowed state,
/// and we can implement `Encode` for `[&T]` like this:
///
/// ```ignore
/// impl<'a, T: ?Sized> Encode<Network, Borrowed<'a, LengthDelimited>> for [&'a T] { ... }
/// ```
///
/// This is quite useful, when you have the following struct:
///
/// ```ignore
/// struct Friends(Vec<String>);
/// ```
///
/// And somehow, in a gossip network, you receive three friends names from different peers.
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
/// let borrowed_friends: &[&str] = &[alice_from_peer1, bob_from_peer2, charlie_from_peer3];
/// ```
///
/// Tranditional serialization frameworks would require you to create a new `Vec<String>`
/// and encode `&[String]`. With the `Borrowed` wrapper, you do not need to create a new `[&String]`
/// to encode it, you can just encode the borrowed slice directly as there is a blanket
/// implementation `impl<'a, T> Encode<Network, Borrowed<'a, LengthDelimited>> for [&'a T]` in
/// this crate.
#[derive(Debug, PartialEq, Eq, Hash, derive_more::Display)]
#[display("borrowed")]
pub struct Borrowed<'a, W: ?Sized>(PhantomData<&'a W>);

impl<W: ?Sized> Clone for Borrowed<'_, W> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<W: ?Sized> Copy for Borrowed<'_, W> {}

impl<'a, W: WireFormat<Network>> From<Borrowed<'a, W>> for WireType {
  fn from(_: Borrowed<'a, W>) -> Self {
    W::WIRE_TYPE
  }
}

impl<'a, W: WireFormat<Network>> WireFormat<Network> for Borrowed<'a, W> {
  const NAME: &'static str = "borrowed";
  const WIRE_TYPE: WireType = W::WIRE_TYPE;
  const SELF: Self = Self(PhantomData);
}

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
/// impl<T: ?Sized> Encode<Network, LengthDelimited> for [T] { ... }
/// ```
///
/// We cannot write another impl for `[&T]`, it will reports conflicting implementations.
///
/// ```ignore
/// // This implementation will conflict with the above one.
/// impl<T: ?Sized> Encode<Network, LengthDelimited> for [&T] { ... }
/// ```
///
/// Hence, we need this wrapper type to indicate that the data is in borrowed state,
/// and we can implement `Encode` for `[&T]` like this:
///
/// ```ignore
/// impl<'a, T: ?Sized> Encode<Network, Borrowed<'a, LengthDelimited>> for [&'a T] { ... }
/// ```
///
/// This is quite useful, when you have the following struct:
///
/// ```ignore
/// struct Friends(Vec<String>);
/// ```
///
/// And somehow, in a gossip network, you receive three friends names from different peers.
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
/// let borrowed_friends: &[&str] = &[alice_from_peer1, bob_from_peer2, charlie_from_peer3];
/// ```
///
/// Tranditional serialization frameworks would require you to create a new `Vec<String>`
/// and encode `&[String]`. With the `Borrowed` wrapper, you do not need to create a new `[&String]`
/// to encode it, you can just encode the borrowed slice directly as there is a blanket
/// implementation `impl<'a, T> Encode<Network, Borrowed<'a, LengthDelimited>> for [&'a T]` in
/// this crate.
#[derive(Debug, PartialEq, Eq, Hash, derive_more::Display)]
#[display("packed")]
pub struct Packed<W: ?Sized>(PhantomData<W>);

impl<W: ?Sized> Clone for Packed<W> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<W: ?Sized> Copy for Packed<W> {}

impl<W: WireFormat<Network>> From<Packed<W>> for WireType {
  fn from(_: Packed<W>) -> Self {
    W::WIRE_TYPE
  }
}

impl<W: WireFormat<Network>> WireFormat<Network> for Packed<W> {
  const NAME: &'static str = "packed";
  const WIRE_TYPE: WireType = W::WIRE_TYPE;
  const SELF: Self = Self(PhantomData);
}

#[derive(Debug, PartialEq, Eq, Hash, derive_more::Display)]
#[display("flatten")]
pub struct Flatten<W: ?Sized, I: ?Sized> {
  _w: PhantomData<W>,
  _i: PhantomData<I>,
}

impl<W: ?Sized, I: ?Sized> Clone for Flatten<W, I> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<W: ?Sized, I: ?Sized> Copy for Flatten<W, I> {}

impl<W: WireFormat<Network>, I: WireFormat<Network>> From<Flatten<W, I>> for WireType {
  fn from(_: Flatten<W, I>) -> Self {
    W::WIRE_TYPE
  }
}

impl<W: WireFormat<Network>, I: WireFormat<Network>> WireFormat<Network> for Flatten<W, I> {
  const NAME: &'static str = "flatten";
  const WIRE_TYPE: WireType = W::WIRE_TYPE;
  const SELF: Self = Self {
    _w: PhantomData,
    _i: PhantomData,
  };
}

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
        const SELF: Self = Self(PhantomData);
      }

      impl<const I: u32> WireFormat<Network> for Repeated<Stream<$wf, I>> {
        const NAME: &'static str = "repeated";
        const WIRE_TYPE: WireType = <$wf>::WIRE_TYPE;
        const SELF: Self = Self(PhantomData);
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
