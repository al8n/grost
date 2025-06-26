use core::marker::PhantomData;

use crate::flavors::WireFormat;

use super::Groto;

wire_type!(
  enum WireType<Groto> {
    /// The varint encoding/decoding wire format
    "varint" = 0,
    /// The length-delimited encoding/decoding wire format
    "length-delimited" = 1,
    /// The fixed 8-bit length encoding/decoding wire format
    "fixed8" = 2,
    /// The fixed 16-bit length encoding/decoding wire format
    "fixed16" = 3,
    /// The fixed 32-bit length encoding/decoding wire format
    "fixed32" = 4,
    /// The fixed 64-bit length encoding/decoding wire format
    "fixed64" = 5,
    /// The fixed 128-bit length encoding/decoding wire format
    "fixed128" = 6,
    /// The fixed 256-bit length encoding/decoding wire format
    "fixed256" = 7,
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
/// impl<T: ?Sized> Encode<Groto, LengthDelimited> for [T] { ... }
/// ```
///
/// We cannot write another impl for `[&T]`, it will reports conflicting implementations.
///
/// ```ignore
/// // This implementation will conflict with the above one.
/// impl<T: ?Sized> Encode<Groto, LengthDelimited> for [&T] { ... }
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
/// implementation `impl<'a, T> Encode<Groto, Borrowed<'a, LengthDelimited>> for [&'a T]` in
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

impl<'a, W: WireFormat<Groto>> From<Borrowed<'a, W>> for WireType {
  fn from(_: Borrowed<'a, W>) -> Self {
    W::WIRE_TYPE
  }
}

impl<'a, W: WireFormat<Groto>> WireFormat<Groto> for Borrowed<'a, W> {
  const NAME: &'static str = "borrowed";
  const WIRE_TYPE: WireType = W::WIRE_TYPE;
  const SELF: Self = Self(PhantomData);
  const REPEATED: bool = W::REPEATED;
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
/// impl<T: ?Sized> Encode<Groto, LengthDelimited> for [T] { ... }
/// ```
///
/// We cannot write another impl for `[&T]`, it will reports conflicting implementations.
///
/// ```ignore
/// // This implementation will conflict with the above one.
/// impl<T: ?Sized> Encode<Groto, LengthDelimited> for [&T] { ... }
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
/// implementation `impl<'a, T> Encode<Groto, Borrowed<'a, LengthDelimited>> for [&'a T]` in
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

impl<W: WireFormat<Groto>> From<Packed<W>> for WireType {
  fn from(_: Packed<W>) -> Self {
    W::WIRE_TYPE
  }
}

impl<W: WireFormat<Groto>> WireFormat<Groto> for Packed<W> {
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

impl<W: WireFormat<Groto>, I: WireFormat<Groto>> From<Flatten<W, I>> for WireType {
  fn from(_: Flatten<W, I>) -> Self {
    W::WIRE_TYPE
  }
}

impl<W: WireFormat<Groto>, I: WireFormat<Groto>> WireFormat<Groto> for Flatten<W, I> {
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
/// `Stream` encoding repeats the field identifier for each element:
///
/// ```text
/// | identifier | elem1 | identifier | elem2 | identifier | elem3 | ... |
/// ```
///
/// This format is useful for:
/// - Streaming scenarios where you need to process elements before receiving the entire collection
/// - Compatibility with Protocol Buffer's default repeated field encoding
/// - Cases where elements may be added incrementally
#[derive(derive_more::Display)]
#[display("stream")]
// TODO(al8n): change const `I: u32` to `I: Identifier` wihen `feature(adt_const_params)` is stable
pub struct Stream<W: ?Sized, B: ?Sized, const I: u32> {
  _w: PhantomData<W>,
  _b: PhantomData<B>,
}

impl<W: ?Sized, B: ?Sized, const I: u32> Clone for Stream<W, B, I> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<W: ?Sized, B: ?Sized, const I: u32> Copy for Stream<W, B, I> {}

impl<W: ?Sized, B: ?Sized, const I: u32> PartialEq for Stream<W, B, I> {
  fn eq(&self, _: &Self) -> bool {
    true
  }
}

impl<W: ?Sized, B: ?Sized, const I: u32> Eq for Stream<W, B, I> {}

impl<W: ?Sized, B: ?Sized, const I: u32> core::hash::Hash for Stream<W, B, I> {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self._w.hash(state);
    self._b.hash(state);
    I.hash(state);
  }
}

impl<W: ?Sized, B: ?Sized, const I: u32> core::fmt::Debug for Stream<W, B, I> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("Stream").finish()
  }
}

impl<W, B, const I: u32> WireFormat<Groto> for Stream<W, B, I>
where
  W: WireFormat<Groto>,
  B: ?Sized,
{
  const NAME: &'static str = "stream";
  const WIRE_TYPE: WireType = W::WIRE_TYPE;
  const SELF: Self = Self {
    _w: PhantomData,
    _b: PhantomData,
  };
  const REPEATED: bool = true;
}

impl<const I: u32, W, B> From<Stream<W, B, I>> for WireType
where
  W: WireFormat<Groto>,
  B: ?Sized,
{
  fn from(_: Stream<W, B, I>) -> Self {
    W::WIRE_TYPE
  }
}
