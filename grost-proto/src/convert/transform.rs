use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  convert::{Partial, PartialRef, Ref, State},
  flavors::{Flavor, WireFormat},
};

/// A trait for transforming the input type `I` into the output type `O`
pub trait Transform<I, O, W, F>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Transforms from the input type `I` into the current type `Self`.
  fn transform(input: I) -> Result<O, F::Error>;
}

pub trait TryFromPartialRef<'a, RB, UB, W, F>: State<PartialRef<'a, RB, UB, W, F>>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Transforms from the input type `PartialRef<'a, RB, UB, W, F>` into the current type `Self`.
  fn try_from_partial_ref(
    ctx: &'a F::Context,
    input: <Self as State<PartialRef<'a, RB, UB, W, F>>>::Output,
  ) -> Result<Self, F::Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, RB, UB, W, F>>>::Output: Sized,
    RB: ReadBuf,
    UB: UnknownBuffer<RB, F>;
}

impl<'a, RB, UB, W, F, T> TryFromPartialRef<'a, RB, UB, W, F> for T
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  T: State<PartialRef<'a, RB, UB, W, F>, Output = Self>,
{
  fn try_from_partial_ref(
    _: &'a F::Context,
    input: <Self as State<PartialRef<'a, RB, UB, W, F>>>::Output,
  ) -> Result<Self, F::Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, RB, UB, W, F>>>::Output: Sized,
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

pub trait TryFromRef<'a, RB, UB, W, F>: State<Ref<'a, RB, UB, W, F>>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Transforms from the input type `PartialRef<'a, RB, UB, W, F>` into the current type `Self`.
  fn try_from_ref(
    ctx: &'a F::Context,
    input: <Self as State<Ref<'a, RB, UB, W, F>>>::Output,
  ) -> Result<Self, F::Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, RB, UB, W, F>>>::Output: Sized,
    RB: ReadBuf + 'a,
    UB: UnknownBuffer<RB, F>;
}

impl<'a, RB, UB, W, F, T> TryFromRef<'a, RB, UB, W, F> for T
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  T: State<Ref<'a, RB, UB, W, F>, Output = Self>,
{
  fn try_from_ref(
    _: &'a F::Context,
    input: <Self as State<Ref<'a, RB, UB, W, F>>>::Output,
  ) -> Result<Self, F::Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, RB, UB, W, F>>>::Output: Sized,
    RB: ReadBuf + 'a,
    UB: UnknownBuffer<RB, F>,
  {
    Ok(input)
  }
}
