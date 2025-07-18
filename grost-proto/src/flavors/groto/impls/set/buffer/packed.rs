use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{Partial, PartialRef, Ref, TryFromPartialRef, TryFromRef},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, Packed, WireFormat,
    groto::{Context, Error, Identifier, PackedSetDecoder, PartialSetBuffer, Tag},
  },
  selection::Selector,
  state::State,
};

impl<'a, K, KW, RB, UB, PB> State<PartialRef<'a, RB, UB, Packed<KW>, Groto>>
  for PartialSetBuffer<K, PB>
{
  type Output = PackedSetDecoder<'a, K, RB, UB, KW>;
}

impl<'a, K, KW, RB, UB, PB> State<Ref<'a, RB, UB, Packed<KW>, Groto>> for PartialSetBuffer<K, PB> {
  type Output = PackedSetDecoder<'a, K, RB, UB, KW>;
}

impl<K, V, KW, VW, PB> Encode<Packed<KW>, Groto> for PartialSetBuffer<K, PB>
where
  K: Encode<KW, Groto>,
  KW: WireFormat<Groto>,
  PB: Buffer<Item = K>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    todo!()
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    todo!()
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    todo!()
  }

  fn encoded_len(&self, context: &Context) -> usize {
    todo!()
  }
}

impl<K, KW, PB> PartialEncode<Packed<KW>, Groto> for PartialSetBuffer<K, PB>
where
  K: PartialEncode<KW, Groto>,
  KW: WireFormat<Groto>,
  PB: Buffer<Item = K>,
{
  fn partial_encode_raw(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    todo!()
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    todo!()
  }

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    todo!()
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    todo!()
  }
}

impl<'a, K, KW, RB, B, PB> Decode1<'a, Packed<KW>, RB, B, Groto> for PartialSetBuffer<K, PB>
where
  KW: WireFormat<Groto> + 'a,
  K: Decode1<'a, KW, RB, B, Groto>,
  PB: Buffer<Item = K>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    todo!()
  }
}
