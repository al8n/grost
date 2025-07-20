use super::HashSet;

use core::hash::{BuildHasher, Hash};

use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{Partial, PartialRef, PartialTryFromRef, Ref, TryFromPartialRef, TryFromRef},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultSetWireFormat, Groto, Packed, WireFormat,
    groto::{Context, Error, PackedSetDecoder},
  },
  selection::{Selectable, Selector},
  state::State,
};

use super::super::{
  DefaultPartialSetBuffer, packed_decode, packed_encode, packed_encode_raw, packed_encoded_len,
  packed_encoded_raw_len, try_from,
};

impl<K, S> DefaultSetWireFormat<Groto> for HashSet<K, S> {
  type Format<KM>
    = Packed<KM>
  where
    KM: WireFormat<Groto> + 'static;
}

impl<'a, K, KW, S, RB, B> State<PartialRef<'a, RB, B, Packed<KW>, Groto>> for HashSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: State<PartialRef<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
{
  type Output = PackedSetDecoder<'a, K::Output, RB, B, KW>;
}

impl<'a, K, KW, S, RB, B> State<Ref<'a, RB, B, Packed<KW>, Groto>> for HashSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: State<Ref<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
{
  type Output = PackedSetDecoder<'a, K::Output, RB, B, KW>;
}

impl<'a, K, KW, S, RB, B> Decode1<'a, Packed<KW>, RB, B, Groto> for HashSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  S: BuildHasher + Default,
  K: Eq + Hash + Decode1<'a, KW, RB, B, Groto>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    packed_decode::<K, KW, Self, RB>(
      context,
      src,
      |cap| {
        if cap == 0 {
          Ok(Self::with_hasher(S::default()))
        } else {
          Ok(Self::with_capacity_and_hasher(cap, S::default()))
        }
      },
      Self::len,
      |set, src| {
        let (read, item) = K::decode(context, src)?;

        context.err_duplicated_set_keys(!set.insert(item))?;

        Ok(read)
      },
    )
  }
}

impl<K, KW, S> Encode<Packed<KW>, Groto> for HashSet<K, S>
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

impl<K, KW, S> PartialEncode<Packed<KW>, Groto> for HashSet<K, S>
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

impl<'a, K, KW, S, RB, B> TryFromRef<'a, RB, B, Packed<KW>, Groto> for HashSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, RB, B, KW, Groto> + Eq + Hash + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, B, Groto>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_ref(
    ctx: &'a Context,
    input: <Self as State<Ref<'a, RB, B, Packed<KW>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, RB, B, Packed<KW>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut set = HashSet::with_capacity_and_hasher(capacity_hint, S::default());

    try_from::<K, K::Output, KW, RB, B, _, _>(
      &mut set,
      iter,
      |set| ctx.err_length_mismatch(capacity_hint, set.len()),
      |set, item| ctx.err_duplicated_set_keys(!set.insert(item)),
      |item| K::try_from_ref(ctx, item),
    )
    .map(|_| set)
  }
}

impl<'a, K, KW, S, RB, B> TryFromPartialRef<'a, RB, B, Packed<KW>, Groto> for HashSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, RB, B, KW, Groto> + Eq + Hash + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, B, Groto>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_partial_ref(
    ctx: &'a Context,
    input: <Self as State<PartialRef<'a, RB, B, Packed<KW>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, RB, B, Packed<KW>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut set = HashSet::with_capacity_and_hasher(capacity_hint, S::default());

    try_from::<K, K::Output, KW, RB, B, _, _>(
      &mut set,
      iter,
      |set| ctx.err_length_mismatch(capacity_hint, set.len()),
      |set, item| ctx.err_duplicated_set_keys(!set.insert(item)),
      |item| K::try_from_partial_ref(ctx, item),
    )
    .map(|_| set)
  }
}

impl<'a, K, KW, S, RB, B> PartialTryFromRef<'a, RB, B, Packed<KW>, Groto> for HashSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, RB, B, KW, Groto> + Eq + Hash + 'a,
  <K as State<PartialRef<'a, RB, B, KW, Groto>>>::Output:
    Sized + Decode1<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
  <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn partial_try_from_ref(
    context: &'a Context,
    input: <Self as State<PartialRef<'a, RB, B, Packed<KW>, Groto>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, Error>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'a, RB, B, Packed<KW>, Groto>>>::Output: Sized,
  {
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
