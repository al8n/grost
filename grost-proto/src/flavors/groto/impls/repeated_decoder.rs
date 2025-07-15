use core::{iter::FusedIterator, marker::PhantomData};

use crate::{
  buffer::{Buffer, DefaultBuffer, ReadBuf, UnknownBuffer},
  convert::{Flattened, Partial, PartialRef, Ref},
  decode::Decode1,
  encode::Encode,
  flavors::{
    Flavor, Groto, Repeated, WireFormat,
    groto::{Context, Error, Identifier, Tag},
  },
  selection::Selectable,
  state::State,
};

/// A lazy iterator for decoding repeated elements from binary protocol data.
///
/// `RepeatedDecoder` provides efficient, on-demand decoding of list-like structures
/// (such as `Vec<T>`, `[T]`, etc.) from binary data using the Groto protocol format.
/// It implements lazy evaluation, meaning elements are only decoded when explicitly
/// requested through iteration.
///
/// ## Wire Format
///
/// The decoder expects binary data in this symmetric format:
/// ```text
/// [identifier][first_element] [identifier][second_element] [identifier][third_element] ...
/// ```
///
/// Key points:
/// - **All elements** are prefixed with their identifier (wire type + tag)
/// - All elements share the same identifier for consistency
/// - No special handling needed for the first element
///
/// ## Construction Strategy
///
/// During construction, the decoder performs a **greedy scan** of the input buffer to:
/// 1. Look for consecutive elements with matching identifiers
/// 2. Count total elements found
/// 3. Determine the exact byte range of all repeated data
/// 4. Validate the structure and set up iteration state
///
/// This upfront work enables efficient lazy iteration and accurate progress reporting.
///
/// ## Error Handling
///
/// The decoder implements fail-fast error semantics:
/// - Any decoding error sets an internal error flag on the iterator
/// - Once an error occurs, all subsequent iterations return `None`
/// - Use [`RepeatedDecoderIter::remaining_hint()`] to check if errors occurred
///
/// ## Performance
///
/// - **Construction**: O(n) time for greedy scan, O(1) space
/// - **Iteration**: O(1) time per element, O(1) space
/// - **Memory**: Zero-copy when possible, minimal internal state
///
/// ## Thread Safety
///
/// `RepeatedDecoder` is `Send` and `Sync` when its buffer type allows it.
/// Multiple iterators can be created from the same decoder safely.
///
/// [`RepeatedDecoderIter::remaining_hint()`]: RepeatedDecoderIter::remaining_hint
pub struct RepeatedDecoder<'a, T, B, UB, W, const TAG: u32> {
  /// The source buffer containing all repeated element data
  src: B,
  /// Total number of elements found during greedy construction scan
  expected_elements: usize,
  /// Size in bytes of each identifier prefix
  identifier_size: usize,
  /// Decoding context for the Groto protocol
  ctx: &'a Context,
  /// Phantom data for type parameter `T`
  _t: PhantomData<T>,
  /// Phantom data for type parameter `W`
  _w: PhantomData<W>,
  /// Phantom data for type parameter `UB`
  _ub: PhantomData<UB>,
}

impl<'a, T, B: Clone, UB, W, const TAG: u32> Clone for RepeatedDecoder<'a, T, B, UB, W, TAG> {
  fn clone(&self) -> Self {
    Self {
      src: self.src.clone(),
      expected_elements: self.expected_elements,
      identifier_size: self.identifier_size,
      ctx: self.ctx,
      _t: PhantomData,
      _w: PhantomData,
      _ub: PhantomData,
    }
  }
}

impl<'a, T, B: Copy, UB, W, const TAG: u32> Copy for RepeatedDecoder<'a, T, B, UB, W, TAG> {}

impl<'de, T, B, UB, W, const TAG: u32> RepeatedDecoder<'de, T, B, UB, W, TAG> {
  /// Creates an iterator that borrows from the decoder.
  ///
  /// The returned iterator will have the same lifetime as the decoder.
  /// Multiple iterators can be created from the same decoder, each maintaining
  /// independent iteration state.
  #[inline]
  pub const fn iter(&self) -> RepeatedDecoderIter<'_, 'de, T, B, UB, W, TAG> {
    RepeatedDecoderIter {
      decoder: self,
      expected_elements: self.expected_elements,
      has_error: false,
      identifier_size: self.identifier_size,
      yielded_elements: 0,
      offset: 0,
    }
  }

  /// Returns a hint for the capacity needed to store all elements.
  ///
  /// This value is determined during construction by scanning the entire buffer
  /// and counting elements with matching identifiers. It represents the number
  /// of elements that were found in the input data.
  ///
  /// **Important**: This is the *expected* count based on the wire format scan,
  /// not a guarantee of successful decoding. The iterator might not successfully
  /// yield all these elements due to:
  /// - Data corruption within individual elements
  /// - Decoding errors in element content
  /// - Early termination by the caller
  ///
  /// Use this for pre-allocating containers, but always handle potential
  /// decoding failures gracefully.
  #[inline]
  pub const fn capacity_hint(&self) -> usize {
    self.expected_elements
  }
}

impl<'a, T, B, UB, W, const TAG: u32> Selectable<Groto> for RepeatedDecoder<'a, T, B, UB, W, TAG>
where
  T: Selectable<Groto>,
  W: WireFormat<Groto> + 'a,
{
  type Selector = T::Selector;
}

impl<'a, T, RB, B, W, const TAG: u32> Encode<Repeated<W, TAG>, Groto>
  for RepeatedDecoder<'a, T, RB, B, W, TAG>
where
  W: WireFormat<Groto> + 'a,
  RB: ReadBuf,
{
  fn encode_raw(&self, ctx: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let buf_len = buf.len();
    let src_len = self.encoded_raw_len(ctx);
    if buf_len < src_len {
      return Err(Error::insufficient_buffer(src_len, buf_len));
    }

    buf[..src_len].copy_from_slice(self.src.as_bytes());
    Ok(src_len)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    self.src.len()
  }

  fn encode(&self, ctx: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    self.encode_raw(ctx, buf)
  }

  fn encoded_len(&self, ctx: &Context) -> usize {
    self.encoded_raw_len(ctx)
  }
}

impl<'a, T, RB, B, W, const TAG: u32> Decode1<'a, Repeated<W, TAG>, RB, B, Groto>
  for RepeatedDecoder<'a, T, RB, B, W, TAG>
where
  W: WireFormat<Groto> + 'a,
  Repeated<W, TAG>: WireFormat<Groto> + 'a,
  T: Decode1<'a, W, RB, B, Groto>,
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
    let expected_identifier = Identifier::new(W::WIRE_TYPE, Tag::new(TAG));
    let mut num_elements = 0;
    let mut offset = 0;
    let buf = src.as_bytes();
    let buf_len = buf.len();

    if buf_len == 0 {
      return Err(Error::buffer_underflow());
    }

    // Scan for consecutive elements with matching identifiers
    // Format: [identifier][element] [identifier][element] ...
    loop {
      if offset >= buf_len {
        break;
      }

      // Try to decode the next identifier
      let (id_bytes, next_identifier) = Identifier::decode(&buf[offset..])?;

      // If the identifier doesn't match, we've reached the end of the repeated field
      // Note: We don't consume this identifier as it belongs to the next field
      if next_identifier != expected_identifier {
        break;
      }

      // Skip the identifier and peek at the element size
      offset += id_bytes;

      // Use peek_raw to determine element size without consuming it
      let element_bytes = Groto::peek_raw(ctx, W::WIRE_TYPE, &buf[offset..])?;
      offset += element_bytes;
      num_elements += 1;
    }

    // Return the decoder configured for the exact range of repeated data
    Ok((
      offset,
      Self {
        src: src.slice(..offset),
        expected_elements: num_elements,
        identifier_size: expected_identifier.encoded_len(),
        ctx,
        _t: PhantomData,
        _w: PhantomData,
        _ub: PhantomData,
      },
    ))
  }
}

impl<'a, T, B, UB, W, const TAG: u32> State<Partial<Groto>>
  for RepeatedDecoder<'a, T, B, UB, W, TAG>
{
  type Output = Self;
}

impl<'a, T, B, UB, W, const TAG: u32> State<PartialRef<'a, B, UB, Repeated<W, TAG>, Groto>>
  for RepeatedDecoder<'a, T, B, UB, W, TAG>
{
  type Output = Self;
}

impl<'a, T, B, UB, W, const TAG: u32> State<Ref<'a, B, UB, Repeated<W, TAG>, Groto>>
  for RepeatedDecoder<'a, T, B, UB, W, TAG>
{
  type Output = Self;
}

impl<'a, T, B, UB, W, S, const TAG: u32> State<Flattened<S>>
  for RepeatedDecoder<'a, T, B, UB, W, TAG>
where
  S: ?Sized,
{
  type Output = Self;
}

/// Iterator for lazily decoding repeated elements.
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
/// - Number of elements successfully decoded
/// - Error state for fail-fast behavior
/// - Expected element count (cached from decoder)
pub struct RepeatedDecoderIter<'a, 'de: 'a, T, RB, UB, W, const TAG: u32> {
  /// Reference to the parent decoder
  decoder: &'a RepeatedDecoder<'de, T, RB, UB, W, TAG>,
  /// Total elements expected (cached from decoder)
  expected_elements: usize,
  /// Error flag - once set, iteration stops permanently
  has_error: bool,
  /// Size in bytes of each identifier prefix
  identifier_size: usize,
  /// Number of elements successfully decoded so far
  yielded_elements: usize,
  /// Current byte offset within the source buffer
  offset: usize,
}

impl<'a, 'de: 'a, T, B, UB, W, const TAG: u32> RepeatedDecoderIter<'a, 'de, T, B, UB, W, TAG> {
  /// Returns the current byte position within the source buffer.
  ///
  /// This represents how many bytes have been consumed during iteration.
  /// Useful for tracking progress through the binary data or debugging.
  #[inline]
  pub const fn position(&self) -> usize {
    self.offset
  }

  /// Returns the number of elements that have been successfully decoded so far.
  ///
  /// This count increases each time [`next()`] successfully returns an element.
  /// It will never exceed the expected element count from the wire format.
  ///
  /// Use this for progress reporting or determining how many elements
  /// have been processed.
  ///
  /// [`next()`]: Iterator::next
  #[inline]
  pub const fn decoded(&self) -> usize {
    self.yielded_elements
  }

  /// Returns the total number of elements expected according to the wire format.
  ///
  /// This value is determined during decoder construction. See
  /// [`RepeatedDecoder::capacity_hint()`] for more details.
  ///
  /// [`RepeatedDecoder::capacity_hint()`]: RepeatedDecoder::capacity_hint
  #[inline]
  pub const fn capacity_hint(&self) -> usize {
    self.expected_elements
  }

  /// Returns the number of elements remaining to be attempted, or `None` if an error occurred.
  ///
  /// This is calculated as `capacity_hint() - decoded()`. It represents
  /// how many elements the iterator will *attempt* to decode, not a guarantee
  /// of successful decoding.
  ///
  /// # Error Indication
  ///
  /// If the iterator has encountered an error during iteration, this returns `None`
  /// to indicate that the remaining count is unreliable. This happens when:
  /// - Buffer underflow occurs
  /// - Element decoding fails
  /// - Data corruption is detected
  ///
  /// Once an error occurs, the iterator enters a permanent error state and
  /// will not attempt to decode any more elements.
  ///
  /// # Returns
  ///
  /// - `Some(count)` - Number of elements left to attempt decoding
  /// - `None` - An error occurred and remaining count is unreliable
  #[inline]
  pub const fn remaining_hint(&self) -> Option<usize> {
    if self.has_error {
      return None;
    }

    Some(self.expected_elements.saturating_sub(self.yielded_elements))
  }
}

impl<'a, 'de: 'a, RB, B, W, T, const TAG: u32> Iterator
  for RepeatedDecoderIter<'a, 'de, T, RB, B, W, TAG>
where
  W: WireFormat<Groto> + 'de,
  T: Decode1<'de, W, RB, B, Groto> + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
{
  type Item = Result<(usize, T), Error>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let src_len = self.decoder.src.len();

    // Check if we've reached the end of the buffer
    if self.offset >= src_len {
      return None;
    }

    // Check if we've decoded all expected elements
    if self.yielded_elements >= self.expected_elements {
      return None;
    }

    // Check if we're in an error state
    if self.has_error {
      return None;
    }

    // Ensure we have enough bytes for the identifier
    if self.offset + self.identifier_size > src_len {
      // This indicates the buffer is malformed - we expected more elements
      // but don't have enough bytes even for the identifier
      self.has_error = true;
      return Some(Err(Error::buffer_underflow()));
    }

    // Skip the identifier and decode the element
    // All elements follow the same [identifier][element] pattern
    Some(
      T::decode(
        self.decoder.ctx,
        self.decoder.src.slice(self.offset + self.identifier_size..),
      )
      .inspect(|(read, _)| {
        // Update position: skip identifier + consumed bytes
        self.offset += self.identifier_size + read;
        self.yielded_elements += 1;
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

impl<'a, 'de, RB, B, W, T, const TAG: u32> FusedIterator
  for RepeatedDecoderIter<'a, 'de, T, RB, B, W, TAG>
where
  W: WireFormat<Groto> + 'de,
  T: Decode1<'de, W, RB, B, Groto> + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
{
}

/// A buffer that holds multiple `RepeatedDecoder` instances.
///
/// This buffer handles repeated fields that may appear multiple times in a message,
/// potentially interleaved with other fields. Each decoder instance represents a
/// contiguous batch of repeated elements found during parsing.
///
/// ## Use Cases
///
/// Protocol buffers allow repeated fields to appear multiple times in a message:
/// ```text
/// [other_field] [repeated_field_batch_1] [other_field] [repeated_field_batch_2] ...
/// ```
///
/// This buffer aggregates all batches and provides unified iteration over all elements.
///
/// ## Wire Format Handling
///
/// The buffer automatically handles:
/// - Multiple batches of the same repeated field
/// - Proper sequencing during iteration
/// - Encoding back to the original interleaved format
/// - Error propagation across all batches
///
/// ## Performance
///
/// - **Construction**: O(k) where k is the number of batches
/// - **Iteration**: O(1) amortized per element across all batches
/// - **Memory**: Stores references to decoder instances, minimal overhead
pub struct RepeatedDecoderBuffer<
  'a,
  T,
  RB,
  UB,
  W,
  const TAG: u32,
  B = DefaultBuffer<RepeatedDecoder<'a, T, RB, UB, W, TAG>>,
> {
  /// Buffer containing multiple RepeatedDecoder instances
  buffer: B,
  /// Phantom data for various lifetimes and types
  _lt: PhantomData<&'a ()>,
  _t: PhantomData<T>,
  _rb: PhantomData<RB>,
  _ub: PhantomData<UB>,
  _w: PhantomData<W>,
}

impl<'a, T, RB, UB, W, const TAG: u32, B> Clone for RepeatedDecoderBuffer<'a, T, RB, UB, W, TAG, B>
where
  B: Clone,
{
  fn clone(&self) -> Self {
    Self {
      buffer: self.buffer.clone(),
      _lt: PhantomData,
      _t: PhantomData,
      _rb: PhantomData,
      _ub: PhantomData,
      _w: PhantomData,
    }
  }
}

impl<'a, T, RB, UB, W, const TAG: u32, B> Copy for RepeatedDecoderBuffer<'a, T, RB, UB, W, TAG, B> where
  B: Copy
{
}

impl<'de, T, RB, UB, W, const TAG: u32, B> RepeatedDecoderBuffer<'de, T, RB, UB, W, TAG, B>
where
  B: Buffer<Item = RepeatedDecoder<'de, T, RB, UB, W, TAG>>,
{
  /// Creates an iterator over all elements across all decoder batches.
  ///
  /// The iterator processes elements from each decoder sequentially,
  /// automatically advancing to the next decoder when the current one
  /// is exhausted.
  pub fn iter(&self) -> RepeatedDecoderBufferIter<'_, 'de, T, RB, UB, W, TAG, B>
  where
    B: Copy,
  {
    let total_expected = self
      .buffer
      .as_slice()
      .iter()
      .map(|d| d.expected_elements)
      .sum();

    RepeatedDecoderBufferIter {
      buffer: self,
      has_error: false,
      current_index: 0,
      current_decoder_offset: 0,
      current_decoder_yielded_elements: 0,
      total_expected_elements: total_expected,
      total_yielded_elements: 0,
    }
  }

  /// Returns a hint for the total capacity needed to store all elements across all decoders.
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
  /// Each decoder represents a separate contiguous batch of repeated elements
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

impl<'a, T, RB, UB, W, const TAG: u32, B> Encode<Repeated<W, TAG>, Groto>
  for RepeatedDecoderBuffer<'a, T, RB, UB, W, TAG, B>
where
  W: WireFormat<Groto> + 'a,
  Repeated<W, TAG>: WireFormat<Groto> + 'a,
  T: Decode1<'a, W, RB, UB, Groto>,
  B: Buffer<Item = RepeatedDecoder<'a, T, RB, UB, W, TAG>>,
  RB: ReadBuf + 'a,
  UB: UnknownBuffer<RB, Groto> + 'a,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let encoded_raw_len = self.encoded_raw_len(context);
    let buf_len = buf.len();
    if buf_len < encoded_raw_len {
      return Err(Error::insufficient_buffer(encoded_raw_len, buf_len));
    }

    let mut offset = 0;
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

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    self.encode_raw(context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    self.encoded_raw_len(context)
  }
}

impl<'a, T, RB, UB, W, const TAG: u32, B> Decode1<'a, Repeated<W, TAG>, RB, UB, Groto>
  for RepeatedDecoderBuffer<'a, T, RB, UB, W, TAG, B>
where
  W: WireFormat<Groto> + 'a,
  Repeated<W, TAG>: WireFormat<Groto> + 'a,
  T: Decode1<'a, W, RB, UB, Groto>,
  B: Buffer<Item = RepeatedDecoder<'a, T, RB, UB, W, TAG>>,
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
      _t: PhantomData,
      _rb: PhantomData,
      _ub: PhantomData,
      _w: PhantomData,
    };

    <Self as Decode1<'a, Repeated<W, TAG>, RB, UB, Groto>>::merge_decode(&mut this, context, src)
      .map(|size| (size, this))
  }

  fn merge_decode(&mut self, ctx: &'a Context, src: RB) -> Result<usize, Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    UB: UnknownBuffer<RB, Groto> + 'a,
  {
    let (read, decoder) = <RepeatedDecoder<'a, T, RB, UB, W, TAG> as Decode1<
      'a,
      Repeated<W, TAG>,
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

impl<'a, T, B, UB, W, const TAG: u32> State<Partial<Groto>>
  for RepeatedDecoderBuffer<'a, T, B, UB, W, TAG>
{
  type Output = Self;
}

impl<'a, T, B, UB, W, const TAG: u32> State<PartialRef<'a, B, UB, Repeated<W, TAG>, Groto>>
  for RepeatedDecoderBuffer<'a, T, B, UB, W, TAG>
{
  type Output = Self;
}

impl<'a, T, B, UB, W, const TAG: u32> State<Ref<'a, B, UB, Repeated<W, TAG>, Groto>>
  for RepeatedDecoderBuffer<'a, T, B, UB, W, TAG>
{
  type Output = Self;
}

impl<'a, T, B, UB, W, S, const TAG: u32> State<Flattened<S>>
  for RepeatedDecoderBuffer<'a, T, B, UB, W, TAG>
where
  S: ?Sized,
{
  type Output = Self;
}

/// Iterator that processes elements from multiple `RepeatedDecoder` instances sequentially.
///
/// This iterator maintains state for traversing multiple decoder batches, automatically
/// advancing to the next decoder when the current one is exhausted. It provides
/// unified iteration over all elements across all decoders in the buffer.
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
/// - Elements yielded from current decoder (`current_decoder_yielded_elements`)
/// - Total progress across all decoders (`total_yielded_elements`)
///
/// ## Implementation Notes
///
/// The iterator creates temporary `RepeatedDecoderIter` instances and manually
/// restores their state to maintain consistent iteration across decoder boundaries.
/// This approach allows seamless transitions between decoder batches.
pub struct RepeatedDecoderBufferIter<'a, 'de: 'a, T, RB, UB, W, const TAG: u32, B> {
  /// Reference to the parent buffer
  buffer: &'a RepeatedDecoderBuffer<'de, T, RB, UB, W, TAG, B>,
  /// Error flag - once set, iteration stops permanently
  has_error: bool,
  /// Index of the currently active decoder batch
  current_index: usize,
  /// Byte offset within the current decoder
  current_decoder_offset: usize,
  /// Elements yielded from the current decoder
  current_decoder_yielded_elements: usize,
  /// Total elements expected across all decoders
  total_expected_elements: usize,
  /// Total elements yielded across all decoders
  total_yielded_elements: usize,
}

impl<'a, 'de, T, RB, UB, W, const TAG: u32, B>
  RepeatedDecoderBufferIter<'a, 'de, T, RB, UB, W, TAG, B>
{
  /// Returns the total capacity hint across all decoder batches.
  #[inline]
  pub const fn capacity_hint(&self) -> usize {
    self.total_expected_elements
  }

  /// Returns the total number of elements decoded so far across all batches.
  #[inline]
  pub const fn decoded(&self) -> usize {
    self.total_yielded_elements
  }

  /// Returns the estimated number of elements remaining across all batches,
  /// or `None` if an error occurred.
  ///
  /// This considers the total progress across all decoders in the buffer.
  ///
  /// # Returns
  ///
  /// - `Some(count)` - Number of elements left to attempt decoding
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
  /// This can be useful for tracking which batch of repeated elements
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

impl<'a, 'de, T, RB, UB, W, const TAG: u32, B>
  RepeatedDecoderBufferIter<'a, 'de, T, RB, UB, W, TAG, B>
where
  B: Buffer<Item = RepeatedDecoder<'de, T, RB, UB, W, TAG>>,
{
  /// Returns the total number of decoder batches.
  #[inline]
  pub fn batch_count(&self) -> usize {
    self.buffer.buffer.len()
  }
}

impl<'a, 'de: 'a, T, RB, UB, W, const TAG: u32, B> Iterator
  for RepeatedDecoderBufferIter<'a, 'de, T, RB, UB, W, TAG, B>
where
  W: WireFormat<Groto> + 'de,
  T: Decode1<'de, W, RB, UB, Groto> + Sized + 'de,
  RB: ReadBuf + 'de,
  UB: UnknownBuffer<RB, Groto> + 'de,
  B: Buffer<Item = RepeatedDecoder<'de, T, RB, UB, W, TAG>>,
{
  type Item = Result<(usize, T), Error>;

  fn next(&mut self) -> Option<Self::Item> {
    // Check if we've processed all decoder batches
    if self.current_index >= self.buffer.buffer.len() {
      return None;
    }

    // Check if we've decoded all expected elements
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

impl<'a, 'de: 'a, T, RB, UB, W, const TAG: u32, B> FusedIterator
  for RepeatedDecoderBufferIter<'a, 'de, T, RB, UB, W, TAG, B>
where
  W: WireFormat<Groto> + 'de,
  T: Decode1<'de, W, RB, UB, Groto> + Sized + 'de,
  RB: ReadBuf + 'de,
  UB: UnknownBuffer<RB, Groto> + 'de,
  B: Buffer<Item = RepeatedDecoder<'de, T, RB, UB, W, TAG>>,
{
}
