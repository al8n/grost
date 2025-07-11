use core::marker::PhantomData;

use crate::flavors::groto::Context;

pub struct PackedEntriesDecoder<'a, K: ?Sized, V: ?Sized, B, UB: ?Sized, KW: ?Sized, VW: ?Sized> {
  /// the source buffer
  src: B,
  /// the length of the length prefix
  data_offset: usize,
  /// the current offset
  offset: usize,
  ctx: &'a Context,
  _k: PhantomData<K>,
  _v: PhantomData<V>,
  _kw: PhantomData<KW>,
  _vw: PhantomData<VW>,
  _ub: PhantomData<UB>,
}
