use indexmap_2::IndexSet;

use crate::{
  convert::{Extracted, Inner, PartialIdentity},
  flavors::Groto,
  selection::Selectable,
  state::{Partial, State},
};

mod packed;
mod repeated;

impl<K, S> crate::encode::Length for IndexSet<K, S> {
  fn length(&self) -> usize {
    self.len()
  }
}

impl<K, S> State<Extracted<Inner>> for IndexSet<K, S> {
  type Output = K;
}

impl<K, S> State<Partial<Groto>> for IndexSet<K, S> {
  type Output = IndexSet<K, S>;
}

impl<K, S> Selectable<Groto> for IndexSet<K, S> {
  type Selector = bool;
}

impl<K, S> PartialIdentity<Groto> for IndexSet<K, S>
where
  K: PartialIdentity<Groto> + Ord,
  K::Output: Sized + Selectable<Groto, Selector = K::Selector>,
{
  fn partial_identity<'a>(
    input: &'a mut Self::Output,
    _: &'a Self::Selector,
  ) -> &'a mut Self::Output {
    input
  }
}
