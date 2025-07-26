use core::hash::{BuildHasher, Hash};

use super::{
  super::{
    DefaultPartialMapBuffer, DecomposableMapEntry, packed_decode, packed_encode, packed_encode_raw,
    packed_encoded_len, packed_encoded_raw_len, try_from,
  },
  IndexMap,
};

use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{PartialTryFromRef, TryFromPartialRef, TryFromRef},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultMapWireFormat, Groto, PackedEntry, WireFormat,
    groto::{Context, Error, PackedMapDecoder},
  },
  selection::{Selectable, Selector},
  state::{Partial, PartialRef, Ref, State},
};

impl<K, V, S> DefaultMapWireFormat<Groto> for IndexMap<K, V, S> {
  type Format<KM, VM>
    = PackedEntry<KM, VM>
  where
    KM: WireFormat<Groto> + 'static,
    VM: WireFormat<Groto> + 'static;
}

impl<'a, K, V, KW, VW, S, RB, B> State<PartialRef<'a, PackedEntry<KW, VW>, RB, B, Groto>>
  for IndexMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: State<PartialRef<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
  V: State<PartialRef<'a, VW, RB, B, Groto>>,
  V::Output: Sized,
  PackedEntry<KW, VW>: WireFormat<Groto> + 'a,
{
  type Output = PackedMapDecoder<'a, K::Output, V::Output, RB, B, KW, VW>;
}

impl<'a, K, KW, V, VW, S, RB, B> State<Ref<'a, PackedEntry<KW, VW>, RB, B, Groto>>
  for IndexMap<K, V, S>
where
  PackedEntry<KW, VW>: WireFormat<Groto> + 'a,
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: State<Ref<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
  V: State<Ref<'a, VW, RB, B, Groto>>,
  V::Output: Sized,
{
  type Output = PackedMapDecoder<'a, K::Output, V::Output, RB, B, KW, VW>;
}

impl<'a, K, KW, V, VW, S, RB, B> Decode<'a, PackedEntry<KW, VW>, RB, B, Groto> for IndexMap<K, V, S>
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
        let (read, item) = DecomposableMapEntry::<K, V>::decode_packed_entry(context, src, ki, vi)?;
        let (k, v) = item.into_components();
        context.err_duplicated_map_keys(!map.insert(k, v).is_some())?;

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
      |item, ki, vi, buf| DecomposableMapEntry::from(item).encode_packed::<KW, VW>(context, buf, ki, vi),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    packed_encoded_raw_len::<K, V, KW, VW, _, _>(self.iter(), |item, ki, vi| {
      DecomposableMapEntry::from(item).encoded_packed_len::<KW, VW>(context, ki, vi)
    })
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    packed_encode::<K, V, KW, VW, _, _, _, _>(
      buf,
      self.len(),
      self.iter(),
      || <Self as Encode<PackedEntry<KW, VW>, Groto>>::encoded_raw_len(self, context),
      |item, ki, vi, buf| DecomposableMapEntry::from(item).encode_packed::<KW, VW>(context, buf, ki, vi),
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
        DecomposableMapEntry::from(item).partial_encode_packed::<KW, VW>(context, buf, ki, vi, selector)
      },
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    packed_encoded_raw_len::<K, V, KW, VW, _, _>(self.iter(), |item, ki, vi| {
      DecomposableMapEntry::from(item).partial_encoded_packed_len::<KW, VW>(context, ki, vi, selector)
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
        DecomposableMapEntry::from(item).partial_encode_packed::<KW, VW>(context, buf, ki, vi, selector)
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

// impl<'a, K, KW, V, VW, S, RB, B> TryFromRef<'a, PackedEntry<KW, VW>, RB, B, Groto>
//   for IndexMap<K, V, S>
// where
//   KW: WireFormat<Groto> + 'a,
//   VW: WireFormat<Groto> + 'a,
//   K: TryFromRef<'a, KW, RB, B, Groto> + Eq + Hash + 'a,
//   K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
//   V: TryFromRef<'a, VW, RB, B, Groto> + 'a,
//   V::Output: Sized + Decode<'a, VW, RB, B, Groto>,
//   S: BuildHasher + Default,
//   RB: ReadBuf + 'a,
//   B: UnknownBuffer<RB, Groto> + 'a,
// {
//   fn try_from_ref(
//     ctx: &'a Context,
//     input: <Self as State<Ref<'a, PackedEntry<KW, VW>, RB, B, Groto>>>::Output,
//   ) -> Result<Self, Error>
//   where
//     Self: Sized,
//     <Self as State<Ref<'a, PackedEntry<KW, VW>, RB, B, Groto>>>::Output: Sized,
//     RB: ReadBuf + 'a,
//     B: UnknownBuffer<RB, Groto>,
//   {
//     let iter = input.iter();
//     let capacity_hint = iter.capacity_hint();
//     let mut map = IndexMap::with_capacity_and_hasher(capacity_hint, S::default());

//     try_from::<K, V, K::Output, V::Output, KW, VW, RB, B, _, _>(
//       &mut map,
//       iter,
//       |map| ctx.err_length_mismatch(capacity_hint, map.len()),
//       |map, k, v| ctx.err_duplicated_map_keys(map.insert(k, v).is_some()),
//       |k| K::try_from_ref(ctx, k),
//       |v| V::try_from_ref(ctx, v),
//     )
//     .map(|_| map)
//   }
// }

// impl<'a, K, KW, V, VW, S, RB, B> TryFromPartialRef<'a, PackedEntry<KW, VW>, RB, B, Groto>
//   for IndexMap<K, V, S>
// where
//   KW: WireFormat<Groto> + 'a,
//   VW: WireFormat<Groto> + 'a,
//   K: TryFromPartialRef<'a, KW, RB, B, Groto> + Eq + Hash + 'a,
//   K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
//   V: TryFromPartialRef<'a, VW, RB, B, Groto> + 'a,
//   V::Output: Sized + Decode<'a, VW, RB, B, Groto>,
//   S: BuildHasher + Default,
//   RB: ReadBuf + 'a,
//   B: UnknownBuffer<RB, Groto> + 'a,
// {
//   fn try_from_partial_ref(
//     ctx: &'a Context,
//     input: <Self as State<PartialRef<'a, PackedEntry<KW, VW>, RB, B, Groto>>>::Output,
//   ) -> Result<Self, Error>
//   where
//     Self: Sized,
//     <Self as State<PartialRef<'a, PackedEntry<KW, VW>, RB, B, Groto>>>::Output: Sized,
//     RB: ReadBuf + 'a,
//     B: UnknownBuffer<RB, Groto>,
//   {
//     let iter = input.iter();
//     let capacity_hint = iter.capacity_hint();
//     let mut map = IndexMap::with_capacity_and_hasher(capacity_hint, S::default());

//     try_from::<K, V, K::Output, V::Output, KW, VW, RB, B, _, _>(
//       &mut map,
//       iter,
//       |map| ctx.err_length_mismatch(capacity_hint, map.len()),
//       |map, k, v| ctx.err_duplicated_map_keys(map.insert(k, v).is_some()),
//       |k| K::try_from_partial_ref(ctx, k),
//       |v| V::try_from_partial_ref(ctx, v),
//     )
//     .map(|_| map)
//   }
// }

// impl<'a, K, KW, V, VW, S, RB, B> PartialTryFromRef<'a, PackedEntry<KW, VW>, RB, B, Groto>
//   for IndexMap<K, V, S>
// where
//   KW: WireFormat<Groto> + 'a,
//   VW: WireFormat<Groto> + 'a,
//   K: PartialTryFromRef<'a, KW, RB, B, Groto> + Eq + Hash + 'a,
//   <K as State<PartialRef<'a, KW, RB, B, Groto>>>::Output:
//     Sized + Decode<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
//   <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
//   V: PartialTryFromRef<'a, VW, RB, B, Groto> + 'a,
//   <V as State<PartialRef<'a, VW, RB, B, Groto>>>::Output:
//     Sized + Decode<'a, VW, RB, B, Groto> + Selectable<Groto, Selector = V::Selector>,
//   <V as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = V::Selector>,
//   S: BuildHasher + Default,
//   RB: ReadBuf + 'a,
//   B: UnknownBuffer<RB, Groto> + 'a,
// {
//   fn partial_try_from_ref(
//     context: &'a Context,
//     input: <Self as State<PartialRef<'a, PackedEntry<KW, VW>, RB, B, Groto>>>::Output,
//     selector: &Self::Selector,
//   ) -> Result<<Self as State<Partial<Groto>>>::Output, Error>
//   where
//     <Self as State<Partial<Groto>>>::Output: Sized,
//     <Self as State<PartialRef<'a, PackedEntry<KW, VW>, RB, B, Groto>>>::Output: Sized,
//   {
//     if selector.is_empty() {
//       return Ok(<DefaultPartialMapBuffer<_, _> as Buffer>::new());
//     }

//     let iter = input.iter();
//     let capacity_hint = iter.capacity_hint();
//     let Some(mut partial_map) =
//       <DefaultPartialMapBuffer<_, _> as Buffer>::with_capacity(capacity_hint)
//     else {
//       return Err(Error::allocation_failed("map"));
//     };

//     for res in iter {
//       match res {
//         Ok((_, item)) => {
//           if <DefaultPartialMapBuffer<_, _> as Buffer>::push(
//             &mut partial_map,
//             item.and_then(
//               |k| K::partial_try_from_ref(context, k, selector.key()),
//               |v| V::partial_try_from_ref(context, v, selector.value()),
//             )?,
//           )
//           .is_some()
//           {
//             return Err(Error::capacity_exceeded("map"));
//           }
//         }
//         Err(e) => return Err(e),
//       }
//     }

//     Ok(partial_map)
//   }
// }
