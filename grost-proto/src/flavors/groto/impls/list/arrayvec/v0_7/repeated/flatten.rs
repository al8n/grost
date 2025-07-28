use crate::{
  convert::{Extracted, Innermost},
  flavors::{Borrowed, Flatten},
  reflection::{Reflectable, SchemaType, SchemaTypeReflection},
  state::State,
};

use super::*;

impl<'a, T, N, W, const TAG: u32, const CAP: usize>
  Encode<Flatten<Borrowed<'a, Repeated<W, TAG>>, W>, Groto> for ArrayVec<&N, CAP>
where
  W: WireFormat<Groto>,
  N: State<Extracted<Innermost>, Output = T> + Encode<Repeated<W, TAG>, Groto> + ?Sized,
  T: Encode<W, Groto> + ?Sized,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
{
  fn encode_raw<WB>(&self, context: &Context, buf: &mut WB) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
  {
    <[&N] as Encode<Flatten<Borrowed<'_, Repeated<W, TAG>>, W>, Groto>>::encode_raw(
      self.as_slice(),
      context,
      buf,
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    <[&N] as Encode<Flatten<Borrowed<'_, Repeated<W, TAG>>, W>, Groto>>::encoded_raw_len(
      self.as_slice(),
      context,
    )
  }

  fn encode<B>(&self, context: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: WriteBuf + ?Sized,
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

impl<T, N, W, const TAG: u32, const CAP: usize> Encode<Flatten<Repeated<W, TAG>, W>, Groto>
  for ArrayVec<N, CAP>
where
  W: WireFormat<Groto>,
  N: State<Extracted<Innermost>, Output = T> + Encode<Repeated<W, TAG>, Groto>,
  T: Encode<W, Groto> + Sized,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
{
  fn encode_raw<WB>(&self, context: &Context, buf: &mut WB) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
  {
    <[N] as Encode<Flatten<Repeated<W, TAG>, W>, Groto>>::encode_raw(self.as_slice(), context, buf)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    <[N] as Encode<Flatten<Repeated<W, TAG>, W>, Groto>>::encoded_raw_len(self.as_slice(), context)
  }

  fn encode<B>(&self, context: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: WriteBuf + ?Sized,
  {
    <Self as Encode<Flatten<Repeated<W, TAG>, W>, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Flatten<Repeated<W, TAG>, W>, Groto>>::encoded_raw_len(self, context)
  }
}

impl<T, N, W, const TAG: u32, const CAP: usize> PartialEncode<Flatten<Repeated<W, TAG>, W>, Groto>
  for ArrayVec<N, CAP>
where
  W: WireFormat<Groto>,
  N: State<Extracted<Innermost>, Output = T> + PartialEncode<W, Groto, Selector = T::Selector>,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
  T: PartialEncode<W, Groto> + Sized,
{
  fn partial_encode_raw<WB>(
    &self,
    context: &Context,
    buf: &mut WB,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
  {
    <[N] as PartialEncode<Flatten<Repeated<W, TAG>, W>, Groto>>::partial_encode_raw(
      self.as_slice(),
      context,
      buf,
      selector,
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <[N] as PartialEncode<Flatten<Repeated<W, TAG>, W>, Groto>>::partial_encoded_raw_len(
      self.as_slice(),
      context,
      selector,
    )
  }

  fn partial_encode<WB>(
    &self,
    context: &Context,
    buf: &mut WB,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
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

impl<'a, T, N, W, const TAG: u32, const CAP: usize>
  PartialEncode<Flatten<Borrowed<'a, Repeated<W, TAG>>, W>, Groto> for ArrayVec<&N, CAP>
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
    buf: &mut WB,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
  {
    <[&N] as PartialEncode<Flatten<Borrowed<'a, Repeated<W, TAG>>, W>, Groto>>::partial_encode_raw(
      self.as_slice(),
      context,
      buf,
      selector,
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <[&N] as PartialEncode<Flatten<Borrowed<'a, Repeated<W, TAG>>, W>, Groto>>::partial_encoded_raw_len(
      self.as_slice(), context, selector,
    )
  }

  fn partial_encode<WB>(
    &self,
    context: &Context,
    buf: &mut WB,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
  {
    <Self as PartialEncode<Flatten<Borrowed<'_, Repeated<W, TAG>>, W>, Groto>>::partial_encode_raw(
      self, context, buf, selector,
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <Self as PartialEncode<Flatten<Borrowed<'_, Repeated<W, TAG>>, W>, Groto>>::partial_encoded_raw_len(self, context, selector)
  }
}
