use core::{iter::FusedIterator, marker::PhantomData};

use crate::{
  buffer::{Buffer, DefaultBuffer, ReadBuf, UnknownBuffer, WriteBuf},
  convert::{Extracted, PartialIdentity},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    Flavor, Groto, RepeatedEntry, WireFormat,
    groto::{Context, Error, Identifier, Tag},
  },
  selection::{Selectable, Selector},
  state::{Partial, PartialRef, Ref, State},
};

use super::{DecomposableMapSelector, PartialDecomposableMapEntry};

/// A lazy iterator for decoding repeated map entries from binary protocol data.
///
/// `DecomposableRepeatedMapDecoder` provides efficient, on-demand decoding of map entries
/// (key-value pairs) from binary data using the Groto protocol format.
/// It implements lazy evaluation, meaning entries are only decoded when explicitly
/// requested through iteration.
///
/// ## Wire Format
///
/// The decoder expects binary data in this symmetric format:
/// ```text
/// [identifier][length][key_id][key][value_id][value] [identifier][length][key_id][key][value_id][value] ...
/// ```
///
/// Key points:
/// - **All entries** are prefixed with their identifier (wire type + tag)
/// - Each entry contains a length-delimited key-value pair
/// - All entries share the same identifier for consistency
/// - No special handling needed for the first entry
///
/// ## Construction Strategy
///
/// During construction, the decoder performs a **greedy scan** of the input buffer to:
/// 1. Look for consecutive entries with matching identifiers
/// 2. Count total entries found
/// 3. Determine the exact byte range of all repeated map data
/// 4. Validate the structure and set up iteration state
///
/// This upfront work enables efficient lazy iteration and accurate progress reporting.
///
/// ## Error Handling
///
/// The decoder implements fail-fast error semantics:
/// - Any decoding error sets an internal error flag on the iterator
/// - Once an error occurs, all subsequent iterations return `None`
/// - Use [`DecomposableRepeatedMapDecoderIter::remaining_hint()`] to check if errors occurred
///
/// ## Performance
///
/// - **Construction**: O(n) time for greedy scan, O(1) space
/// - **Iteration**: O(1) time per entry, O(1) space
/// - **Memory**: Zero-copy when possible, minimal internal state
///
/// ## Thread Safety
///
/// `DecomposableRepeatedMapDecoder` is `Send` and `Sync` when its buffer type allows it.
/// Multiple iterators can be created from the same decoder safely.
pub struct DecomposableRepeatedMapDecoder<'a, K, V, B, UB, KW, VW, const TAG: u32> {
  /// The source buffer containing all repeated map entry data
  src: B,
  /// Total number of map entries found during greedy construction scan
  expected_elements: usize,
  /// Size in bytes of each identifier prefix
  identifier_size: usize,
  /// Decoding context for the Groto protocol
  ctx: &'a Context,
  /// Identifier for map entries
  entry_identifier: Identifier,
  /// Identifier for keys within entries
  key_identifier: Identifier,
  /// Identifier for values within entries
  value_identifier: Identifier,
  /// Phantom data for type parameter `K`
  _k: PhantomData<K>,
  /// Phantom data for type parameter `V`
  _v: PhantomData<V>,
  /// Phantom data for type parameter `KW`
  _kw: PhantomData<KW>,
  /// Phantom data for type parameter `VW`
  _vw: PhantomData<VW>,
  /// Phantom data for type parameter `UB`
  _ub: PhantomData<UB>,
}

impl<'a, K, V, B: Clone, UB, KW, VW, const TAG: u32> Clone
  for DecomposableRepeatedMapDecoder<'a, K, V, B, UB, KW, VW, TAG>
{
  fn clone(&self) -> Self {
    Self {
      src: self.src.clone(),
      expected_elements: self.expected_elements,
      identifier_size: self.identifier_size,
      ctx: self.ctx,
      entry_identifier: self.entry_identifier,
      key_identifier: self.key_identifier,
      value_identifier: self.value_identifier,
      _k: PhantomData,
      _v: PhantomData,
      _kw: PhantomData,
      _vw: PhantomData,
      _ub: PhantomData,
    }
  }
}

impl<'a, K, V, B: Copy, UB, KW, VW, const TAG: u32> Copy
  for DecomposableRepeatedMapDecoder<'a, K, V, B, UB, KW, VW, TAG>
{
}

impl<'de, K, V, B, UB, KW, VW, const TAG: u32>
  DecomposableRepeatedMapDecoder<'de, K, V, B, UB, KW, VW, TAG>
{
  /// Creates an iterator that borrows from the decoder.
  ///
  /// The returned iterator will have the same lifetime as the decoder.
  /// Multiple iterators can be created from the same decoder, each maintaining
  /// independent iteration state.
  #[inline]
  pub const fn iter(
    &self,
  ) -> DecomposableRepeatedMapDecoderIter<'_, 'de, K, V, B, UB, KW, VW, TAG> {
    DecomposableRepeatedMapDecoderIter {
      decoder: self,
      expected_elements: self.expected_elements,
      has_error: false,
      identifier_size: self.identifier_size,
      yielded_elements: 0,
      offset: 0,
    }
  }

  /// Returns a hint for the capacity needed to store all map entries.
  ///
  /// This value is determined during construction by scanning the entire buffer
  /// and counting entries with matching identifiers. It represents the number
  /// of map entries that were found in the input data.
  ///
  /// **Important**: This is the *expected* count based on the wire format scan,
  /// not a guarantee of successful decoding. The iterator might not successfully
  /// yield all these entries due to:
  /// - Data corruption within individual entries
  /// - Decoding errors in key or value content
  /// - Early termination by the caller
  ///
  /// Use this for pre-allocating containers, but always handle potential
  /// decoding failures gracefully.
  #[inline]
  pub const fn capacity_hint(&self) -> usize {
    self.expected_elements
  }
}

impl<'a, K, V, B, UB, KW, VW, const TAG: u32> PartialIdentity<Groto>
  for DecomposableRepeatedMapDecoder<'a, K, V, B, UB, KW, VW, TAG>
where
  K: PartialIdentity<Groto> + Selectable<Groto>,
  V: PartialIdentity<Groto> + Selectable<Groto>,
  K::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  V::Output: Sized + Selectable<Groto, Selector = V::Selector>,
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
{
  fn partial_identity<'b>(
    input: &'b mut Self::Output,
    _: &'b Self::Selector,
  ) -> &'b mut Self::Output {
    input
  }
}

impl<'a, K, V, B, UB, KW, VW, const TAG: u32> Selectable<Groto>
  for DecomposableRepeatedMapDecoder<'a, K, V, B, UB, KW, VW, TAG>
where
  K: Selectable<Groto>,
  V: Selectable<Groto>,
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
{
  type Selector = DecomposableMapSelector<K::Selector, V::Selector>;
}

impl<'a, K, V, RB, B, KW, VW, const TAG: u32> Encode<RepeatedEntry<KW, VW, TAG>, Groto>
  for DecomposableRepeatedMapDecoder<'a, K, V, RB, B, KW, VW, TAG>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  RB: ReadBuf,
{
  fn encode_raw<WB>(&self, ctx: &Context, buf: &mut WB) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
  {
    let buf_len = buf.len();
    let src_len = self.encoded_raw_len(ctx);

    match buf.prefix_mut_checked(src_len) {
      None => Err(Error::insufficient_buffer(src_len, buf_len)),
      Some(buf) => {
        buf.copy_from_slice(self.src.as_bytes());
        Ok(src_len)
      }
    }
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    self.src.len()
  }

  fn encode<WB>(&self, ctx: &Context, buf: &mut WB) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
  {
    self.encode_raw(ctx, buf)
  }

  fn encoded_len(&self, ctx: &Context) -> usize {
    self.encoded_raw_len(ctx)
  }
}

impl<'a, K, V, RB, UB, KW, VW, const TAG: u32> PartialEncode<RepeatedEntry<KW, VW, TAG>, Groto>
  for DecomposableRepeatedMapDecoder<'a, K, V, RB, UB, KW, VW, TAG>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  RepeatedEntry<KW, VW, TAG>: WireFormat<Groto> + 'a,
  K: Decode<'a, KW, RB, UB, Groto> + Selectable<Groto>,
  V: Decode<'a, VW, RB, UB, Groto> + Selectable<Groto>,
  RB: ReadBuf + 'a,
  UB: UnknownBuffer<RB, Groto> + 'a,
{
  fn partial_encode_raw<WB>(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut WB,
    selector: &Self::Selector,
  ) -> Result<usize, <Groto as Flavor>::Error>
  where
    WB: WriteBuf + ?Sized,
  {
    if selector.is_empty() {
      return Ok(0);
    }

    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encode_raw(self, context, buf)
  }

  fn partial_encoded_raw_len(
    &self,
    context: &<Groto as Flavor>::Context,
    selector: &Self::Selector,
  ) -> usize {
    if selector.is_empty() {
      return 0;
    }

    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encoded_raw_len(self, context)
  }

  fn partial_encode<WB>(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut WB,
    selector: &Self::Selector,
  ) -> Result<usize, <Groto as Flavor>::Error>
  where
    WB: WriteBuf + ?Sized,
  {
    if selector.is_empty() {
      return Ok(0);
    }

    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encode(self, context, buf)
  }

  fn partial_encoded_len(
    &self,
    context: &<Groto as Flavor>::Context,
    selector: &Self::Selector,
  ) -> usize {
    if selector.is_empty() {
      return 0;
    }

    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encoded_len(self, context)
  }
}

impl<'a, K, V, RB, B, KW, VW, const TAG: u32> Decode<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>
  for DecomposableRepeatedMapDecoder<'a, K, V, RB, B, KW, VW, TAG>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  RepeatedEntry<KW, VW, TAG>: WireFormat<Groto> + 'a,
  K: Decode<'a, KW, RB, B, Groto>,
  V: Decode<'a, VW, RB, B, Groto>,
{
  fn decode(ctx: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: crate::buffer::ReadBuf,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let expected_identifier =
      Identifier::new(RepeatedEntry::<KW, VW, TAG>::WIRE_TYPE, Tag::new(TAG));
    let mut num_elements = 0;
    let mut offset = 0;
    let buf = src.as_bytes();
    let buf_len = buf.len();

    if buf_len == 0 {
      return Err(Error::buffer_underflow());
    }

    // Scan for consecutive entries with matching identifiers
    // Format: [identifier][length][key_id][key][value_id][value] [identifier][length][key_id][key][value_id][value] ...
    loop {
      if offset >= buf_len {
        break;
      }

      // Try to decode the next identifier
      let (id_bytes, next_identifier) = Identifier::decode(&buf[offset..])?;

      // If the identifier doesn't match, we've reached the end of the repeated field
      if next_identifier != expected_identifier {
        break;
      }

      // Skip the identifier
      offset += id_bytes;

      // Read the length-delimited entry size
      let (len_size, entry_size) = varing::decode_u32_varint(&buf[offset..])?;
      offset += len_size;

      // Skip the entry data
      offset += entry_size as usize;
      num_elements += 1;

      if offset > buf_len {
        return Err(Error::buffer_underflow());
      }
    }

    // Return the decoder configured for the exact range of repeated map data
    Ok((
      offset,
      Self {
        src: src.slice(..offset),
        expected_elements: num_elements,
        identifier_size: expected_identifier.encoded_len(),
        ctx,
        entry_identifier: expected_identifier,
        key_identifier: Identifier::new(KW::WIRE_TYPE, Tag::new(1)),
        value_identifier: Identifier::new(VW::WIRE_TYPE, Tag::new(2)),
        _k: PhantomData,
        _v: PhantomData,
        _kw: PhantomData,
        _vw: PhantomData,
        _ub: PhantomData,
      },
    ))
  }
}

impl<'a, K, V, B, UB, KW, VW, const TAG: u32> State<Partial<Groto>>
  for DecomposableRepeatedMapDecoder<'a, K, V, B, UB, KW, VW, TAG>
{
  type Output = Self;
}

impl<'a, K, V, B, UB, KW, VW, const TAG: u32>
  State<PartialRef<'a, RepeatedEntry<KW, VW, TAG>, B, UB, Groto>>
  for DecomposableRepeatedMapDecoder<'a, K, V, B, UB, KW, VW, TAG>
{
  type Output = Self;
}

impl<'a, K, V, B, UB, KW, VW, const TAG: u32>
  State<Ref<'a, RepeatedEntry<KW, VW, TAG>, B, UB, Groto>>
  for DecomposableRepeatedMapDecoder<'a, K, V, B, UB, KW, VW, TAG>
{
  type Output = Self;
}

impl<'a, K, V, B, UB, KW, VW, S, const TAG: u32> State<Extracted<S>>
  for DecomposableRepeatedMapDecoder<'a, K, V, B, UB, KW, VW, TAG>
where
  S: ?Sized,
{
  type Output = Self;
}

/// Iterator for lazily decoding repeated map entries.
///
/// This iterator maintains its own state and can be created multiple times
/// from the same decoder. Each iterator starts from the beginning and
/// maintains independent progress tracking.
///
/// # Error Handling
///
/// The iterator implements fail-fast semantics:
/// - First error sets the `has_error` flag
/// - All subsequent `next()` calls return `None`
/// - Use `remaining_hint()` to detect error state
///
/// # State Management
///
/// The iterator tracks:
/// - Current byte offset in the source buffer
/// - Number of entries successfully decoded
/// - Error state for fail-fast behavior
/// - Expected entry count (cached from decoder)
pub struct DecomposableRepeatedMapDecoderIter<'a, 'de: 'a, K, V, RB, UB, KW, VW, const TAG: u32> {
  /// Reference to the parent decoder
  decoder: &'a DecomposableRepeatedMapDecoder<'de, K, V, RB, UB, KW, VW, TAG>,
  /// Total entries expected (cached from decoder)
  expected_elements: usize,
  /// Error flag - once set, iteration stops permanently
  has_error: bool,
  /// Size in bytes of each identifier prefix
  identifier_size: usize,
  /// Number of entries successfully decoded so far
  yielded_elements: usize,
  /// Current byte offset within the source buffer
  offset: usize,
}

impl<'a, 'de: 'a, K, V, B, UB, KW, VW, const TAG: u32>
  DecomposableRepeatedMapDecoderIter<'a, 'de, K, V, B, UB, KW, VW, TAG>
{
  /// Returns the current byte position within the source buffer.
  ///
  /// This represents how many bytes have been consumed during iteration.
  /// Useful for tracking progress through the binary data or debugging.
  #[inline]
  pub const fn position(&self) -> usize {
    self.offset
  }

  /// Returns the number of entries that have been successfully decoded so far.
  ///
  /// This count increases each time [`next()`] successfully returns an entry.
  /// It will never exceed the expected entry count from the wire format.
  ///
  /// Use this for progress reporting or determining how many entries
  /// have been processed.
  ///
  /// [`next()`]: Iterator::next
  #[inline]
  pub const fn decoded(&self) -> usize {
    self.yielded_elements
  }

  /// Returns the total number of entries expected according to the wire format.
  ///
  /// This value is determined during decoder construction. See
  /// [`DecomposableRepeatedMapDecoder::capacity_hint()`] for more details.
  ///
  /// [`DecomposableRepeatedMapDecoder::capacity_hint()`]: DecomposableRepeatedMapDecoder::capacity_hint
  #[inline]
  pub const fn capacity_hint(&self) -> usize {
    self.expected_elements
  }

  /// Returns the number of entries remaining to be attempted, or `None` if an error occurred.
  ///
  /// This is calculated as `capacity_hint() - decoded()`. It represents
  /// how many entries the iterator will *attempt* to decode, not a guarantee
  /// of successful decoding.
  ///
  /// # Error Indication
  ///
  /// If the iterator has encountered an error during iteration, this returns `None`
  /// to indicate that the remaining count is unreliable. This happens when:
  /// - Buffer underflow occurs
  /// - Entry decoding fails
  /// - Data corruption is detected
  ///
  /// Once an error occurs, the iterator enters a permanent error state and
  /// will not attempt to decode any more entries.
  ///
  /// # Returns
  ///
  /// - `Some(count)` - Number of entries left to attempt decoding
  /// - `None` - An error occurred and remaining count is unreliable
  #[inline]
  pub const fn remaining_hint(&self) -> Option<usize> {
    if self.has_error {
      return None;
    }

    Some(self.expected_elements.saturating_sub(self.yielded_elements))
  }
}

impl<'a, 'de: 'a, RB, B, KW, VW, K, V, const TAG: u32> Iterator
  for DecomposableRepeatedMapDecoderIter<'a, 'de, K, V, RB, B, KW, VW, TAG>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: Decode<'de, KW, RB, B, Groto> + 'de,
  V: Decode<'de, VW, RB, B, Groto> + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
{
  type Item = Result<(usize, PartialDecomposableMapEntry<K, V>), Error>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let src_len = self.decoder.src.len();

    // Check if we've reached the end of the buffer
    if self.offset >= src_len {
      return None;
    }

    // Check if we've decoded all expected entries
    if self.yielded_elements >= self.expected_elements {
      return None;
    }

    // Check if we're in an error state
    if self.has_error {
      return None;
    }

    // Ensure we have enough bytes for the identifier
    if self.offset + self.identifier_size > src_len {
      // This indicates the buffer is malformed - we expected more entries
      // but don't have enough bytes even for the identifier
      self.has_error = true;
      return Some(Err(Error::buffer_underflow()));
    }

    // Decode the repeated map entry
    // All entries follow the same [identifier][length][key_id][key][value_id][value] pattern
    Some(
      PartialDecomposableMapEntry::decode_repeated(
        self.decoder.ctx,
        self.decoder.src.slice(self.offset..),
        &self.decoder.entry_identifier,
        &self.decoder.key_identifier,
        &self.decoder.value_identifier,
      )
      .and_then(|(read, ent)| {
        match ent {
          Some(ent) => {
            // Update position
            self.offset += read;
            self.yielded_elements += 1;
            Ok((read, ent))
          }
          None => {
            // If we reach here, it means the entry was malformed or incomplete
            self.has_error = true;
            Err(Error::custom("malformed repeated map entry"))
          }
        }
      })
      .inspect_err(|_| {
        // Set error flag to prevent further iteration attempts
        self.has_error = true;
      }),
    )
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let remaining = self.remaining_hint();
    (0, remaining)
  }
}

impl<'a, 'de, RB, B, KW, VW, K, V, const TAG: u32> FusedIterator
  for DecomposableRepeatedMapDecoderIter<'a, 'de, K, V, RB, B, KW, VW, TAG>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: Decode<'de, KW, RB, B, Groto> + 'de,
  V: Decode<'de, VW, RB, B, Groto> + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
{
}

/// A buffer that holds multiple `DecomposableRepeatedMapDecoder` instances.
///
/// This buffer handles repeated map fields that may appear multiple times in a message,
/// potentially interleaved with other fields. Each decoder instance represents a
/// contiguous batch of repeated map entries found during parsing.
///
/// ## Use Cases
///
/// Protocol buffers allow repeated fields to appear multiple times in a message:
/// ```text
/// [other_field] [repeated_map_batch_1] [other_field] [repeated_map_batch_2] ...
/// ```
///
/// This buffer aggregates all batches and provides unified iteration over all entries.
///
/// ## Wire Format Handling
///
/// The buffer automatically handles:
/// - Multiple batches of the same repeated map field
/// - Proper sequencing during iteration
/// - Encoding back to the original interleaved format
/// - Error propagation across all batches
///
/// ## Performance
///
/// - **Construction**: O(k) where k is the number of batches
/// - **Iteration**: O(1) amortized per entry across all batches
/// - **Memory**: Stores references to decoder instances, minimal overhead
pub struct DecomposableRepeatedMapDecoderBuffer<
  'a,
  K,
  V,
  RB,
  UB,
  KW,
  VW,
  const TAG: u32,
  B = DefaultBuffer<DecomposableRepeatedMapDecoder<'a, K, V, RB, UB, KW, VW, TAG>>,
> {
  /// Buffer containing multiple DecomposableRepeatedMapDecoder instances
  buffer: B,
  /// Phantom data for various lifetimes and types
  _lt: PhantomData<&'a ()>,
  _k: PhantomData<K>,
  _v: PhantomData<V>,
  _rb: PhantomData<RB>,
  _ub: PhantomData<UB>,
  _kw: PhantomData<KW>,
  _vw: PhantomData<VW>,
}

impl<'a, K, V, RB, UB, KW, VW, const TAG: u32, B> Clone
  for DecomposableRepeatedMapDecoderBuffer<'a, K, V, RB, UB, KW, VW, TAG, B>
where
  B: Clone,
{
  fn clone(&self) -> Self {
    Self {
      buffer: self.buffer.clone(),
      _lt: PhantomData,
      _k: PhantomData,
      _v: PhantomData,
      _rb: PhantomData,
      _ub: PhantomData,
      _kw: PhantomData,
      _vw: PhantomData,
    }
  }
}

impl<'a, K, V, RB, UB, KW, VW, const TAG: u32, B> Copy
  for DecomposableRepeatedMapDecoderBuffer<'a, K, V, RB, UB, KW, VW, TAG, B>
where
  B: Copy,
{
}

impl<'de, K, V, RB, UB, KW, VW, const TAG: u32, B> Selectable<Groto>
  for DecomposableRepeatedMapDecoderBuffer<'de, K, V, RB, UB, KW, VW, TAG, B>
where
  K: Selectable<Groto>,
  V: Selectable<Groto>,
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
{
  type Selector = DecomposableMapSelector<K::Selector, V::Selector>;
}

impl<'de, K, V, RB, UB, KW, VW, const TAG: u32, B>
  DecomposableRepeatedMapDecoderBuffer<'de, K, V, RB, UB, KW, VW, TAG, B>
where
  B: Buffer<Item = DecomposableRepeatedMapDecoder<'de, K, V, RB, UB, KW, VW, TAG>>,
{
  /// Creates an iterator over all entries across all decoder batches.
  ///
  /// The iterator processes entries from each decoder sequentially,
  /// automatically advancing to the next decoder when the current one
  /// is exhausted.
  pub fn iter(
    &self,
  ) -> DecomposableRepeatedMapDecoderBufferIter<'_, 'de, K, V, RB, UB, KW, VW, TAG, B> {
    let total_expected = self
      .buffer
      .as_slice()
      .iter()
      .map(|d| d.expected_elements)
      .sum();

    DecomposableRepeatedMapDecoderBufferIter {
      buffer: self,
      has_error: false,
      current_index: 0,
      current_decoder_offset: 0,
      current_decoder_yielded_elements: 0,
      total_expected_elements: total_expected,
      total_yielded_elements: 0,
    }
  }

  /// Returns a hint for the total capacity needed to store all entries across all decoders.
  ///
  /// This sums up the capacity hints from each individual decoder in the buffer.
  /// Use this for pre-allocating containers, but always handle potential
  /// decoding failures gracefully.
  ///
  /// **Important**: This is a capacity *hint* based on wire format scanning,
  /// not a guarantee of successful decoding.
  #[inline]
  pub fn capacity_hint(&self) -> usize {
    self
      .buffer
      .as_slice()
      .iter()
      .map(|d| d.expected_elements)
      .sum()
  }

  /// Returns the number of individual decoder batches in this buffer.
  ///
  /// Each decoder represents a separate contiguous batch of repeated entries
  /// that were found in the wire format.
  #[inline]
  pub fn batch_count(&self) -> usize {
    self.buffer.len()
  }

  /// Returns whether the buffer is empty (contains no decoder batches).
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.buffer.is_empty()
  }
}

impl<'a, K, V, RB, UB, KW, VW, const TAG: u32, B> Encode<RepeatedEntry<KW, VW, TAG>, Groto>
  for DecomposableRepeatedMapDecoderBuffer<'a, K, V, RB, UB, KW, VW, TAG, B>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  RepeatedEntry<KW, VW, TAG>: WireFormat<Groto> + 'a,
  K: Decode<'a, KW, RB, UB, Groto>,
  V: Decode<'a, VW, RB, UB, Groto>,
  B: Buffer<Item = DecomposableRepeatedMapDecoder<'a, K, V, RB, UB, KW, VW, TAG>>,
  RB: ReadBuf + 'a,
  UB: UnknownBuffer<RB, Groto> + 'a,
{
  fn encode_raw<WB>(&self, context: &Context, buf: &mut WB) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
  {
    let encoded_raw_len = self.encoded_raw_len(context);
    let buf_len = buf.len();
    if buf_len < encoded_raw_len {
      return Err(Error::insufficient_buffer(encoded_raw_len, buf_len));
    }

    let mut offset = 0;
    let buf = buf.as_mut_slice();
    for decoder in self.buffer.as_slice() {
      // Double-check bounds for each decoder
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(encoded_raw_len, buf_len));
      }

      let size = decoder.encode_raw(context, &mut buf[offset..])?;
      offset += size;
    }
    Ok(offset)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    self
      .buffer
      .as_slice()
      .iter()
      .map(|decoder| decoder.encoded_raw_len(context))
      .sum()
  }

  fn encode<WB>(&self, context: &Context, buf: &mut WB) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
  {
    self.encode_raw(context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    self.encoded_raw_len(context)
  }
}

impl<'a, K, V, RB, UB, KW, VW, const TAG: u32, B> PartialEncode<RepeatedEntry<KW, VW, TAG>, Groto>
  for DecomposableRepeatedMapDecoderBuffer<'a, K, V, RB, UB, KW, VW, TAG, B>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  RepeatedEntry<KW, VW, TAG>: WireFormat<Groto> + 'a,
  K: Decode<'a, KW, RB, UB, Groto> + Selectable<Groto>,
  V: Decode<'a, VW, RB, UB, Groto> + Selectable<Groto>,
  B: Buffer<Item = DecomposableRepeatedMapDecoder<'a, K, V, RB, UB, KW, VW, TAG>>,
  RB: ReadBuf + 'a,
  UB: UnknownBuffer<RB, Groto> + 'a,
{
  fn partial_encode_raw<WB>(
    &self,
    context: &Context,
    buf: &mut WB,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
  {
    if selector.is_empty() {
      return Ok(0);
    }

    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encode_raw(self, context, buf)
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encoded_raw_len(self, context)
  }

  fn partial_encode<WB>(
    &self,
    context: &Context,
    buf: &mut WB,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
  {
    if selector.is_empty() {
      return Ok(0);
    }

    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encode(self, context, buf)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encoded_len(self, context)
  }
}

impl<'a, K, V, RB, UB, KW, VW, const TAG: u32, B>
  Decode<'a, RepeatedEntry<KW, VW, TAG>, RB, UB, Groto>
  for DecomposableRepeatedMapDecoderBuffer<'a, K, V, RB, UB, KW, VW, TAG, B>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  RepeatedEntry<KW, VW, TAG>: WireFormat<Groto> + 'a,
  K: Decode<'a, KW, RB, UB, Groto>,
  V: Decode<'a, VW, RB, UB, Groto>,
  B: Buffer<Item = DecomposableRepeatedMapDecoder<'a, K, V, RB, UB, KW, VW, TAG>>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    UB: UnknownBuffer<RB, Groto> + 'a,
  {
    let mut this = Self {
      buffer: B::new(),
      _lt: PhantomData,
      _k: PhantomData,
      _v: PhantomData,
      _rb: PhantomData,
      _ub: PhantomData,
      _kw: PhantomData,
      _vw: PhantomData,
    };

    <Self as Decode<'a, RepeatedEntry<KW, VW, TAG>, RB, UB, Groto>>::merge_decode(
      &mut this, context, src,
    )
    .map(|size| (size, this))
  }

  fn merge_decode(&mut self, ctx: &'a Context, src: RB) -> Result<usize, Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    UB: UnknownBuffer<RB, Groto> + 'a,
  {
    let (read, decoder) =
      <DecomposableRepeatedMapDecoder<'a, K, V, RB, UB, KW, VW, TAG> as Decode<
        'a,
        RepeatedEntry<KW, VW, TAG>,
        RB,
        UB,
        Groto,
      >>::decode(ctx, src)?;

    if self.buffer.push(decoder).is_none() {
      return Err(Error::custom(
        "failed to push decoder into buffer: capacity exceeded",
      ));
    }

    Ok(read)
  }
}

impl<'a, K, V, RB, UB, KW, VW, B, const TAG: u32> State<Partial<Groto>>
  for DecomposableRepeatedMapDecoderBuffer<'a, K, V, RB, UB, KW, VW, TAG, B>
{
  type Output = Self;
}

impl<'a, K, V, RB, UB, KW, VW, B, const TAG: u32>
  State<PartialRef<'a, RepeatedEntry<KW, VW, TAG>, RB, UB, Groto>>
  for DecomposableRepeatedMapDecoderBuffer<'a, K, V, RB, UB, KW, VW, TAG, B>
{
  type Output = Self;
}

impl<'a, K, V, RB, UB, KW, VW, B, const TAG: u32>
  State<Ref<'a, RepeatedEntry<KW, VW, TAG>, RB, UB, Groto>>
  for DecomposableRepeatedMapDecoderBuffer<'a, K, V, RB, UB, KW, VW, TAG, B>
{
  type Output = Self;
}

impl<'a, K, V, RB, UB, KW, VW, S, B, const TAG: u32> State<Extracted<S>>
  for DecomposableRepeatedMapDecoderBuffer<'a, K, V, RB, UB, KW, VW, TAG, B>
where
  S: ?Sized,
{
  type Output = Self;
}

impl<'a, K, V, RB, UB, KW, VW, B, const TAG: u32> PartialIdentity<Groto>
  for DecomposableRepeatedMapDecoderBuffer<'a, K, V, RB, UB, KW, VW, TAG, B>
where
  K: PartialIdentity<Groto> + Selectable<Groto>,
  V: PartialIdentity<Groto> + Selectable<Groto>,
  K::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  V::Output: Sized + Selectable<Groto, Selector = V::Selector>,
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
{
  fn partial_identity<'b>(
    input: &'b mut Self::Output,
    _: &'b Self::Selector,
  ) -> &'b mut Self::Output {
    input
  }
}

/// Iterator that processes entries from multiple `DecomposableRepeatedMapDecoder` instances sequentially.
///
/// This iterator maintains state for traversing multiple decoder batches, automatically
/// advancing to the next decoder when the current one is exhausted. It provides
/// unified iteration over all entries across all decoders in the buffer.
///
/// ## Error Handling
///
/// The iterator implements fail-fast semantics:
/// - First error sets the `has_error` flag  
/// - All subsequent `next()` calls return `None`
/// - Use `remaining_hint()` to detect error state
///
/// ## State Management
///
/// The iterator tracks:
/// - Which decoder batch is currently being processed (`current_index`)
/// - Position within the current decoder (`current_decoder_offset`)
/// - Entries yielded from current decoder (`current_decoder_yielded_elements`)
/// - Total progress across all decoders (`total_yielded_elements`)
///
/// ## Implementation Notes
///
/// The iterator creates temporary `DecomposableRepeatedMapDecoderIter` instances and manually
/// restores their state to maintain consistent iteration across decoder boundaries.
/// This approach allows seamless transitions between decoder batches.
pub struct DecomposableRepeatedMapDecoderBufferIter<
  'a,
  'de: 'a,
  K,
  V,
  RB,
  UB,
  KW,
  VW,
  const TAG: u32,
  B,
> {
  /// Reference to the parent buffer
  buffer: &'a DecomposableRepeatedMapDecoderBuffer<'de, K, V, RB, UB, KW, VW, TAG, B>,
  /// Error flag - once set, iteration stops permanently
  has_error: bool,
  /// Index of the currently active decoder batch
  current_index: usize,
  /// Byte offset within the current decoder
  current_decoder_offset: usize,
  /// Entries yielded from the current decoder
  current_decoder_yielded_elements: usize,
  /// Total entries expected across all decoders
  total_expected_elements: usize,
  /// Total entries yielded across all decoders
  total_yielded_elements: usize,
}

impl<'a, 'de, K, V, RB, UB, KW, VW, const TAG: u32, B>
  DecomposableRepeatedMapDecoderBufferIter<'a, 'de, K, V, RB, UB, KW, VW, TAG, B>
{
  /// Returns the total capacity hint across all decoder batches.
  #[inline]
  pub const fn capacity_hint(&self) -> usize {
    self.total_expected_elements
  }

  /// Returns the total number of entries decoded so far across all batches.
  #[inline]
  pub const fn decoded(&self) -> usize {
    self.total_yielded_elements
  }

  /// Returns the estimated number of entries remaining across all batches,
  /// or `None` if an error occurred.
  ///
  /// This considers the total progress across all decoders in the buffer.
  ///
  /// # Returns
  ///
  /// - `Some(count)` - Number of entries left to attempt decoding
  /// - `None` - An error occurred and remaining count is unreliable
  #[inline]
  pub const fn remaining_hint(&self) -> Option<usize> {
    if self.has_error {
      return None;
    }

    Some(
      self
        .total_expected_elements
        .saturating_sub(self.total_yielded_elements),
    )
  }

  /// Returns the index of the currently active decoder batch.
  ///
  /// This can be useful for tracking which batch of repeated entries
  /// is currently being processed.
  #[inline]
  pub const fn current_batch_index(&self) -> usize {
    self.current_index
  }

  /// Returns whether the iterator has encountered an error.
  #[inline]
  pub const fn has_error(&self) -> bool {
    self.has_error
  }
}

impl<'a, 'de, K, V, RB, UB, KW, VW, const TAG: u32, B>
  DecomposableRepeatedMapDecoderBufferIter<'a, 'de, K, V, RB, UB, KW, VW, TAG, B>
where
  B: Buffer<Item = DecomposableRepeatedMapDecoder<'de, K, V, RB, UB, KW, VW, TAG>>,
{
  /// Returns the total number of decoder batches.
  #[inline]
  pub fn batch_count(&self) -> usize {
    self.buffer.buffer.len()
  }
}

impl<'a, 'de: 'a, K, V, RB, UB, KW, VW, const TAG: u32, B> Iterator
  for DecomposableRepeatedMapDecoderBufferIter<'a, 'de, K, V, RB, UB, KW, VW, TAG, B>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: Decode<'de, KW, RB, UB, Groto> + Sized + 'de,
  V: Decode<'de, VW, RB, UB, Groto> + Sized + 'de,
  RB: ReadBuf + 'de,
  UB: UnknownBuffer<RB, Groto> + 'de,
  B: Buffer<Item = DecomposableRepeatedMapDecoder<'de, K, V, RB, UB, KW, VW, TAG>>,
{
  type Item = Result<(usize, PartialDecomposableMapEntry<K, V>), Error>;

  fn next(&mut self) -> Option<Self::Item> {
    // Check if we've processed all decoder batches
    if self.current_index >= self.buffer.buffer.len() {
      return None;
    }

    // Check if we've decoded all expected entries
    if self.total_yielded_elements >= self.total_expected_elements {
      return None;
    }

    // Check if we're in an error state
    if self.has_error {
      return None;
    }

    // Get the current decoder batch
    let decoder = &self.buffer.buffer.as_slice()[self.current_index];

    // Create a temporary iterator and restore its state
    let mut iter = decoder.iter();
    iter.offset = self.current_decoder_offset;
    iter.yielded_elements = self.current_decoder_yielded_elements;

    match iter.next() {
      Some(Ok((read, item))) => {
        // Save the updated iterator state
        self.current_decoder_offset = iter.offset;
        self.current_decoder_yielded_elements = iter.yielded_elements;
        self.total_yielded_elements += 1;

        // Check if current decoder batch is exhausted
        if self.current_decoder_yielded_elements >= decoder.expected_elements {
          // Move to the next decoder batch
          self.current_index += 1;
          self.current_decoder_offset = 0;
          self.current_decoder_yielded_elements = 0;
        }

        Some(Ok((read, item)))
      }
      Some(Err(e)) => {
        // Propagate error and set error state
        self.has_error = true;
        Some(Err(e))
      }
      None => {
        // Current decoder batch is exhausted, move to the next one
        self.current_index += 1;
        self.current_decoder_offset = 0;
        self.current_decoder_yielded_elements = 0;
        // Recursively try the next batch
        self.next()
      }
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, self.remaining_hint())
  }
}

impl<'a, 'de: 'a, K, V, RB, UB, KW, VW, const TAG: u32, B> FusedIterator
  for DecomposableRepeatedMapDecoderBufferIter<'a, 'de, K, V, RB, UB, KW, VW, TAG, B>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: Decode<'de, KW, RB, UB, Groto> + Sized + 'de,
  V: Decode<'de, VW, RB, UB, Groto> + Sized + 'de,
  RB: ReadBuf + 'de,
  UB: UnknownBuffer<RB, Groto> + 'de,
  B: Buffer<Item = DecomposableRepeatedMapDecoder<'de, K, V, RB, UB, KW, VW, TAG>>,
{
}
