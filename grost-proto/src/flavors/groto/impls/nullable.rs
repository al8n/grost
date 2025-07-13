use crate::{convert::Partial, flavors::Groto, selection::Selectable, state::State};

mod flatten;

#[allow(clippy::module_inception)]
mod nullable;

impl<T> State<Partial<Groto>> for Option<T>
where
  T: State<Partial<Groto>>,
  T::Output: Sized,
{
  type Output = Option<T::Output>;
}

impl<T> Selectable<Groto> for Option<T>
where
  T: Selectable<Groto>,
{
  type Selector = T::Selector;
}
