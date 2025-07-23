use crate::{
  flavors::{Flavor, WireFormat, groto::Context},
  selection::Selectable,
  state::{Partial, PartialRef, State},
};

pub trait PartialTryFromRef<'a, RB, UB, W, F>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  RB: ?Sized,
  UB: ?Sized,
  Self: Selectable<F> + State<PartialRef<'a, RB, UB, W, F>> + State<Partial<F>>,
  <Self as State<PartialRef<'a, RB, UB, W, F>>>::Output: Selectable<F, Selector = Self::Selector>,
  <Self as State<Partial<F>>>::Output: Selectable<F, Selector = Self::Selector>,
{
  /// Partially transforms from the input type `PartialRef<'a, RB, UB, W, F>` into the current type `Self`.
  fn partial_try_from_ref(
    context: &'a Context,
    input: <Self as State<PartialRef<'a, RB, UB, W, F>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as State<Partial<F>>>::Output, F::Error>
  where
    <Self as State<Partial<F>>>::Output: Sized,
    <Self as State<PartialRef<'a, RB, UB, W, F>>>::Output: Sized;
}

impl<'a, RB, UB, W, F, T> PartialTryFromRef<'a, RB, UB, W, F> for T
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  RB: ?Sized,
  UB: ?Sized,
  Self: Selectable<F>
    + PartialIdentity<F>
    + State<PartialRef<'a, RB, UB, W, F>, Output = <Self as State<Partial<F>>>::Output>
    + State<Partial<F>>,
  <Self as State<PartialRef<'a, RB, UB, W, F>>>::Output: Selectable<F, Selector = Self::Selector>,
  <Self as State<Partial<F>>>::Output: Sized + Selectable<F, Selector = Self::Selector>,
{
  fn partial_try_from_ref(
    _: &'a Context,
    mut input: <Self as State<PartialRef<'a, RB, UB, W, F>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as State<Partial<F>>>::Output, F::Error>
  where
    <Self as State<Partial<F>>>::Output: Sized,
    <Self as State<PartialRef<'a, RB, UB, W, F>>>::Output: Sized,
  {
    <Self as PartialIdentity<F>>::partial_identity(&mut input, selector);
    Ok(input)
  }
}

pub trait PartialIdentity<F>: State<Partial<F>>
where
  F: Flavor + ?Sized,
  Self: Selectable<F>,
  Self::Output: Sized + Selectable<F, Selector = Self::Selector>,
{
  fn partial_identity<'a>(
    input: &'a mut Self::Output,
    selector: &'a Self::Selector,
  ) -> &'a mut Self::Output;
}
