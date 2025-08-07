use crate::{
  buffer::{BufMut, WriteBuf},
  encode::{Encode, PartialEncode},
  flavors::{
    Borrowed, Groto, Repeated, WireFormat,
    groto::{
      Context, Error,
      impls::{repeated_encode, repeated_encoded_len},
    },
  },
  selection::Selector,
};

use super::check_list_type;

mod flatten;

impl<T, W, const TAG: u32> Encode<Repeated<W, TAG>, Groto> for [T]
where
  T: Encode<W, Groto>,
  W: WireFormat<Groto>,
{
  fn encode_raw<WB>(&self, context: &Context, buf: impl Into<WriteBuf<WB>>) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    repeated_encode::<WB, T, W, _, TAG>(
      buf.into(),
      || self.iter(),
      |k| k.encoded_len(context),
      |k, buf| k.encode::<&mut WB>(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    repeated_encoded_len::<T, W, _, TAG>(self.iter(), |k| k.encoded_len(context))
  }

  fn encode<WB>(&self, context: &Context, buf: impl Into<WriteBuf<WB>>) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    <Self as Encode<Repeated<W, TAG>, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Repeated<W, TAG>, Groto>>::encoded_raw_len(self, context)
  }
}

impl<T, W, const TAG: u32> PartialEncode<Repeated<W, TAG>, Groto> for [T]
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

    repeated_encode::<WB, T, W, _, TAG>(
      buf.into(),
      || self.iter(),
      |k| k.partial_encoded_len(context, selector),
      |k, buf| k.partial_encode::<&mut WB>(context, buf, selector),
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    repeated_encoded_len::<T, W, _, TAG>(self.iter(), |k| k.partial_encoded_len(context, selector))
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

    repeated_encode::<WB, T, W, _, TAG>(
      buf.into(),
      || self.iter(),
      |k| k.partial_encoded_len(context, selector),
      |k, buf| k.partial_encode::<&mut WB>(context, buf, selector),
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    repeated_encoded_len::<T, W, _, TAG>(self.iter(), |k| k.partial_encoded_len(context, selector))
  }
}

impl<'b, T: 'b, W, const TAG: u32> Encode<Borrowed<'b, Repeated<W, TAG>>, Groto> for [&'b T]
where
  T: Encode<W, Groto>,
  W: WireFormat<Groto>,
{
  fn encode_raw<WB>(&self, context: &Context, buf: impl Into<WriteBuf<WB>>) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    repeated_encode::<WB, T, W, _, TAG>(
      buf.into(),
      || self.iter().copied(),
      |k| k.encoded_len(context),
      |k, buf| k.encode::<&mut WB>(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    repeated_encoded_len::<T, W, _, TAG>(self.iter().copied(), |k| k.encoded_len(context))
  }

  fn encode<WB>(&self, context: &Context, buf: impl Into<WriteBuf<WB>>) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    <Self as Encode<Borrowed<'b, Repeated<W, TAG>>, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Borrowed<'b, Repeated<W, TAG>>, Groto>>::encoded_raw_len(self, context)
  }
}

impl<'b, T: 'b, W, const TAG: u32> PartialEncode<Borrowed<'b, Repeated<W, TAG>>, Groto>
  for [&'b T]
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

    repeated_encode::<WB, T, W, _, TAG>(
      buf.into(),
      || self.iter().copied(),
      |k| k.partial_encoded_len(context, selector),
      |k, buf| k.partial_encode::<&mut WB>(context, buf, selector),
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    repeated_encoded_len::<T, W, _, TAG>(self.iter().copied(), |k| {
      k.partial_encoded_len(context, selector)
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
    <Self as PartialEncode<Borrowed<'b, Repeated<W, TAG>>, Groto>>::partial_encode_raw(
      self, context, buf, selector,
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <Self as PartialEncode<Borrowed<'b, Repeated<W, TAG>>, Groto>>::partial_encoded_raw_len(
      self, context, selector,
    )
  }
}
