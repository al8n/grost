use crate::{
  convert::{Flattened, Innermost}, encode::{Encode, EquivalentEncode, PartialEncode}, flavors::{
    groto::{Context, Error, LengthDelimited}, Borrowed, Flatten, Groto, Packed, WireFormat
  }, reflection::{Reflectable, SchemaType, SchemaTypeReflection}, selection::Selector, state::State
};

mod join;

impl<T, W> Encode<Packed<W>, Groto> for [T]
where
  T: Encode<W, Groto>,
  W: WireFormat<Groto>,
{
  encode!(@impl(Packed<W>));
}

impl<T, W> PartialEncode<Packed<W>, Groto> for [T]
where
  T: PartialEncode<W, Groto>,
  W: WireFormat<Groto>,
{
  partial_encode!(@impl(Packed<W>));
}

impl<'b, T: 'b, W> Encode<Borrowed<'b, Packed<W>>, Groto> for [&'b T]
where
  T: Encode<W, Groto>,
  W: WireFormat<Groto>,
{
  encode!(@impl(Borrowed<'b, Packed<W>>));
}

impl<'b, T: 'b, W> PartialEncode<Borrowed<'b, Packed<W>>, Groto> for [&'b T]
where
  T: PartialEncode<W, Groto>,
  W: WireFormat<Groto>,
{
  partial_encode!(@impl(Borrowed<'b, Packed<W>>));
}

impl Encode<LengthDelimited, Groto> for [u8] {
  #[inline]
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let buf_len = buf.len();
    let this_len = self.len();
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(
        <Self as crate::encode::Encode<LengthDelimited, Groto>>::encoded_len(self, context),
        buf_len,
      ));
    }

    buf[..this_len].copy_from_slice(self);
    Ok(this_len)
  }

  #[inline]
  fn encoded_raw_len(&self, _: &<Groto as crate::flavors::Flavor>::Context) -> usize {
    self.len()
  }

  #[inline]
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let buf_len = buf.len();
    let this_len = self.len();
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(
        <Self as Encode<LengthDelimited, Groto>>::encoded_len(self, context),
        buf_len,
      ));
    }

    let len_size = varing::encode_u32_varint_to(this_len as u32, buf).map_err(|e| {
      Error::from_varint_encode_error(e).update(
        <Self as Encode<LengthDelimited, Groto>>::encoded_len(self, context),
        buf_len,
      )
    })?;

    <Self as Encode<LengthDelimited, Groto>>::encode_raw(self, context, &mut buf[len_size..])
      .map(|write| len_size + write)
      .map_err(|e| {
        e.update(
          <Self as Encode<LengthDelimited, Groto>>::encoded_len(self, context),
          buf_len,
        )
      })
  }

  #[inline]
  fn encoded_len(&self, _: &Context) -> usize {
    let len = self.len();
    let len_size = varing::encoded_u32_varint_len(len as u32);
    len_size + len
  }
}

impl PartialEncode<LengthDelimited, Groto> for [u8] {
  #[inline]
  fn partial_encode_raw(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if *selector {
      <Self as Encode<LengthDelimited, Groto>>::encode_raw(self, context, buf)
    } else {
      Ok(0)
    }
  }

  #[inline]
  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if *selector {
      <Self as Encode<LengthDelimited, Groto>>::encoded_raw_len(self, context)
    } else {
      0
    }
  }

  #[inline]
  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if *selector {
      <Self as Encode<LengthDelimited, Groto>>::encode(self, context, buf)
    } else {
      Ok(0)
    }
  }

  #[inline]
  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if *selector {
      <Self as Encode<LengthDelimited, Groto>>::encoded_len(self, context)
    } else {
      0
    }
  }
}

impl<'a, T, N, W> Encode<Flatten<Borrowed<'a, Packed<W>>, W>, Groto> for [&N]
where
  W: WireFormat<Groto>,
  N: State<Flattened<Innermost>, Output = T> + Encode<Packed<W>, Groto> + ?Sized,
  T: Encode<W, Groto> + ?Sized,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    check_list_type::<N>()?;

    let buf_len = buf.len();
    let this_len =
      <Self as Encode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::encoded_raw_len(self, context);
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(this_len, buf_len));
    }

    let mut offset = 0;
    for value in self.iter() {
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(this_len, buf_len));
      }

      offset += value.encode_raw(context, &mut buf[offset..])?;
    }

    Ok(offset)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    self
      .iter()
      .map(|n| n.encoded_raw_len(context))
      .sum::<usize>()
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    check_list_type::<N>()?;

    let buf_len = buf.len();
    let data_len =
      <Self as Encode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::encoded_raw_len(self, context);
    let this_len = varing::encoded_u32_varint_len(data_len as u32) + data_len;
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(this_len, buf_len));
    }

    let mut offset = varing::encode_u32_varint_to(data_len as u32, buf)
      .map_err(|e| Error::from_varint_encode_error(e).update(this_len, buf_len))?;
    for value in self.iter() {
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(this_len, buf_len));
      }

      offset += value.encode_raw(context, &mut buf[offset..])?;
    }

    Ok(offset)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    let encoded_len =
      <Self as Encode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::encoded_raw_len(self, context);
    varing::encoded_u32_varint_len(encoded_len as u32) + encoded_len
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
  N: State<Flattened<Innermost>, Output = T> + Encode<Packed<W>, Groto>,
  T: Encode<W, Groto> + Sized,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    check_list_type::<N>()?;

    let buf_len = buf.len();
    let this_len = <Self as Encode<Flatten<Packed<W>, W>, Groto>>::encoded_raw_len(self, context);
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(this_len, buf_len));
    }

    let mut offset = 0;
    for value in self.iter() {
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(this_len, buf_len));
      }

      offset += value.encode_raw(context, &mut buf[offset..])?;
    }

    Ok(offset)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    self
      .iter()
      .map(|n| n.encoded_raw_len(context))
      .sum::<usize>()
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    check_list_type::<N>()?;

    let buf_len = buf.len();
    let data_len = <Self as Encode<Flatten<Packed<W>, W>, Groto>>::encoded_raw_len(self, context);
    let this_len = varing::encoded_u32_varint_len(data_len as u32) + data_len;
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(this_len, buf_len));
    }

    let mut offset = varing::encode_u32_varint_to(data_len as u32, buf)
      .map_err(|e| Error::from_varint_encode_error(e).update(this_len, buf_len))?;
    for value in self.iter() {
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(this_len, buf_len));
      }

      offset += value.encode_raw(context, &mut buf[offset..])?;
    }

    Ok(offset)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    let encoded_len =
      <Self as Encode<Flatten<Packed<W>, W>, Groto>>::encoded_raw_len(self, context);
    varing::encoded_u32_varint_len(encoded_len as u32) + encoded_len
  }
}

impl<T, N, W> PartialEncode<Flatten<Packed<W>, W>, Groto> for [N]
where
  W: WireFormat<Groto>,
  N: State<Flattened<Innermost>, Output = T> + PartialEncode<W, Groto, Selector = T::Selector>,
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
      return Ok(0); // If the selector is empty, no encoding is needed
    }

    check_list_type::<N>()?;

    let buf_len = buf.len();
    let this_len = <Self as PartialEncode<Flatten<Packed<W>, W>, Groto>>::partial_encoded_len(
      self, context, selector,
    );
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(this_len, buf_len));
    }

    let mut offset = 0;
    for value in self.iter() {
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(this_len, buf_len));
      }

      offset += value.partial_encode_raw(context, &mut buf[offset..], selector)?;
    }
    Ok(offset)
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0; // If the selector is empty, no encoding is needed
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
      return Ok(0); // If the selector is empty, no encoding is needed
    }

    check_list_type::<N>()?;

    let buf_len = buf.len();
    let data_len = <Self as PartialEncode<Flatten<Packed<W>, W>, Groto>>::partial_encoded_raw_len(
      self, context, selector,
    );
    let this_len = varing::encoded_u32_varint_len(data_len as u32) + data_len;
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(this_len, buf_len));
    }

    let mut offset = varing::encode_u32_varint_to(data_len as u32, buf)
      .map_err(|e| Error::from_varint_encode_error(e).update(this_len, buf_len))?;
    for value in self.iter() {
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(this_len, buf_len));
      }

      offset += value.partial_encode_raw(context, &mut buf[offset..], selector)?;
    }

    Ok(offset)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0; // If the selector is empty, no encoding is needed
    }

    let encoded_len =
      <Self as PartialEncode<Flatten<Packed<W>, W>, Groto>>::partial_encoded_raw_len(
        self, context, selector,
      );
    varing::encoded_u32_varint_len(encoded_len as u32) + encoded_len
  }
}

impl<'a, T, N, W> PartialEncode<Flatten<Borrowed<'a, Packed<W>>, W>, Groto> for [&N]
where
  W: WireFormat<Groto>,
  N: State<Flattened<Innermost>, Output = T>
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
      return Ok(0); // If the selector is empty, no encoding is needed
    }

    check_list_type::<N>()?;

    let buf_len = buf.len();
    let this_len =
      <Self as PartialEncode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::partial_encoded_len(
        self, context, selector,
      );
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(this_len, buf_len));
    }

    let mut offset = 0;
    for value in self.iter() {
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(this_len, buf_len));
      }

      offset += value.partial_encode_raw(context, &mut buf[offset..], selector)?;
    }
    Ok(offset)
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0; // If the selector is empty, no encoding is needed
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
      return Ok(0); // If the selector is empty, no encoding is needed
    }

    check_list_type::<N>()?;

    let buf_len = buf.len();
    let data_len =
      <Self as PartialEncode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::partial_encoded_raw_len(
        self, context, selector,
      );
    let this_len = varing::encoded_u32_varint_len(data_len as u32) + data_len;
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(this_len, buf_len));
    }

    let mut offset = varing::encode_u32_varint_to(data_len as u32, buf)
      .map_err(|e| Error::from_varint_encode_error(e).update(this_len, buf_len))?;
    for value in self.iter() {
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(this_len, buf_len));
      }

      offset += value.partial_encode_raw(context, &mut buf[offset..], selector)?;
    }

    Ok(offset)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0; // If the selector is empty, no encoding is needed
    }

    let encoded_len =
      <Self as PartialEncode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::partial_encoded_raw_len(
        self, context, selector,
      );
    varing::encoded_u32_varint_len(encoded_len as u32) + encoded_len
  }
}

fn check_list_type<T>() -> Result<(), Error>
where
  T: ?Sized,
  SchemaTypeReflection<T>: Reflectable<T, Reflection = SchemaType>,
{
  if !SchemaTypeReflection::<T>::REFLECTION.is_list() {
    return Err(Error::custom(
      "cannot encode a non-nested list type as flatten format",
    ));
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::{encode::EquivalentEncode, flavors::groto::Varint};

  use super::*;

  #[test]
  fn t() {
    fn equivalent_encoded_len<T, W>(input: &T, ctx: &Context) -> usize
    where
      W: WireFormat<Groto>,
      T: Encode<W, Groto> + ?Sized,
      [u16]: EquivalentEncode<T, W, Groto, WireFormat = Packed<Varint>, Flavor = Groto>,
    {
      input.encoded_len(ctx)
    }

    let a: &[&[u16]] = &[&[1u16, 2, 3], &[4, 5, 6]];
    let flatten_a: &[u16] = &[1u16, 2, 3, 4, 5, 6];

    let context = Context::default();
    let encoded_len = equivalent_encoded_len(a, &context);
    let flatten_encoded_len = equivalent_encoded_len(flatten_a, &context);
    assert_eq!(encoded_len, flatten_encoded_len);

    let mut buf = [0u8; 100];
    let mut flatten_buf = [0u8; 100];

    let encoded_len =
      <[&[u16]] as Encode<Flatten<Borrowed<'_, Packed<Varint>>, Varint>, Groto>>::encode(
        a, &context, &mut buf,
      )
      .unwrap();
    let flatten_encoded_len =
      <[u16] as Encode<Packed<Varint>, Groto>>::encode(flatten_a, &context, &mut flatten_buf)
        .unwrap();
    assert_eq!(encoded_len, flatten_encoded_len);
    assert_eq!(&buf[..encoded_len], &flatten_buf[..flatten_encoded_len]);
  }
}
