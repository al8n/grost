use core::{iter::FusedIterator, marker::PhantomData};

use crate::{
  buffer::{Buf, BufMut, UnknownBuffer},
  convert::{Extracted, PartialIdentity},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, PackedEntry, WireFormat,
    groto::{Context, Error, Identifier, Tag},
  },
  selection::{Selectable, Selector},
  state::{Partial, PartialRef, Ref, State},
};

use super::PartialMapEntry;

/// A lazy decoder for packed map entries from binary protocol data.
///
/// `PackedMapDecoder` provides efficient decoding of packed map-like structures
/// (such as `HashMap<K, V>`, `BTreeMap<K, V>`, etc.) from binary data using the Groto
/// packed format. It implements lazy evaluation, meaning key-value pairs are only
/// decoded when explicitly requested through iteration.
///
/// ## Packed Wire Format
///
/// The decoder expects binary data in this packed format:
/// ```text
/// [total_length][element_count][key identifier][key1][value identifier][value1][key identifier][key2][value identifer][value2]...
/// ```
///
/// Key characteristics:
/// - **Length-prefixed**: Total byte length of all key-value data
/// - **Count-prefixed**: Number of key-value pairs that follow
/// - **Contiguous pairs**: Keys and values alternate without identifiers
/// - **Efficient**: Minimal wire overhead for map entries
///
/// ## Construction Strategy
///
/// During construction, the decoder:
/// 1. Reads the total length prefix (varint)
/// 2. Reads the element count prefix (varint) - number of key-value pairs
/// 3. Validates buffer contains enough data
/// 4. Sets up iteration over the contiguous key-value data
///
/// ## Error Handling
///
/// The decoder implements fail-fast error semantics:
/// - Construction errors for malformed headers or insufficient data
/// - Iterator errors set an internal flag and stop iteration
/// - Use [`PackedMapDecoderIter::remaining_hint()`] to check error state
///
/// ## Performance
///
/// - **Construction**: O(1) time and space (just header parsing)
/// - **Iteration**: O(1) time per key-value pair, O(1) space
/// - **Memory**: Zero-copy when possible, minimal internal state
///
/// ## Thread Safety
///
/// `PackedMapDecoder` is `Send` and `Sync` when its buffer type allows it.
/// Multiple iterators can be created from the same decoder safely.
pub struct PackedMapDecoder<'a, K, V, B, UB, KW, VW> {
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
  key_identifier: Identifier,
  value_identifier: Identifier,
  _k: PhantomData<K>,
  _v: PhantomData<V>,
  _kw: PhantomData<KW>,
  _vw: PhantomData<VW>,
  _ub: PhantomData<UB>,
}

impl<'a, K, V, B: Clone, UB, KW, VW> Clone for PackedMapDecoder<'a, K, V, B, UB, KW, VW> {
  fn clone(&self) -> Self {
    Self {
      src: self.src.clone(),
      data_offset: self.data_offset,
      expected_elements: self.expected_elements,
      num_elements_size: self.num_elements_size,
      key_identifier: self.key_identifier,
      value_identifier: self.value_identifier,
      ctx: self.ctx,
      _k: PhantomData,
      _v: PhantomData,
      _kw: PhantomData,
      _vw: PhantomData,
      _ub: PhantomData,
    }
  }
}

impl<'a, K, V, B: Copy, UB, KW, VW> Copy for PackedMapDecoder<'a, K, V, B, UB, KW, VW> {}

impl<'de, K, V, B, UB, KW, VW> PackedMapDecoder<'de, K, V, B, UB, KW, VW> {
  /// Creates an iterator that borrows from the decoder.
  ///
  /// The returned iterator will have the same lifetime as the decoder.
  /// Multiple iterators can be created from the same decoder, each maintaining
  /// independent iteration state.
  #[inline]
  pub const fn iter(&self) -> PackedMapDecoderIter<'_, 'de, K, V, B, UB, KW, VW> {
    PackedMapDecoderIter {
      decoder: self,
      offset: self.data_offset + self.num_elements_size,
      yielded_elements: 0,
      expected_elements: self.expected_elements,
      has_error: false,
    }
  }

  /// Returns a hint for the capacity needed to store all key-value pairs.
  ///
  /// This value is read from the element count prefix during construction.
  /// It represents the number of key-value pairs that the encoder claimed to have packed.
  ///
  /// **Important**: This is a capacity *hint* based on the wire format,
  /// not a guarantee of successful decoding.
  #[inline]
  pub const fn capacity_hint(&self) -> usize {
    self.expected_elements
  }

  /// Returns whether the decoder contains any key-value pairs.
  #[inline]
  pub const fn is_empty(&self) -> bool {
    self.expected_elements == 0
  }
}

impl<'a, K, V, RB, B, KW, VW> Encode<PackedEntry<KW, VW>, Groto>
  for PackedMapDecoder<'a, K, V, RB, B, KW, VW>
where
  PackedEntry<KW, VW>: WireFormat<Groto> + 'a,
  RB: Buf,
{
  fn encode_raw<WB>(&self, ctx: &Context, buf: &mut WB) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    let buf_len = buf.len();
    let src_len = self.encoded_raw_len(ctx);

    let start_offset = self.data_offset + self.num_elements_size;

    match buf.prefix_mut_checked(src_len) {
      None => Err(Error::buffer_too_small(src_len, buf_len)),
      Some(buf) => {
        buf.copy_from_slice(&self.src.remaining_slice()[start_offset..]);
        Ok(src_len)
      }
    }
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    let start_offset = self.data_offset + self.num_elements_size;
    self.src.remaining().saturating_sub(start_offset)
  }

  fn encode<WB>(&self, _: &Context, buf: &mut WB) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    let src = &self.src;
    let buf_len = buf.len();
    let src_len = src.remaining();

    match buf.prefix_mut_checked(src_len) {
      None => Err(Error::buffer_too_small(src_len, buf_len)),
      Some(buf) => {
        buf.copy_from_slice(src.remaining_slice());
        Ok(src_len)
      }
    }
  }

  fn encoded_len(&self, _: &Context) -> usize {
    self.src.remaining()
  }
}

impl<'a, K, V, RB, B, KW, VW> PartialEncode<PackedEntry<KW, VW>, Groto>
  for PackedMapDecoder<'a, K, V, RB, B, KW, VW>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  PackedEntry<KW, VW>: WireFormat<Groto> + 'a,
  RB: Buf,
  V: Selectable<Groto>,
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
    // Check if either key or value selector is empty
    if selector.is_empty() {
      return Ok(0);
    }

    <Self as Encode<PackedEntry<KW, VW>, Groto>>::encode_raw(self, context, buf)
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    <Self as Encode<PackedEntry<KW, VW>, Groto>>::encoded_raw_len(self, context)
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
    if selector.is_empty() {
      return Ok(0);
    }

    <Self as Encode<PackedEntry<KW, VW>, Groto>>::encode(self, context, buf)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    <Self as Encode<PackedEntry<KW, VW>, Groto>>::encoded_len(self, context)
  }
}

impl<'a, K, V, B, KW, VW, RB> Decode<'a, PackedEntry<KW, VW>, RB, B, Groto>
  for PackedMapDecoder<'a, K, V, RB, B, KW, VW>
where
  PackedEntry<KW, VW>: WireFormat<Groto> + 'a,
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  RB: Buf,
{
  fn decode(ctx: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: crate::buffer::Buf,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let buf = src.remaining_slice();
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

    // Decode the element count prefix (number of key-value pairs)
    let count_start = length_prefix_size;
    let (count_prefix_size, element_count) = varing::decode_u32_varint(&buf[count_start..])?;
    let element_count = element_count as usize;

    Ok((
      total_consumed,
      Self {
        src: src.segment(..total_consumed),
        data_offset: length_prefix_size,
        expected_elements: element_count,
        num_elements_size: count_prefix_size,
        key_identifier: Identifier::new(KW::WIRE_TYPE, Tag::new(1)),
        value_identifier: Identifier::new(VW::WIRE_TYPE, Tag::new(2)),
        ctx,
        _k: PhantomData,
        _v: PhantomData,
        _kw: PhantomData,
        _vw: PhantomData,
        _ub: PhantomData,
      },
    ))
  }
}

impl<'a, K, V, B, UB, KW, VW> Selectable<Groto> for PackedMapDecoder<'a, K, V, B, UB, KW, VW>
where
  V: Selectable<Groto>,
  VW: WireFormat<Groto> + 'a,
{
  // For maps, we might use a composite selector that can select both keys and values
  type Selector = V::Selector;
}

impl<'a, K, V, B, UB, KW, VW> State<Partial<Groto>> for PackedMapDecoder<'a, K, V, B, UB, KW, VW> {
  type Output = Self;
}

impl<'a, K, V, B, UB, KW, VW> State<PartialRef<'a, PackedEntry<KW, VW>, B, UB, Groto>>
  for PackedMapDecoder<'a, K, V, B, UB, KW, VW>
{
  type Output = Self;
}

impl<'a, K, V, B, UB, KW, VW> State<Ref<'a, PackedEntry<KW, VW>, B, UB, Groto>>
  for PackedMapDecoder<'a, K, V, B, UB, KW, VW>
{
  type Output = Self;
}

impl<'a, K, V, B, UB, KW, VW, S> State<Extracted<S>> for PackedMapDecoder<'a, K, V, B, UB, KW, VW> {
  type Output = Self;
}

impl<'a, K, V, B, UB, KW, VW> PartialIdentity<Groto> for PackedMapDecoder<'a, K, V, B, UB, KW, VW>
where
  V: PartialIdentity<Groto> + Selectable<Groto>,
  V::Output: Sized + Selectable<Groto, Selector = V::Selector>,
  VW: WireFormat<Groto> + 'a,
{
  fn partial_identity<'b>(
    input: &'b mut Self::Output,
    _: &'b Self::Selector,
  ) -> &'b mut Self::Output {
    input
  }
}

/// Iterator for lazily decoding packed map entries.
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
/// The iterator processes contiguous key-value pairs without identifier
/// prefixes between elements. Each call to `next()` decodes both a key
/// and its associated value sequentially.
pub struct PackedMapDecoderIter<'a, 'de: 'a, K, V, RB, UB, KW, VW> {
  /// Reference to the parent decoder
  decoder: &'a PackedMapDecoder<'de, K, V, RB, UB, KW, VW>,
  /// Current byte offset within the source buffer
  offset: usize,
  /// Number of key-value pairs successfully decoded so far
  yielded_elements: usize,
  /// Total key-value pairs expected (from count prefix)
  expected_elements: usize,
  /// Error flag - once set, iteration stops permanently
  has_error: bool,
}

impl<'a, 'de: 'a, K, V, B, UB, KW, VW> PackedMapDecoderIter<'a, 'de, K, V, B, UB, KW, VW> {
  /// Returns the current byte position within the source buffer.
  #[inline]
  pub const fn position(&self) -> usize {
    self.offset
  }

  /// Returns the number of key-value pairs that have been successfully decoded so far.
  #[inline]
  pub const fn decoded(&self) -> usize {
    self.yielded_elements
  }

  /// Returns the capacity hint from the count prefix.
  #[inline]
  pub const fn capacity_hint(&self) -> usize {
    self.expected_elements
  }

  /// Returns the estimated number of key-value pairs remaining, or `None` if an error occurred.
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

impl<'a, 'de: 'a, RB, UB, KW, VW, K, V> Iterator
  for PackedMapDecoderIter<'a, 'de, K, V, RB, UB, KW, VW>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: Decode<'de, KW, RB, UB, Groto> + 'de,
  V: Decode<'de, VW, RB, UB, Groto> + 'de,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: Buf + 'de,
{
  type Item = Result<(usize, PartialMapEntry<K, V>), Error>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let src_len = self.decoder.src.remaining();

    // Check if we've reached the end of the buffer
    if self.offset >= src_len {
      return None;
    }

    // Check if we've decoded all expected key-value pairs
    if self.yielded_elements >= self.expected_elements {
      return None;
    }

    // Check if we're in an error state
    if self.has_error {
      return None;
    }

    Some(
      PartialMapEntry::decode_packed_entry(
        self.decoder.ctx,
        self.decoder.src.segment(self.offset..),
        &self.decoder.key_identifier,
        &self.decoder.value_identifier,
      )
      .inspect(|(read, _)| {
        self.offset += read;
        self.yielded_elements += 1;
      })
      .inspect_err(|_| {
        self.has_error = true;
      }),
    )
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, self.remaining_hint())
  }
}

impl<'a, 'de: 'a, RB, B, KW, VW, K, V> FusedIterator
  for PackedMapDecoderIter<'a, 'de, K, V, RB, B, KW, VW>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: Decode<'de, KW, RB, B, Groto> + 'de,
  V: Decode<'de, VW, RB, B, Groto> + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: Buf + 'de,
{
}
