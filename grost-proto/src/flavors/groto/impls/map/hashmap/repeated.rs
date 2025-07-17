use core::hash::{BuildHasher, Hash};

use super::{
  super::{DefaultPartialMapBuffer, MapEntry},
  HashMap,
};

use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{Partial, PartialRef, PartialTryFromRef, Ref, TryFromPartialRef, TryFromRef},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultRepeatedEntryWireFormat, Groto, RepeatedEntry, WireFormat,
    groto::{Context, Error, Identifier, RepeatedMapDecoderBuffer, Tag},
  },
  selection::{Selectable, Selector},
  state::State,
};

impl<K, V, S> DefaultRepeatedEntryWireFormat<Groto> for HashMap<K, V, S> {
  type Format<KM, VM, const TAG: u32>
    = RepeatedEntry<KM, VM, TAG>
  where
    KM: WireFormat<Groto> + 'static,
    VM: WireFormat<Groto> + 'static;
}

impl<'a, K, V, KW, VW, S, RB, B, const TAG: u32>
  State<PartialRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>> for HashMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: State<PartialRef<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
  V: State<PartialRef<'a, RB, B, VW, Groto>>,
  V::Output: Sized,
  RepeatedEntry<KW, VW, TAG>: WireFormat<Groto> + 'a,
{
  type Output = RepeatedMapDecoderBuffer<'a, K::Output, V::Output, RB, B, KW, VW, TAG>;
}

impl<'a, K, KW, V, VW, S, RB, B, const TAG: u32>
  State<Ref<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>> for HashMap<K, V, S>
where
  RepeatedEntry<KW, VW, TAG>: WireFormat<Groto> + 'a,
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: State<Ref<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
  V: State<Ref<'a, RB, B, VW, Groto>>,
  V::Output: Sized,
{
  type Output = RepeatedMapDecoderBuffer<'a, K::Output, V::Output, RB, B, KW, VW, TAG>;
}

impl<'a, K, KW, V, VW, S, RB, B, const TAG: u32>
  Decode1<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto> for HashMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  S: BuildHasher + Default,
  K: Eq + Hash + Decode1<'a, KW, RB, B, Groto>,
  V: Decode1<'a, VW, RB, B, Groto>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let mut this = HashMap::with_hasher(S::default());
    <Self as Decode1<'a, RepeatedEntry<KW, VW, TAG>, RB, B, Groto>>::merge_decode(
      &mut this, context, src,
    )
    .map(|size| (size, this))
  }

  fn merge_decode(&mut self, context: &'a Context, src: RB) -> Result<usize, Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let ei = Identifier::new(RepeatedEntry::<KW, VW, TAG>::WIRE_TYPE, Tag::try_new(TAG)?);
    let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
    let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

    let mut offset = 0;
    let buf_len = src.len();

    while offset < buf_len {
      let (read, entry) = MapEntry::decode_repeated(context, src.slice(offset..), &ei, &ki, &vi)?;

      match entry {
        Some(entry) => {
          offset += read;

          let (k, v) = entry.into_components();
          if self.insert(k, v).is_some() && context.err_on_duplicated_map_keys() {
            return Err(Error::custom("duplicated keys in map"));
          }
        }
        None => break,
      }
    }

    Ok(offset)
  }
}

impl<K, KW, V, VW, S, const TAG: u32> Encode<RepeatedEntry<KW, VW, TAG>, Groto> for HashMap<K, V, S>
where
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  K: Encode<KW, Groto>,
  V: Encode<VW, Groto>,
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
    for item in self {
      let item_encoded_len =
        MapEntry::from(item).encode_repeated(context, &mut buf[offset..], &ei, &ki, &vi)?;
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
      .map(|item| MapEntry::from(item).encoded_repeated_len(context, &ei, &ki, &vi))
      .sum()
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encode_raw(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <Self as Encode<RepeatedEntry<KW, VW, TAG>, Groto>>::encoded_raw_len(self, context)
  }
}

impl<K, KW, V, VW, S, const TAG: u32> PartialEncode<RepeatedEntry<KW, VW, TAG>, Groto>
  for HashMap<K, V, S>
where
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  K: PartialEncode<KW, Groto>,
  V: PartialEncode<VW, Groto>,
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
    for item in self {
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }

      offset += MapEntry::from(item).partial_encode_repeated(
        context,
        &mut buf[offset..],
        &ei,
        &ki,
        &vi,
        selector,
      )?;
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
      .map(|item| {
        MapEntry::from(item).partial_encoded_repeated_len(context, &ei, &ki, &vi, selector)
      })
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

impl<'a, K, KW, V, VW, S, RB, B, const TAG: u32>
  TryFromRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto> for HashMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, RB, B, KW, Groto> + Eq + Hash + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, B, Groto>,
  V: TryFromRef<'a, RB, B, VW, Groto> + 'a,
  V::Output: Sized + Decode1<'a, VW, RB, B, Groto>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_ref(
    ctx: &'a Context,
    input: <Self as State<Ref<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut map = HashMap::with_capacity_and_hasher(capacity_hint, S::default());

    for res in iter {
      match res {
        Ok((_, item)) => {
          let (k, v) = item.try_into_entry()?.into_components();
          let k = K::try_from_ref(ctx, k)?;
          let v = V::try_from_ref(ctx, v)?;
          if map.insert(k, v).is_some() && ctx.err_on_duplicated_map_keys() {
            return Err(Error::custom("duplicated keys in map"));
          }
        }
        Err(e) => return Err(e),
      }
    }

    Ok(map)
  }
}

impl<'a, K, KW, V, VW, S, RB, B, const TAG: u32>
  TryFromPartialRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto> for HashMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, RB, B, KW, Groto> + Eq + Hash + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, B, Groto>,
  V: TryFromPartialRef<'a, RB, B, VW, Groto> + 'a,
  V::Output: Sized + Decode1<'a, VW, RB, B, Groto>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_partial_ref(
    ctx: &'a Context,
    input: <Self as State<PartialRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut map = HashMap::with_capacity_and_hasher(capacity_hint, S::default());

    for res in iter {
      match res {
        Ok((_, item)) => {
          let (k, v) = item.try_into_entry()?.into_components();
          let k = K::try_from_partial_ref(ctx, k)?;
          let v = V::try_from_partial_ref(ctx, v)?;
          if map.insert(k, v).is_some() && ctx.err_on_duplicated_map_keys() {
            return Err(Error::custom("duplicated keys in map"));
          }
        }
        Err(e) => return Err(e),
      }
    }

    Ok(map)
  }
}

impl<'a, K, KW, V, VW, S, RB, B, const TAG: u32>
  PartialTryFromRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto> for HashMap<K, V, S>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, RB, B, KW, Groto> + Eq + Hash + 'a,
  <K as State<PartialRef<'a, RB, B, KW, Groto>>>::Output:
    Sized + Decode1<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
  <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  V: PartialTryFromRef<'a, RB, B, VW, Groto> + 'a,
  <V as State<PartialRef<'a, RB, B, VW, Groto>>>::Output:
    Sized + Decode1<'a, VW, RB, B, Groto> + Selectable<Groto, Selector = V::Selector>,
  <V as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = V::Selector>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn partial_try_from_ref(
    input: <Self as State<PartialRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, Error>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'a, RB, B, RepeatedEntry<KW, VW, TAG>, Groto>>>::Output: Sized,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let Some(mut partial_map) =
      <DefaultPartialMapBuffer<_, _> as Buffer>::with_capacity(capacity_hint)
    else {
      return Err(Error::custom("failed to allocate partial map buffer"));
    };

    for res in iter {
      match res {
        Ok((_, item)) => {
          if <DefaultPartialMapBuffer<_, _> as Buffer>::push(
            &mut partial_map,
            item.and_then(
              |k| K::partial_try_from_ref(k, selector.key()),
              |v| V::partial_try_from_ref(v, selector.value()),
            )?,
          )
          .is_some()
          {
            return Err(Error::custom("capacity exceeded for partial map buffer"));
          }
        }
        Err(e) => return Err(e),
      }
    }

    Ok(partial_map)
  }
}
