use core::marker::PhantomData;

use crate::{
  Decoded, State,
  buffer::Buffer,
  decode::Decode,
  encode::Encode,
  flavors::{
    Network, WireFormat,
    network::{Context, DecodeError, EncodeError, LengthDelimited, Unknown, WireType},
  },
  reflection::{Reflectable, Type, TypeReflection},
};

/// A lazy decoder for repeated types (e.g. `Vec<T>`, `[T]`, `HashSet<T>`, `HashMap<K, V>` and etc.) of data that
/// iterates through the underlying buffer and decode elements on demand.
///
/// `RepeatedDecoder` provides functionality to decode list-like structures from binary data.
/// It operates lazily, decoding elements only when requested through iteration.
///
/// # Special Case
///
/// When `L` implements `Deref<Target = [u8]>`, the decoder considers the decoding process
/// complete since the raw bytes are the final representation. In this case, `as_slice()`
/// returns the entire decoded byte slice.
///
/// For other types, the decoder will yield decoded elements one by one through iteration
/// until it reaches the end of the source data.
pub struct RepeatedDecoder<'a, L: ?Sized, UB: ?Sized, W: ?Sized> {
  repr: Repr<'a>,
  ctx: &'a Context,
  _t: PhantomData<L>,
  _w: PhantomData<W>,
  _ub: PhantomData<UB>,
}

impl<'a, L, UB> RepeatedDecoder<'a, L, UB, LengthDelimited>
where
  L: core::ops::Deref<Target = [u8]>,
  UB: ?Sized,
{
  /// Returns a slice to the fully decoded byte data.
  ///
  /// This method is specifically optimized for the case where `L` implements `Deref<Target = [u8]>`.
  /// Since the target type is a byte slice, no further decoding is needed for individual elements.
  /// The decoder can immediately provide the complete decoded byte slice.
  #[inline]
  pub const fn as_slice(&self) -> &'a [u8] {
    match self.repr {
      Repr::LengthDelimited {
        src, data_offset, ..
      } => {
        if src.is_empty() {
          return src;
        }

        let src_len = src.len();
        if src_len <= data_offset {
          return &[];
        }

        src.split_at(data_offset).1
      }
      Repr::Data { src, .. } => src,
    }
  }
}

impl<'a, L, UB> core::ops::Deref for RepeatedDecoder<'a, L, UB, LengthDelimited>
where
  L: core::ops::Deref<Target = [u8]>,
  UB: ?Sized,
{
  type Target = [u8];

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.as_slice()
  }
}

impl<'a, L, UB> AsRef<[u8]> for RepeatedDecoder<'a, L, UB, LengthDelimited>
where
  L: core::ops::Deref<Target = [u8]>,
  UB: ?Sized,
{
  #[inline]
  fn as_ref(&self) -> &[u8] {
    self
  }
}

impl<'a, L, UB, W> RepeatedDecoder<'a, L, UB, W>
where
  L: ?Sized,
  UB: ?Sized,
  W: ?Sized,
{
  // pub fn new(src: &'a [u8], offset: usize, len: usize) -> Self {
  //   Self {
  //     src,
  //     offset,
  //     len,
  //     _wf: PhantomData,
  //     _t: PhantomData,
  //   }
  // }

  /// Returns the current offset to the source byte slice
  #[inline]
  pub const fn offset(&self) -> usize {
    match self.repr {
      Repr::LengthDelimited { offset, .. } => offset,
      Repr::Data { offset, .. } => offset,
    }
  }
}

impl<'a, L, UB, W, T> Iterator for RepeatedDecoder<'a, L, UB, W>
where
  L: core::ops::Deref<Target = [T]>,
  W: sealed::Sealed,
  T: State<Decoded<'a, Network, W>, Input = &'a [u8]> + Decode<'a, Network, W, T::Output> + 'a,
  T::Output: Sized,
  TypeReflection<T>: Reflectable<T, Reflection = Type>,
  UB: Buffer<Unknown<&'a [u8]>> + 'a,
{
  type Item = Result<(usize, T::Output), DecodeError>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    match W::WIRE_TYPE {
      WireType::Zst => None,
      WireType::LengthDelimited => self.repr.decode_length_delimited::<T, W, UB>(self.ctx),
      _ => self.repr.decode::<T, W, UB>(self.ctx),
    }
  }
}

impl<'a, L, UB, W> Encode<Network, W> for RepeatedDecoder<'a, L, UB, W>
where
  L: ?Sized,
  W: sealed::Sealed,
  UB: ?Sized,
{
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    match self.repr {
      Repr::LengthDelimited {
        src, data_offset, ..
      } => {
        let buf_len = buf.len();
        let src_len = src.len();
        let len = src_len.saturating_sub(data_offset);
        if buf_len < len {
          return Err(EncodeError::insufficient_buffer(len, buf_len));
        }

        if len == 0 {
          return Ok(0);
        }

        buf[..len].copy_from_slice(&src[data_offset..]);
        Ok(len)
      }
      Repr::Data { src, .. } => {
        let buf_len = buf.len();
        let src_len = src.len();
        if buf_len < src_len {
          return Err(EncodeError::insufficient_buffer(src_len, buf_len));
        }

        buf[..src_len].copy_from_slice(src);
        Ok(src_len)
      }
    }
  }

  fn encoded_len(&self, _: &Context) -> usize {
    self.repr.encoded_len()
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    match self.repr {
      Repr::LengthDelimited { src, .. } => {
        let buf_len = buf.len();
        let src_len = src.len();
        if src_len > buf_len {
          return Err(EncodeError::insufficient_buffer(src_len, buf_len));
        }

        buf[..src_len].copy_from_slice(src);
        Ok(src_len)
      }
      Repr::Data { src, .. } => {
        let buf_len = buf.len();
        let src_len = src.len();
        let len_size = varing::encode_u32_varint_to(src_len as u32, buf).map_err(|e| {
          EncodeError::from_varint_error(e)
            .update(self.encoded_length_delimited_len(context), buf_len)
        })?;
        let total = src_len + len_size;
        if total > buf_len {
          return Err(EncodeError::insufficient_buffer(
            src_len + len_size,
            buf_len,
          ));
        }

        if len_size >= buf_len {
          return Err(EncodeError::insufficient_buffer(
            src_len + len_size,
            buf_len,
          ));
        }
        buf[len_size..total].copy_from_slice(src);
        Ok(total)
      }
    }
  }

  fn encoded_length_delimited_len(
    &self,
    _: &<Network as crate::flavors::Flavor>::Context,
  ) -> usize {
    self.repr.encoded_length_delimited_len()
  }
}

/// Internal representation of list data.
///
/// This enum represents the two possible storage formats for the list:
/// - `LengthDelimited`: The data is prefixed with its length (common in network protocols)
/// - `Data`: The data has a known capacity limit (common in fixed-size buffers)
enum Repr<'a> {
  LengthDelimited {
    /// the source buffer
    src: &'a [u8],
    /// the length of the length prefix
    data_offset: usize,
    /// the current offset
    offset: usize,
  },
  Data {
    /// the source buffer
    src: &'a [u8],
    /// the current offset
    offset: usize,
  },
}

impl<'a> Repr<'a> {
  #[inline]
  const fn encoded_len(&self) -> usize {
    match self {
      Repr::LengthDelimited {
        data_offset, src, ..
      } => src.len().saturating_sub(*data_offset),
      Repr::Data { src, .. } => src.len(),
    }
  }

  #[inline]
  const fn encoded_length_delimited_len(&self) -> usize {
    match self {
      Repr::LengthDelimited { src, .. } => src.len(),
      Repr::Data { src, .. } => {
        let len = src.len();
        len + varing::encoded_u32_varint_len(len as u32)
      }
    }
  }

  fn decode<T, W, UB>(&mut self, ctx: &Context) -> Option<Result<(usize, T::Output), DecodeError>>
  where
    W: sealed::Sealed,
    T: State<Decoded<'a, Network, W>, Input = &'a [u8]> + Decode<'a, Network, W, T::Output> + 'a,
    T::Output: Sized,
    TypeReflection<T>: Reflectable<T, Reflection = Type>,
    UB: Buffer<Unknown<&'a [u8]>> + 'a,
  {
    match self {
      Repr::LengthDelimited { src, offset, .. } | Repr::Data { src, offset } => {
        let src_len = src.len();
        if *offset >= src_len {
          return None;
        }

        Some(T::decode::<UB>(ctx, &src[*offset..]).inspect(|(read, _)| {
          *offset += read;
        }))
      }
    }
  }

  fn decode_length_delimited<T, W, UB>(
    &mut self,
    ctx: &Context,
  ) -> Option<Result<(usize, T::Output), DecodeError>>
  where
    W: sealed::Sealed,
    T: State<Decoded<'a, Network, W>, Input = &'a [u8]> + Decode<'a, Network, W, T::Output> + 'a,
    T::Output: Sized,
    TypeReflection<T>: Reflectable<T, Reflection = Type>,
    UB: Buffer<Unknown<&'a [u8]>> + 'a,
  {
    match self {
      Repr::LengthDelimited { src, offset, .. } | Repr::Data { src, offset } => {
        let src_len = src.len();
        if *offset >= src_len {
          return None;
        }

        Some(
          T::decode_length_delimited::<UB>(ctx, &src[*offset..]).inspect(|(read, _)| {
            *offset += read;
          }),
        )
      }
    }
  }
}

mod sealed {
  pub trait Sealed: super::WireFormat<super::Network> {}

  impl Sealed for super::LengthDelimited {}
}
