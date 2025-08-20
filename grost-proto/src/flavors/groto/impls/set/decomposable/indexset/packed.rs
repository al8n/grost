use indexmap_2::IndexSet;

use core::hash::{BuildHasher, Hash};

use crate::{
  buffer::{Buffer, Chunk, ChunkMut, UnknownBuffer},
  convert::{PartialTryFromRef, TryFromPartialRef, TryFromRef},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultSetWireFormat, Groto, Packed, WireFormat,
    groto::{Context, DecodeError, EncodeError, PackedSetDecoder},
  },
  selection::{Selectable, Selector},
  state::{Partial, PartialRef, Ref, State},
  utils::Decomposable,
};

use super::super::super::{
  super::{
    packed_decode, packed_encode, packed_encode_raw, packed_encoded_len, packed_encoded_raw_len,
    try_from,
  },
  DefaultPartialSetBuffer,
};

impl<K, S> DefaultSetWireFormat<Groto> for Decomposable<IndexSet<K, S>> {
  type Format<KM>
    = Packed<KM>
  where
    KM: WireFormat<Groto> + 'static;
}

impl<'a, K, KW, S, RB, B> State<PartialRef<'a, Packed<KW>, RB, B, Groto>>
  for Decomposable<IndexSet<K, S>>
where
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: State<PartialRef<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
{
  type Output = PackedSetDecoder<'a, K::Output, RB, B, KW>;
}

impl<'a, K, KW, S, RB, B> State<Ref<'a, Packed<KW>, RB, B, Groto>> for Decomposable<IndexSet<K, S>>
where
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: State<Ref<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
{
  type Output = PackedSetDecoder<'a, K::Output, RB, B, KW>;
}

impl<'a, K, KW, S, RB, B> Decode<'a, Packed<KW>, RB, B, Groto> for Decomposable<IndexSet<K, S>>
where
  KW: WireFormat<Groto> + 'a,
  S: BuildHasher + Default,
  K: Eq + Hash + Decode<'a, KW, RB, B, Groto>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'a,
    RB: Chunk + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    packed_decode::<K, KW, IndexSet<_, _>, RB>(
      context,
      src,
      |cap| {
        if cap == 0 {
          Ok(IndexSet::with_hasher(S::default()))
        } else {
          Ok(IndexSet::with_capacity_and_hasher(cap, S::default()))
        }
      },
      IndexSet::len,
      |set, src| {
        let (read, item) = K::decode(context, src)?;
        context.err_duplicated_set_keys(!set.insert(item))?;

        Ok(read)
      },
    )
    .map(|(read, set)| (read, set.into()))
  }
}

impl<K, KW, S> Encode<Packed<KW>, Groto> for Decomposable<IndexSet<K, S>>
where
  KW: WireFormat<Groto>,
  K: Encode<KW, Groto>,
{
  fn encode_raw<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
  ) -> Result<usize, EncodeError>
  where
    WB: ChunkMut,
  {
    packed_encode_raw::<K, _, _, _>(
      buf.buffer_mut(),
      self.0.iter(),
      || <Self as Encode<Packed<KW>, Groto>>::encoded_raw_len(self, context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    packed_encoded_raw_len::<K, KW, _, _>(self.0.len(), self.0.iter(), |item| {
      item.encoded_len(context)
    })
  }

  fn encode<B>(&self, context: &Context, buf: impl Into<ChunkWriter<B>>) -> Result<usize, Error>
  where
    B: crate::buffer::ChunkMut + ?Sized,
  {
    packed_encode::<K, _, _, _>(
      buf.buffer_mut(),
      self.0.len(),
      self.0.iter(),
      || <Self as Encode<Packed<KW>, Groto>>::encoded_raw_len(self, context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_len(&self, context: &Context) -> usize {
    packed_encoded_len::<_>(self.0.len(), || {
      <Self as Encode<Packed<KW>, Groto>>::encoded_raw_len(self, context)
    })
  }
}

impl<K, KW, S> PartialEncode<Packed<KW>, Groto> for Decomposable<IndexSet<K, S>>
where
  KW: WireFormat<Groto>,
  K: PartialEncode<KW, Groto>,
{
  fn partial_encode_raw<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: ChunkMut,
  {
    if selector.is_empty() {
      return Ok(0);
    }

    packed_encode_raw::<K, _, _, _>(
      buf.buffer_mut(),
      self.0.iter(),
      || {
        <Self as PartialEncode<Packed<KW>, Groto>>::partial_encoded_raw_len(self, context, selector)
      },
      |item, buf| item.partial_encode(context, buf, selector),
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    packed_encoded_raw_len::<K, KW, _, _>(self.0.len(), self.0.iter(), |item| {
      item.partial_encoded_len(context, selector)
    })
  }

  fn partial_encode<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: ChunkMut,
  {
    if selector.is_empty() {
      return Ok(0);
    }

    packed_encode::<K, _, _, _>(
      buf.buffer_mut(),
      self.0.len(),
      self.0.iter(),
      || {
        <Self as PartialEncode<Packed<KW>, Groto>>::partial_encoded_raw_len(self, context, selector)
      },
      |item, buf| item.partial_encode(context, buf, selector),
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    packed_encoded_len::<_>(self.0.len(), || {
      <Self as PartialEncode<Packed<KW>, Groto>>::partial_encoded_raw_len(self, context, selector)
    })
  }
}

impl<'a, K, KW, S, RB, B> TryFromRef<'a, Packed<KW>, RB, B, Groto> for Decomposable<IndexSet<K, S>>
where
  KW: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, KW, RB, B, Groto> + Eq + Hash + 'a,
  K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
  S: BuildHasher + Default,
  RB: Chunk + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_ref(
    ctx: &'a Context,
    input: <Self as State<Ref<'a, Packed<KW>, RB, B, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, Packed<KW>, RB, B, Groto>>>::Output: Sized,
    RB: Chunk + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut set = IndexSet::with_capacity_and_hasher(capacity_hint, S::default());

    try_from::<K, K::Output, KW, RB, B, _, _>(
      &mut set,
      iter,
      |set| ctx.err_length_mismatch(capacity_hint, set.len()),
      |set, item| ctx.err_duplicated_set_keys(!set.insert(item)),
      |item| K::try_from_ref(ctx, item),
    )
    .map(|_| set.into())
  }
}

impl<'a, K, KW, S, RB, B> TryFromPartialRef<'a, Packed<KW>, RB, B, Groto>
  for Decomposable<IndexSet<K, S>>
where
  KW: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, KW, RB, B, Groto> + Eq + Hash + 'a,
  K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
  S: BuildHasher + Default,
  RB: Chunk + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_partial_ref(
    ctx: &'a Context,
    input: <Self as State<PartialRef<'a, Packed<KW>, RB, B, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, Packed<KW>, RB, B, Groto>>>::Output: Sized,
    RB: Chunk + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut set = IndexSet::with_capacity_and_hasher(capacity_hint, S::default());

    try_from::<K, K::Output, KW, RB, B, _, _>(
      &mut set,
      iter,
      |set| ctx.err_length_mismatch(capacity_hint, set.len()),
      |set, item| ctx.err_duplicated_set_keys(!set.insert(item)),
      |item| K::try_from_partial_ref(ctx, item),
    )
    .map(|_| set.into())
  }
}

impl<'a, K, KW, S, RB, B> PartialTryFromRef<'a, Packed<KW>, RB, B, Groto>
  for Decomposable<IndexSet<K, S>>
where
  KW: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, KW, RB, B, Groto> + Eq + Hash + 'a,
  <K as State<PartialRef<'a, KW, RB, B, Groto>>>::Output:
    Sized + Decode<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
  <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  S: BuildHasher + Default,
  RB: Chunk + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn partial_try_from_ref(
    context: &'a Context,
    input: <Self as State<PartialRef<'a, Packed<KW>, RB, B, Groto>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, Error>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'a, Packed<KW>, RB, B, Groto>>>::Output: Sized,
  {
    if selector.is_empty() {
      return Ok(<DefaultPartialSetBuffer<_> as Buffer>::new());
    }

    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let Some(mut partial_set) =
      <DefaultPartialSetBuffer<_> as Buffer>::with_capacity(capacity_hint)
    else {
      return Err(Error::allocation_failed("set"));
    };

    try_from::<_, _, KW, RB, B, _, _>(
      &mut partial_set,
      iter,
      |set| context.err_length_mismatch(capacity_hint, set.len()),
      |set, k| {
        if <DefaultPartialSetBuffer<_> as Buffer>::push(set, k).is_some() {
          return Err(Error::capacity_exceeded("set"));
        }
        Ok(())
      },
      |item| K::partial_try_from_ref(context, item, selector),
    )
    .map(|_| partial_set)
  }
}
