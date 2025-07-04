use core::marker::PhantomData;

use crate::{
  convert::{Flattened, Inner, State},
  marker::{MapMarker, Marker},
};

use super::Flavor;

pub use default_wire_format::*;

mod default_wire_format;

/// The wire format used for encoding and decoding.
pub trait WireFormat<F: Flavor + ?Sized>:
  Copy + Eq + core::hash::Hash + core::fmt::Debug + core::fmt::Display + Into<F::WireType>
{
  /// The cooresponding value to the wire type.
  const WIRE_TYPE: F::WireType;
  /// The name of the wire format.
  const NAME: &'static str;
  /// The self.
  const SELF: Self;
  /// `true` if the wire format is repeated.
  const REPEATED: bool = false;
}
