use super::{
  super::{DefaultPartialMapBuffer, MapEntry, repeated_encode, repeated_encoded_len, try_from},
  BTreeMap,
};

use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{Partial, PartialRef, PartialTryFromRef, Ref, TryFromPartialRef, TryFromRef},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultRepeatedEntryWireFormat, Groto, RepeatedEntry, WireFormat,
    groto::{Context, Error, RepeatedMapDecoder, RepeatedMapDecoderBuffer},
  },
  selection::{Selectable, Selector},
  state::State,
};

impl<K, V> DefaultRepeatedEntryWireFormat<Groto> for BTreeMap<K, V> {
  type Format<KM, VM, const TAG: u32>
    = RepeatedEntry<KM, VM, TAG>
  where
    KM: WireFormat<Groto> + 'static,
    VM: WireFormat<Groto> + 'static;
}

impl<'a, K, V, KW, VW, RB, B, const TAG: u32>
  State<PartialRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>> for BTreeMap<K, V>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: State<PartialRef<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
  V: State<PartialRef<'a, RB, B, VW, Groto>>,
  V::Output: Sized,
  RepeatedEntry<KW, VW, TAG>: WireFormat<Groto> + 'a,
{
  type Output = RepeatedMapDecoderBuffer<'a, K::Output, V::Output, RB, B, KW, VW, TAG>;
}

impl<'a, K, KW, V, VW, RB, B, const TAG: u32>
  State<Ref<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>> for BTreeMap<K, V>
where
  RepeatedEntry<KW, VW, TAG>: WireFormat<Groto> + 'a,
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: State<Ref<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
  V: State<Ref<'a, RB, B, VW, Groto>>,
  V::Output: Sized,
{
  type Output = RepeatedMapDecoderBuffer<'a, K::Output, V::Output, RB, B, KW, VW, TAG>;
}

impl<'a, K, KW, V, VW, RB, B, const TAG: u32> Decode1<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>
  for BTreeMap<K, V>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: Ord + Decode1<'a, KW, RB, B, Groto>,
  V: Decode1<'a, VW, RB, B, Groto>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let mut this = BTreeMap::new();
    <Self as Decode1<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>::merge_decode(
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
    let (read, decoder) = RepeatedMapDecoder::<K, V, RB, B, KW, VW, TAG>::decode(context, src)?;
    for item in decoder.iter() {
      let (_, ent) = item?;
      let (k, v) = ent.try_into_entry()?.into();
      if self.insert(k, v).is_some() && context.err_on_duplicated_map_keys() {
        return Err(Error::custom("duplicated keys in map"));
      }
    }

    Ok(read)
  }
}

impl<K, KW, V, VW, const TAG: u32> Encode<RepeatedEntry<KW, VW, TAG>, Groto> for BTreeMap<K, V>
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
      |item, ei, ki, vi, buf| MapEntry::from(item).encode_repeated(context, buf, ei, ki, vi),
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

impl<K, KW, V, VW, const TAG: u32> PartialEncode<RepeatedEntry<KW, VW, TAG>, Groto>
  for BTreeMap<K, V>
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
        MapEntry::from(item).partial_encode_repeated(context, buf, ei, ki, vi, selector)
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

impl<'a, K, KW, V, VW, RB, B, const TAG: u32>
  TryFromRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto> for BTreeMap<K, V>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, RB, B, KW, Groto> + Ord + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, B, Groto>,
  V: TryFromRef<'a, RB, B, VW, Groto> + 'a,
  V::Output: Sized + Decode1<'a, VW, RB, B, Groto>,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_ref(
    ctx: &'a Context,
    input: <Self as State<Ref<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let mut map = BTreeMap::new();
    let capacity_hint = iter.capacity_hint();

    try_from::<K, V, K::Output, V::Output, KW, VW, RB, B, _, _>(
      &mut map,
      iter,
      |map| {
        if map.len() != capacity_hint && ctx.err_on_length_mismatch() {
          return Err(Error::custom(format!(
            "expected {capacity_hint} elements in map, but got {} elements",
            map.len()
          )));
        }
        Ok(())
      },
      |map, k, v| {
        if map.insert(k, v).is_some() && ctx.err_on_duplicated_map_keys() {
          return Err(Error::custom("duplicated keys in map"));
        }
        Ok(())
      },
      |k| K::try_from_ref(ctx, k),
      |v| V::try_from_ref(ctx, v),
    )
    .map(|_| map)
  }
}

impl<'a, K, KW, V, VW, RB, B, const TAG: u32>
  TryFromPartialRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto> for BTreeMap<K, V>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, RB, B, KW, Groto> + Ord + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, B, Groto>,
  V: TryFromPartialRef<'a, RB, B, VW, Groto> + 'a,
  V::Output: Sized + Decode1<'a, VW, RB, B, Groto>,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_partial_ref(
    ctx: &'a Context,
    input: <Self as State<PartialRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let mut map = BTreeMap::new();
    let capacity_hint = iter.capacity_hint();

    try_from::<K, V, K::Output, V::Output, KW, VW, RB, B, _, _>(
      &mut map,
      iter,
      |map| {
        if map.len() != capacity_hint && ctx.err_on_length_mismatch() {
          return Err(Error::custom(format!(
            "expected {capacity_hint} elements in map, but got {} elements",
            map.len()
          )));
        }
        Ok(())
      },
      |map, k, v| {
        if map.insert(k, v).is_some() && ctx.err_on_duplicated_map_keys() {
          return Err(Error::custom("duplicated keys in map"));
        }
        Ok(())
      },
      |k| K::try_from_partial_ref(ctx, k),
      |v| V::try_from_partial_ref(ctx, v),
    )
    .map(|_| map)
  }
}

impl<'a, K, KW, V, VW, RB, B, const TAG: u32>
  PartialTryFromRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto> for BTreeMap<K, V>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, RB, B, KW, Groto> + Ord + 'a,
  <K as State<PartialRef<'a, RB, B, KW, Groto>>>::Output:
    Sized + Decode1<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
  <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  V: PartialTryFromRef<'a, RB, B, VW, Groto> + 'a,
  <V as State<PartialRef<'a, RB, B, VW, Groto>>>::Output:
    Sized + Decode1<'a, VW, RB, B, Groto> + Selectable<Groto, Selector = V::Selector>,
  <V as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = V::Selector>,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn partial_try_from_ref(
    context: &'a Context,
    input: <Self as State<PartialRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, Error>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output: Sized,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let Some(mut partial_map) =
      <DefaultPartialMapBuffer<_, _> as Buffer>::with_capacity(capacity_hint)
    else {
      return Err(Error::custom("failed to allocate partial map buffer"));
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
            return Err(Error::custom("capacity exceeded for partial map buffer"));
          }
        }
        Err(e) => return Err(e),
      }
    }

    Ok(partial_map)
  }
}
