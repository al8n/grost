use tinyvec_1::{Array, TinyVec};

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  convert::{PartialTryFromRef, TryFromPartialRef, TryFromRef},
  decode::Decode,
  flavors::{
    Borrowed, Groto, Packed, WireFormat,
    groto::{Context, Error},
  },
  selection::{Selectable, Selector},
  state::{Partial, PartialRef, Ref, State},
};

use super::super::super::super::super::{packed_decode, try_from};
use super::{DefaultEncode, DefaultPartialEncode};

mod flatten;

bidi_equivalent!(@encode :<T: DefaultEncode<W>, W: WireFormat<Groto>>:[const N: usize] impl <TinyVec<[T; N]>, Packed<W>> for <[T], Packed<W>>);
bidi_equivalent!(@partial_encode :<T: DefaultPartialEncode<W>, W: WireFormat<Groto>>:[const N: usize] impl <TinyVec<[T; N]>, Packed<W>> for <[T], Packed<W>>);

bidi_equivalent!(@encode 'a:<T: DefaultEncode<W>, W:WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <TinyVec<[T; N]>, Packed<W>>);
bidi_equivalent!(@partial_encode 'a:<T: DefaultPartialEncode<W>, W: WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <TinyVec<[T; N]>, Packed<W>>);

impl<'a, K, KW, RB, B, A> Decode<'a, Packed<KW>, RB, B, Groto> for TinyVec<A>
where
  A: Array<Item = K>,
  KW: WireFormat<Groto> + 'a,
  K: Ord + Decode<'a, KW, RB, B, Groto> + 'a,
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
      |_| Ok(Self::new()),
      Self::len,
      |set, src| {
        let (read, item) = K::decode(context, src)?;
        set.push(item);

        Ok(read)
      },
    )
  }
}

impl<'a, K, KW, RB, B, A> TryFromRef<'a, RB, B, Packed<KW>, Groto> for TinyVec<A>
where
  A: Array<Item = K>,
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, RB, B, KW, Groto> + 'a,
  K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
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
    let capacity_hint = input.capacity_hint();
    let mut set = TinyVec::new();

    try_from::<K, K::Output, KW, RB, B, _, _>(
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

impl<'a, K, KW, RB, B, A> TryFromPartialRef<'a, RB, B, Packed<KW>, Groto> for TinyVec<A>
where
  A: Array<Item = K>,
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, RB, B, KW, Groto> + 'a,
  K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
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
    let capacity_hint = input.capacity_hint();
    let mut set = TinyVec::new();

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

impl<'a, K, KW, RB, B, const CAP: usize> PartialTryFromRef<'a, RB, B, Packed<KW>, Groto>
  for TinyVec<[K; CAP]>
where
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, RB, B, KW, Groto> + Default + 'a,
  <K as State<PartialRef<'a, RB, B, KW, Groto>>>::Output:
    Sized + Decode<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
  <K as State<Partial<Groto>>>::Output: Sized + Default + Selectable<Groto, Selector = K::Selector>,
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
    if selector.is_empty() {
      return Ok(TinyVec::new());
    }

    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut output = TinyVec::with_capacity(capacity_hint);

    try_from::<<K as State<Partial<Groto>>>::Output, _, KW, RB, B, _, _>(
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
