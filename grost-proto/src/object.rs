// use core::marker::PhantomData;

// use crate::{
//   buffer::{Chunk, ChunkMut, UnknownBuffer},
//   decode::{Decode, EquivalentDecode},
//   flavors::{Flavor, WireFormat},
// };

// /// The raw field returned by object decoders.
// ///
// /// This type
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct RawField<B, T, W, F: Flavor + ?Sized> {
//   identifier: F::Identifier,
//   data: B,
//   _t: PhantomData<T>,
//   _w: PhantomData<W>,
// }

// impl<RB, T, W, F: Flavor + ?Sized> RawField<RB, T, W, F> {
//   /// Creates a new `RawField` with the given identifier and data.
//   #[inline]
//   pub const fn new(identifier: F::Identifier, data: RB) -> Self {
//     Self {
//       identifier,
//       data,
//       _t: PhantomData,
//       _w: PhantomData,
//     }
//   }

//   /// Returns the identifier of the field.
//   #[inline]
//   pub const fn identifier(&self) -> &F::Identifier {
//     &self.identifier
//   }

//   /// Returns the data of the field.
//   ///
//   /// Note: The data does not include the wire type and the tag.
//   #[inline]
//   pub const fn data(&self) -> &RB {
//     &self.data
//   }

//   /// Consumes the `RawField` and returns the identifier and data.
//   #[inline]
//   pub fn into_components(self) -> (F::Identifier, RB) {
//     (self.identifier, self.data)
//   }

//   pub fn decode<'a, O, OW, UB>(self, ctx: &'a F::Context) -> Result<(usize, O), F::Error>
//   where
//     RB: Chunk + 'a,
//     O: Decode<'a, OW, RB, UB, F> + 'a,
//     OW: WireFormat<F>,
//     UB: UnknownBuffer<RB, F> + 'a,
//     T: EquivalentDecode<'a, O, OW, RB, UB, F, WireFormat = W, Flavor = F>,
//   {
//     O::decode(ctx, self.data)
//   }
// }
