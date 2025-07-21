use std::collections::BTreeMap;

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

impl<K, V> crate::encode::Length for BTreeMap<K, V> {
  fn len(&self) -> usize {
    self.len()
  }
}

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

impl<K, V> TryFromPartial<Groto> for BTreeMap<K, V>
where
  K: TryFromPartial<Groto> + Ord,
  K::Output: Sized,
  V: TryFromPartial<Groto>,
  V::Output: Sized,
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
    let mut map = BTreeMap::new();

    for ent in input.into_iter() {
      let (k, v) = ent
        .and_then(
          |k| K::try_from_partial(ctx, k),
          |v| V::try_from_partial(ctx, v),
        )?
        .try_into_entry()?
        .into();
      ctx.err_duplicated_map_keys(map.insert(k, v).is_some())?;
    }

    ctx.err_length_mismatch(expected, map.len())?;

    Ok(map)
  }
}
