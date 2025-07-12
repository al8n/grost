use core::marker::PhantomData;

use crate::{buffer::{UnknownBuffer, ReadBuf}, decode::{Decode1, EquivalentDecode}, flavors::{Flavor, WireFormat}};

/// The raw field returned by object decoders.
/// 
/// This type 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawField<B, T, W, F: Flavor + ?Sized> {
  identifier: F::Identifier,
  data: B,
  _t: PhantomData<T>,
  _w: PhantomData<W>,
}

impl<RB, T, W, F: Flavor + ?Sized> RawField<RB, T, W, F> {
  /// Creates a new `RawField` with the given identifier and data.
  #[inline]
  pub const fn new(identifier: F::Identifier, data: RB) -> Self {
    Self {
      identifier,
      data,
      _t: PhantomData,
      _w: PhantomData,
    }
  }

  /// Returns the identifier of the field.
  #[inline]
  pub const fn identifier(&self) -> &F::Identifier {
    &self.identifier
  }

  /// Returns the data of the field.
  ///
  /// Note: The data does not include the wire type and the tag.
  #[inline]
  pub const fn data(&self) -> &RB {
    &self.data
  }

  /// Consumes the `RawField` and returns the identifier and data.
  #[inline]
  pub fn into_components(self) -> (F::Identifier, RB) {
    (self.identifier, self.data)
  }

  pub fn decode<'a, O, OW, OF, UB, D>(self, ctx: &F::Context) -> Result<O, F::Error>
  where
    RB: ReadBuf,
    D: FnOnce(RB) -> Result<O, F::Error>,
    O: Decode1<'a, OW, RB, UB, OF>,
    OW: WireFormat<OF>,
    OF: Flavor + ?Sized,
    UB: UnknownBuffer<RB, F>,
    T: EquivalentDecode<'a, O, OW, RB, UB, OF, WireFormat = W, Flavor = F>,
  {
    O::decode(ctx, self.data)
  }
}
