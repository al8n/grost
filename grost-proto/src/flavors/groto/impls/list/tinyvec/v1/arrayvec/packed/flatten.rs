use super::*;

use crate::{
  convert::{Flattened, Innermost},
  encode::{Encode, EquivalentEncode, Length, PartialEncode},
  flavors::{Borrowed, Flatten},
  reflection::{Reflectable, SchemaType, SchemaTypeReflection},
  state::State,
};

impl<'a, T, N, W, A> Encode<Flatten<Borrowed<'a, Packed<W>>, W>, Groto> for ArrayVec<A>
where
  A: Array<Item = &'a N>,
  W: WireFormat<Groto>,
  N: State<Flattened<Innermost>, Output = T> + Length + Encode<Packed<W>, Groto> + ?Sized + 'a,
  T: Encode<W, Groto> + ?Sized,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <[&N] as Encode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::encode_raw(
      self.as_slice(),
      context,
      buf,
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    <[&N] as Encode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::encoded_raw_len(
      self.as_slice(),
      context,
    )
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <[&N] as Encode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::encode(
      self.as_slice(),
      context,
      buf,
    )
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <[&N] as Encode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::encoded_len(
      self.as_slice(),
      context,
    )
  }
}

unsafe impl<T, W, A> EquivalentEncode<[&[T]], Flatten<Borrowed<'_, Packed<W>>, W>, Groto>
  for ArrayVec<A>
where
  A: Array<Item = T>,
  W: WireFormat<Groto>,
  [T]: State<Flattened<Innermost>, Output = T> + Encode<Packed<W>, Groto>,
  T: Encode<W, Groto>,
  SchemaTypeReflection<[T]>: Reflectable<[T], Reflection = SchemaType>,
{
  type WireFormat = Packed<W>;

  type Flavor = Groto;
}

impl<T, N, W, A> Encode<Flatten<Packed<W>, W>, Groto> for ArrayVec<A>
where
  A: Array<Item = N>,
  W: WireFormat<Groto>,
  N: State<Flattened<Innermost>, Output = T> + Length + Encode<Packed<W>, Groto>,
  T: Encode<W, Groto> + Sized,
  SchemaTypeReflection<N>: Reflectable<N, Reflection = SchemaType>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <[N] as Encode<Flatten<Packed<W>, W>, Groto>>::encode_raw(self.as_slice(), context, buf)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    <[N] as Encode<Flatten<Packed<W>, W>, Groto>>::encoded_raw_len(self.as_slice(), context)
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <[N] as Encode<Flatten<Packed<W>, W>, Groto>>::encode_raw(self.as_slice(), context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <[N] as Encode<Flatten<Packed<W>, W>, Groto>>::encoded_raw_len(self.as_slice(), context)
  }
}

impl<T, N, W, A> PartialEncode<Flatten<Packed<W>, W>, Groto> for ArrayVec<A>
where
  A: Array<Item = N>,
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
    <[N] as PartialEncode<Flatten<Packed<W>, W>, Groto>>::partial_encode_raw(
      self.as_slice(),
      context,
      buf,
      selector,
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <[N] as PartialEncode<Flatten<Packed<W>, W>, Groto>>::partial_encoded_raw_len(
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
    <[N] as PartialEncode<Flatten<Packed<W>, W>, Groto>>::partial_encode_raw(
      self.as_slice(),
      context,
      buf,
      selector,
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <[N] as PartialEncode<Flatten<Packed<W>, W>, Groto>>::partial_encoded_raw_len(
      self.as_slice(),
      context,
      selector,
    )
  }
}

impl<'a, T, N, W, A> PartialEncode<Flatten<Borrowed<'a, Packed<W>>, W>, Groto> for ArrayVec<A>
where
  A: Array<Item = &'a N>,
  W: WireFormat<Groto>,
  N: State<Flattened<Innermost>, Output = T>
    + Length
    + PartialEncode<Packed<W>, Groto, Selector = T::Selector>
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
    <[&N] as PartialEncode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::partial_encode_raw(
      self.as_slice(),
      context,
      buf,
      selector,
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <[&N] as PartialEncode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::partial_encoded_raw_len(
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
    <[&N] as PartialEncode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::partial_encode_raw(
      self.as_slice(),
      context,
      buf,
      selector,
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <[&N] as PartialEncode<Flatten<Borrowed<'_, Packed<W>>, W>, Groto>>::partial_encoded_len(
      self.as_slice(),
      context,
      selector,
    )
  }
}
