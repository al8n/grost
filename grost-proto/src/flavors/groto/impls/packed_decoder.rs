use core::marker::PhantomData;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  convert::{Flattened, Partial, PartialRef},
  decode::Decode,
  encode::Encode,
  flavors::{
    Groto, WireFormat,
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
  /// the length of the length prefix
  data_offset: usize,
  /// the current offset
  offset: usize,
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
      offset: self.offset,
      ctx: self.ctx,
      _t: PhantomData,
      _w: PhantomData,
      _ub: PhantomData,
    }
  }
}

impl<'a, T: ?Sized, B: Copy, UB: ?Sized, W: ?Sized> Copy for PackedDecoder<'a, T, B, UB, W> {}

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

impl<'a, T, B, UB, W> PackedDecoder<'a, T, B, UB, W>
where
  T: ?Sized,
  UB: ?Sized,
  W: ?Sized,
{
  /// Returns the current offset to the source byte slice
  #[inline]
  pub const fn offset(&self) -> usize {
    self.offset
  }

  pub(super) fn new(ctx: &'a Context, src: B, data_offset: usize) -> Self {
    Self {
      src,
      data_offset,
      offset: data_offset,
      ctx,
      _t: PhantomData,
      _w: PhantomData,
      _ub: PhantomData,
    }
  }
}

impl<'a, RB, B, W, T> Iterator for PackedDecoder<'a, T, RB, B, W>
where
  W: WireFormat<Groto> + 'a,
  T: State<PartialRef<'a, RB, B, W, Groto>> + Decode<'a, T::Output, W, RB, B, Groto> + 'a,
  T::Output: Sized,
  B: UnknownBuffer<RB, Groto> + 'a,
  RB: ReadBuf + 'a,
{
  type Item = Result<(usize, T::Output), Error>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let src_len = self.src.len();
    if self.offset >= src_len {
      return None;
    }

    Some(
      T::decode(self.ctx, self.src.slice(self.offset..)).inspect(|(read, _)| {
        self.offset += read;
      }),
    )
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

impl<'a, T, RB, B, W> Encode<W, Groto> for PackedDecoder<'a, T, RB, B, W>
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

impl<'a, T, RB, B, W, PW> Decode<'a, Self, PW, RB, B, Groto> for PackedDecoder<'a, T, RB, B, W>
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

    Ok((
      total,
      Self {
        src,
        data_offset: len,
        offset: len,
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

impl<'a, T, B, UB, W, S> State<Flattened<S>> for PackedDecoder<'a, T, B, UB, W>
where
  T: ?Sized,
  W: ?Sized,
  UB: ?Sized,
  S: ?Sized,
{
  type Output = Self;
}
