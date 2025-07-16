#[cfg(all(not(feature = "std"), feature = "alloc", feature = "hashbrown_0_15"))]
use hashbrown_0_15::HashSet;

#[cfg(feature = "std")]
use std::collections::HashSet;

use core::hash::{BuildHasher, Hash};

use crate::{
  convert::{Flattened, Inner, Partial, PartialIdentity, TryFromPartial},
  flavors::{
    Groto,
    groto::{Context, Error},
  },
  selection::Selectable,
  state::State,
};

use super::DefaultPartialSetBuffer;

mod packed;
mod repeated;

impl<K, S> State<Flattened<Inner>> for HashSet<K, S> {
  type Output = K;
}

impl<K, S> State<Partial<Groto>> for HashSet<K, S>
where
  K: State<Partial<Groto>>,
  K::Output: Sized,
{
  type Output = super::DefaultPartialSetBuffer<K::Output>;
}

impl<K, S> Selectable<Groto> for HashSet<K, S>
where
  K: Selectable<Groto>,
{
  type Selector = K::Selector;
}

impl<K, S> PartialIdentity<Groto> for HashSet<K, S>
where
  K: PartialIdentity<Groto> + Ord,
  K::Output: Sized + Selectable<Groto, Selector = K::Selector>,
{
  fn partial_identity<'a>(
    input: &'a mut Self::Output,
    selector: &'a Self::Selector,
  ) -> &'a mut Self::Output {
    input.as_mut_slice().iter_mut().for_each(|item| {
      K::partial_identity(item, selector);
    });

    input
  }
}

impl<K, S> TryFromPartial<Groto> for HashSet<K, S>
where
  K: TryFromPartial<Groto> + Eq + Hash,
  K::Output: Sized,
  S: BuildHasher + Default,
{
  fn try_from_partial(ctx: &Context, input: Self::Output) -> Result<Self, Error> {
    let mut set = HashSet::with_capacity_and_hasher(input.len(), S::default());

    for item in input {
      let item = K::try_from_partial(ctx, item)?;
      if !set.insert(item) && ctx.err_on_duplicated_set_keys() {
        return Err(Error::custom("duplicated keys in set"));
      }
    }

    Ok(set)
  }
}
