use std::collections::BTreeSet;

use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{
    Flattened, Inner, Partial, PartialIdentity, PartialRef, PartialTryFromRef, Ref, TryFromPartial,
    TryFromPartialRef, TryFromRef,
  },
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultSetWireFormat, Groto, Packed, WireFormat,
    groto::{Context, Error, PackedSetDecoder},
  },
  selection::Selectable,
  state::State,
};

use super::DefaultPartialSetBuffer;

mod packed;
mod repeated;

impl<K> State<Flattened<Inner>> for BTreeSet<K> {
  type Output = K;
}

impl<K> State<Partial<Groto>> for BTreeSet<K>
where
  K: State<Partial<Groto>>,
  K::Output: Sized,
{
  type Output = DefaultPartialSetBuffer<K::Output>;
}

impl<K> Selectable<Groto> for BTreeSet<K>
where
  K: Selectable<Groto>,
{
  type Selector = K::Selector;
}
