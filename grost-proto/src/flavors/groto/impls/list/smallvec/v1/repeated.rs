use smallvec_1::SmallVec;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  convert::{Partial, PartialRef, PartialTryFromRef, Ref, TryFromPartialRef, TryFromRef},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    Borrowed, Groto, Repeated, WireFormat,
    groto::{Context, Error},
  },
  selection::{Selectable, Selector},
  state::State,
};

use super::super::super::{super::try_from, repeated_decode_list};

mod flatten;

bidi_equivalent!(@encode :<T: Encode<W, Groto>, W: WireFormat<Groto>>:[const N: usize, const TAG: u32] impl <SmallVec<[T; N]>, Repeated<W, TAG>> for <[T], Repeated<W, TAG>>);
bidi_equivalent!(@partial_encode :<T: PartialEncode<W, Groto>, W: WireFormat<Groto>>:[const N: usize, const TAG: u32] impl <SmallVec<[T; N]>, Repeated<W, TAG>> for <[T], Repeated<W, TAG>>);

bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize, const TAG: u32] impl <[&'a T], Borrowed<'a, Repeated<W, TAG>>> for <SmallVec<[T; N]>, Repeated<W, TAG>>);
bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize, const TAG: u32] impl <[&'a T], Borrowed<'a, Repeated<W, TAG>>> for <SmallVec<[T; N]>, Repeated<W, TAG>>);

bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize, const TAG: u32] impl <SmallVec<[&'a T; N]>, Borrowed<'a, Repeated<W, TAG>>> for <SmallVec<[T; N]>, Repeated<W, TAG>>);
bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize, const TAG: u32] impl <SmallVec<[&'a T; N]>, Borrowed<'a, Repeated<W, TAG>>> for <SmallVec<[T; N]>, Repeated<W, TAG>>);

impl<'a, K, KW, RB, B, const TAG: u32, const CAP: usize>
  Decode1<'a, Repeated<KW, TAG>, RB, B, Groto> for SmallVec<[K; CAP]>
where
  KW: WireFormat<Groto> + 'a,
  K: Ord + Decode1<'a, KW, RB, B, Groto>,
{
  fn decode(ctx: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let mut this = SmallVec::new();
    <Self as Decode1<'a, Repeated<KW, TAG>, RB, B, Groto>>::merge_decode(&mut this, ctx, src)
      .map(|size| (size, this))
  }

  fn merge_decode(&mut self, ctx: &'a Context, src: RB) -> Result<usize, Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    repeated_decode_list::<K, KW, RB, B, Self, TAG>(
      ctx,
      self,
      src,
      |list, item| {
        list.push(item);
        Ok(())
      },
      |list, capacity_hint| Ok(list.reserve_exact(capacity_hint)),
    )
  }
}

impl<'a, K, KW, RB, UB, const TAG: u32, const CAP: usize>
  TryFromRef<'a, RB, UB, Repeated<KW, TAG>, Groto> for SmallVec<[K; CAP]>
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, RB, UB, KW, Groto> + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, UB, Groto>,
  RB: ReadBuf + 'a,
  UB: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_ref(
    ctx: &'a Context,
    input: <Self as State<Ref<'a, RB, UB, Repeated<KW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, RB, UB, Repeated<KW, TAG>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    UB: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    let mut set = SmallVec::new();

    try_from::<K, K::Output, KW, RB, UB, _, _>(
      &mut set,
      input.iter(),
      |set| ctx.err_length_mismatch(capacity_hint, set.len()),
      |set, k| {
        set.push(k);
        Ok(())
      },
      |item| K::try_from_ref(ctx, item),
    )
    .map(|_| set)
  }
}

impl<'a, K, KW, RB, B, const TAG: u32, const CAP: usize>
  TryFromPartialRef<'a, RB, B, Repeated<KW, TAG>, Groto> for SmallVec<[K; CAP]>
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, RB, B, KW, Groto> + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, B, Groto>,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_partial_ref(
    ctx: &'a Context,
    input: <Self as State<PartialRef<'a, RB, B, Repeated<KW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, RB, B, Repeated<KW, TAG>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    let mut set = SmallVec::new();

    try_from::<K, K::Output, KW, RB, B, _, _>(
      &mut set,
      input.iter(),
      |set| ctx.err_length_mismatch(capacity_hint, set.len()),
      |set, k| {
        set.push(k);
        Ok(())
      },
      |item| K::try_from_partial_ref(ctx, item),
    )
    .map(|_| set)
  }
}

impl<'a, K, KW, RB, B, const TAG: u32, const CAP: usize>
  PartialTryFromRef<'a, RB, B, Repeated<KW, TAG>, Groto> for SmallVec<[K; CAP]>
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, RB, B, KW, Groto> + 'a,
  <K as State<PartialRef<'a, RB, B, KW, Groto>>>::Output:
    Sized + Decode1<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
  <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn partial_try_from_ref(
    context: &'a Context,
    input: <Self as State<PartialRef<'a, RB, B, Repeated<KW, TAG>, Groto>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, Error>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'a, RB, B, Repeated<KW, TAG>, Groto>>>::Output: Sized,
  {
    if selector.is_empty() {
      return Ok(SmallVec::new());
    }

    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut output = SmallVec::with_capacity(capacity_hint);

    try_from::<_, _, KW, RB, B, _, _>(
      &mut output,
      iter,
      |set| context.err_length_mismatch(capacity_hint, set.len()),
      |set, k| {
        set.push(k);
        Ok(())
      },
      |item| K::partial_try_from_ref(context, item, selector),
    )
    .map(|_| output)
  }
}
