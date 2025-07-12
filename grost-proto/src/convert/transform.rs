use crate::{
  buffer::{UnknownBuffer, ReadBuf},
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
    input: <Self as State<PartialRef<'a, RB, UB, W, F>>>::Output,
  ) -> Result<Self, F::Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, RB, UB, W, F>>>::Output: Sized,
    RB: ReadBuf,
    UB: UnknownBuffer<RB, F>;
}

pub trait TryFromPartial<W, F>: State<Partial<F>>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Transforms from the input type `Partial<F>` into the current type `Self`.
  fn try_from_partial(input: <Self as State<Partial<F>>>::Output) -> Result<Self, F::Error>
  where
    Self: Sized,
    <Self as State<Partial<F>>>::Output: Sized;
}

pub trait TryFromRef<'a, RB, UB, W, F>: State<Ref<'a, RB, UB, W, F>>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Transforms from the input type `PartialRef<'a, RB, UB, W, F>` into the current type `Self`.
  fn try_from_ref(input: <Self as State<Ref<'a, RB, UB, W, F>>>::Output) -> Result<Self, F::Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, RB, UB, W, F>>>::Output: Sized,
    RB: ReadBuf + 'a,
    UB: UnknownBuffer<RB, F>;
}
