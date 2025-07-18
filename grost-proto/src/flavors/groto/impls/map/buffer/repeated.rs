use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{Partial, PartialRef, Ref, TryFromPartialRef, TryFromRef},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, RepeatedEntry, WireFormat,
    groto::{
      Context, Error, Identifier, PartialMapBuffer, PartialMapEntry, RepeatedMapDecoderBuffer, Tag,
    },
  },
  state::State,
};

impl<'a, K, V, KW, VW, RB, UB, PB, const TAG: u32>
  State<PartialRef<'a, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto>> for PartialMapBuffer<K, V, PB>
{
  type Output = RepeatedMapDecoderBuffer<'a, K, V, RB, UB, KW, VW, TAG>;
}

impl<'a, K, V, KW, VW, RB, UB, PB, const TAG: u32>
  State<Ref<'a, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto>> for PartialMapBuffer<K, V, PB>
{
  type Output = RepeatedMapDecoderBuffer<'a, K, V, RB, UB, KW, VW, TAG>;
}

impl<K, V, KW, VW, PB, const TAG: u32> Encode<RepeatedEntry<KW, VW, TAG>, Groto>
  for PartialMapBuffer<K, V, PB>
where
  K: Encode<KW, Groto>,
  V: Encode<VW, Groto>,
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let encoded_len =
      <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encoded_raw_len(self, context);
    let buf_len = buf.len();
    if buf_len < encoded_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let ei = Identifier::new(RepeatedEntry::<KW, VW, TAG>::WIRE_TYPE, Tag::try_new(TAG)?);
    let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
    let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

    let mut offset = 0;
    for item in self.iter() {
      let item_encoded_len = item.encode_repeated(context, &mut buf[offset..], &ei, &ki, &vi)?;
      offset += item_encoded_len;
    }

    Ok(offset)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    let ei = Identifier::new(RepeatedEntry::<KW, VW, TAG>::WIRE_TYPE, Tag::new(TAG));
    let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
    let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

    self
      .iter()
      .map(|item| item.encoded_repeated_len(context, &ei, &ki, &vi))
      .sum()
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encoded_raw_len(self, context)
  }
}

impl<K, V, KW, VW, PB, const TAG: u32> PartialEncode<RepeatedEntry<KW, VW, TAG>, Groto>
  for PartialMapBuffer<K, V, PB>
where
  K: PartialEncode<KW, Groto>,
  V: PartialEncode<VW, Groto>,
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
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

    let encoded_len =
      <Self as PartialEncode<RepeatedEntry<KW, VW, TAG>, Groto>>::partial_encoded_raw_len(
        self, context, selector,
      );
    let buf_len = buf.len();
    if buf_len < encoded_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let ei = Identifier::new(RepeatedEntry::<KW, VW, TAG>::WIRE_TYPE, Tag::try_new(TAG)?);
    let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
    let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

    let mut offset = 0;
    for item in self.iter() {
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }

      offset +=
        item.partial_encode_repeated(context, &mut buf[offset..], &ei, &ki, &vi, selector)?;
    }

    Ok(offset)
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    let ei = Identifier::new(RepeatedEntry::<KW, VW, TAG>::WIRE_TYPE, Tag::new(TAG));
    let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
    let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

    self
      .iter()
      .map(|item| item.partial_encoded_repeated_len(context, &ei, &ki, &vi, selector))
      .sum()
  }

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    <Self as PartialEncode<RepeatedEntry<KW, VW, TAG>, Groto>>::partial_encode_raw(
      self, context, buf, selector,
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    <Self as PartialEncode<RepeatedEntry<KW, VW, TAG>, Groto>>::partial_encoded_raw_len(
      self, context, selector,
    )
  }
}

impl<'a, K, KW, V, VW, S, RB, B, PB, const TAG: u32>
  Decode1<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto> for PartialMapBuffer<K, V, PB>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  S: BuildHasher + Default,
  K: Decode1<'a, KW, RB, B, Groto>,
  V: Decode1<'a, VW, RB, B, Groto>,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
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

impl<'de, K, V, RB, UB, PB, KW, VW, const TAG: u32>
  TryFromRef<'de, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto> for PartialMapBuffer<K, V, PB>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: TryFromRef<'de, RB, UB, KW, Groto> + Decode1<'de, KW, RB, UB, Groto> + 'de,
  K::Output: Sized,
  V: TryFromRef<'de, RB, UB, VW, Groto> + Decode1<'de, VW, RB, UB, Groto> + 'de,
  V::Output: Sized,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
{
  fn try_from_ref(
    ctx: &'de Context,
    input: <Self as State<Ref<'de, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'de, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output: Sized,
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
          "exceeded buffer capacity while pushing map entry",
        ));
      }
    }

    Ok(buffer)
  }
}

impl<'de, K, V, RB, UB, PB, KW, VW, const TAG: u32>
  TryFromPartialRef<'de, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto> for PartialMapBuffer<K, V, PB>
where
  KW: WireFormat<Groto> + 'de,
  VW: WireFormat<Groto> + 'de,
  K: TryFromPartialRef<'de, RB, UB, KW, Groto> + Decode1<'de, KW, RB, UB, Groto> + 'de,
  K::Output: Sized,
  V: TryFromPartialRef<'de, RB, UB, VW, Groto> + Decode1<'de, VW, RB, UB, Groto> + 'de,
  V::Output: Sized,
  UB: UnknownBuffer<RB, Groto> + 'de,
  RB: ReadBuf + 'de,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
{
  fn try_from_partial_ref(
    ctx: &'de Context,
    input: <Self as State<PartialRef<'de, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'de, RB, UB, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output: Sized,
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
          "exceeded buffer capacity while pushing map entry",
        ));
      }
    }

    Ok(buffer)
  }
}
