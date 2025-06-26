use crate::flavors::{Flavor, WireFormat};

/// A trait for transforming the input type `I` into the output type `O`
pub trait Transform<I, O, W, F>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Transforms from the input type `I` into the current type `Self`.
  fn transform(input: I) -> Result<O, F::Error>;
}
