use ghost::phantom;

use crate::flavors::{Flavor, WireFormat};

/// A marker type for merging two wire formats to a single wire format.
#[derive(PartialEq, Eq, Hash, Debug)]
#[phantom]
pub struct MergedWireFormat<W1: ?Sized, W2: ?Sized>;

impl<W1: ?Sized, W2: ?Sized> Clone for MergedWireFormat<W1, W2> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<W1: ?Sized, W2: ?Sized> Copy for MergedWireFormat<W1, W2> {}

impl<W1, W2> core::fmt::Display for MergedWireFormat<W1, W2>
where
  W1: ?Sized,
  W2: ?Sized,
{
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "merged")
  }
}

/// A trait for merging two wire formats into a single wire format.
pub trait MergeableWireFormat<F: ?Sized + Flavor, Rhs = Self> {
  /// The merged wire format of the two wire formats.
  type Merged: WireFormat<F>;
}
