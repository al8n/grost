use std::collections::BTreeSet;

use crate::{
  convert::{Extracted, Inner, PartialIdentity},
  encode::Length,
  flavors::Groto,
  selection::Selectable,
  state::{Partial, State},
};

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

impl<K> State<Partial<Groto>> for BTreeSet<K> {
  type Output = Self;
}

impl<K> Selectable<Groto> for BTreeSet<K> {
  type Selector = bool;
}

impl<K> PartialIdentity<Groto> for BTreeSet<K> {
  fn partial_identity<'a>(
    input: &'a mut Self::Output,
    _: &'a Self::Selector,
  ) -> &'a mut Self::Output {
    input
  }
}
