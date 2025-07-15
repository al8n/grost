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
/// The decoder expects binary data in this format:
/// ```text
/// [identifier][first_element] [identifier][second_element] [identifier][third_element] ...
/// ```
///
/// Key points:
/// - Elements are prefixed with their identifier (wire type + tag)
/// - All elements share the same identifier
///
/// ## Construction Strategy
///
/// During construction, the decoder performs a **greedy scan** of the input buffer to:
/// 1. Count total elements with matching identifiers
/// 2. Determine the byte length of all repeated data
/// 3. Validate the structure and set up iteration state
///
/// This upfront work enables efficient lazy iteration and accurate progress reporting.
///
/// ## Error Handling
///
/// The decoder implements fail-fast error semantics:
/// - Any decoding error sets an internal error flag
/// - Once an error occurs, all subsequent iterations return `None`
/// - Use [`remaining_expected()`] to check if errors occurred
///
/// ## Performance
///
/// - **Construction**: O(n) time for greedy scan, O(1) space
/// - **Iteration**: O(1) time per element, O(1) space
/// - **Memory**: Zero-copy when possible, minimal internal state
///
/// [`remaining_expected()`]: RepeatedDecoder::remaining_expected
pub struct RepeatedDecoder<'a, T: ?Sized, B, UB: ?Sized, W: ?Sized, const TAG: u32> {
  /// the source buffer
  src: B,
  expected_elements: usize,
  identifier_size: usize,
  ctx: &'a Context,
  _t: PhantomData<T>,
  _w: PhantomData<W>,
  _ub: PhantomData<UB>,
}

impl<'a, T: ?Sized, B: Clone, UB: ?Sized, W: ?Sized, const TAG: u32> Clone
  for RepeatedDecoder<'a, T, B, UB, W, TAG>
{
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

impl<'a, T: ?Sized, B: Copy, UB: ?Sized, W: ?Sized, const TAG: u32> Copy
  for RepeatedDecoder<'a, T, B, UB, W, TAG>
{
}

impl<'de, T, B, UB, W, const TAG: u32> RepeatedDecoder<'de, T, B, UB, W, TAG>
where
  T: ?Sized,
  W: ?Sized,
{
  /// Returns a lazy decoder iterator for this repeated decoder.
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
    self.expected_elements
  }
}

impl<'a, T, B, UB, W, const TAG: u32> Selectable<Groto> for RepeatedDecoder<'a, T, B, UB, W, TAG>
where
  T: ?Sized + Selectable<Groto>,
  W: WireFormat<Groto> + 'a,
  UB: ?Sized,
{
  type Selector = T::Selector;
}

impl<'a, T, RB, B, W, const TAG: u32> Encode<Repeated<W, TAG>, Groto>
  for RepeatedDecoder<'a, T, RB, B, W, TAG>
where
  T: ?Sized,
  W: WireFormat<Groto> + 'a,
  B: ?Sized,
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
    let identifier = Identifier::new(W::WIRE_TYPE, Tag::new(TAG));
    let mut num_elements = 0;
    let mut offset = 0;
    let buf = src.as_bytes();
    let buf_len = buf.len();
    if buf_len == 0 {
      return Err(Error::buffer_underflow());
    }

    // The following elements should be prefixed with the identifier.
    // | identifier | element | identifier | element | ...
    loop {
      if offset >= buf_len {
        break;
      }

      let (read, next_id) = Identifier::decode(&buf[offset..])?;

      // If the next identifier does not match the expected identifier, which means we have reached the end of the repeated elements.
      // We should stop decoding. We do not need to increment the offset here because we are not consuming the next identifier.
      if next_id != identifier {
        break;
      }

      // increment the offset by the size of the identifier
      offset += read;
      // consum the next element
      offset += Groto::peek_raw(ctx, W::WIRE_TYPE, &buf[offset..])?;
      num_elements += 1;
    }

    Ok((
      offset,
      Self {
        src: src.slice(..offset),
        expected_elements: num_elements,
        identifier_size: identifier.encoded_len(),
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
where
  T: ?Sized,
  W: ?Sized,
  UB: ?Sized,
{
  type Output = Self;
}

impl<'a, T, B, UB, W, const TAG: u32> State<PartialRef<'a, B, UB, Repeated<W, TAG>, Groto>>
  for RepeatedDecoder<'a, T, B, UB, W, TAG>
where
  T: ?Sized,
  W: ?Sized,
  UB: ?Sized,
{
  type Output = Self;
}

impl<'a, T, B, UB, W, const TAG: u32> State<Ref<'a, B, UB, Repeated<W, TAG>, Groto>>
  for RepeatedDecoder<'a, T, B, UB, W, TAG>
where
  T: ?Sized,
  W: ?Sized,
  UB: ?Sized,
{
  type Output = Self;
}

impl<'a, T, B, UB, W, S, const TAG: u32> State<Flattened<S>>
  for RepeatedDecoder<'a, T, B, UB, W, TAG>
where
  T: ?Sized,
  W: ?Sized,
  UB: ?Sized,
  S: ?Sized,
{
  type Output = Self;
}

pub struct RepeatedDecoderIter<'a, 'de: 'a, T: ?Sized, RB, UB, W: ?Sized, const TAG: u32> {
  decoder: &'a RepeatedDecoder<'de, T, RB, UB, W, TAG>,
  expected_elements: usize,
  has_error: bool,
  identifier_size: usize,
  yielded_elements: usize,
  offset: usize,
}

impl<'a, 'de: 'a, T, B, UB, W, const TAG: u32> RepeatedDecoderIter<'a, 'de, T, B, UB, W, TAG>
where
  T: ?Sized,
  W: ?Sized,
{
  /// Returns the current byte position within the source buffer.
  ///
  /// This represents how many bytes have been consumed during iteration.
  #[inline]
  pub const fn current_position(&self) -> usize {
    self.offset
  }

  /// Returns the number of elements that have been successfully decoded so far.
  ///
  /// This count increases each time `next()` successfully returns an element.
  /// It will never exceed the expected element count from the wire format.
  #[inline]
  pub const fn decoded_count(&self) -> usize {
    self.yielded_elements
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
    self.expected_elements
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
  T: Decode1<'de, W, RB, B, Groto> + Sized + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
{
  type Item = Result<(usize, T), Error>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let src_len = self.decoder.src.len();
    if self.offset >= src_len {
      return None;
    }

    if self.yielded_elements >= self.expected_elements {
      return None;
    }

    if self.has_error {
      return None;
    }

    if self.offset + self.identifier_size >= src_len {
      // If the offset plus the identifier size exceeds the source length,
      // it means something wrong happened during `T` decoding.
      // Because when we construct `RepeatedDecoder`, we already checked there
      // should be exactly `expected_elements` elements in the source.
      // So we can safely assume that the source is malformed.
      // We set `has_error` to true and return an error.
      // This will prevent further decoding attempts.
      self.has_error = true;
      return Some(Err(Error::buffer_underflow()));
    }

    Some(
      T::decode(
        self.decoder.ctx,
        self.decoder.src.slice(self.offset + self.identifier_size..),
      )
      .inspect(|(read, _)| {
        self.offset += self.identifier_size + read;
        self.yielded_elements += 1;
      })
      .inspect_err(|_| {
        self.has_error = true;
      }),
    )
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, self.remaining_expected())
  }
}

impl<'a, 'de, RB, B, W, T, const TAG: u32> FusedIterator
  for RepeatedDecoderIter<'a, 'de, T, RB, B, W, TAG>
where
  W: WireFormat<Groto> + 'de,
  T: Decode1<'de, W, RB, B, Groto> + Sized + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
{
}

/// A buffer that holds multiple `RepeatedDecoder` instances.
///
/// As repeated wire format allows elements to be encoded with gaps(i.e., the source buffer may contain other fields between different repeated elements),
/// this buffer is designed to hold multiple decoders, each representing a separate repeated elements.
pub struct RepeatedDecoderBuffer<
  'a,
  T,
  RB,
  UB,
  W,
  const TAG: u32,
  B = DefaultBuffer<RepeatedDecoder<'a, T, RB, UB, W, TAG>>,
> {
  buffer: B,
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
  B: Buffer<Item = RepeatedDecoder<'de, T, RB, UB, W, TAG>> + Copy,
{
  pub fn iter(&self) -> RepeatedDecoderBufferIter<'_, 'de, T, RB, UB, W, TAG, B> {
    RepeatedDecoderBufferIter {
      buffer: self,
      has_error: false,
      current_index: 0,
      current_decoder_offset: 0,
      current_decoder_yielded_elements: 0,
      total_expected_elements: self
        .buffer
        .as_slice()
        .iter()
        .map(|d| d.expected_elements)
        .sum(),
      total_yielded_elements: 0,
    }
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
  pub fn expected_count(&self) -> usize {
    self
      .buffer
      .as_slice()
      .iter()
      .map(|d| d.expected_elements)
      .sum()
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
      return Err(Error::custom("failed to push decoder into buffer"));
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

pub struct RepeatedDecoderBufferIter<'a, 'de: 'a, T, RB, UB, W, const TAG: u32, B> {
  buffer: &'a RepeatedDecoderBuffer<'de, T, RB, UB, W, TAG, B>,
  has_error: bool,
  current_index: usize,
  current_decoder_offset: usize,
  current_decoder_yielded_elements: usize,
  total_expected_elements: usize,
  total_yielded_elements: usize,
}

impl<'a, 'de, T, RB, UB, W, const TAG: u32, B>
  RepeatedDecoderBufferIter<'a, 'de, T, RB, UB, W, TAG, B>
{
  /// Returns the number of elements expected.
  #[inline]
  pub const fn expected_count(&self) -> usize {
    self.total_expected_elements
  }

  /// Returns the number of elements yielded so far.
  #[inline]
  pub const fn decoded_count(&self) -> usize {
    self.total_yielded_elements
  }

  /// Returns the remaining expected elements to be yielded.
  #[inline]
  pub const fn remaining_expected(&self) -> Option<usize> {
    if self.has_error {
      return None;
    }

    Some(
      self
        .total_expected_elements
        .saturating_sub(self.total_yielded_elements),
    )
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
    if self.current_index >= self.buffer.buffer.len() {
      return None;
    }

    if self.total_yielded_elements >= self.total_expected_elements {
      return None;
    }

    if self.has_error {
      return None;
    }

    let decoder = &self.buffer.buffer.as_slice()[self.current_index];
    let mut iter = decoder.iter();
    iter.offset = self.current_decoder_offset;
    iter.yielded_elements = self.current_decoder_yielded_elements;

    match iter.next() {
      Some(Ok((read, item))) => {
        self.current_decoder_offset = iter.offset;
        self.current_decoder_yielded_elements = iter.yielded_elements;
        self.total_yielded_elements += 1;

        if self.current_decoder_yielded_elements >= decoder.expected_elements {
          self.current_index += 1;
          self.current_decoder_offset = 0;
          self.current_decoder_yielded_elements = 0;
        }

        Some(Ok((read, item)))
      }
      Some(Err(e)) => {
        self.has_error = true;
        Some(Err(e))
      }
      None => {
        // If the current decoder has no more elements, move to the next one.
        self.current_index += 1;
        self.current_decoder_offset = 0;
        self.current_decoder_yielded_elements = 0;
        self.next()
      }
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, self.remaining_expected())
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
