#[allow(unused_macros)]
macro_rules! bytes_message {
  ($ty:ty => $owned_ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialMessage<$crate::__private::flavors::Network> for $ty {
      type UnknownBuffer<B> = ();

      type Encoded<'a> = &'a [::core::primitive::u8]
        where
          Self: Sized + 'a;

      type Borrowed<'a> = &'a Self
        where
          Self: 'a;

      type EncodedOwned = $owned_ty
        where
          Self: Sized + 'static;
    }

    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Message<$crate::__private::flavors::Network> for $ty {
      type Partial = Self;

      type Encoded<'a> = &'a [::core::primitive::u8]
        where
          Self: Sized + 'a;

      type Borrowed<'a> = &'a Self
        where
          Self: 'a;

      type EncodedOwned = $owned_ty
        where
          Self: Sized + 'static;
    }
  };
}

mod array;

#[cfg(any(feature = "std", feature = "alloc"))]
mod arc;

#[cfg(any(feature = "std", feature = "alloc"))]
mod boxed;

#[cfg(any(feature = "std", feature = "alloc"))]
#[allow(clippy::module_inception)]
mod bytes;

// #[cfg(feature = "heapless")]
// mod heapless;

#[cfg(any(feature = "std", feature = "alloc"))]
mod rc;

#[cfg(any(feature = "std", feature = "alloc"))]
mod smallvec;

#[cfg(any(feature = "std", feature = "alloc"))]
mod triomphe;

mod tinyvec;

#[cfg(any(feature = "std", feature = "alloc"))]
mod vec;
