use core::{iter::FusedIterator, marker::PhantomData};

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  convert::{Flattened, Partial, PartialRef, Ref},
  decode::Decode1,
  encode::Encode,
  flavors::{
    Groto, Packed, WireFormat,
    groto::{Context, Error, Fixed8},
  },
  selection::Selectable,
  state::State,
};

/// A lazy decoder for repeated types (e.g. `Vec<T>`, `[T]`, `HashSet<T>`, `HashMap<K, V>` and etc.) of data that
/// iterates through the underlying buffer and decode elements on demand.
///
/// `PackedDecoder` provides functionality to decode list-like structures from binary data.
/// It operates lazily, decoding elements only when requested through iteration.
///
/// # Special Case
///
/// When `T` is `u8`, the decoder considers the decoding process
/// complete since the raw bytes are the final representation. In this case, `as_slice()`
/// returns the entire decoded byte slice.
///
/// For other types, the decoder will yield decoded elements one by one through iteration
/// until it reaches the end of the source data.
pub struct PackedDecoder<'a, T: ?Sized, B, UB: ?Sized, W: ?Sized> {
  /// the source buffer
  src: B,
  expected_elements: usize,
  num_elements_size: usize,
  /// the length of the length prefix
  data_offset: usize,
  ctx: &'a Context,
  _t: PhantomData<T>,
  _w: PhantomData<W>,
  _ub: PhantomData<UB>,
}

impl<'a, T: ?Sized, B: Clone, UB: ?Sized, W: ?Sized> Clone for PackedDecoder<'a, T, B, UB, W> {
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

impl<'a, T: ?Sized, B: Copy, UB: ?Sized, W: ?Sized> Copy for PackedDecoder<'a, T, B, UB, W> {}

impl<'de, T, B, UB, W> PackedDecoder<'de, T, B, UB, W>
where
  T: ?Sized,
  UB: ?Sized,
  W: ?Sized,
{
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

impl<'a, RB, B> PackedDecoder<'a, u8, RB, B, Fixed8>
where
  B: ?Sized,
  RB: ReadBuf,
{
  /// Returns a slice to the fully decoded byte data.
  ///
  /// This method is specifically optimized for the case where `L` implements `Deref<Target = [u8]>`.
  /// Since the target type is a byte slice, no further decoding is needed for individual elements.
  /// The decoder can immediately provide the complete decoded byte slice.
  #[inline]
  pub fn as_slice(&self) -> &[u8] {
    let src = &self.src;
    if src.is_empty() {
      return src.as_bytes();
    }

    let src_len = src.len();
    if src_len <= self.data_offset {
      return &[];
    }

    src.as_bytes().split_at(self.data_offset).1
  }
}

impl<'a, RB, B> core::ops::Deref for PackedDecoder<'a, u8, RB, B, Fixed8>
where
  B: ?Sized,
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
  B: ?Sized,
  RB: ReadBuf,
{
  #[inline]
  fn as_ref(&self) -> &[u8] {
    self
  }
}

impl<'a, T, B, UB, W> Selectable<Groto> for PackedDecoder<'a, T, B, UB, W>
where
  T: ?Sized + Selectable<Groto>,
  W: WireFormat<Groto> + 'a,
  UB: ?Sized,
{
  type Selector = T::Selector;
}

impl<'a, T, RB, B, W> Encode<Packed<W>, Groto> for PackedDecoder<'a, T, RB, B, W>
where
  T: ?Sized,
  Packed<W>: WireFormat<Groto> + 'a,
  B: ?Sized,
  RB: ReadBuf,
{
  fn encode_raw(&self, ctx: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let buf_len = buf.len();
    let src_len = self.encoded_raw_len(ctx);
    if buf_len < src_len {
      return Err(Error::insufficient_buffer(src_len, buf_len));
    }

    buf[..src_len].copy_from_slice(&self.src.as_bytes()[self.data_offset..]);
    Ok(src_len)
  }

  fn encoded_raw_len(&self, _: &Context) -> usize {
    self.src.len() - self.data_offset
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

impl<'a, T, B, W, RB> Decode1<'a, Packed<W>, RB, B, Groto> for PackedDecoder<'a, T, RB, B, W>
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

    let (len, data_len) = varing::decode_u32_varint(buf)?;
    let total = len + data_len as usize;
    if total > buf_len {
      return Err(Error::buffer_underflow());
    }

    let (num_elements_size, num_elements) = varing::decode_u32_varint(&buf[len..])?;
    let num_elements = num_elements as usize;

    Ok((
      total,
      Self {
        src: src.slice(..total),
        data_offset: len,
        expected_elements: num_elements,
        num_elements_size: len + num_elements_size,
        ctx,
        _t: PhantomData,
        _w: PhantomData,
        _ub: PhantomData,
      },
    ))
  }
}

impl<'a, T, B, UB, W> State<Partial<Groto>> for PackedDecoder<'a, T, B, UB, W>
where
  T: ?Sized,
  W: ?Sized,
  UB: ?Sized,
{
  type Output = Self;
}

impl<'a, T, B, UB, W> State<PartialRef<'a, B, UB, Packed<W>, Groto>>
  for PackedDecoder<'a, T, B, UB, W>
where
  T: ?Sized,
  W: ?Sized,
  UB: ?Sized,
{
  type Output = Self;
}

impl<'a, T, B, UB, W> State<Ref<'a, B, UB, Packed<W>, Groto>> for PackedDecoder<'a, T, B, UB, W>
where
  T: ?Sized,
  W: ?Sized,
  UB: ?Sized,
{
  type Output = Self;
}

impl<'a, T, B, UB, W, S> State<Flattened<S>> for PackedDecoder<'a, T, B, UB, W>
where
  T: ?Sized,
  W: ?Sized,
  UB: ?Sized,
  S: ?Sized,
{
  type Output = Self;
}

pub struct PackedDecoderIter<'a, 'de: 'a, T, RB, UB, W>
where
  T: ?Sized,
  UB: ?Sized,
  W: ?Sized,
{
  decoder: &'a PackedDecoder<'de, T, RB, UB, W>,
  offset: usize,
  yielded_elements: usize,
  expected_elements: usize,
  has_error: bool,
}

impl<'a, 'de: 'a, T, B, UB, W> PackedDecoderIter<'a, 'de, T, B, UB, W>
where
  T: ?Sized,
  UB: ?Sized,
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

impl<'a, 'de: 'a, RB, B, W, T> Iterator for PackedDecoderIter<'a, 'de, T, RB, B, W>
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
    if self.offset >= src_len {
      return None;
    }

    if self.yielded_elements >= self.expected_elements {
      return None;
    }

    if self.has_error {
      return None;
    }

    Some(
      T::decode(self.decoder.ctx, self.decoder.src.slice(self.offset..))
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
    (0, self.remaining_expected())
  }
}

impl<'a, 'de: 'a, RB, B, W, T> FusedIterator for PackedDecoderIter<'a, 'de, T, RB, B, W>
where
  W: WireFormat<Groto> + 'de,
  T: Decode1<'de, W, RB, B, Groto> + 'de,
  B: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
{
}
