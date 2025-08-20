use crate::{
  convert::{Extracted, Innermost},
  encode::EquivalentEncode,
  flavors::{Flatten, groto::EncodeError},
  reflection::{Reflectable, SchemaType, SchemaTypeReflection},
  state::State,
};

use super::*;

impl<'a, T, N, W, const TAG: u32> Encode<Flatten<Borrowed<'a, Repeated<W, TAG>>, W>, Groto>
  for [&N]
where
  W: WireFormat<Groto>,
  N: State<Extracted<Innermost>, Output = T> + Encode<Repeated<W, TAG>, Groto> + ?Sized,
  T: Encode<W, Groto> + ?Sized,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
{
  fn encode_raw<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
  ) -> Result<usize, EncodeError>
  where
    WB: ChunkMut,
  {
    check_list_type::<N>()?;

    repeated_encode::<_, Repeated<W, TAG>, _, TAG>(
      buf.buffer_mut(),
      || self.iter(),
      |item| item.encoded_len(context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    self.iter().map(|n| n.encoded_len(context)).sum::<usize>()
  }

  fn encode<B>(&self, context: &Context, buf: impl Into<ChunkWriter<B>>) -> Result<usize, Error>
  where
    B: ChunkMut,
  {
    <Self as Encode<Flatten<Borrowed<'_, Repeated<W, TAG>>, W>, Groto>>::encode_raw(
      self, context, buf,
    )
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Flatten<Borrowed<'_, Repeated<W, TAG>>, W>, Groto>>::encoded_raw_len(
      self, context,
    )
  }
}

impl<T, N, W, const TAG: u32> Encode<Flatten<Repeated<W, TAG>, W>, Groto> for [N]
where
  W: WireFormat<Groto>,
  N: State<Extracted<Innermost>, Output = T> + Encode<Repeated<W, TAG>, Groto>,
  T: Encode<W, Groto> + Sized,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
{
  fn encode_raw<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
  ) -> Result<usize, EncodeError>
  where
    WB: ChunkMut,
  {
    check_list_type::<N>()?;

    repeated_encode::<_, Repeated<W, TAG>, _, TAG>(
      buf.buffer_mut(),
      || self.iter(),
      |item| item.encoded_len(context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    self.iter().map(|n| n.encoded_len(context)).sum::<usize>()
  }

  fn encode<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
  ) -> Result<usize, EncodeError>
  where
    WB: ChunkMut,
  {
    <Self as Encode<Flatten<Repeated<W, TAG>, W>, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Flatten<Repeated<W, TAG>, W>, Groto>>::encoded_raw_len(self, context)
  }
}

impl<T, N, W, const TAG: u32> PartialEncode<Flatten<Repeated<W, TAG>, W>, Groto> for [N]
where
  W: WireFormat<Groto>,
  N: State<Extracted<Innermost>, Output = T> + PartialEncode<W, Groto, Selector = T::Selector>,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
  T: PartialEncode<W, Groto> + Sized,
{
  fn partial_encode_raw<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: ChunkMut,
  {
    if selector.is_empty() {
      return Ok(0);
    }

    check_list_type::<N>()?;

    repeated_encode::<_, Repeated<W, TAG>, _, TAG>(
      buf.buffer_mut(),
      || self.iter(),
      |item| item.partial_encoded_len(context, selector),
      |item, buf| item.partial_encode(context, buf, selector),
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    self
      .iter()
      .map(|n| n.partial_encoded_len(context, selector))
      .sum::<usize>()
  }

  fn partial_encode<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: ChunkMut,
  {
    <Self as PartialEncode<Flatten<Repeated<W, TAG>, W>, Groto>>::partial_encode_raw(
      self, context, buf, selector,
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <Self as PartialEncode<Flatten<Repeated<W, TAG>, W>, Groto>>::partial_encoded_raw_len(
      self, context, selector,
    )
  }
}

impl<'a, T, N, W, const TAG: u32> PartialEncode<Flatten<Borrowed<'a, Repeated<W, TAG>>, W>, Groto>
  for [&N]
where
  W: WireFormat<Groto>,
  N: State<Extracted<Innermost>, Output = T>
    + PartialEncode<Repeated<W, TAG>, Groto, Selector = T::Selector>,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
  T: PartialEncode<W, Groto> + Sized,
{
  fn partial_encode_raw<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: ChunkMut,
  {
    if selector.is_empty() {
      return Ok(0);
    }

    check_list_type::<N>()?;

    repeated_encode::<_, Repeated<W, TAG>, _, TAG>(
      buf.buffer_mut(),
      || self.iter(),
      |item| item.partial_encoded_len(context, selector),
      |item, buf| item.partial_encode(context, buf, selector),
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    self
      .iter()
      .map(|n| n.partial_encoded_len(context, selector))
      .sum::<usize>()
  }

  fn partial_encode<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: ChunkMut,
  {
    <Self as PartialEncode<Flatten<Borrowed<'_, Repeated<W, TAG>>, W>, Groto>>::partial_encode_raw(
      self, context, buf, selector,
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <Self as PartialEncode<Flatten<Borrowed<'_, Repeated<W, TAG>>, W>, Groto>>::partial_encoded_raw_len(self, context, selector)
  }
}

fn repeated_encode<'a, B, K: 'a, KW, I, const TAG: u32>(
  mut buf: ChunkWriter<B>,
  iter: impl Fn() -> I,
  encoded_len: impl Fn(&K) -> usize,
  mut encode: impl FnMut(&K, ChunkWriter<&mut B>) -> Result<usize, EncodeError>,
) -> Result<usize, EncodeError>
where
  I: Iterator<Item = &'a K>,
  KW: WireFormat<Groto>,
  B: ChunkMut,
{
  let encoded_len = iter().map(encoded_len).sum::<usize>();
  let buf_len = buf.remaining_mut();
  if encoded_len > buf_len {
    return Err(EncodeError::buffer_too_small(encoded_len, buf_len));
  }

  let mut offset = 0;
  for k in iter() {
    offset +=
      encode(k, buf.as_mut()).map_err(|e| e.propagate_buffer_info(|| encoded_len, || buf_len))?;
  }

  Ok(offset)
}

unsafe impl<T, W, const TAG: u32>
  EquivalentEncode<[&[T]], Flatten<Borrowed<'_, Repeated<W, TAG>>, W>, Groto> for [T]
where
  W: WireFormat<Groto>,
  [T]: State<Extracted<Innermost>, Output = T> + Encode<Repeated<W, TAG>, Groto>,
  T: Encode<W, Groto>,
  SchemaTypeReflection<[T]>: Reflectable<[T], Reflection = SchemaType>,
{
  type WireFormat = Repeated<W, TAG>;
}
