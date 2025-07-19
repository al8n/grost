use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{Partial, PartialRef, Ref, TryFromPartialRef, TryFromRef},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, Packed, WireFormat,
    groto::{Context, Error, Identifier, PackedSetDecoder, PartialSetBuffer, Tag},
  },
  selection::Selector,
  state::State,
};

use super::super::{
  packed_decode, packed_encode, packed_encode_raw, packed_encoded_len, packed_encoded_raw_len,
};

impl<'a, K, KW, RB, UB, PB> State<PartialRef<'a, RB, UB, Packed<KW>, Groto>>
  for PartialSetBuffer<K, PB>
{
  type Output = PackedSetDecoder<'a, K, RB, UB, KW>;
}

impl<'a, K, KW, RB, UB, PB> State<Ref<'a, RB, UB, Packed<KW>, Groto>> for PartialSetBuffer<K, PB> {
  type Output = PackedSetDecoder<'a, K, RB, UB, KW>;
}

impl<K, KW, PB> Encode<Packed<KW>, Groto> for PartialSetBuffer<K, PB>
where
  K: Encode<KW, Groto>,
  KW: WireFormat<Groto>,
  PB: Buffer<Item = K>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    packed_encode_raw::<K, _, _, _>(
      buf,
      self.iter(),
      || <Self as Encode<Packed<KW>, Groto>>::encoded_raw_len(self, context),
      |item, buf| item.encode(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    packed_encoded_raw_len::<K, KW, _, _>(self.len(), self.iter(), |item| item.encoded_len(context))
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    packed_encode::<K, _, _, _>(
      buf,
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
  fn partial_encode_raw(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if selector.is_empty() {
      return Ok(0);
    }

    packed_encode_raw::<K, _, _, _>(
      buf,
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

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if selector.is_empty() {
      return Ok(0);
    }

    packed_encode::<K, _, _, _>(
      buf,
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

impl<'a, K, KW, RB, B, PB> Decode1<'a, Packed<KW>, RB, B, Groto> for PartialSetBuffer<K, PB>
where
  KW: WireFormat<Groto> + 'a,
  K: Decode1<'a, KW, RB, B, Groto>,
  PB: Buffer<Item = K>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    packed_decode::<K, KW, Self, RB>(
      context,
      src,
      |cap| {
        PartialSetBuffer::with_capacity(cap)
          .ok_or_else(|| Error::custom("failed to create buffer with given capacity"))
      },
      Self::len,
      |set, src| {
        let (read, item) = K::decode(context, src)?;

        if set.push(item).is_some() {
          return Err(Error::custom("exceeded set buffer capacity"));
        }

        if context.err_on_duplicated_set_keys() {
          return Err(Error::custom("duplicated keys in set"));
        }

        Ok(read)
      },
    )
  }
}

impl<'de, K, RB, UB, PB, KW> TryFromRef<'de, RB, UB, Packed<KW>, Groto> for PartialSetBuffer<K, PB>
where
  KW: WireFormat<Groto> + 'de,
  K: TryFromRef<'de, RB, UB, KW, Groto> + Decode1<'de, KW, RB, UB, Groto> + 'de,
  K::Output: Sized,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
  PB: Buffer<Item = K>,
{
  fn try_from_ref(
    ctx: &'de Context,
    input: <Self as State<Ref<'de, RB, UB, Packed<KW>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'de, RB, UB, Packed<KW>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'de,
    UB: UnknownBuffer<RB, Groto>,
  {
    let Some(mut buffer) = Self::with_capacity(input.capacity_hint()) else {
      return Err(Error::custom("failed to create buffer with given capacity"));
    };

    for res in input.iter() {
      let (_, ent) = res?;
      if buffer.push(ent).is_none() && ctx.err_on_length_mismatch() {
        return Err(Error::custom(
          "exceeded buffer capacity while pushing set entry",
        ));
      }
    }

    Ok(buffer)
  }
}

impl<'de, K, RB, UB, PB, KW> TryFromPartialRef<'de, RB, UB, Packed<KW>, Groto>
  for PartialSetBuffer<K, PB>
where
  KW: WireFormat<Groto> + 'de,
  K: TryFromPartialRef<'de, RB, UB, KW, Groto> + Decode1<'de, KW, RB, UB, Groto> + 'de,
  K::Output: Sized,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
  PB: Buffer<Item = K>,
{
  fn try_from_partial_ref(
    ctx: &'de Context,
    input: <Self as State<PartialRef<'de, RB, UB, Packed<KW>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'de, RB, UB, Packed<KW>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'de,
    UB: UnknownBuffer<RB, Groto>,
  {
    let Some(mut buffer) = Self::with_capacity(input.capacity_hint()) else {
      return Err(Error::custom("failed to create buffer with given capacity"));
    };

    for res in input.iter() {
      let (_, ent) = res?;
      if buffer.push(ent).is_none() && ctx.err_on_length_mismatch() {
        return Err(Error::custom(
          "exceeded buffer capacity while pushing set entry",
        ));
      }
    }

    Ok(buffer)
  }
}
