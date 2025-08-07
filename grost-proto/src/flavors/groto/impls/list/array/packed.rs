use core::{cell::RefCell, mem::MaybeUninit};

use crate::{
  buffer::{Buf, BufMut, UnknownBuffer},
  convert::{PartialTryFromRef, TryFromPartialRef, TryFromRef},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    Borrowed, Groto, Packed, WireFormat,
    groto::{Context, Error},
  },
  selection::Selectable,
  state::{Partial, PartialRef, Ref, State},
};

use super::super::super::{packed_decode, try_from};

mod flatten;

bidi_equivalent!(@encode :<T: Encode<W, Groto>, W: WireFormat<Groto>>:[const N: usize] impl <[T; N], Packed<W>> for <[T], Packed<W>>);
bidi_equivalent!(@partial_encode :<T: PartialEncode<W, Groto>, W: WireFormat<Groto>>:[const N: usize] impl <[T; N], Packed<W>> for <[T], Packed<W>>);

bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T; N], Borrowed<'a, Packed<W>>> for <[T], Packed<W>>);
bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T; N], Borrowed<'a, Packed<W>>> for <[T], Packed<W>>);

bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <[T; N], Packed<W>>);
bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <[T; N], Packed<W>>);

bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T; N], Borrowed<'a, Packed<W>>> for <[T; N], Packed<W>>);
bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T; N], Borrowed<'a, Packed<W>>> for <[T; N], Packed<W>>);

impl<'a, K, KW, RB, B, const CAP: usize> Decode<'a, Packed<KW>, RB, B, Groto> for [K; CAP]
where
  KW: WireFormat<Groto> + 'a,
  K: Ord + Decode<'a, KW, RB, B, Groto>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: Buf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let idx = RefCell::new(0);
    packed_decode::<K, KW, [MaybeUninit<K>; CAP], RB>(
      context,
      src,
      |_| Ok(core::array::from_fn(|_| MaybeUninit::uninit())),
      |_| *idx.borrow(),
      |set, src| {
        let (read, item) = K::decode(context, src)?;

        let mut idx = idx.borrow_mut();
        if *idx >= CAP {
          return Err(super::larger_than_array_capacity::<CAP>());
        }

        set[*idx].write(item);
        *idx += 1;

        Ok(read)
      },
    )
    .map(|(read, this)| {
      // SAFETY: We have ensured that CAP's elements are initialized
      // and that we have not read more than CAP elements.
      (read, unsafe {
        this.map(|val| MaybeUninit::assume_init(val))
      })
    })
  }
}

impl<'a, K, KW, RB, B, const CAP: usize> TryFromRef<'a, Packed<KW>, RB, B, Groto> for [K; CAP]
where
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, KW, RB, B, Groto> + 'a,
  K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
  RB: Buf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_ref(
    ctx: &'a Context,
    input: <Self as State<Ref<'a, Packed<KW>, RB, B, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, Packed<KW>, RB, B, Groto>>>::Output: Sized,
    RB: Buf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();

    if capacity_hint >= CAP {
      return Err(super::larger_than_array_capacity::<CAP>());
    }

    let mut set: [MaybeUninit<K>; CAP] = core::array::from_fn(|_| MaybeUninit::uninit());
    let idx = RefCell::new(0);
    try_from::<K, K::Output, KW, RB, B, _, _>(
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

    // SAFETY: We have ensured that CAP's elements are initialized
    // and that we have not read more than CAP elements.
    Ok(unsafe { set.map(|val| MaybeUninit::assume_init(val)) })
  }
}

impl<'a, K, KW, RB, B, const CAP: usize> TryFromPartialRef<'a, Packed<KW>, RB, B, Groto>
  for [K; CAP]
where
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, KW, RB, B, Groto> + 'a,
  K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
  RB: Buf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_partial_ref(
    ctx: &'a Context,
    input: <Self as State<PartialRef<'a, Packed<KW>, RB, B, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, Packed<KW>, RB, B, Groto>>>::Output: Sized,
    RB: Buf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    if capacity_hint >= CAP {
      return Err(super::larger_than_array_capacity::<CAP>());
    }

    let mut set: [MaybeUninit<K>; CAP] = core::array::from_fn(|_| MaybeUninit::uninit());
    let idx = RefCell::new(0);
    try_from::<K, K::Output, KW, RB, B, _, _>(
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
    // and that we have not read more than CAP elements.
    Ok(unsafe { set.map(|val| MaybeUninit::assume_init(val)) })
  }
}

impl<'a, K, KW, RB, B, const CAP: usize> PartialTryFromRef<'a, Packed<KW>, RB, B, Groto>
  for [K; CAP]
where
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, KW, RB, B, Groto> + 'a,
  <K as State<PartialRef<'a, KW, RB, B, Groto>>>::Output:
    Sized + Decode<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
  <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  RB: Buf + 'a,
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
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    if capacity_hint >= CAP {
      return Err(super::larger_than_array_capacity::<CAP>());
    }

    let mut output: [MaybeUninit<<K as State<Partial<Groto>>>::Output>; CAP] =
      core::array::from_fn(|_| MaybeUninit::uninit());
    let idx = RefCell::new(0);

    try_from::<<K as State<Partial<Groto>>>::Output, _, KW, RB, B, _, _>(
      &mut output,
      iter,
      |_| context.err_length_mismatch(capacity_hint, *idx.borrow()),
      |set, k| {
        let mut idx = idx.borrow_mut();
        if *idx >= CAP {
          return Err(super::larger_than_array_capacity::<CAP>());
        }
        set[*idx].write(k);
        *idx += 1;

        Ok(())
      },
      |item| K::partial_try_from_ref(context, item, selector),
    )?;

    // SAFETY: We have ensured that CAP's elements are initialized
    // and that we have not written more than CAP elements.
    Ok(unsafe { output.map(|val| MaybeUninit::assume_init(val)) })
  }
}
