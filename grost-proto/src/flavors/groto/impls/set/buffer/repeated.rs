use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{TryFromPartialRef, TryFromRef},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, Repeated, WireFormat,
    groto::{
      Context, Error, PartialSetBuffer, RepeatedDecoder, RepeatedDecoderBuffer,
      context::RepeatedDecodePolicy,
    },
  },
  selection::Selector,
  state::{Partial, PartialRef, Ref, State},
};

use super::super::super::{repeated_decode, repeated_encode, repeated_encoded_len};

impl<'a, K, KW, RB, UB, PB, const TAG: u32> State<PartialRef<'a, RB, UB, Repeated<KW, TAG>, Groto>>
  for PartialSetBuffer<K, PB>
{
  type Output = RepeatedDecoderBuffer<'a, K, RB, UB, KW, TAG>;
}

impl<'a, K, KW, RB, UB, PB, const TAG: u32> State<Ref<'a, RB, UB, Repeated<KW, TAG>, Groto>>
  for PartialSetBuffer<K, PB>
{
  type Output = RepeatedDecoderBuffer<'a, K, RB, UB, KW, TAG>;
}

impl<K, KW, PB, const TAG: u32> Encode<Repeated<KW, TAG>, Groto> for PartialSetBuffer<K, PB>
where
  K: Encode<KW, Groto>,
  KW: WireFormat<Groto>,
  PB: Buffer<Item = K>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    repeated_encode::<K, KW, _, TAG>(
      buf,
      || self.iter(),
      |k| k.encoded_len(context),
      |k, buf| k.encode(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    repeated_encoded_len::<K, KW, _, TAG>(self.iter(), |k| k.encoded_len(context))
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<Repeated<KW, TAG>, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<Repeated<KW, TAG>, Groto>>::encoded_raw_len(self, context)
  }
}

impl<K, KW, PB, const TAG: u32> PartialEncode<Repeated<KW, TAG>, Groto> for PartialSetBuffer<K, PB>
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

    repeated_encode::<K, KW, _, TAG>(
      buf,
      || self.iter(),
      |k| k.partial_encoded_len(context, selector),
      |k, buf| k.partial_encode(context, buf, selector),
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    repeated_encoded_len::<K, KW, _, TAG>(self.iter(), |k| k.partial_encoded_len(context, selector))
  }

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
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

impl<'a, K, KW, RB, B, PB, const TAG: u32> Decode<'a, Repeated<KW, TAG>, RB, B, Groto>
  for PartialSetBuffer<K, PB>
where
  KW: WireFormat<Groto> + 'a,
  K: Decode<'a, KW, RB, B, Groto>,
  PB: Buffer<Item = K>,
{
  fn decode(ctx: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let mut this = PartialSetBuffer::new();
    <Self as Decode<'a, Repeated<KW, TAG>, RB, B, Groto>>::merge_decode(&mut this, ctx, src)
      .map(|size| (size, this))
  }

  fn merge_decode(&mut self, ctx: &'a Context, src: RB) -> Result<usize, Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    match ctx.repeated_decode_policy() {
      RepeatedDecodePolicy::PreallocateCapacity => {
        let (read, decoder) = RepeatedDecoder::<K, RB, B, KW, TAG>::decode(ctx, src)?;

        if !self.try_reserve_exact(decoder.capacity_hint()) {
          return Err(Error::fail_to_reserve_capacity("set"));
        }

        for item in decoder.iter() {
          let (_, ent) = item?;
          if self.push(ent).is_some() {
            return Err(Error::capacity_exceeded("set"));
          }
        }

        Ok(read)
      }
      RepeatedDecodePolicy::GrowIncrementally => {
        repeated_decode::<K, KW, Self, RB, B, TAG>(self, src, |set, src| {
          let (read, item) = K::decode(ctx, src)?;

          if set.push(item).is_some() {
            return Err(Error::capacity_exceeded("set"));
          }

          Ok(read)
        })
      }
    }
  }
}

impl<'de, K, RB, UB, PB, KW, const TAG: u32> TryFromRef<'de, RB, UB, Repeated<KW, TAG>, Groto>
  for PartialSetBuffer<K, PB>
where
  KW: WireFormat<Groto> + 'de,
  K: TryFromRef<'de, RB, UB, KW, Groto> + Decode<'de, KW, RB, UB, Groto> + 'de,
  K::Output: Sized,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
  PB: Buffer<Item = K>,
{
  fn try_from_ref(
    ctx: &'de Context,
    input: <Self as State<Ref<'de, RB, UB, Repeated<KW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'de, RB, UB, Repeated<KW, TAG>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'de,
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

impl<'de, K, RB, UB, PB, KW, const TAG: u32>
  TryFromPartialRef<'de, RB, UB, Repeated<KW, TAG>, Groto> for PartialSetBuffer<K, PB>
where
  KW: WireFormat<Groto> + 'de,
  K: TryFromPartialRef<'de, RB, UB, KW, Groto> + Decode<'de, KW, RB, UB, Groto> + 'de,
  K::Output: Sized,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
  PB: Buffer<Item = K>,
{
  fn try_from_partial_ref(
    ctx: &'de Context,
    input: <Self as State<PartialRef<'de, RB, UB, Repeated<KW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'de, RB, UB, Repeated<KW, TAG>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'de,
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
