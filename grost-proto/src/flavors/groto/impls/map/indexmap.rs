use indexmap_2::IndexMap;

use crate::{
  convert::{Extracted, Inner, MapKey, MapValue},
  flavors::Groto,
  selection::Selectable,
  state::{Partial, State},
};

mod packed;
mod repeated;

impl<K, V, S> crate::encode::Length for IndexMap<K, V, S> {
  fn length(&self) -> usize {
    self.len()
  }
}

impl<K, V, S> State<Extracted<Inner>> for IndexMap<K, V, S> {
  type Output = (K, V);
}

impl<K, V, S> State<Extracted<MapKey>> for IndexMap<K, V, S> {
  type Output = K;
}

impl<K, V, S> State<Extracted<MapValue>> for IndexMap<K, V, S> {
  type Output = V;
}

impl<K, V, S> State<Partial<Groto>> for IndexMap<K, V, S>
where
  V: State<Partial<Groto>>,
  V::Output: Sized,
{
  type Output = IndexMap<K, V::Output, S>;
}

impl<K, V, S> Selectable<Groto> for IndexMap<K, V, S>
where
  V: Selectable<Groto>,
{
  type Selector = V::Selector;
}

// impl<K, V, S> TryFromPartial<Groto> for IndexMap<K, V, S>
// where
//   K: TryFromPartial<Groto> + Eq + core::hash::Hash,
//   K::Output: Sized,
//   V: TryFromPartial<Groto>,
//   V::Output: Sized,
//   S: Default + core::hash::BuildHasher,
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
//     let mut map = IndexMap::with_capacity_and_hasher(expected, S::default());

//     for (k, v) in input.into_iter() {
//       let v = V::try_from_partial(ctx, v)?;
//       ctx.err_duplicated_map_keys(map.insert(k, v).is_some())?;
//     }

//     ctx.err_length_mismatch(expected, map.len())?;

//     Ok(map)
//   }
// }
