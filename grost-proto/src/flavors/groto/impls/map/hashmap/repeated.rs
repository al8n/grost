use core::hash::{BuildHasher, Hash};

use super::{
  super::{
    DefaultPartialMapBuffer, MapEntry, repeated_decode, repeated_encode, repeated_encoded_len,
    try_from,
  },
  HashMap,
};

use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{PartialTryFromRef, TryFromPartialRef, TryFromRef},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultRepeatedEntryWireFormat, Groto, RepeatedEntry, WireFormat,
    groto::{
      Context, Error, RepeatedMapDecoder, RepeatedMapDecoderBuffer, context::RepeatedDecodePolicy,
    },
  },
  selection::{Selectable, Selector},
  state::{Partial, PartialRef, Ref, State},
};

impl<K, V, S> DefaultRepeatedEntryWireFormat<Groto> for HashMap<K, V, S> {
  type Format<KM, VM, const TAG: u32>
    = RepeatedEntry<KM, VM, TAG>
  where
    KM: WireFormat<Groto> + 'static,
    VM: WireFormat<Groto> + 'static;
}

impl<'a, K, V, KW, VW, S, RB, B, const TAG: u32>
  State<PartialRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>> for HashMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: State<PartialRef<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
  V: State<PartialRef<'a, VW, RB, B, Groto>>,
  V::Output: Sized,
  RepeatedEntry<KW, VW, TAG>: WireFormat<Groto> + 'a,
{
  type Output = RepeatedMapDecoderBuffer<'a, K::Output, V::Output, RB, B, KW, VW, TAG>;
}

impl<'a, K, KW, V, VW, S, RB, B, const TAG: u32>
  State<Ref<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>> for HashMap<K, V, S>
where
  RepeatedEntry<KW, VW, TAG>: WireFormat<Groto> + 'a,
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: State<Ref<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
  V: State<Ref<'a, VW, RB, B, Groto>>,
  V::Output: Sized,
{
  type Output = RepeatedMapDecoderBuffer<'a, K::Output, V::Output, RB, B, KW, VW, TAG>;
}

impl<'a, K, KW, V, VW, S, RB, B, const TAG: u32>
  Decode<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto> for HashMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  S: BuildHasher + Default,
  K: Eq + Hash + Decode<'a, KW, RB, B, Groto>,
  V: Decode<'a, VW, RB, B, Groto>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let mut this = HashMap::with_hasher(S::default());
    <Self as Decode<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>::merge_decode(
      &mut this, context, src,
    )
    .map(|size| (size, this))
  }

  fn merge_decode(&mut self, context: &'a Context, src: RB) -> Result<usize, Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    match context.repeated_decode_policy() {
      RepeatedDecodePolicy::GrowIncrementally => {
        repeated_decode::<KW, VW, Self, RB, TAG>(src, |ei, ki, vi, src| {
          let (read, entry) = MapEntry::decode_repeated(context, src, ei, ki, vi)?;
          match entry {
            Some(entry) => {
              let (k, v) = entry.into_components();
              context.err_duplicated_map_keys(self.insert(k, v).is_some())?;

              Ok(Some(read))
            }
            None => Ok(None),
          }
        })
      }
      RepeatedDecodePolicy::PreallocateCapacity => {
        let (read, decoder) = RepeatedMapDecoder::<K, V, RB, B, KW, VW, TAG>::decode(context, src)?;
        self.reserve(decoder.capacity_hint());

        for item in decoder.iter() {
          let (_, ent) = item?;
          let (k, v) = ent.try_into_entry()?.into();
          context.err_duplicated_map_keys(self.insert(k, v).is_some())?;
        }

        Ok(read)
      }
    }
  }
}

impl<K, KW, V, VW, S, const TAG: u32> Encode<RepeatedEntry<KW, VW, TAG>, Groto> for HashMap<K, V, S>
where
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  K: Encode<KW, Groto>,
  V: Encode<VW, Groto>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    repeated_encode::<KW, VW, _, _, TAG>(
      buf,
      self.iter(),
      || <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encoded_raw_len(self, context),
      |item, ei, ki, vi, buf| {
        MapEntry::from(item).encode_repeated::<KW, VW>(context, buf, ei, ki, vi)
      },
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    repeated_encoded_len::<KW, VW, _, _, TAG>(self.iter(), |item, ei, ki, vi| {
      MapEntry::from(item).encoded_repeated_len(context, ei, ki, vi)
    })
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encoded_raw_len(self, context)
  }
}

impl<K, KW, V, VW, S, const TAG: u32> PartialEncode<RepeatedEntry<KW, VW, TAG>, Groto>
  for HashMap<K, V, S>
where
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  K: PartialEncode<KW, Groto>,
  V: PartialEncode<VW, Groto>,
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
        MapEntry::from(item).partial_encode_repeated::<KW, VW>(context, buf, ei, ki, vi, selector)
      },
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    repeated_encoded_len::<KW, VW, _, _, TAG>(self.iter(), |item, ei, ki, vi| {
      MapEntry::from(item).partial_encoded_repeated_len(context, ei, ki, vi, selector)
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

impl<'a, K, KW, V, VW, S, RB, B, const TAG: u32>
  TryFromRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto> for HashMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, KW, RB, B, Groto> + Eq + Hash + 'a,
  K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
  V: TryFromRef<'a, RB, B, VW, Groto> + 'a,
  V::Output: Sized + Decode<'a, VW, RB, B, Groto>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_ref(
    ctx: &'a Context,
    input: <Self as State<Ref<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut map = HashMap::with_capacity_and_hasher(capacity_hint, S::default());

    try_from::<K, V, K::Output, V::Output, KW, VW, RB, B, _, _>(
      &mut map,
      iter,
      |map| ctx.err_length_mismatch(capacity_hint, map.len()),
      |map, k, v| ctx.err_duplicated_map_keys(map.insert(k, v).is_some()),
      |k| K::try_from_ref(ctx, k),
      |v| V::try_from_ref(ctx, v),
    )
    .map(|_| map)
  }
}

impl<'a, K, KW, V, VW, S, RB, B, const TAG: u32>
  TryFromPartialRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto> for HashMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, KW, RB, B, Groto> + Eq + Hash + 'a,
  K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
  V: TryFromPartialRef<'a, VW, RB, B, Groto> + 'a,
  V::Output: Sized + Decode<'a, VW, RB, B, Groto>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_partial_ref(
    ctx: &'a Context,
    input: <Self as State<PartialRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut map = HashMap::with_capacity_and_hasher(capacity_hint, S::default());

    try_from::<K, V, K::Output, V::Output, KW, VW, RB, B, _, _>(
      &mut map,
      iter,
      |map| ctx.err_length_mismatch(capacity_hint, map.len()),
      |map, k, v| ctx.err_duplicated_map_keys(map.insert(k, v).is_some()),
      |k| K::try_from_partial_ref(ctx, k),
      |v| V::try_from_partial_ref(ctx, v),
    )
    .map(|_| map)
  }
}

impl<'a, K, KW, V, VW, S, RB, B, const TAG: u32>
  PartialTryFromRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto> for HashMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, KW, RB, B, Groto> + Eq + Hash + 'a,
  <K as State<PartialRef<'a, KW, RB, B, Groto>>>::Output:
    Sized + Decode<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
  <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  V: PartialTryFromRef<'a, RB, B, VW, Groto> + 'a,
  <V as State<PartialRef<'a, VW, RB, B, Groto>>>::Output:
    Sized + Decode<'a, VW, RB, B, Groto> + Selectable<Groto, Selector = V::Selector>,
  <V as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = V::Selector>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn partial_try_from_ref(
    context: &'a Context,
    input: <Self as State<PartialRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, Error>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>>::Output: Sized,
  {
    if selector.is_empty() {
      return Ok(<DefaultPartialMapBuffer<_, _> as Buffer>::new());
    }

    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let Some(mut partial_map) =
      <DefaultPartialMapBuffer<_, _> as Buffer>::with_capacity(capacity_hint)
    else {
      return Err(Error::allocation_failed("map"));
    };

    for res in iter {
      match res {
        Ok((_, item)) => {
          if <DefaultPartialMapBuffer<_, _> as Buffer>::push(
            &mut partial_map,
            item.and_then(
              |k| K::partial_try_from_ref(context, k, selector.key()),
              |v| V::partial_try_from_ref(context, v, selector.value()),
            )?,
          )
          .is_some()
          {
            return Err(Error::capacity_exceeded("map"));
          }
        }
        Err(e) => return Err(e),
      }
    }

    Ok(partial_map)
  }
}
