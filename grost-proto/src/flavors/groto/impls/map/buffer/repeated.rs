use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{PartialRef, Ref, TryFromPartialRef, TryFromRef},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    groto::{context::RepeatedDecodePolicy, Context, Error, PartialMapBuffer, PartialMapEntry, RepeatedMapDecoder, RepeatedMapDecoderBuffer}, Groto, RepeatedEntry, WireFormat
  },
  selection::Selector,
  state::State,
};

use super::super::{repeated_decode, repeated_encode, repeated_encoded_len};

impl<'a, K, V, KW, VW, RB, UB, PB, const TAG: u32>
  State<PartialRef<'a, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto>> for PartialMapBuffer<K, V, PB>
{
  type Output = RepeatedMapDecoderBuffer<'a, K, V, RB, UB, KW, VW, TAG>;
}

impl<'a, K, V, KW, VW, RB, UB, PB, const TAG: u32>
  State<Ref<'a, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto>> for PartialMapBuffer<K, V, PB>
{
  type Output = RepeatedMapDecoderBuffer<'a, K, V, RB, UB, KW, VW, TAG>;
}

impl<K, V, KW, VW, PB, const TAG: u32> Encode<RepeatedEntry<KW, VW, TAG>, Groto>
  for PartialMapBuffer<K, V, PB>
where
  K: Encode<KW, Groto>,
  V: Encode<VW, Groto>,
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    repeated_encode::<KW, VW, _, _, TAG>(
      buf,
      self.iter(),
      || <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encoded_raw_len(self, context),
      |item, ei, ki, vi, buf| item.encode_repeated::<KW, VW>(context, buf, ei, ki, vi),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    repeated_encoded_len::<KW, VW, _, _, TAG>(self.iter(), |item, ei, ki, vi| {
      item.encoded_repeated_len(context, ei, ki, vi)
    })
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encoded_raw_len(self, context)
  }
}

impl<K, V, KW, VW, PB, const TAG: u32> PartialEncode<RepeatedEntry<KW, VW, TAG>, Groto>
  for PartialMapBuffer<K, V, PB>
where
  K: PartialEncode<KW, Groto>,
  V: PartialEncode<VW, Groto>,
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
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

    repeated_encode::<KW, VW, _, _, TAG>(
      buf,
      self.iter(),
      || {
        <Self as PartialEncode<RepeatedEntry<KW, VW, TAG>, Groto>>::partial_encoded_raw_len(
          self, context, selector,
        )
      },
      |item, ei, ki, vi, buf| {
        item.partial_encode_repeated::<KW, VW>(context, buf, ei, ki, vi, selector)
      },
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    repeated_encoded_len::<KW, VW, _, _, TAG>(self.iter(), |item, ei, ki, vi| {
      item.partial_encoded_repeated_len(context, ei, ki, vi, selector)
    })
  }

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    <Self as PartialEncode<RepeatedEntry<KW, VW, TAG>, Groto>>::partial_encode_raw(
      self, context, buf, selector,
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <Self as PartialEncode<RepeatedEntry<KW, VW, TAG>, Groto>>::partial_encoded_raw_len(
      self, context, selector,
    )
  }
}

impl<'a, K, KW, V, VW, RB, B, PB, const TAG: u32>
  Decode1<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto> for PartialMapBuffer<K, V, PB>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: Decode1<'a, KW, RB, B, Groto>,
  V: Decode1<'a, VW, RB, B, Groto>,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let mut this = Self::new();
    <Self as Decode1<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>::merge_decode(
      &mut this, context, src,
    )
    .map(|size| (size, this))
  }

  fn merge_decode(&mut self, ctx: &'a Context, src: RB) -> Result<usize, Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    match ctx.repeated_decode_policy() {
      RepeatedDecodePolicy::PreallocateCapacity => {
        let (read, decoder) = RepeatedMapDecoder::<K, V, RB, B, KW, VW, TAG>::decode(ctx, src)?;

        if !self.try_reserve_exact(decoder.capacity_hint()) {
          return Err(Error::custom("failed to reserve capacity for map entries"));
        }

        for item in decoder.iter() {
          let (_, ent) = item?;
          if self.push(ent).is_some() {
            return Err(Error::custom("exceeded map buffer capacity"));
          }
        }

        Ok(read)
      },
      RepeatedDecodePolicy::GrowIncrementally => {
        repeated_decode::<KW, VW, Self, RB, TAG>(src, |ei, ki, vi, src| {
          let (read, entry) = PartialMapEntry::<K, V>::decode_repeated(ctx, src, ei, ki, vi)?;
          match entry {
            Some(entry) => {
              if self.push(entry).is_some() {
                return Err(Error::custom("exceeded map buffer capacity"));
              }

              Ok(Some(read))
            }
            None => Ok(None),
          }
        })
      },
    }
  }
}

impl<'de, K, V, RB, UB, PB, KW, VW, const TAG: u32>
  TryFromRef<'de, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto> for PartialMapBuffer<K, V, PB>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: TryFromRef<'de, RB, UB, KW, Groto> + Decode1<'de, KW, RB, UB, Groto> + 'de,
  K::Output: Sized,
  V: TryFromRef<'de, RB, UB, VW, Groto> + Decode1<'de, VW, RB, UB, Groto> + 'de,
  V::Output: Sized,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
{
  fn try_from_ref(
    ctx: &'de Context,
    input: <Self as State<Ref<'de, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'de, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'de,
    UB: UnknownBuffer<RB, Groto>,
  {
    let Some(mut buffer) = Self::with_capacity(input.capacity_hint()) else {
      return Err(Error::custom("failed to create buffer with given capacity"));
    };

    for res in input.iter() {
      let (_, ent) = res?;
      if buffer.push(ent).is_none() && ctx.err_on_length_mismatch() {
        return Err(Error::custom(
          "exceeded buffer capacity while pushing map entry",
        ));
      }
    }

    Ok(buffer)
  }
}

impl<'de, K, V, RB, UB, PB, KW, VW, const TAG: u32>
  TryFromPartialRef<'de, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto> for PartialMapBuffer<K, V, PB>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: TryFromPartialRef<'de, RB, UB, KW, Groto> + Decode1<'de, KW, RB, UB, Groto> + 'de,
  K::Output: Sized,
  V: TryFromPartialRef<'de, RB, UB, VW, Groto> + Decode1<'de, VW, RB, UB, Groto> + 'de,
  V::Output: Sized,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
{
  fn try_from_partial_ref(
    ctx: &'de Context,
    input: <Self as State<PartialRef<'de, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'de, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'de,
    UB: UnknownBuffer<RB, Groto>,
  {
    let Some(mut buffer) = Self::with_capacity(input.capacity_hint()) else {
      return Err(Error::custom("failed to create buffer with given capacity"));
    };

    for res in input.iter() {
      let (_, ent) = res?;
      if buffer.push(ent).is_none() && ctx.err_on_length_mismatch() {
        return Err(Error::custom(
          "exceeded buffer capacity while pushing map entry",
        ));
      }
    }

    Ok(buffer)
  }
}
