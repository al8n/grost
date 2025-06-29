use crate::{
  flavors::{Flavor, WireFormat},
  selection::Selectable,
};

use super::{Partial, State};

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

pub trait IdentityPartialTransform<W, F>: PartialTransform<Self, Self::Output, W, F>
where
  W: WireFormat<F>,
  F: Flavor + ?Sized,
  Self: Sized + Selectable<F> + State<Partial<F>>,
  Self::Output: Sized + Selectable<F, Selector = Self::Selector>,
{
}

impl<W, F, T> IdentityPartialTransform<W, F> for T
where
  W: WireFormat<F>,
  F: Flavor + ?Sized,
  T: Sized + Selectable<F> + State<Partial<F>> + PartialTransform<T, T::Output, W, F>,
  T::Output: Sized + Selectable<F, Selector = T::Selector>,
{
}
