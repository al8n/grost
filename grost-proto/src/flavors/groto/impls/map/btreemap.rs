use std::collections::BTreeMap;

use crate::{
  convert::{Extracted, Inner, MapKey, MapValue},
  flavors::Groto,
  selection::Selectable,
  state::{Partial, State},
};

mod packed;
mod repeated;

impl<K, V> crate::encode::Length for BTreeMap<K, V> {
  fn length(&self) -> usize {
    self.len()
  }
}

impl<K, V> State<Extracted<Inner>> for BTreeMap<K, V> {
  type Output = (K, V);
}

impl<K, V> State<Extracted<MapKey>> for BTreeMap<K, V> {
  type Output = K;
}

impl<K, V> State<Extracted<MapValue>> for BTreeMap<K, V> {
  type Output = V;
}

impl<K, V> State<Partial<Groto>> for BTreeMap<K, V>
where
  V: State<Partial<Groto>>,
  V::Output: Sized,
{
  type Output = BTreeMap<K, V::Output>;
}

impl<K, V> Selectable<Groto> for BTreeMap<K, V>
where
  V: Selectable<Groto>,
{
  type Selector = V::Selector;
}

// impl<K, V> TryFromPartial<Groto> for BTreeMap<K, V>
// where
//   K: TryFromPartial<Groto> + Ord,
//   K::Output: Sized,
//   V: TryFromPartial<Groto>,
//   V::Output: Sized,
// {
//   fn try_from_partial(
//     ctx: &Context,
//     input: <Self as State<Partial<Groto>>>::Output,
//   ) -> Result<Self, Error>
//   where
//     Self: Sized,
//     <Self as State<Partial<Groto>>>::Output: Sized,
//   {
//     let expected = input.len();
//     let mut map = BTreeMap::new();

//     for (k, v) in input.into_iter() {
//       ctx.err_duplicated_map_keys(map.insert(k, V::try_from_partial(ctx, v)?).is_some())?;
//     }

//     ctx.err_length_mismatch(expected, map.len())?;

//     Ok(map)
//   }
// }
