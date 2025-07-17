use std::collections::BTreeMap;

use crate::{
  convert::{Flattened, Inner, MapKey, MapValue, Partial},
  flavors::Groto,
  selection::Selectable,
  state::State,
};

mod packed;
mod repeated;

impl<K, V> State<Flattened<Inner>> for BTreeMap<K, V> {
  type Output = (K, V);
}

impl<K, V> State<Flattened<MapKey>> for BTreeMap<K, V> {
  type Output = K;
}

impl<K, V> State<Flattened<MapValue>> for BTreeMap<K, V> {
  type Output = V;
}

impl<K, V> State<Partial<Groto>> for BTreeMap<K, V>
where
  K: State<Partial<Groto>>,
  K::Output: Sized,
  V: State<Partial<Groto>>,
  V::Output: Sized,
{
  type Output = super::DefaultPartialMapBuffer<K::Output, V::Output>;
}

impl<K, V> Selectable<Groto> for BTreeMap<K, V>
where
  K: Selectable<Groto>,
  V: Selectable<Groto>,
{
  type Selector = super::MapSelector<K::Selector, V::Selector>;
}
