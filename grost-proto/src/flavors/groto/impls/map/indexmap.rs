use indexmap_2::IndexMap;

use crate::{
  buffer::Buffer,
  convert::{Flattened, Inner, MapKey, MapValue, Partial, TryFromPartial},
  flavors::{
    Groto,
    groto::{Context, Error},
  },
  selection::Selectable,
  state::State,
};

mod packed;
mod repeated;

impl<K, V, S> State<Flattened<Inner>> for IndexMap<K, V, S> {
  type Output = (K, V);
}

impl<K, V, S> State<Flattened<MapKey>> for IndexMap<K, V, S> {
  type Output = K;
}

impl<K, V, S> State<Flattened<MapValue>> for IndexMap<K, V, S> {
  type Output = V;
}

impl<K, V, S> State<Partial<Groto>> for IndexMap<K, V, S>
where
  K: State<Partial<Groto>>,
  K::Output: Sized,
  V: State<Partial<Groto>>,
  V::Output: Sized,
{
  type Output = super::DefaultPartialMapBuffer<K::Output, V::Output>;
}

impl<K, V, S> Selectable<Groto> for IndexMap<K, V, S>
where
  K: Selectable<Groto>,
  V: Selectable<Groto>,
{
  type Selector = super::MapSelector<K::Selector, V::Selector>;
}

impl<K, V, S> TryFromPartial<Groto> for IndexMap<K, V, S>
where
  K: TryFromPartial<Groto> + Eq + core::hash::Hash,
  K::Output: Sized,
  V: TryFromPartial<Groto>,
  V::Output: Sized,
  S: Default + core::hash::BuildHasher,
{
  fn try_from_partial(
    ctx: &Context,
    input: <Self as State<Partial<Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Partial<Groto>>>::Output: Sized,
  {
    let expected = input.len();
    let mut map = IndexMap::with_capacity_and_hasher(expected, S::default());

    for ent in input.into_iter() {
      let (k, v) = ent
        .and_then(
          |k| K::try_from_partial(ctx, k),
          |v| V::try_from_partial(ctx, v),
        )?
        .try_into_entry()?
        .into();
      if map.insert(k, v).is_some() && ctx.err_on_duplicated_map_keys() {
        return Err(Error::custom("duplicated keys in map"));
      }
    }

    if map.len() != expected && ctx.err_on_length_mismatch() {
      return Err(Error::custom(format!(
        "expected {expected} elements in map, but got {} elements",
        map.len()
      )));
    }

    Ok(map)
  }
}
