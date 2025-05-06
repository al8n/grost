/// A trait that means this type is indexable.
pub trait Indexable<F: ?Sized> {
  /// The index type of this indexable.
  type Indexer: Copy + core::fmt::Debug;
}

impl<F: ?Sized, T: Indexable<F>> Indexable<F> for Option<T> {
  type Indexer = T::Indexer;
}

#[cfg(any(feature = "std", feature = "alloc", feature = "triomphe_0_1"))]
macro_rules! impl_indexable {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<F: ?Sized, T: Indexable<F> + ?Sized> Indexable<F> for $ty {
        type Indexer = T::Indexer;
      }
    )*
  };
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::{boxed::Box, rc::Rc, sync::Arc};

  impl_indexable! {
    Box<T>,
    Rc<T>,
    Arc<T>,
  };
};

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  use triomphe_0_1::Arc;

  impl_indexable! {
    Arc<T>,
  };
};
