use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{PartialRef, Ref, TryFromPartialRef, TryFromRef},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, PackedEntry, WireFormat,
    groto::{Context, Error, PackedMapDecoder, PartialMapBuffer, PartialMapEntry},
  },
  selection::Selector,
  state::State,
};

use super::super::{
  packed_decode, packed_encode, packed_encode_raw, packed_encoded_len, packed_encoded_raw_len,
};

impl<'a, K, V, KW, VW, RB, UB, PB> State<PartialRef<'a, RB, UB, PackedEntry<KW, VW>, Groto>>
  for PartialMapBuffer<K, V, PB>
where
  K: State<PartialRef<'a, RB, UB, KW, Groto>>,
  K::Output: Sized,
  V: State<PartialRef<'a, RB, UB, VW, Groto>>,
  V::Output: Sized,
{
  type Output = PackedMapDecoder<'a, K::Output, V::Output, RB, UB, KW, VW>;
}

impl<'a, K, V, KW, VW, RB, UB, PB> State<Ref<'a, RB, UB, PackedEntry<KW, VW>, Groto>>
  for PartialMapBuffer<K, V, PB>
where
  K: State<Ref<'a, RB, UB, KW, Groto>>,
  K::Output: Sized,
  V: State<Ref<'a, RB, UB, VW, Groto>>,
  V::Output: Sized,
{
  type Output = PackedMapDecoder<'a, K::Output, V::Output, RB, UB, KW, VW>;
}

impl<K, V, KW, VW, PB> Encode<PackedEntry<KW, VW>, Groto> for PartialMapBuffer<K, V, PB>
where
  K: Encode<KW, Groto>,
  V: Encode<VW, Groto>,
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    packed_encode_raw::<K, V, KW, VW, _, _, _, _>(
      buf,
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

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    packed_encode::<K, V, KW, VW, _, _, _, _>(
      buf,
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

impl<K, V, KW, VW, PB> PartialEncode<PackedEntry<KW, VW>, Groto> for PartialMapBuffer<K, V, PB>
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

    packed_encode_raw::<K, V, KW, VW, _, _, _, _>(
      buf,
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

impl<'a, K, KW, V, VW, RB, UB, PB> Decode1<'a, PackedEntry<KW, VW>, RB, UB, Groto>
  for PartialMapBuffer<K, V, PB>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: Decode1<'a, KW, RB, UB, Groto>,
  V: Decode1<'a, VW, RB, UB, Groto>,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
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
        PartialMapBuffer::with_capacity(cap)
          .ok_or_else(|| Error::custom("failed to create buffer with given capacity"))
      },
      |map| map.len(),
      |map, ki, vi, src| {
        let (read, item) = PartialMapEntry::<K, V>::decode_packed_entry(context, src, ki, vi)?;

        if map.push(item).is_some() {
          return Err(Error::custom("exceeded map buffer capacity"));
        }

        if context.err_on_duplicated_map_keys() {
          return Err(Error::custom("duplicated keys in map"));
        }

        Ok(read)
      },
    )
  }
}

impl<'de, K, V, RB, UB, PB, KW, VW> TryFromRef<'de, RB, UB, PackedEntry<KW, VW>, Groto>
  for PartialMapBuffer<K, V, PB>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: TryFromRef<'de, RB, UB, KW, Groto> + 'de,
  K::Output: Sized + Decode1<'de, KW, RB, UB, Groto>,
  V: TryFromRef<'de, RB, UB, VW, Groto> + 'de,
  V::Output: Sized + Decode1<'de, VW, RB, UB, Groto>,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
{
  fn try_from_ref(
    ctx: &'de Context,
    input: <Self as State<Ref<'de, RB, UB, PackedEntry<KW, VW>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'de, RB, UB, PackedEntry<KW, VW>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'de,
    UB: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    let Some(mut buffer) = Self::with_capacity(capacity_hint) else {
      return Err(Error::custom("failed to create buffer with given capacity"));
    };

    for res in input.iter() {
      let (_, ent) = res?;
      let ent = ent.and_then(|k| K::try_from_ref(ctx, k), |v| V::try_from_ref(ctx, v))?;

      if buffer.push(ent).is_some() {
        return Err(Error::custom("exceeded map buffer capacity"));
      }
    }

    if buffer.len() != capacity_hint && ctx.err_on_length_mismatch() {
      return Err(Error::custom(format!(
        "expected {capacity_hint} elements in map, but got {} elements",
        buffer.len()
      )));
    }

    Ok(buffer)
  }
}

impl<'de, K, V, RB, UB, PB, KW, VW> TryFromPartialRef<'de, RB, UB, PackedEntry<KW, VW>, Groto>
  for PartialMapBuffer<K, V, PB>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: TryFromPartialRef<'de, RB, UB, KW, Groto> + 'de,
  K::Output: Sized + Decode1<'de, KW, RB, UB, Groto>,
  V: TryFromPartialRef<'de, RB, UB, VW, Groto> + 'de,
  V::Output: Sized + Decode1<'de, VW, RB, UB, Groto>,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
{
  fn try_from_partial_ref(
    ctx: &'de Context,
    input: <Self as State<PartialRef<'de, RB, UB, PackedEntry<KW, VW>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'de, RB, UB, PackedEntry<KW, VW>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'de,
    UB: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    let Some(mut buffer) = Self::with_capacity(capacity_hint) else {
      return Err(Error::custom("failed to create buffer with given capacity"));
    };

    for res in input.iter() {
      let (_, ent) = res?;
      let ent = ent.and_then(
        |k| K::try_from_partial_ref(ctx, k),
        |v| V::try_from_partial_ref(ctx, v),
      )?;
      if buffer.push(ent).is_some() {
        return Err(Error::custom(
          "exceeded buffer capacity while pushing map entry",
        ));
      }
    }

    if buffer.len() != capacity_hint && ctx.err_on_length_mismatch() {
      return Err(Error::custom(format!(
        "expected {capacity_hint} elements in map, but got {} elements",
        buffer.len()
      )));
    }

    Ok(buffer)
  }
}
