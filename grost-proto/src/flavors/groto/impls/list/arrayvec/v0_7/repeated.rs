use arrayvec_0_7::ArrayVec;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  convert::{PartialTryFromRef, TryFromPartialRef, TryFromRef},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    Borrowed, Groto, Repeated, WireFormat,
    groto::{Context, Error},
  },
  selection::{Selectable, Selector},
  state::{Partial, PartialRef, Ref, State},
};

use super::super::super::{super::try_from, repeated_decode_list};

mod flatten;

bidi_equivalent!(@encode :<T: Encode<W, Groto>, W: WireFormat<Groto>>:[const N: usize, const TAG: u32] impl <ArrayVec<T, N>, Repeated<W, TAG>> for <[T], Repeated<W, TAG>>);
bidi_equivalent!(@partial_encode :<T: PartialEncode<W, Groto>, W: WireFormat<Groto>>:[const N: usize, const TAG: u32] impl <ArrayVec<T, N>, Repeated<W, TAG>> for <[T], Repeated<W, TAG>>);

bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize, const TAG: u32] impl <[&'a T], Borrowed<'a, Repeated<W, TAG>>> for <ArrayVec<T, N>, Repeated<W, TAG>>);
bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize, const TAG: u32] impl <[&'a T], Borrowed<'a, Repeated<W, TAG>>> for <ArrayVec<T, N>, Repeated<W, TAG>>);

bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize, const TAG: u32] impl <ArrayVec<&'a T, N>, Borrowed<'a, Repeated<W, TAG>>> for <ArrayVec<T, N>, Repeated<W, TAG>>);
bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize, const TAG: u32] impl <ArrayVec<&'a T, N>, Borrowed<'a, Repeated<W, TAG>>> for <ArrayVec<T, N>, Repeated<W, TAG>>);

impl<'a, K, KW, RB, B, const TAG: u32, const CAP: usize> Decode<'a, Repeated<KW, TAG>, RB, B, Groto>
  for ArrayVec<K, CAP>
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
    let mut this = ArrayVec::new();
    <Self as Decode<'a, Repeated<KW, TAG>, RB, B, Groto>>::merge_decode(&mut this, ctx, src)
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
        if list.try_push(item).is_err() {
          return Err(super::larger_than_array_capacity::<CAP>());
        }

        Ok(())
      },
      |_, _| Ok(()),
    )
  }
}

impl<'a, K, KW, RB, UB, const TAG: u32, const CAP: usize>
  TryFromRef<'a, Repeated<KW, TAG>, RB, UB, Groto> for ArrayVec<K, CAP>
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, KW, RB, UB, Groto> + 'a,
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

    if capacity_hint >= CAP {
      return Err(super::larger_than_array_capacity::<CAP>());
    }

    let mut set = ArrayVec::new();

    try_from::<K, K::Output, KW, RB, UB, _, _>(
      &mut set,
      input.iter(),
      |set| ctx.err_length_mismatch(capacity_hint, set.len()),
      |set, k| {
        if set.try_push(k).is_err() {
          return Err(super::larger_than_array_capacity::<CAP>());
        }

        Ok(())
      },
      |item| K::try_from_ref(ctx, item),
    )
    .map(|_| set)
  }
}

impl<'a, K, KW, RB, B, const TAG: u32, const CAP: usize>
  TryFromPartialRef<'a, Repeated<KW, TAG>, RB, B, Groto> for ArrayVec<K, CAP>
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, KW, RB, B, Groto> + 'a,
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
    if capacity_hint >= CAP {
      return Err(super::larger_than_array_capacity::<CAP>());
    }

    let mut set = ArrayVec::new();

    try_from::<K, K::Output, KW, RB, B, _, _>(
      &mut set,
      input.iter(),
      |set| ctx.err_length_mismatch(capacity_hint, set.len()),
      |set, k| {
        if set.try_push(k).is_err() {
          return Err(super::larger_than_array_capacity::<CAP>());
        }

        Ok(())
      },
      |item| K::try_from_partial_ref(ctx, item),
    )
    .map(|_| set)
  }
}

impl<'a, K, KW, RB, B, const TAG: u32, const CAP: usize>
  PartialTryFromRef<'a, Repeated<KW, TAG>, RB, B, Groto> for ArrayVec<K, CAP>
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, KW, RB, B, Groto> + 'a,
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
      return Ok(ArrayVec::new());
    }

    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    if capacity_hint >= CAP {
      return Err(super::larger_than_array_capacity::<CAP>());
    }

    let mut output = ArrayVec::new();

    try_from::<_, _, KW, RB, B, _, _>(
      &mut output,
      iter,
      |set| context.err_length_mismatch(capacity_hint, set.len()),
      |set, k| {
        if set.try_push(k).is_err() {
          return Err(super::larger_than_array_capacity::<CAP>());
        }
        Ok(())
      },
      |item| K::partial_try_from_ref(context, item, selector),
    )
    .map(|_| output)
  }
}
