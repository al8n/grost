#[cfg(all(not(feature = "std"), feature = "alloc", feature = "hashbrown_0_15"))]
use hashbrown_0_15::HashMap;

#[cfg(feature = "std")]
use std::collections::HashMap;

use crate::{
  convert::{Flattened, Inner, MapKey, MapValue, Partial},
  flavors::Groto,
  selection::Selectable,
  state::State,
};

mod packed;
mod repeated;

impl<K, V, S> State<Flattened<Inner>> for HashMap<K, V, S> {
  type Output = (K, V);
}

impl<K, V, S> State<Flattened<MapKey>> for HashMap<K, V, S> {
  type Output = K;
}

impl<K, V, S> State<Flattened<MapValue>> for HashMap<K, V, S> {
  type Output = V;
}

impl<K, V, S> State<Partial<Groto>> for HashMap<K, V, S>
where
  K: State<Partial<Groto>>,
  K::Output: Sized,
  V: State<Partial<Groto>>,
  V::Output: Sized,
{
  type Output = super::DefaultPartialMapBuffer<K::Output, V::Output>;
}

impl<K, V, S> Selectable<Groto> for HashMap<K, V, S>
where
  K: Selectable<Groto>,
  V: Selectable<Groto>,
{
  type Selector = super::MapSelector<K::Selector, V::Selector>;
}
