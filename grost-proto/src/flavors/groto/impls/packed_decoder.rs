use core::{iter::FusedIterator, marker::PhantomData};

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  convert::Flattened,
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, Packed, WireFormat,
    groto::{Context, Error, Fixed8},
  },
  selection::{Selectable, Selector},
  state::{Partial, PartialRef, Ref, State},
};

/// A lazy decoder for packed repeated elements from binary protocol data.
///
/// `PackedDecoder` provides efficient decoding of packed list-like structures
/// (such as `Vec<T>`, `[T]`, `HashSet<T>`, etc.) from binary data using the Groto
/// packed format. It implements lazy evaluation, meaning elements are only decoded
/// when explicitly requested through iteration.
///
/// ## Packed Wire Format
///
/// The decoder expects binary data in this packed format:
/// ```text
/// [total_length][element_count][element1][element2][element3]...
/// ```
///
/// Key characteristics:
/// - **Length-prefixed**: Total byte length of all element data
/// - **Count-prefixed**: Number of elements that follow
/// - **Contiguous elements**: No individual identifiers between elements
/// - **Efficient**: Minimal wire overhead for primitive types
///
/// ## Packed vs Repeated Format
///
/// Unlike `RepeatedDecoder` which handles:
/// ```text
/// [id][element] [id][element] [id][element]...
/// ```
///
/// `PackedDecoder` handles the more compact:
/// ```text
/// [length][count][element][element][element]...
/// ```
///
/// ## Construction Strategy
///
/// During construction, the decoder:
/// 1. Reads the total length prefix (varint)
/// 2. Reads the element count prefix (varint)
/// 3. Validates buffer contains enough data
/// 4. Sets up iteration over the contiguous element data
///
/// ## Special Optimizations
///
/// ### Byte Arrays (`T = u8`)
/// For byte arrays, the decoder provides direct slice access via:
/// - `as_slice()` - Returns the complete byte slice
/// - `Deref<Target = [u8]>` - Automatic dereferencing to slice
/// - `AsRef<[u8]>` - Reference conversion
///
/// This avoids element-by-element iteration for raw byte data.
///
/// ## Error Handling
///
/// The decoder implements fail-fast error semantics:
/// - Construction errors for malformed headers or insufficient data
/// - Iterator errors set an internal flag and stop iteration
/// - Use [`PackedDecoderIter::remaining_hint()`] to check error state
///
/// ## Performance
///
/// - **Construction**: O(1) time and space (just header parsing)
/// - **Iteration**: O(1) time per element, O(1) space
/// - **Memory**: Zero-copy when possible, minimal internal state
/// - **Byte optimization**: O(1) slice access for `u8` arrays
///
/// ## Thread Safety
///
/// `PackedDecoder` is `Send` and `Sync` when its buffer type allows it.
/// Multiple iterators can be created from the same decoder safely.
///
/// [`PackedDecoderIter::remaining_hint()`]: PackedDecoderIter::remaining_hint
pub struct PackedDecoder<'a, T, B, UB, W> {
  /// The source buffer containing all packed element data
  src: B,
  /// Number of elements as read from the count prefix
  expected_elements: usize,
  /// Size in bytes of the element count prefix
  num_elements_size: usize,
  /// Offset to the start of element data (after length and count prefixes)
  data_offset: usize,
  /// Decoding context for the Groto protocol
  ctx: &'a Context,
  /// Phantom data for type parameter `T`
  _t: PhantomData<T>,
  /// Phantom data for type parameter `W`
  _w: PhantomData<W>,
  /// Phantom data for type parameter `UB`
  _ub: PhantomData<UB>,
}

impl<'a, T, B: Clone, UB, W> Clone for PackedDecoder<'a, T, B, UB, W> {
  fn clone(&self) -> Self {
    Self {
      src: self.src.clone(),
      data_offset: self.data_offset,
      expected_elements: self.expected_elements,
      num_elements_size: self.num_elements_size,
      ctx: self.ctx,
      _t: PhantomData,
      _w: PhantomData,
      _ub: PhantomData,
    }
  }
}

impl<'a, T, B: Copy, UB, W> Copy for PackedDecoder<'a, T, B, UB, W> {}

impl<'de, T, B, UB, W> PackedDecoder<'de, T, B, UB, W> {
  /// Creates an iterator that borrows from the decoder.
  ///
  /// The returned iterator will have the same lifetime as the decoder.
  /// Multiple iterators can be created from the same decoder, each maintaining
  /// independent iteration state.
  ///
  /// For byte arrays (`T = u8`), consider using `as_slice()` for better performance.
  #[inline]
  pub const fn iter(&self) -> PackedDecoderIter<'_, 'de, T, B, UB, W> {
    PackedDecoderIter {
      decoder: self,
      offset: self.data_offset + self.num_elements_size,
      yielded_elements: 0,
      expected_elements: self.expected_elements,
      has_error: false,
    }
  }

  /// Returns a hint for the capacity needed to store all elements.
  ///
  /// This value is read from the element count prefix during construction.
  /// It represents the number of elements that the encoder claimed to have packed.
  ///
  /// **Important**: This is a capacity *hint* based on the wire format,
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

  /// Returns whether the decoder contains any elements.
  #[inline]
  pub const fn is_empty(&self) -> bool {
    self.expected_elements == 0
  }
}

impl<'a, RB, B> PackedDecoder<'a, u8, RB, B, Fixed8>
where
  RB: ReadBuf,
{
  /// Returns a slice to the fully decoded byte data.
  ///
  /// This method is specifically optimized for packed byte arrays (`T = u8`).
  /// Since the target type is a byte slice, no element-by-element decoding
  /// is needed. The decoder can immediately provide the complete decoded
  /// byte slice with zero overhead.
  ///
  /// For non-byte types, use the iterator instead.
  ///
  /// # Performance
  ///
  /// This is an O(1) operation that provides direct access to the underlying
  /// byte data without any copying or additional processing.
  #[inline]
  pub fn as_slice(&self) -> &[u8] {
    let src = &self.src;
    if src.is_empty() {
      return src.as_bytes();
    }

    let src_len = src.len();
    let start_offset = self.data_offset + self.num_elements_size;

    if src_len <= start_offset {
      return &[];
    }

    &src.as_bytes()[start_offset..]
  }
}

impl<'a, RB, B> core::ops::Deref for PackedDecoder<'a, u8, RB, B, Fixed8>
where
  RB: ReadBuf,
{
  type Target = [u8];

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.as_slice()
  }
}

impl<'a, RB, B> AsRef<[u8]> for PackedDecoder<'a, u8, RB, B, Fixed8>
where
  RB: ReadBuf,
{
  #[inline]
  fn as_ref(&self) -> &[u8] {
    self
  }
}

impl<'a, T, B, UB, W> Selectable<Groto> for PackedDecoder<'a, T, B, UB, W>
where
  T: Selectable<Groto>,
  W: WireFormat<Groto> + 'a,
{
  type Selector = T::Selector;
}

impl<'a, T, RB, B, W> Encode<Packed<W>, Groto> for PackedDecoder<'a, T, RB, B, W>
where
  Packed<W>: WireFormat<Groto> + 'a,
  RB: ReadBuf,
{
  fn encode_raw(&self, ctx: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let buf_len = buf.len();
    let src_len = self.encoded_raw_len(ctx);
    if buf_len < src_len {
      return Err(Error::insufficient_buffer(src_len, buf_len));
    }

    let start_offset = self.data_offset + self.num_elements_size;
    buf[..src_len].copy_from_slice(&self.src.as_bytes()[start_offset..]);
    Ok(src_len)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    let start_offset = self.data_offset + self.num_elements_size;
    self.src.len().saturating_sub(start_offset)
  }

  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let src = &self.src;
    let buf_len = buf.len();
    let src_len = src.len();

    if buf_len < src_len {
      return Err(Error::insufficient_buffer(src_len, buf_len));
    }

    buf[..src_len].copy_from_slice(src.as_bytes());
    Ok(src_len)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    self.src.len()
  }
}

impl<'a, T, RB, B, W> PartialEncode<Packed<W>, Groto> for PackedDecoder<'a, T, RB, B, W>
where
  W: WireFormat<Groto> + 'a,
  Packed<W>: WireFormat<Groto> + 'a,
  RB: ReadBuf,
  T: Selectable<Groto>,
{
  fn partial_encode_raw(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if selector.is_empty() {
      return Ok(0);
    }

    <Self as Encode<Packed<W>, Groto>>::encode_raw(self, context, buf)
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    <Self as Encode<Packed<W>, Groto>>::encoded_raw_len(self, context)
  }

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if selector.is_empty() {
      return Ok(0);
    }

    <Self as Encode<Packed<W>, Groto>>::encode(self, context, buf)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    <Self as Encode<Packed<W>, Groto>>::encoded_len(self, context)
  }
}

impl<'a, T, B, W, RB> Decode<'a, Packed<W>, RB, B, Groto> for PackedDecoder<'a, T, RB, B, W>
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
    let buf = src.as_bytes();
    let buf_len = buf.len();

    if buf_len == 0 {
      return Err(Error::buffer_underflow());
    }

    // Decode the total length prefix
    let (length_prefix_size, data_length) = varing::decode_u32_varint(buf)?;
    let data_length = data_length as usize;

    // Verify we have enough data for the declared length
    let total_consumed = length_prefix_size + data_length;
    if total_consumed > buf_len {
      return Err(Error::buffer_underflow());
    }

    // Decode the element count prefix
    let count_start = length_prefix_size;
    let (count_prefix_size, element_count) = varing::decode_u32_varint(&buf[count_start..])?;
    let element_count = element_count as usize;

    Ok((
      total_consumed,
      Self {
        src: src.slice(..total_consumed),
        data_offset: length_prefix_size,
        expected_elements: element_count,
        num_elements_size: count_prefix_size,
        ctx,
        _t: PhantomData,
        _w: PhantomData,
        _ub: PhantomData,
      },
    ))
  }
}

impl<'a, T, B, UB, W> State<Partial<Groto>> for PackedDecoder<'a, T, B, UB, W> {
  type Output = Self;
}

impl<'a, T, B, UB, W> State<PartialRef<'a, B, UB, Packed<W>, Groto>>
  for PackedDecoder<'a, T, B, UB, W>
{
  type Output = Self;
}

impl<'a, T, B, UB, W> State<Ref<'a, B, UB, Packed<W>, Groto>> for PackedDecoder<'a, T, B, UB, W> {
  type Output = Self;
}

impl<'a, T, B, UB, W, S> State<Flattened<S>> for PackedDecoder<'a, T, B, UB, W>
where
  S: ?Sized,
{
  type Output = Self;
}

/// Iterator for lazily decoding packed elements.
///
/// This iterator maintains its own state and can be created multiple times
/// from the same decoder. Each iterator starts from the beginning and
/// maintains independent progress tracking.
///
/// ## Error Handling
///
/// The iterator implements fail-fast semantics:
/// - First error sets the `has_error` flag
/// - All subsequent `next()` calls return `None`
/// - Use `remaining_hint()` to detect error state
///
/// ## Packed Format Iteration
///
/// Unlike repeated format iterators, this iterator processes contiguous
/// element data without identifier prefixes between elements. Each call
/// to `next()` advances through the packed data sequentially.
///
/// ## State Management
///
/// The iterator tracks:
/// - Current byte offset in the element data
/// - Number of elements successfully decoded
/// - Error state for fail-fast behavior
/// - Expected element count (from count prefix)
pub struct PackedDecoderIter<'a, 'de: 'a, T, RB, UB, W> {
  /// Reference to the parent decoder
  decoder: &'a PackedDecoder<'de, T, RB, UB, W>,
  /// Current byte offset within the source buffer
  offset: usize,
  /// Number of elements successfully decoded so far
  yielded_elements: usize,
  /// Total elements expected (from count prefix)
  expected_elements: usize,
  /// Error flag - once set, iteration stops permanently
  has_error: bool,
}

impl<'a, 'de: 'a, T, B, UB, W> PackedDecoderIter<'a, 'de, T, B, UB, W> {
  /// Returns the current byte position within the source buffer.
  ///
  /// This represents how many bytes have been consumed during iteration
  /// from the start of the element data (after prefixes).
  #[inline]
  pub const fn position(&self) -> usize {
    self.offset
  }

  /// Returns the number of elements that have been successfully decoded so far.
  ///
  /// This count increases each time [`next()`] successfully returns an element.
  /// It will never exceed the expected element count from the count prefix.
  ///
  /// [`next()`]: Iterator::next
  #[inline]
  pub const fn decoded(&self) -> usize {
    self.yielded_elements
  }

  /// Returns the capacity hint from the count prefix.
  ///
  /// This value is read from the element count prefix during decoder construction.
  /// See [`PackedDecoder::capacity_hint()`] for more details.
  ///
  /// [`PackedDecoder::capacity_hint()`]: PackedDecoder::capacity_hint
  #[inline]
  pub const fn capacity_hint(&self) -> usize {
    self.expected_elements
  }

  /// Returns the estimated number of elements remaining, or `None` if an error occurred.
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

  /// Returns whether the iterator has encountered an error.
  #[inline]
  pub const fn has_error(&self) -> bool {
    self.has_error
  }
}

impl<'a, 'de: 'a, RB, B, W, T> Iterator for PackedDecoderIter<'a, 'de, T, RB, B, W>
where
  W: WireFormat<Groto> + 'de,
  T: Decode<'de, W, RB, B, Groto> + 'de,
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

    // Decode the next element from the packed data
    // No identifier prefixes in packed format - elements are contiguous
    Some(
      T::decode(self.decoder.ctx, self.decoder.src.slice(self.offset..))
        .inspect(|(read, _)| {
          // Update position by the number of bytes consumed
          self.offset += read;
          self.yielded_elements += 1;
        })
        .inspect_err(|_| {
          // Set error flag to prevent further iteration attempts
          self.has_error = true;
        }),
    )
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, self.remaining_hint())
  }
}

impl<'a, 'de: 'a, RB, B, W, T> FusedIterator for PackedDecoderIter<'a, 'de, T, RB, B, W>
where
  W: WireFormat<Groto> + 'de,
  T: Decode<'de, W, RB, B, Groto> + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
{
}
