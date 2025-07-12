use core::mem::MaybeUninit;

use crate::{
  buffer::{UnknownBuffer, ReadBuf},
  convert::{Partial, PartialRef, PartialTransform, Transform},
  decode::Decode,
  flavors::{
    Flavor, Groto, WireFormat,
    groto::{Error, PackedDecoder},
  },
  state::State,
  selection::{Selectable, Selector},
};

impl<'a, T, W, TW, RB, B, const N: usize> Transform<PackedDecoder<'a, T, RB, B, TW>, Self, W, Groto>
  for [T; N]
where
  W: WireFormat<Groto> + 'a,
  TW: WireFormat<Groto> + 'a,
  T: State<PartialRef<'a, RB, B, TW, Groto>>
    + Decode<'a, T::Output, TW, RB, B, Groto>
    + Transform<T::Output, T, TW, Groto>
    + 'a,
  T::Output: Sized,
  B: UnknownBuffer<RB, Groto> + 'a,
  RB: ReadBuf + 'a,
{
  fn transform(input: PackedDecoder<'a, T, RB, B, TW>) -> Result<Self, <Groto as Flavor>::Error>
  where
    Self: Sized,
  {
    let mut array: [MaybeUninit<T>; N] = core::array::from_fn(|_| MaybeUninit::uninit());
    for (index, res) in input.enumerate() {
      let (_, item) = res?;
      if index >= N {
        #[cfg(any(feature = "alloc", feature = "std"))]
        let err_msg = ::std::format!("expected array of length {N}, but got more elements");
        #[cfg(not(any(feature = "alloc", feature = "std")))]
        let err_msg = "got more elements than array capacity";
        return Err(Error::custom(err_msg));
      }
      let item = T::transform(item)?;
      array[index].write(item);
    }

    // Safety: We have filled all elements of the array with initialized values.
    // TODO(al8n): remove the `unsafe` block when https://github.com/rust-lang/rust/issues/79711 is resolved.
    Ok(unsafe { array.map(|item| item.assume_init()) })
  }
}

impl<'a, T, W, TW, RB, B, const N: usize>
  PartialTransform<PackedDecoder<'a, T, RB, B, TW>, Option<[Option<T>; N]>, W, Groto> for [T; N]
where
  W: WireFormat<Groto> + 'a,
  TW: WireFormat<Groto> + 'a,
  T: State<PartialRef<'a, RB, B, TW, Groto>>
    + Selectable<Groto>
    + Decode<'a, T::Output, TW, RB, B, Groto>
    + PartialTransform<
      <T as State<PartialRef<'a, RB, B, TW, Groto>>>::Output,
      ::core::option::Option<T>,
      TW,
      Groto,
    > + 'a,
  <T as State<PartialRef<'a, RB, B, TW, Groto>>>::Output:
    Sized + Selectable<Groto, Selector = T::Selector>,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn partial_transform(
    input: PackedDecoder<'a, T, RB, B, TW>,
    selector: &Self::Selector,
  ) -> Result<Option<[Option<T>; N]>, <Groto as Flavor>::Error> {
    if selector.is_empty() {
      return Ok(None);
    }
    let mut array: [MaybeUninit<Option<T>>; N] = core::array::from_fn(|_| MaybeUninit::uninit());
    for (index, res) in input.enumerate() {
      let (_, item) = res?;
      if index >= N {
        #[cfg(any(feature = "alloc", feature = "std"))]
        let err_msg = ::std::format!("expected array of length {N}, but got more elements");
        #[cfg(not(any(feature = "alloc", feature = "std")))]
        let err_msg = "got more elements than array capacity";
        return Err(Error::custom(err_msg));
      }
      let item = T::partial_transform(item, selector)?;
      array[index].write(item);
    }

    // Safety: We have filled all elements of the array with initialized values.
    // TODO(al8n): remove the `unsafe` block when https://github.com/rust-lang/rust/issues/79711 is resolved.
    let array = unsafe { array.map(|item| item.assume_init()) };
    if array.iter().all(|item| item.is_none()) {
      return Ok(None);
    }

    Ok(Some(array))
  }
}

impl<T, W, const N: usize> PartialTransform<[T; N], Option<[Option<T>; N]>, W, Groto> for [T; N]
where
  W: WireFormat<Groto>,
  T: PartialTransform<T, ::core::option::Option<T>, W, Groto>
    + Selectable<Groto>
    + State<Partial<Groto>>,
  T::Output: Sized + Selectable<Groto, Selector = T::Selector>,
{
  fn partial_transform(
    input: [T; N],
    selector: &Self::Selector,
  ) -> Result<Option<[Option<T>; N]>, <Groto as Flavor>::Error> {
    if selector.is_empty() {
      return Ok(None);
    }
    let mut array: [MaybeUninit<Option<T>>; N] = core::array::from_fn(|_| MaybeUninit::uninit());
    for (index, item) in input.into_iter().enumerate() {
      if index >= N {
        #[cfg(any(feature = "alloc", feature = "std"))]
        let err_msg = ::std::format!("expected array of length {N}, but got more elements");
        #[cfg(not(any(feature = "alloc", feature = "std")))]
        let err_msg = "got more elements than array capacity";
        return Err(Error::custom(err_msg));
      }
      let item = T::partial_transform(item, selector)?;
      array[index].write(item);
    }

    // Safety: We have filled all elements of the array with initialized values.
    // TODO(al8n): remove the `unsafe` block when https://github.com/rust-lang/rust/issues/79711 is resolved.
    let array = unsafe { array.map(|item| item.assume_init()) };
    if array.iter().all(|item| item.is_none()) {
      return Ok(None);
    }

    Ok(Some(array))
  }
}

// impl<T, W> PartialTransform<Vec<T>, Option<Vec<T>>, W, Groto>
//   for Vec<T>
// where
//   W: WireFormat<Groto>,
//   T: PartialTransform<
//       T,
//       ::core::option::Option<T>,
//       W,
//       Groto,
//     >
//     + Selectable<Groto>
//     + State<Partial<Groto>>,
//   T::Output: Sized +  Selectable<Groto, Selector = T::Selector>,
// {
//   fn partial_transform(input: Vec<T>, selector: &Self::Selector) -> Result<Option<Vec<T>>, <Groto as Flavor>::Error> {
//     // if selector.is_empty() {
//     //   return Ok(None);
//     // }
//     // let mut array: [MaybeUninit<Option<T>>; N] = core::array::from_fn(|_| MaybeUninit::uninit());
//     // for (index, item) in input.into_iter().enumerate() {
//     //   if index >= N {
//     //     #[cfg(any(feature = "alloc", feature = "std"))]
//     //     let err_msg = ::std::format!("expected array of length {N}, but got more elements");
//     //     #[cfg(not(any(feature = "alloc", feature = "std")))]
//     //     let err_msg = "got more elements than array capacity";
//     //     return Err(Error::custom(err_msg));
//     //   }
//     //   let item = T::partial_transform(item, selector)?;
//     //   array[index].write(item);
//     // }

//     // // Safety: We have filled all elements of the array with initialized values.
//     // // TODO(al8n): remove the `unsafe` block when https://github.com/rust-lang/rust/issues/79711 is resolved.
//     // let array = unsafe { array.map(|item| item.assume_init()) };
//     // if array.iter().all(|item| item.is_none()) {
//     //   return Ok(None);
//     // }

//     // Ok(Some(array))
//     todo!()
//   }
// }
