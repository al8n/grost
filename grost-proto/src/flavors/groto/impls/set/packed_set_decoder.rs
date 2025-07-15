use core::iter::FusedIterator;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  convert::{Flattened, Partial, PartialRef, Ref},
  decode::Decode1,
  encode::Encode,
  flavors::{
    Groto, Packed, WireFormat,
    groto::{Context, Error, PackedDecoder, PackedDecoderIter},
  },
  selection::Selectable,
  state::State,
};

/// A lazy decoder for repeated types (e.g. `HashSet<T>`, `IndexSet<T>`, `BTreeSet<T>`, and etc.) of data that
/// iterates through the underlying buffer and decode elements on demand.
///
/// `PackedDecoder` provides functionality to decode list-like structures from binary data.
/// It operates lazily, decoding elements only when requested through iteration.
pub struct PackedSetDecoder<'a, T: ?Sized, B, UB: ?Sized, W: ?Sized>(
  PackedDecoder<'a, T, B, UB, W>,
);

impl<'a, T: ?Sized, B: Clone, UB: ?Sized, W: ?Sized> Clone for PackedSetDecoder<'a, T, B, UB, W> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<'a, T: ?Sized, B: Copy, UB: ?Sized, W: ?Sized> Copy for PackedSetDecoder<'a, T, B, UB, W> {}

impl<'de, T, B, UB, W> PackedSetDecoder<'de, T, B, UB, W>
where
  T: ?Sized,
  UB: ?Sized,
  W: ?Sized,
{
  /// Returns an iterator over the elements in the packed set decoder.
  ///
  /// The iterator will yield decoded elements one by one until it reaches the end of the source data.
  pub const fn iter<'a>(&'a self) -> PackedSetDecoderIter<'a, 'de, T, B, UB, W> {
    PackedSetDecoderIter(self.0.iter())
  }

  /// Returns the expected number of elements in the packed set decoder.
  pub const fn expected_count(&self) -> usize {
    self.0.expected_count()
  }
}

pub struct PackedSetDecoderIter<'a, 'de: 'a, T: ?Sized, B, UB: ?Sized, W: ?Sized>(
  PackedDecoderIter<'a, 'de, T, B, UB, W>,
);

impl<'a, 'de: 'a, T, B, UB, W> PackedSetDecoderIter<'a, 'de, T, B, UB, W>
where
  T: ?Sized + 'de,
  UB: ?Sized,
  W: ?Sized,
{
  /// Returns the current byte position within the source buffer.
  ///
  /// This represents how many bytes have been consumed during iteration.
  #[inline]
  pub const fn current_position(&self) -> usize {
    self.0.current_position()
  }

  /// Returns the number of elements that have been successfully decoded so far.
  ///
  /// This count increases each time `next()` successfully returns an element.
  /// It will never exceed the expected element count from the wire format.
  #[inline]
  pub const fn decoded_count(&self) -> usize {
    self.0.decoded_count()
  }

  /// Returns the number of elements expected according to the wire format.
  ///
  /// This value is read from the wire format during initialization. It represents
  /// what the encoder claimed to have written, but the decoder might not successfully
  /// yield all these elements due to errors, corruption, or early termination.
  ///
  /// Use this for pre-allocating containers, but be prepared that the actual
  /// number of successfully decoded elements might be less.
  #[inline]
  pub const fn expected_count(&self) -> usize {
    self.0.expected_count()
  }

  /// Returns the number of elements remaining to be attempted, or `None` if an error occurred.
  ///
  /// This is calculated as `expected_count - decoded_count`. Note that this represents
  /// how many elements the decoder will *attempt* to decode, not a guarantee of how
  /// many will be successfully decoded.
  ///
  /// If the decoder has encountered an error during iteration, this returns `None`
  /// to indicate that the remaining count is unreliable.
  ///
  /// # Returns
  ///
  /// - `Some(count)` - Number of elements left to attempt decoding
  /// - `None` - An error occurred and remaining count is unknown
  #[inline]
  pub const fn remaining_expected(&self) -> Option<usize> {
    self.0.remaining_expected()
  }
}

impl<'a, 'de: 'a, RB, B, W, T> Iterator for PackedSetDecoderIter<'a, 'de, T, RB, B, W>
where
  W: WireFormat<Groto> + 'de,
  T: Decode1<'de, W, RB, B, Groto> + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
{
  type Item = Result<(usize, T), Error>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.0.next()
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, self.remaining_expected())
  }
}

impl<'a, 'de: 'a, RB, B, W, T> FusedIterator for PackedSetDecoderIter<'a, 'de, T, RB, B, W>
where
  W: WireFormat<Groto> + 'de,
  T: Decode1<'de, W, RB, B, Groto> + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
{
}

impl<'a, T, B, UB, W> Selectable<Groto> for PackedSetDecoder<'a, T, B, UB, W>
where
  T: ?Sized + Selectable<Groto>,
  W: WireFormat<Groto> + 'a,
  UB: ?Sized,
{
  type Selector = T::Selector;
}

impl<'a, T, RB, B, W> Encode<Packed<W>, Groto> for PackedSetDecoder<'a, T, RB, B, W>
where
  T: ?Sized,
  Packed<W>: WireFormat<Groto> + 'a,
  B: ?Sized,
  RB: ReadBuf,
{
  fn encode_raw(&self, ctx: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    self.0.encode_raw(ctx, buf)
  }

  fn encoded_raw_len(&self, ctx: &Context) -> usize {
    self.0.encoded_raw_len(ctx)
  }

  fn encode(&self, ctx: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    self.0.encode(ctx, buf)
  }

  fn encoded_len(&self, ctx: &Context) -> usize {
    self.0.encoded_len(ctx)
  }
}

impl<'a, T, B, W, RB> Decode1<'a, Packed<W>, RB, B, Groto> for PackedSetDecoder<'a, T, RB, B, W>
where
  Packed<W>: WireFormat<Groto> + 'a,
  RB: ReadBuf,
{
  fn decode(
    ctx: &'a <Groto as crate::flavors::Flavor>::Context,
    src: RB,
  ) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: crate::buffer::ReadBuf,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let (read, packed_decoder) = PackedDecoder::decode(ctx, src)?;
    Ok((read, PackedSetDecoder(packed_decoder)))
  }
}

impl<'a, T, B, UB, W> State<Partial<Groto>> for PackedSetDecoder<'a, T, B, UB, W>
where
  T: ?Sized,
  W: ?Sized,
  UB: ?Sized,
{
  type Output = Self;
}

impl<'a, T, B, UB, W> State<PartialRef<'a, B, UB, Packed<W>, Groto>>
  for PackedSetDecoder<'a, T, B, UB, W>
where
  T: ?Sized,
  W: ?Sized,
  UB: ?Sized,
{
  type Output = Self;
}

impl<'a, T, B, UB, W> State<Ref<'a, B, UB, Packed<W>, Groto>> for PackedSetDecoder<'a, T, B, UB, W>
where
  T: ?Sized,
  W: ?Sized,
  UB: ?Sized,
{
  type Output = Self;
}

impl<'a, T, B, UB, W, S> State<Flattened<S>> for PackedSetDecoder<'a, T, B, UB, W>
where
  T: ?Sized,
  W: ?Sized,
  UB: ?Sized,
  S: ?Sized,
{
  type Output = Self;
}
