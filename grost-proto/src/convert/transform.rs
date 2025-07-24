use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  flavors::{Flavor, WireFormat},
  state::{Partial, PartialRef, Ref, State},
};

pub trait TryFromPartialRef<'a, W, RB, UB, F>: State<PartialRef<'a, W, RB, UB, F>>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Transforms from the input type `PartialRef<'a, W, RB, UB, F>` into the current type `Self`.
  fn try_from_partial_ref(
    ctx: &'a F::Context,
    input: <Self as State<PartialRef<'a, W, RB, UB, F>>>::Output,
  ) -> Result<Self, F::Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, W, RB, UB, F>>>::Output: Sized,
    RB: ReadBuf,
    UB: UnknownBuffer<RB, F>;
}

impl<'a, W, RB, UB, F, T> TryFromPartialRef<'a, W, RB, UB, F> for T
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  T: State<PartialRef<'a, W, RB, UB, F>, Output = Self>,
{
  fn try_from_partial_ref(
    _: &'a F::Context,
    input: <Self as State<PartialRef<'a, W, RB, UB, F>>>::Output,
  ) -> Result<Self, F::Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, W, RB, UB, F>>>::Output: Sized,
    RB: ReadBuf,
    UB: UnknownBuffer<RB, F>,
  {
    Ok(input)
  }
}

pub trait TryFromPartial<F>: State<Partial<F>>
where
  F: Flavor + ?Sized,
{
  /// Transforms from the input type `Partial<F>` into the current type `Self`.
  fn try_from_partial(
    ctx: &F::Context,
    input: <Self as State<Partial<F>>>::Output,
  ) -> Result<Self, F::Error>
  where
    Self: Sized,
    <Self as State<Partial<F>>>::Output: Sized;
}

impl<F, T> TryFromPartial<F> for T
where
  F: Flavor + ?Sized,
  T: State<Partial<F>, Output = Self>,
{
  fn try_from_partial(
    _: &F::Context,
    input: <Self as State<Partial<F>>>::Output,
  ) -> Result<Self, F::Error>
  where
    Self: Sized,
    <Self as State<Partial<F>>>::Output: Sized,
  {
    Ok(input)
  }
}

pub trait TryFromRef<'a, W, RB, UB, F>: State<Ref<'a, W, RB, UB, F>>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Transforms from the input type `PartialRef<'a, W, RB, UB, F>` into the current type `Self`.
  fn try_from_ref(
    ctx: &'a F::Context,
    input: <Self as State<Ref<'a, W, RB, UB, F>>>::Output,
  ) -> Result<Self, F::Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, W, RB, UB, F>>>::Output: Sized,
    RB: ReadBuf + 'a,
    UB: UnknownBuffer<RB, F>;
}

impl<'a, W, RB, UB, F, T> TryFromRef<'a, W, RB, UB, F> for T
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  T: State<Ref<'a, W, RB, UB, F>, Output = Self>,
{
  fn try_from_ref(
    _: &'a F::Context,
    input: <Self as State<Ref<'a, W, RB, UB, F>>>::Output,
  ) -> Result<Self, F::Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, W, RB, UB, F>>>::Output: Sized,
    RB: ReadBuf + 'a,
    UB: UnknownBuffer<RB, F>,
  {
    Ok(input)
  }
}
