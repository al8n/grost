use core::marker::PhantomData;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  convert::{Flattened, Partial, PartialRef, Ref},
  decode::Decode1,
  encode::Encode,
  flavors::{
    Groto, WireFormat,
    groto::{Context, Error},
  },
  selection::Selectable,
  state::State,
};

/// A lazy decoder for repeated types (e.g. `HashSet<T>`, `BtreeSet` and etc.) of data that
/// iterates through the underlying buffer and decode elements on demand.
///
/// `PackedSetDecoder` provides functionality to decode list-like structures from binary data.
/// It operates lazily, decoding elements only when requested through iteration.
pub struct PackedSetDecoder<'a, T: ?Sized, B, UB: ?Sized, W: ?Sized> {
  /// the source buffer
  src: B,
  expected_elements: usize,
  yielded_elements: usize,
  has_error: bool,
  /// the length of the length prefix
  data_offset: usize,
  /// the current offset
  offset: usize,
  ctx: &'a Context,
  _t: PhantomData<T>,
  _w: PhantomData<W>,
  _ub: PhantomData<UB>,
}

impl<'a, T: ?Sized, B: Clone, UB: ?Sized, W: ?Sized> Clone for PackedSetDecoder<'a, T, B, UB, W> {
  fn clone(&self) -> Self {
    Self {
      src: self.src.clone(),
      data_offset: self.data_offset,
      yielded_elements: self.yielded_elements,
      expected_elements: self.expected_elements,
      offset: self.offset,
      has_error: self.has_error,
      ctx: self.ctx,
      _t: PhantomData,
      _w: PhantomData,
      _ub: PhantomData,
    }
  }
}

impl<'a, T: ?Sized, B: Copy, UB: ?Sized, W: ?Sized> Copy for PackedSetDecoder<'a, T, B, UB, W> {}

impl<'a, T, B, UB, W> PackedSetDecoder<'a, T, B, UB, W>
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

impl<'a, RB, B, W, T> Iterator for PackedSetDecoder<'a, T, RB, B, W>
where
  W: WireFormat<Groto> + 'a,
  T: Decode1<'a, W, RB, B, Groto> + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
  RB: ReadBuf + 'a,
{
  type Item = Result<(usize, T), Error>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let src_len = self.src.len();
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
      T::decode(self.ctx, self.src.slice(self.offset..))
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
    let remaining = self
      .expected_elements
      .saturating_sub(self.expected_elements - self.yielded_elements);
    (0, Some(remaining))
  }
}

impl<'a, T, B, UB, W> Selectable<Groto> for PackedSetDecoder<'a, T, B, UB, W>
where
  T: ?Sized + Selectable<Groto>,
  W: WireFormat<Groto> + 'a,
  UB: ?Sized,
{
  type Selector = T::Selector;
}

impl<'a, T, RB, B, W> Encode<W, Groto> for PackedSetDecoder<'a, T, RB, B, W>
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

impl<'a, T, RB, B, W, PW> Decode1<'a, PW, RB, B, Groto> for PackedSetDecoder<'a, T, RB, B, W>
where
  W: WireFormat<Groto> + 'a,
  PW: WireFormat<Groto> + 'a,
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
        offset: len + num_elements_size,
        yielded_elements: 0,
        has_error: false,
        ctx,
        _t: PhantomData,
        _w: PhantomData,
        _ub: PhantomData,
      },
    ))
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

impl<'a, T, B, UB, W> State<PartialRef<'a, B, UB, W, Groto>> for PackedSetDecoder<'a, T, B, UB, W>
where
  T: ?Sized,
  W: ?Sized,
  UB: ?Sized,
{
  type Output = Self;
}

impl<'a, T, B, UB, W> State<Ref<'a, B, UB, W, Groto>> for PackedSetDecoder<'a, T, B, UB, W>
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
