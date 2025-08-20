use indexmap_2::IndexSet;

use core::hash::BuildHasher;

use crate::{
  buffer::{Chunk, ChunkMut, UnknownBuffer},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultRepeatedWireFormat, Groto, Repeated, WireFormat,
    groto::{
      Context, DecodeError, EncodeError, RepeatedDecoder, RepeatedDecoderBuffer,
      context::RepeatedDecodePolicy,
    },
  },
  state::{PartialRef, Ref, State},
};

use super::super::super::{repeated_decode, repeated_encode, repeated_encoded_len};

impl<K, S> DefaultRepeatedWireFormat<Groto> for IndexSet<K, S> {
  type Format<KM, const TAG: u32>
    = Repeated<KM, TAG>
  where
    KM: WireFormat<Groto> + 'static;
}

impl<'a, K, KW, S, RB, B, const TAG: u32> State<PartialRef<'a, Repeated<KW, TAG>, RB, B, Groto>>
  for IndexSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: State<Ref<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
{
  type Output = RepeatedDecoderBuffer<'a, K::Output, RB, B, KW, TAG>;
}

impl<'a, K, KW, S, RB, B, const TAG: u32> State<Ref<'a, Repeated<KW, TAG>, RB, B, Groto>>
  for IndexSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: State<Ref<'a, KW, RB, B, Groto>>,
  K::Output: Sized,
{
  type Output = RepeatedDecoderBuffer<'a, K::Output, RB, B, KW, TAG>;
}

impl<K, KW, S, const TAG: u32> Encode<Repeated<KW, TAG>, Groto> for IndexSet<K, S>
where
  KW: WireFormat<Groto>,
  K: Encode<KW, Groto>,
{
  fn encode_raw<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
  ) -> Result<usize, EncodeError>
  where
    WB: ChunkMut,
  {
    repeated_encode::<K, KW, _, TAG>(
      buf.buffer_mut(),
      || self.iter(),
      |k| k.encoded_len(context),
      |k, buf| k.encode(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    repeated_encoded_len::<K, KW, _, TAG>(self.iter(), |k| k.encoded_len(context))
  }

  fn encode<B>(&self, context: &Context, buf: impl Into<ChunkWriter<B>>) -> Result<usize, Error>
  where
    B: ChunkMut,
  {
    <Self as Encode<Repeated<KW, TAG>, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Repeated<KW, TAG>, Groto>>::encoded_raw_len(self, context)
  }
}

impl<K, KW, S, const TAG: u32> PartialEncode<Repeated<KW, TAG>, Groto> for IndexSet<K, S>
where
  KW: WireFormat<Groto>,
  K: Encode<KW, Groto>,
{
  fn partial_encode_raw<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: ChunkMut,
  {
    if *selector {
      return Ok(0);
    }

    <Self as Encode<Repeated<KW, TAG>, Groto>>::encode_raw(self, context, buf)
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if *selector {
      return 0;
    }

    repeated_encoded_len::<K, KW, _, TAG>(self.iter(), |k| k.encoded_len(context))
  }

  fn partial_encode<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, Error>
  where
    WB: ChunkMut,
  {
    <Self as PartialEncode<Repeated<KW, TAG>, Groto>>::partial_encode_raw(
      self, context, buf, selector,
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <Self as PartialEncode<Repeated<KW, TAG>, Groto>>::partial_encoded_raw_len(
      self, context, selector,
    )
  }
}

impl<'a, K, KW, S, RB, B, const TAG: u32> Decode<'a, Repeated<KW, TAG>, RB, B, Groto>
  for IndexSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  K: core::hash::Hash + Eq + Decode<'a, KW, RB, B, Groto>,
  S: BuildHasher + Default,
{
  fn decode(ctx: &'a Context, src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'a,
    RB: Chunk + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let mut this = IndexSet::with_hasher(Default::default());
    <Self as Decode<'a, Repeated<KW, TAG>, RB, B, Groto>>::merge_decode(&mut this, ctx, src)
      .map(|size| (size, this))
  }

  fn merge_decode(&mut self, ctx: &'a Context, src: RB) -> Result<usize, Error>
  where
    Self: Sized + 'a,
    RB: Chunk + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    match ctx.repeated_decode_policy() {
      RepeatedDecodePolicy::GrowIncrementally => {
        repeated_decode::<K, KW, Self, RB, B, TAG>(self, src, |set, src| {
          let (read, item) = K::decode(ctx, src)?;
          ctx.err_duplicated_set_keys(!set.insert(item))?;

          Ok(read)
        })
      }
      RepeatedDecodePolicy::PreallocateCapacity => {
        let (read, decoder) = RepeatedDecoder::<K, RB, B, KW, TAG>::decode(ctx, src)?;
        self.reserve(decoder.capacity_hint());

        for item in decoder.iter() {
          let (_, k) = item?;
          ctx.err_duplicated_set_keys(!self.insert(k))?;
        }

        Ok(read)
      }
    }
  }
}

// impl<'a, K, KW, S, RB, UB, const TAG: u32> TryFromRef<'a, Repeated<KW, TAG>, RB, UB, Groto>
//   for IndexSet<K, S>
// where
//   KW: WireFormat<Groto> + 'a,
//   K: TryFromRef<'a, KW, RB, UB, Groto> + core::hash::Hash + Eq + 'a,
//   K::Output: Sized + Decode<'a, KW, RB, UB, Groto>,
//   RB: Chunk + 'a,
//   UB: UnknownBuffer<RB, Groto> + 'a,
//   S: BuildHasher + Default,
// {
//   fn try_from_ref(
//     ctx: &'a Context,
//     input: <Self as State<Ref<'a, Repeated<KW, TAG>, RB, UB, Groto>>>::Output,
//   ) -> Result<Self, Error>
//   where
//     Self: Sized,
//     <Self as State<Ref<'a, Repeated<KW, TAG>, RB, UB, Groto>>>::Output: Sized,
//     RB: Chunk + 'a,
//     UB: UnknownBuffer<RB, Groto>,
//   {
//     let capacity_hint = input.capacity_hint();
//     let mut set = IndexSet::with_capacity_and_hasher(capacity_hint, Default::default());

//     try_from::<K, K::Output, KW, RB, UB, _, _>(
//       &mut set,
//       input.iter(),
//       |set| ctx.err_length_mismatch(capacity_hint, set.len()),
//       |set, item| ctx.err_duplicated_set_keys(!set.insert(item)),
//       |item| K::try_from_ref(ctx, item),
//     )
//     .map(|_| set)
//   }
// }

// impl<'a, K, KW, S, RB, B, const TAG: u32> TryFromPartialRef<'a, Repeated<KW, TAG>, RB, B, Groto>
//   for IndexSet<K, S>
// where
//   KW: WireFormat<Groto> + 'a,
//   K: TryFromPartialRef<'a, KW, RB, B, Groto> + core::hash::Hash + Eq + 'a,
//   K::Output: Sized + Decode<'a, KW, RB, B, Groto>,
//   RB: Chunk + 'a,
//   B: UnknownBuffer<RB, Groto> + 'a,
//   S: BuildHasher + Default,
// {
//   fn try_from_partial_ref(
//     ctx: &'a Context,
//     input: <Self as State<PartialRef<'a, Repeated<KW, TAG>, RB, B, Groto>>>::Output,
//   ) -> Result<Self, Error>
//   where
//     Self: Sized,
//     <Self as State<PartialRef<'a, Repeated<KW, TAG>, RB, B, Groto>>>::Output: Sized,
//     RB: Chunk + 'a,
//     B: UnknownBuffer<RB, Groto>,
//   {
//     let capacity_hint = input.capacity_hint();
//     let mut set = IndexSet::with_capacity_and_hasher(capacity_hint, Default::default());

//     try_from::<K, K::Output, KW, RB, B, _, _>(
//       &mut set,
//       input.iter(),
//       |set| ctx.err_length_mismatch(capacity_hint, set.len()),
//       |set, item| ctx.err_duplicated_set_keys(!set.insert(item)),
//       |item| K::try_from_partial_ref(ctx, item),
//     )
//     .map(|_| set)
//   }
// }

// impl<'a, K, KW, RB, B, S, const TAG: u32> PartialTryFromRef<'a, Repeated<KW, TAG>, RB, B, Groto>
//   for IndexSet<K, S>
// where
//   KW: WireFormat<Groto> + 'a,
//   K: PartialTryFromRef<'a, KW, RB, B, Groto> + core::hash::Hash + Eq + 'a,
//   <K as State<PartialRef<'a, KW, RB, B, Groto>>>::Output:
//     Sized + Decode<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
//   <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
//   RB: Chunk + 'a,
//   B: UnknownBuffer<RB, Groto> + 'a,
//   S: BuildHasher + Default,
// {
//   fn partial_try_from_ref(
//     context: &'a Context,
//     input: <Self as State<PartialRef<'a, Repeated<KW, TAG>, RB, B, Groto>>>::Output,
//     selector: &Self::Selector,
//   ) -> Result<<Self as State<Partial<Groto>>>::Output, Error>
//   where
//     <Self as State<Partial<Groto>>>::Output: Sized,
//     <Self as State<PartialRef<'a, Repeated<KW, TAG>, RB, B, Groto>>>::Output: Sized,
//   {
//     if selector.is_empty() {
//       return Ok(IndexSet::with_hasher(S::default()));
//     }

//     let iter = input.iter();
//     let capacity_hint = iter.capacity_hint();
//     let Some(mut partial_set) =
//       <DefaultPartialSetBuffer<_> as Buffer>::with_capacity(capacity_hint)
//     else {
//       return Err(Error::allocation_failed("set"));
//     };

//     try_from::<_, _, KW, RB, B, _, _>(
//       &mut partial_set,
//       iter,
//       |set| context.err_length_mismatch(capacity_hint, set.len()),
//       |set, k| {
//         if <DefaultPartialSetBuffer<_> as Buffer>::push(set, k).is_some() {
//           return Err(Error::capacity_exceeded("set"));
//         }
//         Ok(())
//       },
//       |item| K::partial_try_from_ref(context, item, selector),
//     )
//     .map(|_| partial_set)
//   }
// }
