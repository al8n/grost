use varing::encoded_u32_varint_len;

use crate::{
  buffer::BufMut,
  convert::{Extracted, Innermost},
  encode::{Encode, EquivalentEncode, Length, PartialEncode},
  flavors::{
    Borrowed, Flatten, Groto, Packed, WireFormat,
    groto::{
      Context, Error,
      impls::{packed_encode, packed_encode_raw, packed_encoded_len, packed_encoded_raw_len},
    },
  },
  reflection::{Reflectable, SchemaType, SchemaTypeReflection},
  selection::Selector,
  state::State,
};

use super::check_list_type;

mod flatten;

impl<T, W> Encode<Packed<W>, Groto> for [T]
where
  T: Encode<W, Groto>,
  W: WireFormat<Groto>,
{
  #[inline]
  fn encode_raw<WB>(&self, context: &Context, buf: impl Into<WriteBuf<WB>>) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    packed_encode_raw::<T, _, _, _>(
      buf.buffer_mut(),
      self.iter(),
      || <Self as Encode<Packed<W>, Groto>>::encoded_raw_len(self, context),
      |item, buf| item.encode(context, buf),
    )
  }

  #[inline]
  fn encoded_raw_len(&self, context: &Context) -> usize {
    packed_encoded_raw_len::<T, W, _, _>(self.len(), self.iter(), |item| item.encoded_len(context))
  }

  fn encode<B>(&self, context: &Context, buf: impl Into<WriteBuf<B>>) -> Result<usize, Error>
  where
    B: BufMut,
  {
    packed_encode::<T, _, _, _>(
      buf.buffer_mut(),
      self.len(),
      self.iter(),
      || <Self as Encode<Packed<W>, Groto>>::encoded_raw_len(self, context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_len(&self, context: &Context) -> usize {
    packed_encoded_len::<_>(self.len(), || {
      <Self as Encode<Packed<W>, Groto>>::encoded_raw_len(self, context)
    })
  }
}

impl<T, W> PartialEncode<Packed<W>, Groto> for [T]
where
  T: PartialEncode<W, Groto>,
  W: WireFormat<Groto>,
{
  fn partial_encode_raw<WB>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    if selector.is_empty() {
      return Ok(0);
    }

    packed_encode_raw::<T, _, _, _>(
      buf.buffer_mut(),
      self.iter(),
      || {
        <Self as PartialEncode<Packed<W>, Groto>>::partial_encoded_raw_len(self, context, selector)
      },
      |item, buf| item.partial_encode(context, buf, selector),
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    packed_encoded_raw_len::<T, W, _, _>(self.len(), self.iter(), |item| {
      item.partial_encoded_len(context, selector)
    })
  }

  fn partial_encode<WB>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    if selector.is_empty() {
      return Ok(0);
    }

    packed_encode::<T, _, _, _>(
      buf.buffer_mut(),
      self.len(),
      self.iter(),
      || {
        <Self as PartialEncode<Packed<W>, Groto>>::partial_encoded_raw_len(self, context, selector)
      },
      |item, buf| item.partial_encode(context, buf, selector),
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    packed_encoded_len::<_>(self.len(), || {
      <Self as PartialEncode<Packed<W>, Groto>>::partial_encoded_raw_len(self, context, selector)
    })
  }
}

impl<'b, T: 'b, W> Encode<Borrowed<'b, Packed<W>>, Groto> for [&'b T]
where
  T: Encode<W, Groto>,
  W: WireFormat<Groto>,
{
  fn encode_raw<WB>(&self, context: &Context, buf: impl Into<WriteBuf<WB>>) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    packed_encode_raw::<T, _, _, _>(
      buf.buffer_mut(),
      self.into_iter().copied(),
      || <Self as Encode<Borrowed<'b, Packed<W>>, Groto>>::encoded_raw_len(self, context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    packed_encoded_raw_len::<T, W, _, _>(self.len(), self.into_iter().copied(), |item| {
      item.encoded_len(context)
    })
  }

  fn encode<WB>(&self, context: &Context, buf: impl Into<WriteBuf<WB>>) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    packed_encode::<T, _, _, _>(
      buf.buffer_mut(),
      self.len(),
      self.into_iter().copied(),
      || <Self as Encode<Borrowed<'b, Packed<W>>, Groto>>::encoded_raw_len(self, context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_len(&self, context: &Context) -> usize {
    packed_encoded_len::<_>(self.len(), || {
      <Self as Encode<Borrowed<'b, Packed<W>>, Groto>>::encoded_raw_len(self, context)
    })
  }
}

impl<'b, T: 'b, W> PartialEncode<Borrowed<'b, Packed<W>>, Groto> for [&'b T]
where
  T: PartialEncode<W, Groto>,
  W: WireFormat<Groto>,
{
  fn partial_encode_raw<WB>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    if selector.is_empty() {
      return Ok(0);
    }

    packed_encode_raw::<T, _, _, _>(
      buf.buffer_mut(),
      self.into_iter().copied(),
      || {
        <Self as PartialEncode<Borrowed<'b, Packed<W>>, Groto>>::partial_encoded_raw_len(
          self, context, selector,
        )
      },
      |item, buf| item.partial_encode(context, buf, selector),
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    packed_encoded_raw_len::<T, W, _, _>(self.len(), self.into_iter().copied(), |item| {
      item.partial_encoded_len(context, selector)
    })
  }

  fn partial_encode<WB>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    if selector.is_empty() {
      return Ok(0);
    }

    packed_encode::<T, _, _, _>(
      buf.buffer_mut(),
      self.len(),
      self.into_iter().copied(),
      || {
        <Self as PartialEncode<Borrowed<'b, Packed<W>>, Groto>>::partial_encoded_raw_len(
          self, context, selector,
        )
      },
      |item, buf| item.partial_encode(context, buf, selector),
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    packed_encoded_len::<_>(self.len(), || {
      <Self as PartialEncode<Borrowed<'b, Packed<W>>, Groto>>::partial_encoded_raw_len(
        self, context, selector,
      )
    })
  }
}
