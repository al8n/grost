use std::collections::BTreeSet;

use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{PartialTryFromRef, TryFromPartialRef, TryFromRef},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultRepeatedWireFormat, Groto, Repeated, WireFormat,
    groto::{Context, Error, RepeatedDecoderBuffer},
  },
  selection::{Selectable, Selector},
  state::{Partial, PartialRef, Ref, State},
};

use super::{
  super::super::{repeated_decode, repeated_encode, repeated_encoded_len, try_from},
  DefaultPartialSetBuffer,
};

impl<K> DefaultRepeatedWireFormat<Groto> for BTreeSet<K> {
  type Format<KM, const TAG: u32>
    = Repeated<KM, TAG>
  where
    KM: WireFormat<Groto> + 'static;
}

impl<'a, K, KW, RB, B, const TAG: u32> State<PartialRef<'a, Repeated<KW, TAG>, RB, B, Groto>>
  for BTreeSet<K>
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: State<PartialRef<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
{
  type Output = RepeatedDecoderBuffer<'a, K::Output, RB, B, KW, TAG>;
}

impl<'a, K, KW, RB, B, const TAG: u32> State<Ref<'a, Repeated<KW, TAG>, RB, B, Groto>>
  for BTreeSet<K>
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: State<Ref<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
{
  type Output = RepeatedDecoderBuffer<'a, K::Output, RB, B, KW, TAG>;
}

impl<K, KW, const TAG: u32> Encode<Repeated<KW, TAG>, Groto> for BTreeSet<K>
where
  KW: WireFormat<Groto>,
  K: Encode<KW, Groto>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    repeated_encode::<K, KW, _, TAG>(
      buf,
      || self.iter(),
      |k| k.encoded_len(context),
      |k, buf| k.encode(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    repeated_encoded_len::<K, KW, _, TAG>(self.iter(), |k| k.encoded_len(context))
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Repeated<KW, TAG>, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Repeated<KW, TAG>, Groto>>::encoded_raw_len(self, context)
  }
}

impl<K, KW, const TAG: u32> PartialEncode<Repeated<KW, TAG>, Groto> for BTreeSet<K>
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

    repeated_encode::<K, KW, _, TAG>(
      buf,
      || self.iter(),
      |k| k.partial_encoded_len(context, selector),
      |k, buf| k.partial_encode(context, buf, selector),
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    repeated_encoded_len::<K, KW, _, TAG>(self.iter(), |k| k.partial_encoded_len(context, selector))
  }

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    <Self as PartialEncode<Repeated<KW, TAG>, Groto>>::partial_encode_raw(
      self, context, buf, selector,
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <Self as PartialEncode<Repeated<KW, TAG>, Groto>>::partial_encoded_raw_len(
      self, context, selector,
    )
  }
}

impl<'a, K, KW, RB, B, const TAG: u32> Decode<'a, Repeated<KW, TAG>, RB, B, Groto> for BTreeSet<K>
where
  KW: WireFormat<Groto> + 'a,
  K: Ord + Decode<'a, KW, RB, B, Groto>,
{
  fn decode(ctx: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let mut this = BTreeSet::new();
    <Self as Decode<'a, Repeated<KW, TAG>, RB, B, Groto>>::merge_decode(&mut this, ctx, src)
      .map(|size| (size, this))
  }

  fn merge_decode(&mut self, ctx: &'a Context, src: RB) -> Result<usize, Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    repeated_decode::<K, KW, Self, RB, B, TAG>(self, src, |set, src| {
      let (read, item) = K::decode(ctx, src)?;

      ctx.err_duplicated_set_keys(!set.insert(item))?;

      Ok(read)
    })
  }
}

impl<'a, K, KW, RB, UB, const TAG: u32> TryFromRef<'a, Repeated<KW, TAG>, RB, UB, Groto>
  for BTreeSet<K>
where
  KW: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, RB, UB, KW, Groto> + Ord + 'a,
  K::Output: Sized + Decode<'a, KW, RB, UB, Groto>,
  RB: ReadBuf + 'a,
  UB: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_ref(
    ctx: &'a Context,
    input: <Self as State<Ref<'a, Repeated<KW, TAG>, RB, UB, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, Repeated<KW, TAG>, RB, UB, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    UB: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    let mut set = BTreeSet::new();

    try_from::<K, K::Output, KW, RB, UB, _, _>(
      &mut set,
      input.iter(),
      |set| ctx.err_length_mismatch(capacity_hint, set.len()),
      |set, k| ctx.err_duplicated_set_keys(!set.insert(k)),
      |item| K::try_from_ref(ctx, item),
    )
    .map(|_| set)
  }
}

impl<'a, K, KW, RB, B, const TAG: u32> TryFromPartialRef<'a, Repeated<KW, TAG>, RB, B, Groto>
  for BTreeSet<K>
where
  KW: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, KW, RB, B, Groto> + Ord + 'a,
  K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_partial_ref(
    ctx: &'a Context,
    input: <Self as State<PartialRef<'a, Repeated<KW, TAG>, RB, B, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, Repeated<KW, TAG>, RB, B, Groto>>>::Output: Sized,
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
    .map(|_| set)
  }
}

impl<'a, K, KW, RB, B, const TAG: u32> PartialTryFromRef<'a, Repeated<KW, TAG>, RB, B, Groto>
  for BTreeSet<K>
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
    input: <Self as State<PartialRef<'a, Repeated<KW, TAG>, RB, B, Groto>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, Error>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'a, Repeated<KW, TAG>, RB, B, Groto>>>::Output: Sized,
  {
    if selector.is_empty() {
      return Ok(DefaultPartialSetBuffer::new());
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
