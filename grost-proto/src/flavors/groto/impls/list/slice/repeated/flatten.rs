use crate::{
  convert::{Extracted, Innermost},
  encode::{EquivalentEncode, Length},
  flavors::Flatten,
  reflection::{Reflectable, SchemaType, SchemaTypeReflection},
  state::State,
};

use super::*;

impl<'a, T, N, W, const TAG: u32> Encode<Flatten<Borrowed<'a, Repeated<W, TAG>>, W>, Groto>
  for [&N]
where
  W: WireFormat<Groto>,
  N: State<Extracted<Innermost>, Output = T> + Length + Encode<Repeated<W, TAG>, Groto> + ?Sized,
  T: Encode<W, Groto> + ?Sized,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    check_list_type::<N>()?;

    repeated_encode::<_, Repeated<W, TAG>, _, TAG>(
      buf,
      || self.iter(),
      |item| item.encoded_len(context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    self.iter().map(|n| n.encoded_len(context)).sum::<usize>()
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
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
  N: State<Extracted<Innermost>, Output = T> + Length + Encode<Repeated<W, TAG>, Groto>,
  T: Encode<W, Groto> + Sized,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    check_list_type::<N>()?;

    repeated_encode::<_, Repeated<W, TAG>, _, TAG>(
      buf,
      || self.iter(),
      |item| item.encoded_len(context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    self.iter().map(|n| n.encoded_len(context)).sum::<usize>()
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Flatten<Repeated<W, TAG>, W>, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Flatten<Repeated<W, TAG>, W>, Groto>>::encoded_raw_len(self, context)
  }
}

impl<T, N, W, const TAG: u32> PartialEncode<Flatten<Repeated<W, TAG>, W>, Groto> for [N]
where
  W: WireFormat<Groto>,
  N: State<Extracted<Innermost>, Output = T>
    + Length
    + PartialEncode<W, Groto, Selector = T::Selector>,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
  T: PartialEncode<W, Groto> + Sized,
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

    check_list_type::<N>()?;

    repeated_encode::<_, Repeated<W, TAG>, _, TAG>(
      buf,
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

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
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
    + Length
    + PartialEncode<Repeated<W, TAG>, Groto, Selector = T::Selector>,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
  T: PartialEncode<W, Groto> + Sized,
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

    check_list_type::<N>()?;

    repeated_encode::<_, Repeated<W, TAG>, _, TAG>(
      buf,
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

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    <Self as PartialEncode<Flatten<Borrowed<'_, Repeated<W, TAG>>, W>, Groto>>::partial_encode_raw(
      self, context, buf, selector,
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <Self as PartialEncode<Flatten<Borrowed<'_, Repeated<W, TAG>>, W>, Groto>>::partial_encoded_raw_len(self, context, selector)
  }
}

fn repeated_encode<'a, K: 'a, KW, I, const TAG: u32>(
  buf: &'a mut [u8],
  iter: impl Fn() -> I,
  encoded_len: impl Fn(&K) -> usize,
  mut encode: impl FnMut(&K, &mut [u8]) -> Result<usize, Error>,
) -> Result<usize, Error>
where
  I: Iterator<Item = &'a K>,
  KW: WireFormat<Groto>,
{
  let encoded_len = iter().map(encoded_len).sum::<usize>();
  let buf_len = buf.len();
  if encoded_len > buf_len {
    return Err(Error::insufficient_buffer(encoded_len, buf_len));
  }

  let mut offset = 0;
  for k in iter() {
    if offset >= buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    offset += encode(k, &mut buf[offset..]).map_err(|e| e.update(encoded_len, buf_len))?;
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

  type Flavor = Groto;
}
