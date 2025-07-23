macro_rules! selectable {
  (<$g:ident> $ty:ty) => {
    impl<$g: ?Sized + $crate::__private::selection::Selectable<$crate::__private::flavors::Groto>>
      $crate::__private::selection::Selectable<$crate::__private::flavors::Groto> for $ty
    {
      type Selector = <$g as $crate::__private::selection::Selectable<
        $crate::__private::flavors::Groto,
      >>::Selector;
    }
  };
}

#[cfg(any(feature = "std", feature = "alloc"))]
mod arc;
#[cfg(any(feature = "std", feature = "alloc"))]
mod boxed;
#[cfg(any(feature = "std", feature = "alloc"))]
mod rc;
