use core::{cell::RefCell, mem::MaybeUninit};

use crate::{
  buffer::{Buf, BufMut, UnknownBuffer},
  convert::{PartialTryFromRef, TryFromPartialRef, TryFromRef},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    Borrowed, Groto, Repeated, WireFormat,
    groto::{Context, Error},
  },
  selection::Selectable,
  state::{Partial, PartialRef, Ref, State},
};

use super::super::super::try_from;

mod flatten;

bidi_equivalent!(@encode :<T: Encode<W, Groto>, W: WireFormat<Groto>>:[const N: usize, const TAG: u32] impl <[T; N], Repeated<W, TAG>> for <[T], Repeated<W, TAG>>);
bidi_equivalent!(@partial_encode :<T: PartialEncode<W, Groto>, W: WireFormat<Groto>>:[const N: usize, const TAG: u32] impl <[T; N], Repeated<W, TAG>> for <[T], Repeated<W, TAG>>);

bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize, const TAG: u32] impl <[&'a T], Borrowed<'a, Repeated<W, TAG>>> for <[T; N], Repeated<W, TAG>>);
bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize, const TAG: u32] impl <[&'a T], Borrowed<'a, Repeated<W, TAG>>> for <[T; N], Repeated<W, TAG>>);

bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize, const TAG: u32] impl <[&'a T; N], Borrowed<'a, Repeated<W, TAG>>> for <[T; N], Repeated<W, TAG>>);
bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize, const TAG: u32] impl <[&'a T; N], Borrowed<'a, Repeated<W, TAG>>> for <[T; N], Repeated<W, TAG>>);

impl<'a, K, KW, RB, B, const TAG: u32, const CAP: usize> Decode<'a, Repeated<KW, TAG>, RB, B, Groto>
  for [K; CAP]
where
  KW: WireFormat<Groto> + 'a,
  K: Ord + Decode<'a, KW, RB, B, Groto>,
{
  fn decode(ctx: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: Buf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let mut this: [MaybeUninit<K>; CAP] = core::array::from_fn(|_| MaybeUninit::uninit());
    let mut idx = 0;
    crate::flavors::groto::impls::repeated_decode::<K, KW, [MaybeUninit<K>; CAP], RB, B, TAG>(
      &mut this,
      src,
      |list, src| {
        if idx >= CAP {
          return Err(super::larger_than_array_capacity::<CAP>());
        }
        let (read, item) = K::decode(ctx, src)?;
        list[idx].write(item);
        idx += 1;

        Ok(read)
      },
    )
    .map(|read| {
      // SAFETY: We have ensured that CAP's elements are initialized
      // and that we have not read more than CAP elements.
      (read, unsafe {
        this.map(|val| MaybeUninit::assume_init(val))
      })
    })
  }
}

impl<'a, K, KW, RB, UB, const TAG: u32, const CAP: usize>
  TryFromRef<'a, Repeated<KW, TAG>, RB, UB, Groto> for [K; CAP]
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, KW, RB, UB, Groto> + 'a,
  K::Output: Sized + Decode<'a, KW, RB, UB, Groto>,
  RB: Buf + 'a,
  UB: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_ref(
    ctx: &'a Context,
    input: <Self as State<Ref<'a, Repeated<KW, TAG>, RB, UB, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, Repeated<KW, TAG>, RB, UB, Groto>>>::Output: Sized,
    RB: Buf + 'a,
    UB: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();

    if capacity_hint >= CAP {
      return Err(super::larger_than_array_capacity::<CAP>());
    }

    let mut set: [MaybeUninit<K>; CAP] = core::array::from_fn(|_| MaybeUninit::uninit());
    let idx = RefCell::new(0);

    try_from::<K, K::Output, KW, RB, UB, _, [MaybeUninit<K>; CAP]>(
      &mut set,
      input.iter(),
      |_| ctx.err_length_mismatch(capacity_hint, *idx.borrow()),
      |set, k| {
        let mut idx = idx.borrow_mut();
        if *idx >= CAP {
          return Err(super::larger_than_array_capacity::<CAP>());
        }

        set[*idx].write(k);
        *idx += 1;

        Ok(())
      },
      |item| K::try_from_ref(ctx, item),
    )?;

    // SAFETY: We have ensured that CAP's elments are initialized
    // and that we have not written more than CAP elements.
    Ok(unsafe { set.map(|val| MaybeUninit::assume_init(val)) })
  }
}

impl<'a, K, KW, RB, B, const TAG: u32, const CAP: usize>
  TryFromPartialRef<'a, Repeated<KW, TAG>, RB, B, Groto> for [K; CAP]
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, KW, RB, B, Groto> + 'a,
  K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
  RB: Buf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_partial_ref(
    ctx: &'a Context,
    input: <Self as State<PartialRef<'a, Repeated<KW, TAG>, RB, B, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, Repeated<KW, TAG>, RB, B, Groto>>>::Output: Sized,
    RB: Buf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    if capacity_hint >= CAP {
      return Err(super::larger_than_array_capacity::<CAP>());
    }

    let mut set: [MaybeUninit<K>; CAP] = core::array::from_fn(|_| MaybeUninit::uninit());
    let idx = RefCell::new(0);

    try_from::<K, K::Output, KW, RB, B, _, [MaybeUninit<K>; CAP]>(
      &mut set,
      input.iter(),
      |_| ctx.err_length_mismatch(capacity_hint, *idx.borrow()),
      |set, k| {
        let mut idx = idx.borrow_mut();
        if *idx >= CAP {
          return Err(super::larger_than_array_capacity::<CAP>());
        }

        set[*idx].write(k);
        *idx += 1;

        Ok(())
      },
      |item| K::try_from_partial_ref(ctx, item),
    )?;

    // SAFETY: We have ensured that CAP's elements are initialized
    // and that we have not written more than CAP elements.
    Ok(unsafe { set.map(|val| MaybeUninit::assume_init(val)) })
  }
}

impl<'a, K, KW, RB, B, const TAG: u32, const CAP: usize>
  PartialTryFromRef<'a, Repeated<KW, TAG>, RB, B, Groto> for [K; CAP]
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, KW, RB, B, Groto> + 'a,
  <K as State<PartialRef<'a, KW, RB, B, Groto>>>::Output:
    Sized + Decode<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
  <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  RB: Buf + 'a,
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
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    if capacity_hint >= CAP {
      return Err(super::larger_than_array_capacity::<CAP>());
    }

    let mut output: [MaybeUninit<<K as State<Partial<Groto>>>::Output>; CAP] =
      core::array::from_fn(|_| MaybeUninit::uninit());
    let mut idx = 0;

    try_from::<_, _, KW, RB, B, _, _>(
      &mut output,
      iter,
      |set| context.err_length_mismatch(capacity_hint, set.len()),
      |set, k| {
        if idx >= CAP {
          return Err(super::larger_than_array_capacity::<CAP>());
        }

        set[idx].write(k);
        idx += 1;

        Ok(())
      },
      |item| K::partial_try_from_ref(context, item, selector),
    )
    .map(|_| {
      // SAFETY: We have ensured that CAP's elements are initialized
      // and that we have not written more than CAP elements.
      unsafe { output.map(|val| MaybeUninit::assume_init(val)) }
    })
  }
}
