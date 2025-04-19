#[cfg(any(feature = "std", feature = "alloc"))]
macro_rules! from_str {
  ($ty:ident::$new:ident.$try_push_str:ident($src: expr)) => {{
    let mut string = $ty::$new();
    string
      .$try_push_str($src)
      .map(|_| string)
      .map_err(|_| $crate::__private::larger_than_str_capacity::<N>())
  }};
}

#[cfg(any(feature = "std", feature = "alloc"))]
macro_rules! decode_str {
  ($ty:ident::$new:ident.$try_push_str:ident($src:expr)) => {{
    $crate::__private::from_utf8($src)
      .map(|s| {
        ($src.len(), {
          let mut string = $ty::$new();
          string
            .$try_push_str(s)
            .expect("string must be within bounds");
          string
        })
      })
      .map_err(|_| $crate::__private::flavors::network::DecodeError::custom("invalid UTF-8"))
  }};
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[allow(clippy::module_inception)]
mod string;

#[cfg(any(feature = "std", feature = "alloc"))]
mod arc;

#[cfg(any(feature = "std", feature = "alloc"))]
mod rc;

#[cfg(any(feature = "std", feature = "alloc"))]
mod boxed;

#[cfg(any(feature = "std", feature = "alloc"))]
mod smol_str;

#[cfg(feature = "heapless")]
mod heapless;

#[cfg(feature = "arrayvec")]
mod arrayvec;

#[cfg(feature = "tinystr")]
mod tinystr;

#[cfg(feature = "triomphe")]
mod triomphe;
