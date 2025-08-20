use crate::{
  buffer::{Buffer, Chunk, ChunkMut, UnknownBuffer},
  convert::{TryFromPartialRef, TryFromRef},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, Packed, WireFormat,
    groto::{Context, DecodeError, EncodeError, PackedSetDecoder, PartialSetBuffer},
  },
  selection::Selector,
  state::{PartialRef, Ref, State},
};

use super::super::super::{
  packed_decode, packed_encode, packed_encode_raw, packed_encoded_len, packed_encoded_raw_len,
};

impl<'a, K, KW, RB, UB, PB> State<PartialRef<'a, Packed<KW>, RB, UB, Groto>>
  for PartialSetBuffer<K, PB>
{
  type Output = PackedSetDecoder<'a, K, RB, UB, KW>;
}

impl<'a, K, KW, RB, UB, PB> State<Ref<'a, Packed<KW>, RB, UB, Groto>> for PartialSetBuffer<K, PB> {
  type Output = PackedSetDecoder<'a, K, RB, UB, KW>;
}

impl<K, KW, PB> Encode<Packed<KW>, Groto> for PartialSetBuffer<K, PB>
where
  K: Encode<KW, Groto>,
  KW: WireFormat<Groto>,
  PB: Buffer<Item = K>,
{
  fn encode_raw<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
  ) -> Result<usize, EncodeError>
  where
    WB: ChunkMut,
  {
    packed_encode_raw::<K, _, _, _>(
      buf.buffer_mut(),
      self.iter(),
      || <Self as Encode<Packed<KW>, Groto>>::encoded_raw_len(self, context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    packed_encoded_raw_len::<K, KW, _, _>(self.len(), self.iter(), |item| item.encoded_len(context))
  }

  fn encode<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
  ) -> Result<usize, EncodeError>
  where
    WB: ChunkMut,
  {
    packed_encode::<K, _, _, _>(
      buf.buffer_mut(),
      self.len(),
      self.iter(),
      || <Self as Encode<Packed<KW>, Groto>>::encoded_raw_len(self, context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_len(&self, context: &Context) -> usize {
    packed_encoded_len::<_>(self.len(), || {
      <Self as Encode<Packed<KW>, Groto>>::encoded_raw_len(self, context)
    })
  }
}

impl<K, KW, PB> PartialEncode<Packed<KW>, Groto> for PartialSetBuffer<K, PB>
where
  K: PartialEncode<KW, Groto>,
  KW: WireFormat<Groto>,
  PB: Buffer<Item = K>,
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
    if selector.is_empty() {
      return Ok(0);
    }

    packed_encode_raw::<K, _, _, _>(
      buf.buffer_mut(),
      self.iter(),
      || {
        <Self as PartialEncode<Packed<KW>, Groto>>::partial_encoded_raw_len(self, context, selector)
      },
      |item, buf| item.partial_encode(context, buf, selector),
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    packed_encoded_raw_len::<K, KW, _, _>(self.len(), self.iter(), |item| {
      item.partial_encoded_len(context, selector)
    })
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
    if selector.is_empty() {
      return Ok(0);
    }

    packed_encode::<K, _, _, _>(
      buf.buffer_mut(),
      self.len(),
      self.iter(),
      || {
        <Self as PartialEncode<Packed<KW>, Groto>>::partial_encoded_raw_len(self, context, selector)
      },
      |item, buf| item.partial_encode(context, buf, selector),
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    packed_encoded_len::<_>(self.len(), || {
      <Self as PartialEncode<Packed<KW>, Groto>>::partial_encoded_raw_len(self, context, selector)
    })
  }
}

impl<'a, K, KW, RB, B, PB> Decode<'a, Packed<KW>, RB, B, Groto> for PartialSetBuffer<K, PB>
where
  KW: WireFormat<Groto> + 'a,
  K: Decode<'a, KW, RB, B, Groto>,
  PB: Buffer<Item = K>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'a,
    RB: Chunk + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    packed_decode::<K, KW, Self, RB>(
      context,
      src,
      |cap| PartialSetBuffer::with_capacity(cap).ok_or_else(|| Error::allocation_failed("set")),
      Self::len,
      |set, src| {
        let (read, item) = K::decode(context, src)?;

        if set.push(item).is_some() {
          return Err(Error::capacity_exceeded("set"));
        }

        Ok(read)
      },
    )
  }
}

impl<'de, K, RB, UB, PB, KW> TryFromRef<'de, Packed<KW>, RB, UB, Groto> for PartialSetBuffer<K, PB>
where
  KW: WireFormat<Groto> + 'de,
  K: TryFromRef<'de, KW, RB, UB, Groto> + Decode<'de, KW, RB, UB, Groto> + 'de,
  K::Output: Sized,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: Chunk + 'de,
  PB: Buffer<Item = K>,
{
  fn try_from_ref(
    ctx: &'de Context,
    input: <Self as State<Ref<'de, Packed<KW>, RB, UB, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'de, Packed<KW>, RB, UB, Groto>>>::Output: Sized,
    RB: Chunk + 'de,
    UB: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    let Some(mut buffer) = Self::with_capacity(capacity_hint) else {
      return Err(Error::allocation_failed("set"));
    };

    for res in input.iter() {
      let (_, ent) = res?;
      if buffer.push(ent).is_some() {
        return Err(Error::capacity_exceeded("set"));
      }
    }

    ctx
      .err_length_mismatch(capacity_hint, buffer.len())
      .map(|_| buffer)
  }
}

impl<'de, K, RB, UB, PB, KW> TryFromPartialRef<'de, Packed<KW>, RB, UB, Groto>
  for PartialSetBuffer<K, PB>
where
  KW: WireFormat<Groto> + 'de,
  K: TryFromPartialRef<'de, KW, RB, UB, Groto> + Decode<'de, KW, RB, UB, Groto> + 'de,
  K::Output: Sized,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: Chunk + 'de,
  PB: Buffer<Item = K>,
{
  fn try_from_partial_ref(
    ctx: &'de Context,
    input: <Self as State<PartialRef<'de, Packed<KW>, RB, UB, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'de, Packed<KW>, RB, UB, Groto>>>::Output: Sized,
    RB: Chunk + 'de,
    UB: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    let Some(mut buffer) = Self::with_capacity(input.capacity_hint()) else {
      return Err(Error::allocation_failed("set"));
    };

    for res in input.iter() {
      let (_, ent) = res?;
      if buffer.push(ent).is_some() {
        return Err(Error::capacity_exceeded("set"));
      }
    }

    ctx
      .err_length_mismatch(capacity_hint, buffer.len())
      .map(|_| buffer)
  }
}
