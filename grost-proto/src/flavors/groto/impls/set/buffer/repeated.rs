use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{PartialRef, Ref, TryFromPartialRef, TryFromRef},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, Repeated, WireFormat,
    groto::{Context, Error, Identifier, PartialSetBuffer, RepeatedDecoderBuffer, Tag},
  },
  selection::Selector,
  state::State,
};

use super::super::{repeated_encode, repeated_encoded_len};

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
      self.iter(),
      |k| k.encoded_len(context),
      |k, buf| k.encode(context, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    repeated_encoded_len(self.iter(), |k| k.encoded_len(context))
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

    repeated_encode(
      buf,
      self.iter(),
      |k| k.partial_encoded_len(context, selector),
      |k, buf| k.partial_encode(context, buf, selector),
    )
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    repeated_encoded_len(self.iter(), |k| k.partial_encoded_len(context, selector))
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

impl<'a, K, KW, RB, B, PB, const TAG: u32> Decode1<'a, Repeated<KW, TAG>, RB, B, Groto>
  for PartialSetBuffer<K, PB>
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
    todo!()
  }
}

impl<'de, K, RB, UB, PB, KW, const TAG: u32> TryFromRef<'de, RB, UB, Repeated<KW, TAG>, Groto>
  for PartialSetBuffer<K, PB>
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
    input: <Self as State<Ref<'de, RB, UB, Repeated<KW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'de, RB, UB, Repeated<KW, TAG>, Groto>>>::Output: Sized,
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

impl<'de, K, RB, UB, PB, KW, const TAG: u32>
  TryFromPartialRef<'de, RB, UB, Repeated<KW, TAG>, Groto> for PartialSetBuffer<K, PB>
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
    input: <Self as State<PartialRef<'de, RB, UB, Repeated<KW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'de, RB, UB, Repeated<KW, TAG>, Groto>>>::Output: Sized,
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
