use core::mem::MaybeUninit;

use crate::{
  Decoded, State,
  buffer::Buffer,
  decode::{Decode, Transform},
  flavors::{
    Network, WireFormat,
    network::{Error, PackedDecoder, Unknown},
  },
};

impl<'a, T, W, TW, UB, const N: usize> Transform<Network, W, PackedDecoder<'a, T, UB, TW>>
  for [T; N]
where
  W: WireFormat<Network> + 'a,
  TW: WireFormat<Network> + 'a,
  T: State<Decoded<'a, Network, TW, UB>, Input = &'a [u8]>
    + Decode<'a, Network, TW, T::Output, UB>
    + Transform<Network, TW, T::Output>
    + 'a,
  T::Output: Sized,
  UB: Buffer<Unknown<&'a [u8]>> + 'a,
{
  fn transform(
    input: PackedDecoder<'a, T, UB, TW>,
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

// impl<'a, T, W, TW, UB> PartialTransform<Network, W, PackedDecoder<'a, T, UB, TW>> for Vec<T>
// where
//   W: WireFormat<Network> + 'a,
//   TW: WireFormat<Network> + 'a,
//   T: State<Decoded<'a, Network, TW, UB>, Input = &'a [u8]>
//     + Decode<'a, Network, TW, T::Output, UB>
//     + Selectable<Network, TW>
//     + PartialTransform<Network, TW, T::Output>
//     + 'a,
//   T::Output: Sized + Selectable<Network, TW, Selector = T::Selector>,
//   UB: Buffer<Unknown<&'a [u8]>> + 'a,
//   Self: Selectable<Network, W, Selector = T::Selector>,
// {
//   fn partial_transform(input: PackedDecoder<'a, T, UB, TW>, selector: &T::Selector) -> Result<Option<Self>, <Network as crate::flavors::Flavor>::Error>
//   where
//     Self: Sized
//   {
//     if selector.is_empty() {
//       return Ok(None);
//     }

//     input.into_iter()
//       .filter_map(|res| {
//         match res.and_then(|(_, inp)| T::partial_transform(inp, selector)) {
//           Ok(val) => val.map(|val| Ok(val)),
//           Err(e) => Some(Err(e)),
//         }
//       })
//       .collect::<Result<Vec<T>, _>>()
//       .map(|val| if val.is_empty() {
//         None
//       } else {
//         Some(val)
//       })
//   }
// }
