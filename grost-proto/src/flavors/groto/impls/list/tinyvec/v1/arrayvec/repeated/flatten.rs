use crate::{
  convert::{Extracted, Innermost},
  encode::{Encode, Length, PartialEncode},
  flavors::{Borrowed, Flatten},
  reflection::{Reflectable, SchemaType, SchemaTypeReflection},
  state::State,
};

use super::*;

impl<'a, T, N, W, A, const TAG: u32> Encode<Flatten<Borrowed<'a, Repeated<W, TAG>>, W>, Groto>
  for ArrayVec<A>
where
  A: Array<Item = &'a N>,
  W: WireFormat<Groto>,
  N: State<Extracted<Innermost>, Output = T>
    + Length
    + Encode<Repeated<W, TAG>, Groto>
    + ?Sized
    + 'a,
  T: Encode<W, Groto> + ?Sized,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
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

impl<T, N, W, A, const TAG: u32> Encode<Flatten<Repeated<W, TAG>, W>, Groto> for ArrayVec<A>
where
  A: Array<Item = N>,
  W: WireFormat<Groto>,
  N: State<Extracted<Innermost>, Output = T> + Length + Encode<Repeated<W, TAG>, Groto>,
  T: Encode<W, Groto> + Sized,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <[N] as Encode<Flatten<Repeated<W, TAG>, W>, Groto>>::encode_raw(self.as_slice(), context, buf)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    <[N] as Encode<Flatten<Repeated<W, TAG>, W>, Groto>>::encoded_raw_len(self.as_slice(), context)
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Flatten<Repeated<W, TAG>, W>, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Flatten<Repeated<W, TAG>, W>, Groto>>::encoded_raw_len(self, context)
  }
}

impl<T, N, W, A, const TAG: u32> PartialEncode<Flatten<Repeated<W, TAG>, W>, Groto> for ArrayVec<A>
where
  A: Array<Item = N>,
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

impl<'a, T, N, W, A, const TAG: u32>
  PartialEncode<Flatten<Borrowed<'a, Repeated<W, TAG>>, W>, Groto> for ArrayVec<A>
where
  A: Array<Item = &'a N>,
  W: WireFormat<Groto>,
  N: State<Extracted<Innermost>, Output = T>
    + Length
    + PartialEncode<Repeated<W, TAG>, Groto, Selector = T::Selector>
    + 'a,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
  T: PartialEncode<W, Groto> + Sized,
{
  fn partial_encode_raw(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
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
