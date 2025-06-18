use core::marker::PhantomData;

use crate::{
  Decoded, State,
  buffer::Buffer,
  decode::Decode,
  encode::Encode,
  flavors::{
    Network, WireFormat,
    network::{Context, Error, LengthDelimited, Unknown},
  },
  selection::Selectable,
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
pub struct PackedDecoder<'a, T: ?Sized, UB: ?Sized, W: ?Sized> {
  /// the source buffer
  src: &'a [u8],
  /// the length of the length prefix
  data_offset: usize,
  /// the current offset
  offset: usize,
  ctx: &'a Context,
  _t: PhantomData<T>,
  _w: PhantomData<W>,
  _ub: PhantomData<UB>,
}

impl<'a, T: ?Sized, UB: ?Sized, W: ?Sized> Clone for PackedDecoder<'a, T, UB, W> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<'a, T: ?Sized, UB: ?Sized, W: ?Sized> Copy for PackedDecoder<'a, T, UB, W> {}

impl<'a, UB> PackedDecoder<'a, u8, UB, LengthDelimited>
where
  UB: ?Sized,
{
  /// Returns a slice to the fully decoded byte data.
  ///
  /// This method is specifically optimized for the case where `L` implements `Deref<Target = [u8]>`.
  /// Since the target type is a byte slice, no further decoding is needed for individual elements.
  /// The decoder can immediately provide the complete decoded byte slice.
  #[inline]
  pub const fn as_slice(&self) -> &'a [u8] {
    let src = self.src;
    if src.is_empty() {
      return src;
    }

    let src_len = src.len();
    if src_len <= self.data_offset {
      return &[];
    }

    src.split_at(self.data_offset).1
  }
}

impl<'a, UB> core::ops::Deref for PackedDecoder<'a, u8, UB, LengthDelimited>
where
  UB: ?Sized,
{
  type Target = [u8];

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.as_slice()
  }
}

impl<'a, UB> AsRef<[u8]> for PackedDecoder<'a, u8, UB, LengthDelimited>
where
  UB: ?Sized,
{
  #[inline]
  fn as_ref(&self) -> &[u8] {
    self
  }
}

impl<'a, T, UB, W> PackedDecoder<'a, T, UB, W>
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
}

impl<'a, UB, W, T> Iterator for PackedDecoder<'a, T, UB, W>
where
  W: WireFormat<Network> + 'a,
  T: State<Decoded<'a, Network, W, UB>, Input = &'a [u8]>
    + Decode<'a, Network, W, T::Output, UB>
    + 'a,
  T::Output: Sized,
  UB: Buffer<Unknown<&'a [u8]>> + 'a,
{
  type Item = Result<(usize, T::Output), Error>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let src_len = self.src.len();
    if self.offset >= src_len {
      return None;
    }

    Some(
      T::decode(self.ctx, &self.src[self.offset..]).inspect(|(read, _)| {
        self.offset += read;
      }),
    )
  }
}

impl<'a, T, UB, W> Selectable<Network> for PackedDecoder<'a, T, UB, W>
where
  T: ?Sized + Selectable<Network>,
  W: WireFormat<Network> + 'a,
  UB: ?Sized,
{
  type Selector = T::Selector;
}

impl<'a, T, UB, W> Encode<Network, W> for PackedDecoder<'a, T, UB, W>
where
  T: ?Sized,
  W: WireFormat<Network> + 'a,
  UB: ?Sized,
{
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let src = self.src;

    let buf_len = buf.len();
    let src_len = src.len();
    if buf_len < src_len {
      return Err(Error::insufficient_buffer(src_len, buf_len));
    }

    buf[..src_len].copy_from_slice(src);
    Ok(src_len)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    self.src.len()
  }
}

impl<'a, T, UB, W> Decode<'a, Network, W, Self, UB> for PackedDecoder<'a, T, UB, W>
where
  W: WireFormat<Network> + 'a,
{
  fn decode<B>(
    ctx: &'a <Network as crate::flavors::Flavor>::Context,
    src: B,
  ) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    B: crate::buffer::ReadBuf<'a>,
    UB: Buffer<<Network as crate::flavors::Flavor>::Unknown<B>> + 'a,
  {
    let src = src.as_bytes();
    let buf_len = src.len();
    if buf_len == 0 {
      return Err(Error::buffer_underflow());
    }

    let (len, data_len) = varing::decode_u32_varint(src)?;
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
