use core::hash::{BuildHasher, Hash};

use super::{
  super::{
    DefaultPartialMapBuffer, MapEntry, packed_decode, packed_encode, packed_encode_raw,
    packed_encoded_len, packed_encoded_raw_len, try_from,
  },
  IndexMap,
};

use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{Partial, PartialRef, PartialTryFromRef, Ref, TryFromPartialRef, TryFromRef},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultMapWireFormat, Groto, PackedEntry, WireFormat,
    groto::{Context, Error, PackedMapDecoder},
  },
  selection::{Selectable, Selector},
  state::State,
};

impl<K, V, S> DefaultMapWireFormat<Groto> for IndexMap<K, V, S> {
  type Format<KM, VM>
    = PackedEntry<KM, VM>
  where
    KM: WireFormat<Groto> + 'static,
    VM: WireFormat<Groto> + 'static;
}

impl<'a, K, V, KW, VW, S, RB, B> State<PartialRef<'a, RB, B, PackedEntry<KW, VW>, Groto>>
  for IndexMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: State<PartialRef<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
  V: State<PartialRef<'a, RB, B, VW, Groto>>,
  V::Output: Sized,
  PackedEntry<KW, VW>: WireFormat<Groto> + 'a,
{
  type Output = PackedMapDecoder<'a, K::Output, V::Output, RB, B, KW, VW>;
}

impl<'a, K, KW, V, VW, S, RB, B> State<Ref<'a, RB, B, PackedEntry<KW, VW>, Groto>>
  for IndexMap<K, V, S>
where
  PackedEntry<KW, VW>: WireFormat<Groto> + 'a,
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: State<Ref<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
  V: State<Ref<'a, RB, B, VW, Groto>>,
  V::Output: Sized,
{
  type Output = PackedMapDecoder<'a, K::Output, V::Output, RB, B, KW, VW>;
}

impl<'a, K, KW, V, VW, S, RB, B> Decode1<'a, PackedEntry<KW, VW>, RB, B, Groto>
  for IndexMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  S: BuildHasher + Default,
  K: Eq + Hash + Decode1<'a, KW, RB, B, Groto>,
  V: Decode1<'a, VW, RB, B, Groto>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    packed_decode::<K, KW, V, VW, Self, RB>(
      context,
      src,
      |cap| {
        if cap == 0 {
          Ok(IndexMap::with_hasher(S::default()))
        } else {
          Ok(IndexMap::with_capacity_and_hasher(cap, S::default()))
        }
      },
      |map| map.len(),
      |map, ki, vi, src| {
        let (read, item) = MapEntry::<K, V>::decode_packed_entry(context, src, ki, vi)?;
        let (k, v) = item.into_components();

        if map.insert(k, v).is_some() && context.err_on_duplicated_map_keys() {
          return Err(Error::custom("duplicated keys in map"));
        }

        Ok(read)
      },
    )
  }
}

impl<K, KW, V, VW, S> Encode<PackedEntry<KW, VW>, Groto> for IndexMap<K, V, S>
where
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  K: Encode<KW, Groto>,
  V: Encode<VW, Groto>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    packed_encode_raw::<K, V, KW, VW, _, _, _, _>(
      buf,
      self.iter(),
      || <Self as Encode<PackedEntry<KW, VW>, Groto>>::encoded_raw_len(self, context),
      |item, ki, vi, buf| MapEntry::from(item).encode_packed::<KW, VW>(context, buf, ki, vi),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    packed_encoded_raw_len::<K, V, KW, VW, _, _>(self.iter(), |item, ki, vi| {
      MapEntry::from(item).encoded_packed_len::<KW, VW>(context, ki, vi)
    })
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    packed_encode::<K, V, KW, VW, _, _, _, _>(
      buf,
      self.len(),
      self.iter(),
      || <Self as Encode<PackedEntry<KW, VW>, Groto>>::encoded_raw_len(self, context),
      |item, ki, vi, buf| MapEntry::from(item).encode_packed::<KW, VW>(context, buf, ki, vi),
    )
  }

  fn encoded_len(&self, context: &Context) -> usize {
    packed_encoded_len(self.len(), || {
      <Self as Encode<PackedEntry<KW, VW>, Groto>>::encoded_raw_len(self, context)
    })
  }
}

impl<K, KW, V, VW, S> PartialEncode<PackedEntry<KW, VW>, Groto> for IndexMap<K, V, S>
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

    packed_encode_raw::<K, V, KW, VW, _, _, _, _>(
      buf,
      self.iter(),
      || {
        <Self as PartialEncode<PackedEntry<KW, VW>, Groto>>::partial_encoded_raw_len(
          self, context, selector,
        )
      },
      |item, ki, vi, buf| {
        MapEntry::from(item).partial_encode_packed::<KW, VW>(context, buf, ki, vi, selector)
      },
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    packed_encoded_raw_len::<K, V, KW, VW, _, _>(self.iter(), |item, ki, vi| {
      MapEntry::from(item).partial_encoded_packed_len::<KW, VW>(context, ki, vi, selector)
    })
  }

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if selector.is_empty() {
      return Ok(0);
    }

    packed_encode::<K, V, KW, VW, _, _, _, _>(
      buf,
      self.len(),
      self.iter(),
      || {
        <Self as PartialEncode<PackedEntry<KW, VW>, Groto>>::partial_encoded_raw_len(
          self, context, selector,
        )
      },
      |item, ki, vi, buf| {
        MapEntry::from(item).partial_encode_packed::<KW, VW>(context, buf, ki, vi, selector)
      },
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    packed_encoded_len::<_>(self.len(), || {
      <Self as PartialEncode<PackedEntry<KW, VW>, Groto>>::partial_encoded_raw_len(
        self, context, selector,
      )
    })
  }
}

impl<'a, K, KW, V, VW, S, RB, B> TryFromRef<'a, RB, B, PackedEntry<KW, VW>, Groto>
  for IndexMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, RB, B, KW, Groto> + Eq + Hash + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, B, Groto>,
  V: TryFromRef<'a, RB, B, VW, Groto> + 'a,
  V::Output: Sized + Decode1<'a, VW, RB, B, Groto>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_ref(
    ctx: &'a Context,
    input: <Self as State<Ref<'a, RB, B, PackedEntry<KW, VW>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, RB, B, PackedEntry<KW, VW>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut map = IndexMap::with_capacity_and_hasher(capacity_hint, S::default());

    try_from::<K, V, K::Output, V::Output, KW, VW, RB, B, _, _>(
      &mut map,
      iter,
      |map| {
        if map.is_empty() {
          return Err(Error::custom("map cannot be empty"));
        }
        if map.len() > capacity_hint && ctx.err_on_length_mismatch() {
          return Err(Error::custom(format!(
            "exceeded capacity hint of {capacity_hint} elements in map, but got {} elements",
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

impl<'a, K, KW, V, VW, S, RB, B> TryFromPartialRef<'a, RB, B, PackedEntry<KW, VW>, Groto>
  for IndexMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, RB, B, KW, Groto> + Eq + Hash + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, B, Groto>,
  V: TryFromPartialRef<'a, RB, B, VW, Groto> + 'a,
  V::Output: Sized + Decode1<'a, VW, RB, B, Groto>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_partial_ref(
    ctx: &'a Context,
    input: <Self as State<PartialRef<'a, RB, B, PackedEntry<KW, VW>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, RB, B, PackedEntry<KW, VW>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut map = IndexMap::with_capacity_and_hasher(capacity_hint, S::default());

    try_from::<K, V, K::Output, V::Output, KW, VW, RB, B, _, _>(
      &mut map,
      iter,
      |map| {
        if map.is_empty() {
          return Err(Error::custom("map cannot be empty"));
        }
        if map.len() > capacity_hint && ctx.err_on_length_mismatch() {
          return Err(Error::custom(format!(
            "exceeded capacity hint of {capacity_hint} elements in map, but got {} elements",
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

impl<'a, K, KW, V, VW, S, RB, B> PartialTryFromRef<'a, RB, B, PackedEntry<KW, VW>, Groto>
  for IndexMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, RB, B, KW, Groto> + Eq + Hash + 'a,
  <K as State<PartialRef<'a, RB, B, KW, Groto>>>::Output:
    Sized + Decode1<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
  <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  V: PartialTryFromRef<'a, RB, B, VW, Groto> + 'a,
  <V as State<PartialRef<'a, RB, B, VW, Groto>>>::Output:
    Sized + Decode1<'a, VW, RB, B, Groto> + Selectable<Groto, Selector = V::Selector>,
  <V as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = V::Selector>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn partial_try_from_ref(
    context: &'a Context,
    input: <Self as State<PartialRef<'a, RB, B, PackedEntry<KW, VW>, Groto>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, Error>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'a, RB, B, PackedEntry<KW, VW>, Groto>>>::Output: Sized,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let Some(mut partial_map) =
      <DefaultPartialMapBuffer<_, _> as Buffer>::with_capacity(capacity_hint)
    else {
      return Err(Error::custom("failed to allocate partial set buffer"));
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
            return Err(Error::custom("capacity exceeded for partial set buffer"));
          }
        }
        Err(e) => return Err(e),
      }
    }

    Ok(partial_map)
  }
}
