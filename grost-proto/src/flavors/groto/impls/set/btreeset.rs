use std::collections::BTreeSet;

use crate::{
  buffer::Buffer,
  convert::{Extracted, Inner, PartialIdentity, TryFromPartial},
  encode::Length,
  flavors::{
    Groto,
    groto::{Context, Error},
  },
  selection::Selectable,
  state::{Partial, State},
};

use super::DefaultPartialSetBuffer;

mod packed;
mod repeated;

impl<K> Length for BTreeSet<K> {
  fn length(&self) -> usize {
    self.len()
  }
}

impl<K> State<Extracted<Inner>> for BTreeSet<K> {
  type Output = K;
}

impl<K> State<Partial<Groto>> for BTreeSet<K>
where
  K: State<Partial<Groto>>,
  K::Output: Sized,
{
  type Output = DefaultPartialSetBuffer<K::Output>;
}

impl<K> Selectable<Groto> for BTreeSet<K>
where
  K: Selectable<Groto>,
{
  type Selector = K::Selector;
}

impl<K> PartialIdentity<Groto> for BTreeSet<K>
where
  K: PartialIdentity<Groto>,
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

impl<K> TryFromPartial<Groto> for BTreeSet<K>
where
  K: TryFromPartial<Groto> + Ord,
  K::Output: Sized,
{
  fn try_from_partial(ctx: &Context, input: Self::Output) -> Result<Self, Error> {
    let mut set = BTreeSet::new();

    for item in input.into_inner() {
      ctx.err_duplicated_set_keys(!set.insert(K::try_from_partial(ctx, item)?))?;
    }

    Ok(set)
  }
}
