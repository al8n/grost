use super::{
  super::{MapEntry, repeated_encode, repeated_encoded_len},
  BTreeMap,
};

use crate::{
  buffer::{Buf, BufMut, UnknownBuffer},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultRepeatedEntryWireFormat, Groto, RepeatedEntry, WireFormat,
    groto::{Context, Error, RepeatedMapDecoder, RepeatedMapDecoderBuffer},
  },
  selection::Selector,
  state::{PartialRef, Ref, State},
};

impl<K, V> DefaultRepeatedEntryWireFormat<Groto> for BTreeMap<K, V> {
  type Format<KM, VM, const TAG: u32>
    = RepeatedEntry<KM, VM, TAG>
  where
    KM: WireFormat<Groto> + 'static,
    VM: WireFormat<Groto> + 'static;
}

impl<'a, K, V, KW, VW, RB, B, const TAG: u32>
  State<PartialRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>> for BTreeMap<K, V>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: State<Ref<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
  V: State<PartialRef<'a, VW, RB, B, Groto>>,
  V::Output: Sized,
  RepeatedEntry<KW, VW, TAG>: WireFormat<Groto> + 'a,
{
  type Output = RepeatedMapDecoderBuffer<'a, K::Output, V::Output, RB, B, KW, VW, TAG>;
}

impl<'a, K, KW, V, VW, RB, B, const TAG: u32>
  State<Ref<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>> for BTreeMap<K, V>
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

impl<'a, K, KW, V, VW, RB, B, const TAG: u32> Decode<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>
  for BTreeMap<K, V>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: Ord + Decode<'a, KW, RB, B, Groto>,
  V: Decode<'a, VW, RB, B, Groto>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: Buf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let mut this = BTreeMap::new();
    <Self as Decode<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>::merge_decode(
      &mut this, context, src,
    )
    .map(|size| (size, this))
  }

  fn merge_decode(&mut self, context: &'a Context, src: RB) -> Result<usize, Error>
  where
    Self: Sized + 'a,
    RB: Buf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let (read, decoder) = RepeatedMapDecoder::<K, V, RB, B, KW, VW, TAG>::decode(context, src)?;
    for item in decoder.iter() {
      let (_, ent) = item?;
      let (k, v) = ent.try_into_entry()?.into();
      context.err_duplicated_map_keys(self.insert(k, v).is_some())?;
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
  fn encode_raw<WB>(&self, context: &Context, buf: impl Into<WriteBuf<WB>>) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    repeated_encode::<KW, VW, _, _, TAG>(
      buf.buffer_mut(),
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

  fn encode<WB>(&self, context: &Context, buf: impl Into<WriteBuf<WB>>) -> Result<usize, Error>
  where
    WB: BufMut,
  {
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
  K: Encode<KW, Groto>,
  V: PartialEncode<VW, Groto>,
{
  fn partial_encode_raw<WB>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: BufMut,
  {
    if selector.is_empty() {
      return Ok(0);
    }

    repeated_encode::<KW, VW, _, _, TAG>(
      buf.buffer_mut(),
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

  fn partial_encode<WB>(
    &self,
    context: &Context,
    buf: impl Into<WriteBuf<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: BufMut,
  {
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

// impl<'a, K, KW, V, VW, RB, B, const TAG: u32>
//   TryFromRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto> for BTreeMap<K, V>
// where
//   KW: WireFormat<Groto> + 'a,
//   VW: WireFormat<Groto> + 'a,
//   K: TryFromRef<'a, KW, RB, B, Groto> + Ord + 'a,
//   K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
//   V: TryFromRef<'a, VW, RB, B, Groto> + 'a,
//   V::Output: Sized + Decode<'a, VW, RB, B, Groto>,
//   RB: Buf + 'a,
//   B: UnknownBuffer<RB, Groto> + 'a,
// {
//   fn try_from_ref(
//     ctx: &'a Context,
//     input: <Self as State<Ref<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>>::Output,
//   ) -> Result<Self, Error>
//   where
//     Self: Sized,
//     <Self as State<Ref<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>>::Output: Sized,
//     RB: Buf + 'a,
//     B: UnknownBuffer<RB, Groto>,
//   {
//     let iter = input.iter();
//     let mut map = BTreeMap::new();
//     let capacity_hint = iter.capacity_hint();

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

// impl<'a, K, KW, V, VW, RB, B, const TAG: u32>
//   TryFromPartialRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto> for BTreeMap<K, V>
// where
//   KW: WireFormat<Groto> + 'a,
//   VW: WireFormat<Groto> + 'a,
//   K: TryFromPartialRef<'a, KW, RB, B, Groto> + Ord + 'a,
//   K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
//   V: TryFromPartialRef<'a, VW, RB, B, Groto> + 'a,
//   V::Output: Sized + Decode<'a, VW, RB, B, Groto>,
//   RB: Buf + 'a,
//   B: UnknownBuffer<RB, Groto> + 'a,
// {
//   fn try_from_partial_ref(
//     ctx: &'a Context,
//     input: <Self as State<PartialRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>>::Output,
//   ) -> Result<Self, Error>
//   where
//     Self: Sized,
//     <Self as State<PartialRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>>::Output: Sized,
//     RB: Buf + 'a,
//     B: UnknownBuffer<RB, Groto>,
//   {
//     let iter = input.iter();
//     let mut map = BTreeMap::new();
//     let capacity_hint = iter.capacity_hint();

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

// impl<'a, K, KW, V, VW, RB, B, const TAG: u32>
//   PartialTryFromRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto> for BTreeMap<K, V>
// where
//   KW: WireFormat<Groto> + 'a,
//   VW: WireFormat<Groto> + 'a,
//   K: PartialTryFromRef<'a, KW, RB, B, Groto> + Ord + 'a,
//   <K as State<PartialRef<'a, KW, RB, B, Groto>>>::Output:
//     Sized + Decode<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
//   <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
//   V: PartialTryFromRef<'a, VW, RB, B, Groto> + 'a,
//   <V as State<PartialRef<'a, VW, RB, B, Groto>>>::Output:
//     Sized + Decode<'a, VW, RB, B, Groto> + Selectable<Groto, Selector = V::Selector>,
//   <V as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = V::Selector>,
//   RB: Buf + 'a,
//   B: UnknownBuffer<RB, Groto> + 'a,
// {
//   fn partial_try_from_ref(
//     context: &'a Context,
//     input: <Self as State<PartialRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>>::Output,
//     selector: &Self::Selector,
//   ) -> Result<<Self as State<Partial<Groto>>>::Output, Error>
//   where
//     <Self as State<Partial<Groto>>>::Output: Sized,
//     <Self as State<PartialRef<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>>::Output: Sized,
//   {
//     if selector.is_empty() {
//       return Ok(<PartialMapBuffer<_, _> as Buffer>::new());
//     }

//     let iter = input.iter();
//     let capacity_hint = iter.capacity_hint();
//     let Some(mut partial_map) =
//       <PartialMapBuffer<_, _> as Buffer>::with_capacity(capacity_hint)
//     else {
//       return Err(Error::allocation_failed("map"));
//     };

//     for res in iter {
//       match res {
//         Ok((_, item)) => {
//           if <PartialMapBuffer<_, _> as Buffer>::push(
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
