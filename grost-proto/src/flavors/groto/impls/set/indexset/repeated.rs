use indexmap_2::IndexSet;

use core::hash::BuildHasher;

use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{Partial, PartialRef, PartialTryFromRef, Ref, TryFromPartialRef, TryFromRef},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultRepeatedWireFormat, Groto, Repeated, WireFormat,
    groto::{Context, Error, Identifier, RepeatedDecoderBuffer, Tag},
  },
  selection::{Selectable, Selector},
  state::State,
};

use super::DefaultPartialSetBuffer;

impl<K, S> DefaultRepeatedWireFormat<Groto> for IndexSet<K, S> {
  type Format<KM, const TAG: u32>
    = Repeated<KM, TAG>
  where
    KM: WireFormat<Groto> + 'static;
}

impl<'a, K, KW, S, RB, B, const TAG: u32> State<PartialRef<'a, RB, B, Repeated<KW, TAG>, Groto>>
  for IndexSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: State<PartialRef<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
{
  type Output = RepeatedDecoderBuffer<'a, K::Output, RB, B, KW, TAG>;
}

impl<'a, K, KW, S, RB, B, const TAG: u32> State<Ref<'a, RB, B, Repeated<KW, TAG>, Groto>>
  for IndexSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: State<Ref<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
{
  type Output = RepeatedDecoderBuffer<'a, K::Output, RB, B, KW, TAG>;
}

impl<K, KW, S, const TAG: u32> Encode<Repeated<KW, TAG>, Groto> for IndexSet<K, S>
where
  KW: WireFormat<Groto>,
  K: Encode<KW, Groto>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let identifier = Identifier::new(KW::WIRE_TYPE, Tag::try_new(TAG)?);
    let encoded_identifier = identifier.encode();
    let encoded_identifier_len = encoded_identifier.len();
    let encoded_len = self
      .iter()
      .map(|k| encoded_identifier_len + k.encoded_raw_len(context))
      .sum::<usize>();
    let buf_len = buf.len();
    if encoded_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let mut offset = 0;
    for k in self.iter() {
      if offset + encoded_identifier_len > buf_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }

      buf[offset..offset + encoded_identifier_len].copy_from_slice(&encoded_identifier);
      offset += encoded_identifier_len;

      if offset >= buf_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }

      let k_len = k
        .encode_raw(context, &mut buf[offset..])
        .map_err(|e| e.update(encoded_len, buf_len))?;
      offset += k_len;
    }

    Ok(offset)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    let identifier = Identifier::new(KW::WIRE_TYPE, Tag::new(TAG));
    let encoded_identifier_len = identifier.encoded_len();
    self
      .iter()
      .map(|k| encoded_identifier_len + k.encoded_raw_len(context))
      .sum()
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let identifier = Identifier::new(KW::WIRE_TYPE, Tag::try_new(TAG)?);
    let encoded_identifier = identifier.encode();
    let encoded_identifier_len = encoded_identifier.len();
    let encoded_len = self
      .iter()
      .map(|k| encoded_identifier_len + k.encoded_len(context))
      .sum::<usize>();
    let buf_len = buf.len();
    if encoded_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let mut offset = 0;
    for k in self.iter() {
      if offset + encoded_identifier_len > buf_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }

      buf[offset..offset + encoded_identifier_len].copy_from_slice(&encoded_identifier);
      offset += encoded_identifier_len;

      if offset >= buf_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }

      let k_len = k
        .encode(context, &mut buf[offset..])
        .map_err(|e| e.update(encoded_len, buf_len))?;
      offset += k_len;
    }

    Ok(offset)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    let identifier = Identifier::new(KW::WIRE_TYPE, Tag::new(TAG));
    let encoded_identifier_len = identifier.encoded_len();
    self
      .iter()
      .map(|k| encoded_identifier_len + k.encoded_len(context))
      .sum()
  }
}

impl<K, KW, S, const TAG: u32> PartialEncode<Repeated<KW, TAG>, Groto> for IndexSet<K, S>
where
  KW: WireFormat<Groto>,
  K: PartialEncode<KW, Groto>,
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

    let identifier = Identifier::new(KW::WIRE_TYPE, Tag::try_new(TAG)?);
    let encoded_identifier = identifier.encode();
    let encoded_identifier_len = encoded_identifier.len();
    let encoded_len = self
      .iter()
      .map(|k| encoded_identifier_len + k.partial_encoded_raw_len(context, selector))
      .sum::<usize>();
    let buf_len = buf.len();
    if encoded_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }
    let mut offset = 0;
    for k in self.iter() {
      if offset + encoded_identifier_len > buf_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }
      buf[offset..offset + encoded_identifier_len].copy_from_slice(&encoded_identifier);
      offset += encoded_identifier_len;
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }
      let k_len = k
        .partial_encode_raw(context, &mut buf[offset..], selector)
        .map_err(|e| e.update(encoded_len, buf_len))?;
      offset += k_len;
    }

    Ok(offset)
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    let identifier = Identifier::new(KW::WIRE_TYPE, Tag::new(TAG));
    let encoded_identifier_len = identifier.encoded_len();
    self
      .iter()
      .map(|k| encoded_identifier_len + k.partial_encoded_raw_len(context, selector))
      .sum()
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

    let identifier = Identifier::new(KW::WIRE_TYPE, Tag::try_new(TAG)?);
    let encoded_identifier = identifier.encode();
    let encoded_identifier_len = encoded_identifier.len();
    let encoded_len = self
      .iter()
      .map(|k| encoded_identifier_len + k.partial_encoded_len(context, selector))
      .sum::<usize>();
    let buf_len = buf.len();
    if encoded_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }
    let mut offset = 0;
    for k in self.iter() {
      if offset + encoded_identifier_len > buf_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }
      buf[offset..offset + encoded_identifier_len].copy_from_slice(&encoded_identifier);
      offset += encoded_identifier_len;
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }
      let k_len = k
        .partial_encode(context, &mut buf[offset..], selector)
        .map_err(|e| e.update(encoded_len, buf_len))?;
      offset += k_len;
    }

    Ok(offset)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    let identifier = Identifier::new(KW::WIRE_TYPE, Tag::new(TAG));
    let encoded_identifier_len = identifier.encoded_len();
    self
      .iter()
      .map(|k| encoded_identifier_len + k.partial_encoded_len(context, selector))
      .sum()
  }
}

impl<'a, K, KW, S, RB, B, const TAG: u32> Decode1<'a, Repeated<KW, TAG>, RB, B, Groto>
  for IndexSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  K: core::hash::Hash + Eq + Decode1<'a, KW, RB, B, Groto>,
  S: BuildHasher + Default,
{
  fn decode(ctx: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let mut this = IndexSet::with_hasher(Default::default());
    <Self as Decode1<'a, Repeated<KW, TAG>, RB, B, Groto>>::merge_decode(&mut this, ctx, src)
      .map(|size| (size, this))
  }

  fn merge_decode(&mut self, ctx: &'a Context, src: RB) -> Result<usize, Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let identifier = Identifier::new(KW::WIRE_TYPE, Tag::new(TAG));
    let mut offset = 0;
    let buf = src.as_bytes();
    let buf_len = buf.len();
    if buf_len == 0 {
      return Err(Error::buffer_underflow());
    }

    // The following elements should be prefixed with the identifier.
    // | identifier | element | identifier | element | ...
    loop {
      if offset >= buf_len {
        break;
      }

      let (read, next_id) = Identifier::decode(&buf[offset..])?;

      // If the next identifier does not match the expected identifier, which means we have reached the end of the repeated elements.
      // We should stop decoding. We do not need to increment the offset here because we are not consuming the next identifier.
      if next_id != identifier {
        break;
      }

      // increment the offset by the size of the identifier
      offset += read;
      // consum the next element
      let (read, k) = K::decode(ctx, src.slice(offset..))?;
      offset += read;
      if !self.insert(k) && ctx.err_on_duplicated_set_keys() {
        return Err(Error::custom("duplicated keys in set"));
      }
    }

    Ok(offset)
  }
}

impl<'a, K, KW, S, RB, UB, const TAG: u32> TryFromRef<'a, RB, UB, Repeated<KW, TAG>, Groto>
  for IndexSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, RB, UB, KW, Groto> + core::hash::Hash + Eq + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, UB, Groto>,
  RB: ReadBuf + 'a,
  UB: UnknownBuffer<RB, Groto> + 'a,
  S: BuildHasher + Default,
{
  fn try_from_ref(
    ctx: &'a Context,
    input: <Self as State<Ref<'a, RB, UB, Repeated<KW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, RB, UB, Repeated<KW, TAG>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    UB: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    let mut set = IndexSet::with_capacity_and_hasher(capacity_hint, Default::default());

    for res in input.iter() {
      match res {
        Ok((_, item)) => {
          let item = K::try_from_ref(ctx, item)?;
          if !set.insert(item) && ctx.err_on_duplicated_set_keys() {
            return Err(Error::custom("duplicated keys in set"));
          }
        }
        Err(e) => return Err(e),
      }
    }

    if set.len() != capacity_hint && ctx.err_on_length_mismatch() {
      return Err(Error::custom(format!(
        "expected {capacity_hint} elements in set, but got {} elements",
        set.len()
      )));
    }

    Ok(set)
  }
}

impl<'a, K, KW, S, RB, B, const TAG: u32> TryFromPartialRef<'a, RB, B, Repeated<KW, TAG>, Groto>
  for IndexSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, RB, B, KW, Groto> + core::hash::Hash + Eq + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, B, Groto>,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
  S: BuildHasher + Default,
{
  fn try_from_partial_ref(
    ctx: &'a Context,
    input: <Self as State<PartialRef<'a, RB, B, Repeated<KW, TAG>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, RB, B, Repeated<KW, TAG>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let capacity_hint = input.capacity_hint();
    let mut set = IndexSet::with_capacity_and_hasher(capacity_hint, Default::default());

    for res in input.iter() {
      match res {
        Ok((_, item)) => {
          let item = K::try_from_partial_ref(ctx, item)?;
          if !set.insert(item) && ctx.err_on_duplicated_set_keys() {
            return Err(Error::custom("duplicated keys in set"));
          }
        }
        Err(e) => return Err(e),
      }
    }

    if set.len() != capacity_hint && ctx.err_on_length_mismatch() {
      return Err(Error::custom(format!(
        "expected {capacity_hint} elements in set, but got {} elements",
        set.len()
      )));
    }

    Ok(set)
  }
}

impl<'a, K, KW, RB, B, const TAG: u32> PartialTryFromRef<'a, RB, B, Repeated<KW, TAG>, Groto>
  for IndexSet<K>
where
  KW: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, RB, B, KW, Groto> + core::hash::Hash + Eq + 'a,
  <K as State<PartialRef<'a, RB, B, KW, Groto>>>::Output:
    Sized + Decode1<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
  <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn partial_try_from_ref(
    input: <Self as State<PartialRef<'a, RB, B, Repeated<KW, TAG>, Groto>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, Error>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'a, RB, B, Repeated<KW, TAG>, Groto>>>::Output: Sized,
  {
    if selector.is_empty() {
      return Ok(DefaultPartialSetBuffer::new());
    }

    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let Some(mut partial_set) =
      <DefaultPartialSetBuffer<_> as Buffer>::with_capacity(capacity_hint)
    else {
      return Err(Error::custom("failed to allocate partial set buffer"));
    };

    for res in iter {
      match res {
        Ok((_, item)) => {
          let item = K::partial_try_from_ref(item, selector)?;
          if <DefaultPartialSetBuffer<_> as Buffer>::push(&mut partial_set, item).is_some() {
            return Err(Error::custom("capacity exceeded for partial set buffer"));
          }
        }
        Err(e) => return Err(e),
      }
    }

    Ok(partial_set)
  }
}
