use std::collections::BTreeSet;

use crate::{
  buffer::{ReadBuf, UnknownBuffer, WriteBuf},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultSetWireFormat, Groto, Packed, WireFormat,
    groto::{Context, Error, PackedSetDecoder},
  },
  state::{PartialRef, Ref, State},
};

use super::super::super::{
  packed_decode, packed_encode, packed_encode_raw, packed_encoded_len, packed_encoded_raw_len,
};

impl<K> DefaultSetWireFormat<Groto> for BTreeSet<K> {
  type Format<KM>
    = Packed<KM>
  where
    KM: WireFormat<Groto> + 'static;
}

impl<'a, K, KW, RB, B> State<PartialRef<'a, Packed<KW>, RB, B, Groto>> for BTreeSet<K>
where
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: State<Ref<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
{
  type Output = PackedSetDecoder<'a, K::Output, RB, B, KW>;
}

impl<'a, K, KW, RB, B> State<Ref<'a, Packed<KW>, RB, B, Groto>> for BTreeSet<K>
where
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: State<Ref<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
{
  type Output = PackedSetDecoder<'a, K::Output, RB, B, KW>;
}

impl<'a, K, KW, RB, B> Decode<'a, Packed<KW>, RB, B, Groto> for BTreeSet<K>
where
  KW: WireFormat<Groto> + 'a,
  K: Ord + Decode<'a, KW, RB, B, Groto>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    packed_decode::<K, KW, Self, RB>(
      context,
      src,
      |_| Ok(Self::new()),
      Self::len,
      |set, src| {
        let (read, item) = K::decode(context, src)?;

        context.err_duplicated_set_keys(!set.insert(item))?;

        Ok(read)
      },
    )
  }
}

impl<K, KW> Encode<Packed<KW>, Groto> for BTreeSet<K>
where
  KW: WireFormat<Groto>,
  K: Encode<KW, Groto>,
{
  fn encode_raw<WB>(&self, context: &Context, buf: &mut WB) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
  {
    packed_encode_raw::<K, _, _, _>(
      buf.as_mut_slice(),
      self.iter(),
      || <Self as Encode<Packed<KW>, Groto>>::encoded_raw_len(self, context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    packed_encoded_raw_len::<K, KW, _, _>(self.len(), self.iter(), |item| item.encoded_len(context))
  }

  fn encode<WB>(&self, context: &Context, buf: &mut WB) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
  {
    packed_encode::<K, _, _, _>(
      buf.as_mut_slice(),
      self.len(),
      self.iter(),
      || <Self as Encode<Packed<KW>, Groto>>::encoded_raw_len(self, context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_len(&self, context: &Context) -> usize {
    packed_encoded_len::<_>(self.len(), || {
      <Self as Encode<Packed<KW>, Groto>>::encoded_raw_len(self, context)
    })
  }
}

impl<K, KW> PartialEncode<Packed<KW>, Groto> for BTreeSet<K>
where
  KW: WireFormat<Groto>,
  K: Encode<KW, Groto>,
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
    if *selector {
      return Ok(0);
    }

    <Self as Encode<Packed<KW>, Groto>>::encode_raw(self, context, buf)
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if *selector {
      return 0;
    }

    <Self as Encode<Packed<KW>, Groto>>::encoded_raw_len(self, context)
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
    if *selector {
      return Ok(0);
    }

    <Self as Encode<Packed<KW>, Groto>>::encode(self, context, buf)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if *selector {
      return 0;
    }

    <Self as Encode<Packed<KW>, Groto>>::encoded_len(self, context)
  }
}
