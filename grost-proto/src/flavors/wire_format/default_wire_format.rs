use super::{WireFormat, Flavor};

pub use scalar::*;
pub use bytes::*;
pub use string::*;
pub use object::*;
pub use enumeration::*;
pub use union::*;
pub use list::*;
pub use set::*;
pub use map::*;

mod scalar;
mod bytes;
mod string;
mod object;
mod enumeration;
mod union;
mod list;
mod set;
mod map;
mod nullable;

/// The default wire format for a type on flavor `F`.
pub trait DefaultWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format: WireFormat<F>;
}

