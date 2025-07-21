use super::*;

impl<'a, T, N, W> Encode<Flatten<Borrowed<'a, Packed<W>>, W>, Groto> for [&N]
where
  W: WireFormat<Groto>,
  N: State<Flattened<Innermost>, Output = T> + Length + Encode<Packed<W>, Groto> + ?Sized,
  T: Encode<W, Groto> + ?Sized,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    check_list_type::<N>()?;

    packed_encode_raw::<_, _, _, _>(
      buf,
      self.iter(),
      || {
        <Self as Encode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::encoded_raw_len(self, context)
      },
      |item, buf| item.encode_raw(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    self
      .iter()
      .map(|n| n.encoded_raw_len(context))
      .sum::<usize>()
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let mut num_elems = 0;
    let elems_bytes = self
      .iter()
      .map(|n| {
        num_elems += n.len();
        n.encoded_raw_len(context)
      })
      .sum::<usize>();

    packed_encode::<_, _, _, _>(
      buf,
      num_elems,
      self.iter(),
      || elems_bytes,
      |item, buf| item.encode_raw(context, buf),
    )
  }

  fn encoded_len(&self, context: &Context) -> usize {
    let mut num_elems = 0;
    let num_bytes = self
      .iter()
      .map(|n| {
        num_elems += n.len();
        n.encoded_raw_len(context)
      })
      .sum::<usize>();
    let total_bytes = encoded_u32_varint_len(num_elems as u32) + num_bytes;
    encoded_u32_varint_len(total_bytes as u32) + total_bytes
  }
}

unsafe impl<T, W> EquivalentEncode<[&[T]], Flatten<Borrowed<'_, Packed<W>>, W>, Groto> for [T]
where
  W: WireFormat<Groto>,
  [T]: State<Flattened<Innermost>, Output = T> + Encode<Packed<W>, Groto>,
  T: Encode<W, Groto>,
  SchemaTypeReflection<[T]>: Reflectable<[T], Reflection = SchemaType>,
{
  type WireFormat = Packed<W>;

  type Flavor = Groto;
}

impl<T, N, W> Encode<Flatten<Packed<W>, W>, Groto> for [N]
where
  W: WireFormat<Groto>,
  N: State<Flattened<Innermost>, Output = T> + Length + Encode<Packed<W>, Groto>,
  T: Encode<W, Groto> + Sized,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    check_list_type::<N>()?;

    packed_encode_raw::<_, _, _, _>(
      buf,
      self.iter(),
      || <Self as Encode<Flatten<Packed<W>, W>, Groto>>::encoded_raw_len(self, context),
      |item, buf| item.encode_raw(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    self
      .iter()
      .map(|n| n.encoded_raw_len(context))
      .sum::<usize>()
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let mut num_elems = 0;
    let elems_bytes = self
      .iter()
      .map(|n| {
        num_elems += n.len();
        n.encoded_raw_len(context)
      })
      .sum::<usize>();

    packed_encode::<_, _, _, _>(
      buf,
      num_elems,
      self.iter(),
      || elems_bytes,
      |item, buf| item.encode_raw(context, buf),
    )
  }

  fn encoded_len(&self, context: &Context) -> usize {
    let mut num_elems = 0;
    let num_bytes = self
      .iter()
      .map(|n| {
        num_elems += n.len();
        n.encoded_raw_len(context)
      })
      .sum::<usize>();
    let total_bytes = encoded_u32_varint_len(num_elems as u32) + num_bytes;
    encoded_u32_varint_len(total_bytes as u32) + total_bytes
  }
}

impl<T, N, W> PartialEncode<Flatten<Packed<W>, W>, Groto> for [N]
where
  W: WireFormat<Groto>,
  N: State<Flattened<Innermost>, Output = T>
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

    packed_encode_raw::<_, _, _, _>(
      buf,
      self.iter(),
      || {
        <Self as PartialEncode<Flatten<Packed<W>, W>, Groto>>::partial_encoded_raw_len(
          self, context, selector,
        )
      },
      |item, buf| item.partial_encode_raw(context, buf, selector),
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    self
      .iter()
      .map(|n| n.partial_encoded_raw_len(context, selector))
      .sum::<usize>()
  }

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if selector.is_empty() {
      return Ok(0);
    }

    check_list_type::<N>()?;

    let mut num_elems = 0;
    let elems_bytes = self
      .iter()
      .map(|n| {
        num_elems += n.len();
        n.partial_encoded_raw_len(context, selector)
      })
      .sum::<usize>();

    packed_encode::<_, _, _, _>(
      buf,
      num_elems,
      self.iter(),
      || elems_bytes,
      |item, buf| item.partial_encode_raw(context, buf, selector),
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    let mut num_elems = 0;
    let num_bytes = self
      .iter()
      .map(|n| {
        num_elems += n.len();
        n.partial_encoded_raw_len(context, selector)
      })
      .sum::<usize>();
    let total_bytes = encoded_u32_varint_len(num_elems as u32) + num_bytes;
    encoded_u32_varint_len(total_bytes as u32) + total_bytes
  }
}

impl<'a, T, N, W> PartialEncode<Flatten<Borrowed<'a, Packed<W>>, W>, Groto> for [&N]
where
  W: WireFormat<Groto>,
  N: State<Flattened<Innermost>, Output = T>
    + Length
    + PartialEncode<Packed<W>, Groto, Selector = T::Selector>,
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

    packed_encode_raw::<_, _, _, _>(
      buf,
      self.iter(),
      || {
        <Self as PartialEncode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::partial_encoded_raw_len(
          self, context, selector,
        )
      },
      |item, buf| item.partial_encode_raw(context, buf, selector),
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    self
      .iter()
      .map(|n| n.partial_encoded_raw_len(context, selector))
      .sum::<usize>()
  }

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if selector.is_empty() {
      return Ok(0);
    }

    check_list_type::<N>()?;

    let mut num_elems = 0;
    let elems_bytes = self
      .iter()
      .map(|n| {
        num_elems += n.len();
        n.partial_encoded_raw_len(context, selector)
      })
      .sum::<usize>();

    packed_encode::<_, _, _, _>(
      buf,
      num_elems,
      self.iter(),
      || elems_bytes,
      |item, buf| item.partial_encode_raw(context, buf, selector),
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    let mut num_elems = 0;
    let num_bytes = self
      .iter()
      .map(|n| {
        num_elems += n.len();
        n.partial_encoded_raw_len(context, selector)
      })
      .sum::<usize>();
    let total_bytes = encoded_u32_varint_len(num_elems as u32) + num_bytes;
    encoded_u32_varint_len(total_bytes as u32) + total_bytes
  }
}
