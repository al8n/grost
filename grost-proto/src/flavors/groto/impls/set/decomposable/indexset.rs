use indexmap_2::IndexSet;

use core::hash::{BuildHasher, Hash};

use crate::{
  buffer::Buffer,
  convert::{Extracted, Inner, PartialIdentity, TryFromPartial},
  flavors::{
    Groto,
    groto::{Context, Error},
  },
  selection::Selectable,
  state::{Partial, State},
  utils::Decomposable,
};

use super::super::DefaultPartialSetBuffer;

mod packed;
mod repeated;

impl<K, S> crate::encode::Length for Decomposable<IndexSet<K, S>> {
  fn length(&self) -> usize {
    self.0.len()
  }
}

impl<K, S> State<Extracted<Inner>> for Decomposable<IndexSet<K, S>> {
  type Output = K;
}

impl<K, S> State<Partial<Groto>> for Decomposable<IndexSet<K, S>>
where
  K: State<Partial<Groto>>,
  K::Output: Sized,
{
  type Output = DefaultPartialSetBuffer<K::Output>;
}

impl<K, S> Selectable<Groto> for Decomposable<IndexSet<K, S>>
where
  K: Selectable<Groto>,
{
  type Selector = K::Selector;
}

impl<K, S> PartialIdentity<Groto> for Decomposable<IndexSet<K, S>>
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

impl<K, S> TryFromPartial<Groto> for Decomposable<IndexSet<K, S>>
where
  K: TryFromPartial<Groto> + Eq + Hash,
  K::Output: Sized,
  S: BuildHasher + Default,
{
  fn try_from_partial(ctx: &Context, input: Self::Output) -> Result<Self, Error> {
    let mut set = IndexSet::with_capacity_and_hasher(input.len(), S::default());

    for item in input.into_iter() {
      ctx.err_duplicated_set_keys(!set.insert(K::try_from_partial(ctx, item)?))?;
    }

    Ok(set.into())
  }
}
