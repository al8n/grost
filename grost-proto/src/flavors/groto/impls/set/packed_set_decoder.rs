use core::iter::FusedIterator;

use crate::{
  buffer::{Buf, BufMut, UnknownBuffer},
  convert::Extracted,
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, Packed, WireFormat,
    groto::{Context, Error, PackedDecoder, PackedDecoderIter},
  },
  selection::Selectable,
  state::{Partial, PartialRef, Ref, State},
};

/// A lazy decoder for packed set-like collections from binary protocol data.
///
/// `PackedSetDecoder` provides efficient decoding of set-like structures
/// (such as `HashSet<T>`, `IndexSet<T>`, `BTreeSet<T>`, etc.) from binary data
/// using the Groto packed format.
///
/// ## Wire Format
///
/// Uses the same packed format as [`PackedDecoder`]:
/// ```text
/// [total_length][element_count][element1][element2][element3]...
/// ```
///
/// ## Duplicate Elements
///
/// The decoder does **not** enforce uniqueness at the protocol level.
/// If the wire data contains duplicate elements, they will all be yielded
/// by the iterator. It's the responsibility of the target collection
/// (e.g., `HashSet`, `BTreeSet`) to handle duplicates according to its semantics.
///
///
/// ## Performance
///
/// - **Construction**: O(1) time and space (header parsing only)
/// - **Iteration**: O(1) amortized time per element
/// - **Memory**: Zero-copy when possible, minimal overhead
/// - **Collection building**: Depends on target set type (O(1) for hash, O(log n) for tree)
///
/// ## Error Handling
///
/// Inherits the same fail-fast error semantics as [`PackedDecoder`]:
/// - Construction errors for malformed headers
/// - Iterator errors set internal flag and stop iteration
/// - Use [`PackedSetDecoderIter::remaining_hint()`] to check error state
///
/// [`PackedDecoder`]: crate::flavors::groto::PackedDecoder
/// [`PackedSetDecoderIter::remaining_hint()`]: PackedSetDecoderIter::remaining_hint
pub struct PackedSetDecoder<'a, T, B, UB, W>(PackedDecoder<'a, T, B, UB, W>);

impl<'a, T, B: Clone, UB, W> Clone for PackedSetDecoder<'a, T, B, UB, W> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<'a, T, B: Copy, UB, W> Copy for PackedSetDecoder<'a, T, B, UB, W> {}

impl<'de, T, B, UB, W> PackedSetDecoder<'de, T, B, UB, W> {
  /// Creates an iterator over the elements in the packed set.
  ///
  /// The iterator will yield elements in the order they appear in the wire format.
  /// For unordered sets like `HashSet`, this order is typically not significant.
  /// For ordered sets like `IndexSet` or when building intermediate collections,
  /// this order may be meaningful.
  ///
  /// # Duplicate Elements
  ///
  /// If the wire data contains duplicate elements, the iterator will yield
  /// all of them. It's up to the target collection to handle duplicates
  /// according to its semantics.
  ///
  /// # Examples
  ///
  /// ```rust
  /// let decoder = PackedSetDecoder::decode(ctx, buffer)?;
  ///
  /// // Build a HashSet, automatically handling duplicates
  /// let mut set = HashSet::with_capacity(decoder.capacity_hint());
  /// for result in decoder.iter() {
  ///     if let Ok((_, element)) = result {
  ///         set.insert(element); // Duplicates automatically ignored
  ///     }
  /// }
  /// ```
  #[inline]
  pub const fn iter(&self) -> PackedSetDecoderIter<'_, 'de, T, B, UB, W> {
    PackedSetDecoderIter(self.0.iter())
  }

  /// Returns a hint for the capacity needed to store all unique elements.
  ///
  /// This value is read from the element count prefix during construction.
  /// It represents the number of elements in the wire format, which may
  /// include duplicates.
  ///
  /// **Important**:
  /// - For sets, the actual unique element count may be **less** than this hint
  /// - Use this as an **upper bound** for capacity allocation
  /// - Most set implementations handle over-allocation gracefully
  ///
  /// # Examples
  ///
  /// ```rust
  /// let decoder = PackedSetDecoder::decode(ctx, buffer)?;
  ///
  /// // Safe to use as capacity hint - over-allocation is handled well
  /// let mut set = HashSet::with_capacity(decoder.capacity_hint());
  ///
  /// // Alternative: conservative allocation for memory-sensitive scenarios
  /// let mut set = HashSet::new(); // Start small, let it grow
  /// ```
  #[inline]
  pub const fn capacity_hint(&self) -> usize {
    self.0.capacity_hint()
  }

  /// Returns whether the decoder contains any elements.
  #[inline]
  pub const fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
}

/// Iterator for lazily decoding packed set elements.
///
/// This iterator provides the same functionality as [`PackedDecoderIter`]
/// but with set-specific documentation and semantics. It yields elements
/// in wire format order, which may include duplicates.
///
/// ## Duplicate Handling
///
/// The iterator itself does not filter duplicates - it yields all elements
/// as they appear in the wire format. Duplicate filtering is handled by
/// the target set collection.
///
/// ## Error Handling
///
/// Inherits fail-fast error semantics from the underlying packed decoder:
/// - First error sets the `has_error` flag
/// - All subsequent `next()` calls return `None`
/// - Use `remaining_hint()` to detect error state
///
/// [`PackedDecoderIter`]: crate::flavors::groto::PackedDecoderIter
pub struct PackedSetDecoderIter<'a, 'de: 'a, T, B, UB, W>(PackedDecoderIter<'a, 'de, T, B, UB, W>);

impl<'a, 'de: 'a, T, B, UB, W> PackedSetDecoderIter<'a, 'de, T, B, UB, W> {
  /// Returns the current byte position within the source buffer.
  ///
  /// This represents how many bytes have been consumed during iteration
  /// from the start of the element data (after prefixes).
  #[inline]
  pub const fn position(&self) -> usize {
    self.0.position()
  }

  /// Returns the number of elements that have been successfully decoded so far.
  ///
  /// This count increases each time [`next()`] successfully returns an element.
  /// It includes all elements, even duplicates that may be ignored by the
  /// target set collection.
  ///
  /// [`next()`]: Iterator::next
  #[inline]
  pub const fn decoded(&self) -> usize {
    self.0.decoded()
  }

  /// Returns the capacity hint from the element count prefix.
  ///
  /// For sets, this represents the total number of elements in the wire format,
  /// which may include duplicates. The actual number of unique elements in
  /// the final set may be smaller.
  ///
  /// See [`PackedSetDecoder::capacity_hint()`] for more details.
  ///
  /// [`PackedSetDecoder::capacity_hint()`]: PackedSetDecoder::capacity_hint
  #[inline]
  pub const fn capacity_hint(&self) -> usize {
    self.0.capacity_hint()
  }

  /// Returns the estimated number of elements remaining, or `None` if an error occurred.
  ///
  /// This represents how many more elements will be yielded by the iterator,
  /// including any duplicates. The actual number of unique elements added
  /// to a set may be smaller.
  ///
  /// # Error Indication
  ///
  /// If the iterator has encountered an error during iteration, this returns `None`
  /// to indicate that the remaining count is unreliable.
  ///
  /// # Returns
  ///
  /// - `Some(count)` - Number of elements left to attempt decoding
  /// - `None` - An error occurred and remaining count is unreliable
  #[inline]
  pub const fn remaining_hint(&self) -> Option<usize> {
    self.0.remaining_hint()
  }

  /// Returns whether the iterator has encountered an error.
  #[inline]
  pub const fn has_error(&self) -> bool {
    self.0.has_error()
  }
}

impl<'a, 'de: 'a, RB, B, W, T> Iterator for PackedSetDecoderIter<'a, 'de, T, RB, B, W>
where
  W: WireFormat<Groto> + 'de,
  T: Decode<'de, W, RB, B, Groto> + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: Buf + 'de,
{
  type Item = Result<(usize, T), Error>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.0.next()
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, self.remaining_hint())
  }
}

impl<'a, 'de: 'a, RB, B, W, T> FusedIterator for PackedSetDecoderIter<'a, 'de, T, RB, B, W>
where
  W: WireFormat<Groto> + 'de,
  T: Decode<'de, W, RB, B, Groto> + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: Buf + 'de,
{
}

impl<'a, T, B, UB, W> Selectable<Groto> for PackedSetDecoder<'a, T, B, UB, W>
where
  T: Selectable<Groto>,
  W: WireFormat<Groto> + 'a,
{
  type Selector = T::Selector;
}

impl<'a, T, RB, B, W> Encode<Packed<W>, Groto> for PackedSetDecoder<'a, T, RB, B, W>
where
  Packed<W>: WireFormat<Groto> + 'a,
  RB: Buf,
{
  fn encode_raw<WB>(&self, ctx: &Context, buf: &mut WB) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    self.0.encode_raw(ctx, buf)
  }

  fn encoded_raw_len(&self, ctx: &Context) -> usize {
    self.0.encoded_raw_len(ctx)
  }

  fn encode<WB>(&self, ctx: &Context, buf: &mut WB) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    self.0.encode(ctx, buf)
  }

  fn encoded_len(&self, ctx: &Context) -> usize {
    self.0.encoded_len(ctx)
  }
}

impl<'a, T, RB, B, W> PartialEncode<Packed<W>, Groto> for PackedSetDecoder<'a, T, RB, B, W>
where
  W: WireFormat<Groto> + 'a,
  Packed<W>: WireFormat<Groto> + 'a,
  RB: Buf,
  T: Selectable<Groto>,
{
  fn partial_encode_raw<WB>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    self.0.partial_encode_raw(context, buf, selector)
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    self.0.partial_encoded_raw_len(context, selector)
  }

  fn partial_encode<WB>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    self.0.partial_encode(context, buf, selector)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    self.0.partial_encoded_len(context, selector)
  }
}

impl<'a, T, B, W, RB> Decode<'a, Packed<W>, RB, B, Groto> for PackedSetDecoder<'a, T, RB, B, W>
where
  Packed<W>: WireFormat<Groto> + 'a,
  RB: Buf,
{
  fn decode(
    ctx: &'a <Groto as crate::flavors::Flavor>::Context,
    src: RB,
  ) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: crate::buffer::Buf,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let (read, packed_decoder) = PackedDecoder::decode(ctx, src)?;
    Ok((read, PackedSetDecoder(packed_decoder)))
  }
}

impl<'a, T, B, UB, W> State<Partial<Groto>> for PackedSetDecoder<'a, T, B, UB, W> {
  type Output = Self;
}

impl<'a, T, B, UB, W> State<PartialRef<'a, Packed<W>, B, UB, Groto>>
  for PackedSetDecoder<'a, T, B, UB, W>
{
  type Output = Self;
}

impl<'a, T, B, UB, W> State<Ref<'a, Packed<W>, B, UB, Groto>>
  for PackedSetDecoder<'a, T, B, UB, W>
{
  type Output = Self;
}

impl<'a, T, B, UB, W, S> State<Extracted<S>> for PackedSetDecoder<'a, T, B, UB, W>
where
  S: ?Sized,
{
  type Output = Self;
}
