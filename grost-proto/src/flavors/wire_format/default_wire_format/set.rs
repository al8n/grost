use crate::{
  convert::{Flattened, Inner},
  marker::{Marker, SetMarker},
  state::State,
};

use super::{DefaultWireFormat, Flavor, WireFormat};

/// The default wire format for a set type on flavor `F`.
pub trait DefaultSetWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format<K>: WireFormat<F> + 'static
  where
    K: WireFormat<F> + 'static;
}

impl<T, KM, F> DefaultWireFormat<F> for SetMarker<T, KM>
where
  F: ?Sized + Flavor,
  KM: DefaultWireFormat<F> + Marker,
  T: State<Flattened<Inner>, Output = KM::Marked> + ?Sized + DefaultSetWireFormat<F>,
{
  type Format = T::Format<KM::Format>;
}
