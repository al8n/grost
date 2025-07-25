use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{TryFromPartialRef, TryFromRef},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, RepeatedEntry, WireFormat,
    groto::{
      Context, Error, PartialMapBuffer, PartialMapEntry, RepeatedMapDecoder,
      RepeatedMapDecoderBuffer, context::RepeatedDecodePolicy,
    },
  },
  selection::Selector,
  state::{Partial, PartialRef, Ref, State},
};

use super::super::{repeated_decode, repeated_encode, repeated_encoded_len};

impl<'a, K, V, KW, VW, RB, UB, PB, const TAG: u32>
  State<PartialRef<'a, RepeatedEntry<KW, VW, TAG>, RB, UB, Groto>> for PartialMapBuffer<K, V, PB>
where
  K: State<PartialRef<'a, KW, RB, UB, Groto>>,
  K::Output: Sized,
  V: State<PartialRef<'a, VW, RB, UB, Groto>>,
  V::Output: Sized,
{
  type Output = RepeatedMapDecoderBuffer<'a, K::Output, V::Output, RB, UB, KW, VW, TAG>;
}

impl<'a, K, V, KW, VW, RB, UB, PB, const TAG: u32>
  State<Ref<'a, RepeatedEntry<KW, VW, TAG>, RB, UB, Groto>> for PartialMapBuffer<K, V, PB>
where
  K: State<Ref<'a, KW, RB, UB, Groto>>,
  K::Output: Sized,
  V: State<Ref<'a, VW, RB, UB, Groto>>,
  V::Output: Sized,
{
  type Output = RepeatedMapDecoderBuffer<'a, K::Output, V::Output, RB, UB, KW, VW, TAG>;
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
  Decode<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto> for PartialMapBuffer<K, V, PB>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: Decode<'a, KW, RB, B, Groto>,
  V: Decode<'a, VW, RB, B, Groto>,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let mut this = Self::new();
    <Self as Decode<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>::merge_decode(
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
          return Err(Error::fail_to_reserve_capacity("map"));
        }

        for item in decoder.iter() {
          let (_, ent) = item?;
          if self.push(ent).is_some() {
            return Err(Error::capacity_exceeded("map"));
          }
        }

        Ok(read)
      }
      RepeatedDecodePolicy::GrowIncrementally => {
        repeated_decode::<KW, VW, Self, RB, TAG>(src, |ei, ki, vi, src| {
          let (read, entry) = PartialMapEntry::<K, V>::decode_repeated(ctx, src, ei, ki, vi)?;
          match entry {
            Some(entry) => {
              if self.push(entry).is_some() {
                return Err(Error::capacity_exceeded("map"));
              }

              Ok(Some(read))
            }
            None => Ok(None),
          }
        })
      }
    }
  }
}

impl<'de, K, V, RB, UB, PB, KW, VW, const TAG: u32>
  TryFromRef<'de, RepeatedEntry<KW, VW, TAG>, RB, UB, Groto> for PartialMapBuffer<K, V, PB>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: TryFromRef<'de, KW, RB, UB, Groto> + 'de,
  K::Output: Sized + Decode<'de, KW, RB, UB, Groto>,
  V: TryFromRef<'de, VW, RB, UB, Groto> + 'de,
  V::Output: Sized + Decode<'de, VW, RB, UB, Groto>,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
{
  fn try_from_ref(
    ctx: &'de Context,
    input: <Self as State<Ref<'de, RepeatedEntry<KW, VW, TAG>, RB, UB, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'de, RepeatedEntry<KW, VW, TAG>, RB, UB, Groto>>>::Output: Sized,
    RB: ReadBuf + 'de,
    UB: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    let Some(mut buffer) = Self::with_capacity(capacity_hint) else {
      return Err(Error::allocation_failed("map"));
    };

    for res in input.iter() {
      let (_, ent) = res?;
      let ent = ent.and_then(|k| K::try_from_ref(ctx, k), |v| V::try_from_ref(ctx, v))?;
      if buffer.push(ent).is_some() {
        return Err(Error::capacity_exceeded("map"));
      }
    }

    ctx
      .err_length_mismatch(capacity_hint, buffer.len())
      .map(|_| buffer)
  }
}

impl<'de, K, V, RB, UB, PB, KW, VW, const TAG: u32>
  TryFromPartialRef<'de, RepeatedEntry<KW, VW, TAG>, RB, UB, Groto> for PartialMapBuffer<K, V, PB>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: TryFromPartialRef<'de, KW, RB, UB, Groto> + 'de,
  K::Output: Sized + Decode<'de, KW, RB, UB, Groto>,
  V: TryFromPartialRef<'de, VW, RB, UB, Groto> + 'de,
  V::Output: Sized + Decode<'de, VW, RB, UB, Groto>,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
{
  fn try_from_partial_ref(
    ctx: &'de Context,
    input: <Self as State<PartialRef<'de, RepeatedEntry<KW, VW, TAG>, RB, UB, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'de, RepeatedEntry<KW, VW, TAG>, RB, UB, Groto>>>::Output: Sized,
    RB: ReadBuf + 'de,
    UB: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    let Some(mut buffer) = Self::with_capacity(capacity_hint) else {
      return Err(Error::allocation_failed("map"));
    };

    for res in input.iter() {
      let (_, ent) = res?;
      let ent = ent.and_then(
        |k| K::try_from_partial_ref(ctx, k),
        |v| V::try_from_partial_ref(ctx, v),
      )?;
      if buffer.push(ent).is_some() {
        return Err(Error::capacity_exceeded("map"));
      }
    }

    ctx
      .err_length_mismatch(capacity_hint, buffer.len())
      .map(|_| buffer)
  }
}
