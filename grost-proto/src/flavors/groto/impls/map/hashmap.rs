#[cfg(all(not(feature = "std"), feature = "alloc", feature = "hashbrown_0_15"))]
use hashbrown_0_15::HashMap;

#[cfg(feature = "std")]
use std::collections::HashMap;

use crate::{
  convert::{Flattened, Inner, MapKey, MapValue, Partial, PartialRef},
  state::State,
  flavors::{
    DefaultMapWireFormat, DefaultRepeatedEntryWireFormat, Groto, MergedWireFormat, PackedEntry,
    RepeatedEntry, WireFormat, groto::PackedEntriesDecoder,
  },
};

impl<K, V, S> State<Flattened<Inner>> for HashMap<K, V, S> {
  type Output = (K, V);
}

impl<K, V, S> State<Flattened<MapKey>> for HashMap<K, V, S> {
  type Output = K;
}

impl<K, V, S> State<Flattened<MapValue>> for HashMap<K, V, S> {
  type Output = V;
}

impl<K, V, S> DefaultMapWireFormat<Groto> for HashMap<K, V, S> {
  type Format<KM, VM>
    = PackedEntry<KM, VM>
  where
    KM: WireFormat<Groto> + 'static,
    VM: WireFormat<Groto> + 'static;
}

impl<K, V, S> DefaultRepeatedEntryWireFormat<Groto> for HashMap<K, V, S> {
  type Format<KM, VM, const TAG: u32>
    = RepeatedEntry<KM, VM, TAG>
  where
    KM: WireFormat<Groto> + 'static,
    VM: WireFormat<Groto> + 'static,
    MergedWireFormat<KM, VM>: WireFormat<Groto> + 'static;
}

impl<K, V, S> State<Partial<Groto>> for HashMap<K, V, S>
where
  V: State<Partial<Groto>>,
  V::Output: Sized,
{
  type Output = HashMap<K, V::Output, S>;
}

impl<'a, K, V, KW, VW, S, RB, B> State<PartialRef<'a, RB, B, PackedEntry<KW, VW>, Groto>>
  for HashMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  PackedEntry<KW, VW>: WireFormat<Groto> + 'a,
{
  type Output = PackedEntriesDecoder<'a, K, V, RB, B, KW, VW>;
}
