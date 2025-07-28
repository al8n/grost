use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer, WriteBuf},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, PackedEntry, WireFormat,
    groto::{
      Context, DecomposablePartialMapBuffer, Error, PackedMapDecoder, PartialDecomposableMapEntry,
    },
  },
  selection::Selector,
  state::{PartialRef, Ref, State},
};

use super::super::super::{
  packed_decode, packed_encode, packed_encode_raw, packed_encoded_len, packed_encoded_raw_len,
};

impl<'a, K, V, KW, VW, RB, UB, PB> State<PartialRef<'a, PackedEntry<KW, VW>, RB, UB, Groto>>
  for DecomposablePartialMapBuffer<K, V, PB>
where
  K: State<PartialRef<'a, KW, RB, UB, Groto>>,
  K::Output: Sized,
  V: State<PartialRef<'a, VW, RB, UB, Groto>>,
  V::Output: Sized,
{
  type Output = PackedMapDecoder<'a, K::Output, V::Output, RB, UB, KW, VW>;
}

impl<'a, K, V, KW, VW, RB, UB, PB> State<Ref<'a, PackedEntry<KW, VW>, RB, UB, Groto>>
  for DecomposablePartialMapBuffer<K, V, PB>
where
  K: State<Ref<'a, KW, RB, UB, Groto>>,
  K::Output: Sized,
  V: State<Ref<'a, VW, RB, UB, Groto>>,
  V::Output: Sized,
{
  type Output = PackedMapDecoder<'a, K::Output, V::Output, RB, UB, KW, VW>;
}

impl<K, V, KW, VW, PB> Encode<PackedEntry<KW, VW>, Groto> for DecomposablePartialMapBuffer<K, V, PB>
where
  K: Encode<KW, Groto>,
  V: Encode<VW, Groto>,
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  PB: Buffer<Item = PartialDecomposableMapEntry<K, V>>,
{
  fn encode_raw<WB>(&self, context: &Context, buf: &mut WB) -> Result<usize, Error>
  where
    WB: WriteBuf + ?Sized,
  {
    packed_encode_raw::<K, V, KW, VW, _, _, _, _>(
      buf.as_mut_slice(),
      self.iter(),
      || <Self as Encode<PackedEntry<KW, VW>, Groto>>::encoded_raw_len(self, context),
      |item, ki, vi, buf| item.encode_packed::<KW, VW>(context, buf, ki, vi),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    packed_encoded_raw_len::<K, V, KW, VW, _, _>(self.iter(), |item, ki, vi| {
      item.encoded_packed_len::<KW, VW>(context, ki, vi)
    })
  }

  fn encode<B>(&self, context: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: WriteBuf + ?Sized,
  {
    packed_encode::<K, V, KW, VW, _, _, _, _>(
      buf.as_mut_slice(),
      self.len(),
      self.iter(),
      || <Self as Encode<PackedEntry<KW, VW>, Groto>>::encoded_raw_len(self, context),
      |item, ki, vi, buf| item.encode_packed::<KW, VW>(context, buf, ki, vi),
    )
  }

  fn encoded_len(&self, context: &Context) -> usize {
    packed_encoded_len(self.len(), || {
      <Self as Encode<PackedEntry<KW, VW>, Groto>>::encoded_raw_len(self, context)
    })
  }
}

impl<K, V, KW, VW, PB> PartialEncode<PackedEntry<KW, VW>, Groto>
  for DecomposablePartialMapBuffer<K, V, PB>
where
  K: PartialEncode<KW, Groto>,
  V: PartialEncode<VW, Groto>,
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  PB: Buffer<Item = PartialDecomposableMapEntry<K, V>>,
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
    if selector.is_empty() {
      return Ok(0);
    }

    packed_encode_raw::<K, V, KW, VW, _, _, _, _>(
      buf.as_mut_slice(),
      self.iter(),
      || {
        <Self as PartialEncode<PackedEntry<KW, VW>, Groto>>::partial_encoded_raw_len(
          self, context, selector,
        )
      },
      |item, ki, vi, buf| item.partial_encode_packed::<KW, VW>(context, buf, ki, vi, selector),
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    packed_encoded_raw_len::<K, V, KW, VW, _, _>(self.iter(), |item, ki, vi| {
      item.partial_encoded_packed_len::<KW, VW>(context, ki, vi, selector)
    })
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
    if selector.is_empty() {
      return Ok(0);
    }

    packed_encode::<K, V, KW, VW, _, _, _, _>(
      buf.as_mut_slice(),
      self.len(),
      self.iter(),
      || {
        <Self as PartialEncode<PackedEntry<KW, VW>, Groto>>::partial_encoded_raw_len(
          self, context, selector,
        )
      },
      |item, ki, vi, buf| item.partial_encode_packed::<KW, VW>(context, buf, ki, vi, selector),
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

impl<'a, K, KW, V, VW, RB, UB, PB> Decode<'a, PackedEntry<KW, VW>, RB, UB, Groto>
  for DecomposablePartialMapBuffer<K, V, PB>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: Decode<'a, KW, RB, UB, Groto>,
  V: Decode<'a, VW, RB, UB, Groto>,
  PB: Buffer<Item = PartialDecomposableMapEntry<K, V>>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    UB: UnknownBuffer<RB, Groto> + 'a,
  {
    packed_decode::<K, KW, V, VW, Self, RB>(
      context,
      src,
      |cap| {
        DecomposablePartialMapBuffer::with_capacity(cap)
          .ok_or_else(|| Error::allocation_failed("map"))
      },
      |map| map.len(),
      |map, ki, vi, src| {
        let (read, item) =
          PartialDecomposableMapEntry::<K, V>::decode_packed_entry(context, src, ki, vi)?;

        if map.push(item).is_some() {
          return Err(Error::capacity_exceeded("map"));
        }

        Ok(read)
      },
    )
  }
}

// impl<'de, K, V, RB, UB, PB, KW, VW> TryFromRef<'de, PackedEntry<KW, VW>, RB, UB, Groto>
//   for DecomposablePartialMapBuffer<K, V, PB>
// where
//   KW: WireFormat<Groto> + 'de,
//   VW: WireFormat<Groto> + 'de,
//   K: TryFromRef<'de, KW, RB, UB, Groto> + 'de,
//   K::Output: Sized + Decode<'de, KW, RB, UB, Groto>,
//   V: TryFromRef<'de, VW, RB, UB, Groto> + 'de,
//   V::Output: Sized + Decode<'de, VW, RB, UB, Groto>,
//   UB: UnknownBuffer<RB, Groto> + 'de,
//   RB: ReadBuf + 'de,
//   PB: Buffer<Item = PartialDecomposableMapEntry<K, V>>,
// {
//   fn try_from_ref(
//     ctx: &'de Context,
//     input: <Self as State<Ref<'de, PackedEntry<KW, VW>, RB, UB, Groto>>>::Output,
//   ) -> Result<Self, Error>
//   where
//     Self: Sized,
//     <Self as State<Ref<'de, PackedEntry<KW, VW>, RB, UB, Groto>>>::Output: Sized,
//     RB: ReadBuf + 'de,
//     UB: UnknownBuffer<RB, Groto>,
//   {
//     let capacity_hint = input.capacity_hint();
//     let Some(mut buffer) = Self::with_capacity(capacity_hint) else {
//       return Err(Error::allocation_failed("map"));
//     };

//     for res in input.iter() {
//       let (_, ent) = res?;
//       let ent = ent.and_then(|k| K::try_from_ref(ctx, k), |v| V::try_from_ref(ctx, v))?;

//       if buffer.push(ent).is_some() {
//         return Err(Error::capacity_exceeded("map"));
//       }
//     }

//     ctx
//       .err_length_mismatch(capacity_hint, buffer.len())
//       .map(|_| buffer)
//   }
// }

// impl<'de, K, V, RB, UB, PB, KW, VW> TryFromPartialRef<'de, PackedEntry<KW, VW>, RB, UB, Groto>
//   for DecomposablePartialMapBuffer<K, V, PB>
// where
//   KW: WireFormat<Groto> + 'de,
//   VW: WireFormat<Groto> + 'de,
//   K: TryFromPartialRef<'de, KW, RB, UB, Groto> + 'de,
//   K::Output: Sized + Decode<'de, KW, RB, UB, Groto>,
//   V: TryFromPartialRef<'de, VW, RB, UB, Groto> + 'de,
//   V::Output: Sized + Decode<'de, VW, RB, UB, Groto>,
//   UB: UnknownBuffer<RB, Groto> + 'de,
//   RB: ReadBuf + 'de,
//   PB: Buffer<Item = PartialDecomposableMapEntry<K, V>>,
// {
//   fn try_from_partial_ref(
//     ctx: &'de Context,
//     input: <Self as State<PartialRef<'de, PackedEntry<KW, VW>, RB, UB, Groto>>>::Output,
//   ) -> Result<Self, Error>
//   where
//     Self: Sized,
//     <Self as State<PartialRef<'de, PackedEntry<KW, VW>, RB, UB, Groto>>>::Output: Sized,
//     RB: ReadBuf + 'de,
//     UB: UnknownBuffer<RB, Groto>,
//   {
//     let capacity_hint = input.capacity_hint();
//     let Some(mut buffer) = Self::with_capacity(capacity_hint) else {
//       return Err(Error::allocation_failed("map"));
//     };

//     for res in input.iter() {
//       let (_, ent) = res?;
//       let ent = ent.and_then(
//         |k| K::try_from_partial_ref(ctx, k),
//         |v| V::try_from_partial_ref(ctx, v),
//       )?;
//       if buffer.push(ent).is_some() {
//         return Err(Error::capacity_exceeded("map"));
//       }
//     }

//     ctx
//       .err_length_mismatch(capacity_hint, buffer.len())
//       .map(|_| buffer)
//   }
// }
