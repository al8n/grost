use core::mem::MaybeUninit;

use crate::{
  Decoded, State,
  buffer::Buffer,
  decode::{Decode, Transform},
  flavors::{
    Network,
    network::{Error, PackedDecoder, Unknown, impls::packed_decoder::sealed},
  },
};

impl<'a, T, W, UB, const N: usize> Transform<Network, W, PackedDecoder<'a, T, UB, W>> for [T; N]
where
  W: sealed::Sealed + 'a,
  T: State<Decoded<'a, Network, W, UB>, Input = &'a [u8]>
    + Decode<'a, Network, W, T::Output, UB>
    + Transform<Network, W, T::Output>
    + 'a,
  T::Output: Sized,
  UB: Buffer<Unknown<&'a [u8]>> + 'a,
{
  fn transform(
    input: PackedDecoder<'a, T, UB, W>,
  ) -> Result<Self, <Network as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    let mut array: [MaybeUninit<T>; N] = core::array::from_fn(|_| MaybeUninit::uninit());
    for (index, res) in input.enumerate() {
      let (_, item) = res?;
      if index >= N {
        return Err(Error::custom(format!(
          "expected array of length {N}, but got more elements"
        )));
      }
      let item = T::transform(item)?;
      array[index].write(item);
    }

    // Safety: We have filled all elements of the array with initialized values.
    // TODO(al8n): remove the `unsafe` block when https://github.com/rust-lang/rust/issues/79711 is resolved.
    Ok(unsafe { array.map(|item| item.assume_init()) })
  }
}
