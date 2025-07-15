use crate::{
  flavors::{Flavor, WireFormat},
  selection::Selectable,
};

use super::{Partial, PartialRef, State};

/// A trait for partially transforming the input type `I` to the `<Self as State<STATE>>::Output`.
pub trait PartialTransform<I, O, W, F>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  I: Selectable<F, Selector = Self::Selector>,
  O: Selectable<F, Selector = Self::Selector>,
  Self: Selectable<F>,
{
  /// Partially transforms from the input type `I` into the current type `Self`.
  ///
  /// - If there is nothing selected, it returns `Ok(None)`.
  /// - If the selected fields are not present in the input, such fields will be `None` in the output.
  /// - If the selected fields are present, they will be partially transformed into the corresponding fields in the output type
  ///   according the the fields' selector.
  fn partial_transform(input: I, selector: &Self::Selector) -> Result<O, F::Error>;
}

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
