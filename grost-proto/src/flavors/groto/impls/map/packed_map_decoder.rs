use core::{iter::FusedIterator, marker::PhantomData};

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  convert::{Flattened, Partial, PartialRef, Ref},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, Packed, WireFormat,
    groto::{Context, Error, Fixed8},
  },
  selection::{Selectable, Selector},
  state::State,
};

use super::MapEntry;

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
  _k: PhantomData<K>,
  _v: PhantomData<V>,
  _kw: PhantomData<KW>,
  _vw: PhantomData<VW>,
  _ub: PhantomData<UB>,
}

impl<'a, K, V, B: Clone, UB, KW, VW> Clone 
  for PackedMapDecoder<'a, K, V, B, UB, KW, VW> 
{
  fn clone(&self) -> Self {
    Self {
      src: self.src.clone(),
      data_offset: self.data_offset,
      expected_elements: self.expected_elements,
      num_elements_size: self.num_elements_size,
      ctx: self.ctx,
      _k: PhantomData,
      _v: PhantomData,
      _kw: PhantomData,
      _vw: PhantomData,
      _ub: PhantomData,
    }
  }
}

impl<'a, K, V, B: Copy, UB, KW, VW> Copy 
  for PackedMapDecoder<'a, K, V, B, UB, KW, VW> 
{}

impl<'de, K, V, B, UB, KW, VW> 
  PackedMapDecoder<'de, K, V, B, UB, KW, VW> 
{
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

impl<'a, 'de: 'a, K, V, B, UB, KW, VW> 
  PackedMapDecoderIter<'a, 'de, K, V, B, UB, KW, VW> 
{
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

impl<'a, 'de: 'a, RB, B, KW, VW, K, V> Iterator 
  for PackedMapDecoderIter<'a, 'de, K, V, RB, B, KW, VW>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: Decode1<'de, KW, RB, B, Groto> + 'de,
  V: Decode1<'de, VW, RB, B, Groto> + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
{
  type Item = Result<(usize, MapEntry<K, V>), Error>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let src_len = self.decoder.src.len();

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

    // Decode the key
    let key_result = K::decode(self.decoder.ctx, self.decoder.src.slice(self.offset..));
    
    let (key_size, key) = match key_result {
      Ok((size, k)) => {
        self.offset += size;
        (size, k)
      }
      Err(e) => {
        self.has_error = true;
        return Some(Err(e));
      }
    };

    // Decode the value
    let value_result = V::decode(self.decoder.ctx, self.decoder.src.slice(self.offset..));
    
    match value_result {
      Ok((value_size, value)) => {
        self.offset += value_size;
        self.yielded_elements += 1;
        Some(Ok(((key_size, key), (value_size, value))))
      }
      Err(e) => {
        self.has_error = true;
        Some(Err(e))
      }
    }
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
  K: Decode1<'de, KW, RB, B, Groto> + 'de,
  V: Decode1<'de, VW, RB, B, Groto> + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
{
}

