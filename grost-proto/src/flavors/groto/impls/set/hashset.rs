#[cfg(all(not(feature = "std"), feature = "alloc", feature = "hashbrown_0_15"))]
use hashbrown_0_15::HashSet;

#[cfg(feature = "std")]
use std::collections::HashSet;

use crate::{
  convert::{Extracted, Inner, PartialIdentity},
  flavors::Groto,
  selection::Selectable,
  state::{Partial, State},
};

mod packed;
mod repeated;

impl<K, S> State<Extracted<Inner>> for HashSet<K, S> {
  type Output = K;
}

impl<K, S> State<Partial<Groto>> for HashSet<K, S> {
  type Output = Self;
}

impl<K, S> Selectable<Groto> for HashSet<K, S> {
  type Selector = bool;
}

impl<K, S> crate::encode::Length for HashSet<K, S> {
  fn length(&self) -> usize {
    self.len()
  }
}

impl<K, S> PartialIdentity<Groto> for HashSet<K, S> {
  fn partial_identity<'a>(
    input: &'a mut Self::Output,
    _: &'a Self::Selector,
  ) -> &'a mut Self::Output {
    input
  }
}

// impl<K, S> TryFromPartial<Groto> for HashSet<K, S>
// where
//   K: TryFromPartial<Groto> + Eq + Hash,
//   K::Output: Sized,
//   S: BuildHasher + Default,
// {
//   fn try_from_partial(ctx: &Context, input: Self::Output) -> Result<Self, Error> {
//     let mut set = HashSet::with_capacity_and_hasher(input.len(), S::default());

//     for item in input.into_iter() {
//       ctx.err_duplicated_set_keys(!set.insert(K::try_from_partial(ctx, item)?))?;
//     }

//     Ok(set)
//   }
// }
