use crate::{
  convert::{Flattened, MapKey, MapValue, State},
  marker::{MapMarker, Marker},
};

use super::{DefaultWireFormat, Flavor, StaticWireFormat};

/// The default wire format for a map type on flavor `F`.
pub trait DefaultMapWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format<K, V>: StaticWireFormat<F>
  where
    K: StaticWireFormat<F>,
    V: StaticWireFormat<F>;
}

impl<T, KM, VM, F> DefaultWireFormat<F> for MapMarker<T, KM, VM>
where
  KM: DefaultWireFormat<F> + Marker + ?Sized,
  KM::Marked: Sized,
  VM: DefaultWireFormat<F> + Marker + ?Sized,
  VM::Marked: Sized,
  T: ?Sized
    + State<Flattened<MapKey>, Output = KM::Marked>
    + State<Flattened<MapValue>, Output = VM::Marked>
    + DefaultMapWireFormat<F>,
  F: ?Sized + Flavor,
{
  type Format = T::Format<KM::Format, VM::Format>;
}
