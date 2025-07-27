use std::collections::BTreeSet;

use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{PartialTryFromRef, TryFromPartialRef, TryFromRef},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultSetWireFormat, Groto, Packed, WireFormat,
    groto::{Context, Error, PackedSetDecoder},
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

impl<K> DefaultSetWireFormat<Groto> for Decomposable<BTreeSet<K>> {
  type Format<KM>
    = Packed<KM>
  where
    KM: WireFormat<Groto> + 'static;
}

impl<'a, K, KW, RB, B> State<PartialRef<'a, Packed<KW>, RB, B, Groto>> for Decomposable<BTreeSet<K>>
where
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: State<PartialRef<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
{
  type Output = PackedSetDecoder<'a, K::Output, RB, B, KW>;
}

impl<'a, K, KW, RB, B> State<Ref<'a, Packed<KW>, RB, B, Groto>> for Decomposable<BTreeSet<K>>
where
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: State<Ref<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
{
  type Output = PackedSetDecoder<'a, K::Output, RB, B, KW>;
}

impl<'a, K, KW, RB, B> Decode<'a, Packed<KW>, RB, B, Groto> for Decomposable<BTreeSet<K>>
where
  KW: WireFormat<Groto> + 'a,
  K: Ord + Decode<'a, KW, RB, B, Groto>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    packed_decode::<K, KW, BTreeSet<_>, RB>(
      context,
      src,
      |_| Ok(BTreeSet::new()),
      BTreeSet::len,
      |set, src| {
        let (read, item) = K::decode(context, src)?;

        context.err_duplicated_set_keys(!set.insert(item))?;

        Ok(read)
      },
    )
    .map(|(read, set)| (read, set.into()))
  }
}

impl<K, KW> Encode<Packed<KW>, Groto> for Decomposable<BTreeSet<K>>
where
  KW: WireFormat<Groto>,
  K: Encode<KW, Groto>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    packed_encode_raw::<K, _, _, _>(
      buf,
      self.iter(),
      || <Self as Encode<Packed<KW>, Groto>>::encoded_raw_len(self, context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    packed_encoded_raw_len::<K, KW, _, _>(self.len(), self.iter(), |item| item.encoded_len(context))
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    packed_encode::<K, _, _, _>(
      buf,
      self.len(),
      self.iter(),
      || <Self as Encode<Packed<KW>, Groto>>::encoded_raw_len(self, context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_len(&self, context: &Context) -> usize {
    packed_encoded_len::<_>(self.len(), || {
      <Self as Encode<Packed<KW>, Groto>>::encoded_raw_len(self, context)
    })
  }
}

impl<K, KW> PartialEncode<Packed<KW>, Groto> for Decomposable<BTreeSet<K>>
where
  KW: WireFormat<Groto>,
  K: PartialEncode<KW, Groto>,
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

    packed_encode_raw::<K, _, _, _>(
      buf,
      self.iter(),
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

    packed_encoded_raw_len::<K, KW, _, _>(self.len(), self.iter(), |item| {
      item.partial_encoded_len(context, selector)
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

    packed_encode::<K, _, _, _>(
      buf,
      self.len(),
      self.iter(),
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

    packed_encoded_len::<_>(self.len(), || {
      <Self as PartialEncode<Packed<KW>, Groto>>::partial_encoded_raw_len(self, context, selector)
    })
  }
}

impl<'a, K, KW, RB, B> TryFromRef<'a, Packed<KW>, RB, B, Groto> for Decomposable<BTreeSet<K>>
where
  KW: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, KW, RB, B, Groto> + Ord + 'a,
  K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_ref(
    ctx: &'a Context,
    input: <Self as State<Ref<'a, Packed<KW>, RB, B, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, Packed<KW>, RB, B, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    let mut set = BTreeSet::new();

    try_from::<K, K::Output, KW, RB, B, _, _>(
      &mut set,
      input.iter(),
      |set| ctx.err_length_mismatch(capacity_hint, set.len()),
      |set, k| ctx.err_duplicated_set_keys(!set.insert(k)),
      |item| K::try_from_ref(ctx, item),
    )
    .map(|_| set.into())
  }
}

impl<'a, K, KW, RB, B> TryFromPartialRef<'a, Packed<KW>, RB, B, Groto> for Decomposable<BTreeSet<K>>
where
  KW: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, KW, RB, B, Groto> + Ord + 'a,
  K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_partial_ref(
    ctx: &'a Context,
    input: <Self as State<PartialRef<'a, Packed<KW>, RB, B, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, Packed<KW>, RB, B, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    let mut set = BTreeSet::new();

    try_from::<K, K::Output, KW, RB, B, _, _>(
      &mut set,
      input.iter(),
      |set| ctx.err_length_mismatch(capacity_hint, set.len()),
      |set, k| ctx.err_duplicated_set_keys(!set.insert(k)),
      |item| K::try_from_partial_ref(ctx, item),
    )
    .map(|_| set.into())
  }
}

impl<'a, K, KW, RB, B> PartialTryFromRef<'a, Packed<KW>, RB, B, Groto> for Decomposable<BTreeSet<K>>
where
  KW: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, KW, RB, B, Groto> + Ord + 'a,
  <K as State<PartialRef<'a, KW, RB, B, Groto>>>::Output:
    Sized + Decode<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
  <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  RB: ReadBuf + 'a,
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
