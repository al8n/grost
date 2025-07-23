use core::mem::MaybeUninit;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  decode::Decode,
  flavors::{
    Flavor, Groto, WireFormat,
    groto::{Error, PackedDecoder},
  },
  selection::{Selectable, Selector},
  state::{Partial, PartialRef, Ref, State},
};
