use crate::{
  convert::{Flattened, Inner, State},
  marker::{Marker, SetMarker},
};

use super::{DefaultWireFormat, Flavor, StaticWireFormat};

/// The default wire format for a set type on flavor `F`.
pub trait DefaultSetWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format<K>: StaticWireFormat<F>
  where
    K: StaticWireFormat<F>;
}

impl<T, KM, F> DefaultWireFormat<F> for SetMarker<T, KM>
where
  F: ?Sized + Flavor,
  KM: DefaultWireFormat<F> + Marker,
  T: State<Flattened<Inner>, Output = KM::Marked> + ?Sized + DefaultSetWireFormat<F>,
{
  type Format = T::Format<KM::Format>;
}
