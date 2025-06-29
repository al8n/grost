use core::mem::MaybeUninit;

use crate::{
  buffer::{Buffer, ReadBuf},
  convert::{PartialRef, PartialTransform, State, Transform},
  decode::Decode,
  flavors::{
    groto::{Error, Fixed8, LengthDelimited, PackedDecoder, Unknown}, Groto, WireFormat
  },
};

impl<'a, T, W, TW, B, UB, const N: usize> Transform<PackedDecoder<'a, T, B, UB, TW>, Self, W, Groto>
  for [T; N]
where
  W: WireFormat<Groto> + 'a,
  TW: WireFormat<Groto> + 'a,
  T: State<PartialRef<'a, B, UB, TW, Groto>>
    + Decode<'a, Groto, TW, T::Output, B, UB>
    + Transform<T::Output, T, TW, Groto>
    + 'a,
  T::Output: Sized,
  UB: Buffer<Unknown<B>> + 'a,
  B: ReadBuf + 'a,
{
  fn transform(
    input: PackedDecoder<'a, T, B, UB, TW>,
  ) -> Result<Self, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    let mut array: [MaybeUninit<T>; N] = core::array::from_fn(|_| MaybeUninit::uninit());
    for (index, res) in input.enumerate() {
      let (_, item) = res?;
      if index >= N {
        #[cfg(any(feature = "alloc", feature = "std"))]
        let err_msg = ::std::format!(
          "expected array of length {N}, but got more elements"
        );
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

impl<'a, B, UB> PartialTransform<PackedDecoder<'a, u8, B, UB, Fixed8>, Option<Self>, LengthDelimited, Groto>
  for Vec<u8>
where
  UB: Buffer<Unknown<B>> + 'a,
  B: ReadBuf + 'a,
{
  fn partial_transform(input: PackedDecoder<'a, u8, B, UB, Fixed8>, selector: &Self::Selector) -> Result<Option<Self>, <Groto as crate::flavors::Flavor>::Error> {
    if *selector {
      Ok(Some(Vec::from(input.as_slice())))
    } else {
      Ok(None)
    }
  }
}