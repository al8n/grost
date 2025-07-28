use crate::{
  buffer::{ReadBuf, UnknownBuffer, WriteBuf},
  flavors::{Flavor, WireFormat},
  selection::Selectable,
  state::{Partial, PartialRef, Ref, State},
};

pub trait TryTransform<I, F: Flavor + ?Sized> {
  fn try_transform(context: &F::Context, input: I) -> Result<Self, F::Error>
  where
    Self: Sized;
}

pub trait Transform<I, F: Flavor + ?Sized> {
  fn transform(context: &F::Context, input: I) -> Self
  where
    Self: Sized;
}

pub trait SelectFrom<I, F: Flavor + ?Sized> {
  /// Selects fields from the input type `I` based on the selector.
  ///
  /// This method returns an `Option<Self>` where `Some` contains the selected fields
  /// and `None` indicates that no fields were selected.
  fn select_from(context: &F::Context, input: I, selector: &Self::Selector) -> Option<Self>
  where
    Self: Sized + Selectable<F>,
    I: Selectable<F, Selector = Self::Selector>;
}

pub trait TrySelectFrom<I, F: Flavor + ?Sized>: Selectable<F> {
  /// Attempts to select fields from the input type `I` based on the selector.
  ///
  /// This method returns a `Result<Self, F::Error>` where `Ok` contains the selected fields
  /// and `Err` indicates an error during selection.
  fn try_select_from(
    context: &F::Context,
    input: I,
    selector: &Self::Selector,
  ) -> Result<Option<Self>, F::Error>
  where
    Self: Sized + Selectable<F>,
    I: Selectable<F, Selector = Self::Selector>;
}

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
