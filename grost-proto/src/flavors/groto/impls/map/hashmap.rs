#[cfg(all(not(feature = "std"), feature = "alloc", feature = "hashbrown_0_15"))]
use hashbrown_0_15::HashMap;

#[cfg(feature = "std")]
use std::collections::HashMap;

use crate::{
  convert::{Flattened, Inner, MapKey, MapValue, State},
  flavors::{DefaultMapWireFormat, Groto, PackedEntry, WireFormat},
};

impl<K, V, S> State<Flattened<Inner>> for HashMap<K, V, S> {
  type Output = (K, V);
}

impl<K, V, S> State<Flattened<MapKey>> for HashMap<K, V, S>
where
  K: State<Flattened<MapKey>>,
{
  type Output = K::Output;
}

impl<K, V, S> State<Flattened<MapValue>> for HashMap<K, V, S>
where
  V: State<Flattened<MapValue>>,
{
  type Output = V::Output;
}

impl<K, V, S> DefaultMapWireFormat<Groto> for HashMap<K, V, S> {
  type Format<KM, VM>
    = PackedEntry<KM, VM>
  where
    KM: WireFormat<Groto>,
    VM: WireFormat<Groto>;
}
