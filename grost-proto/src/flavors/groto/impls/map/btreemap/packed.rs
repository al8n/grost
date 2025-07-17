use varing::decode_u32_varint;

use super::{
  super::{DefaultPartialMapBuffer, MapEntry},
  BTreeMap,
};

use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{Partial, PartialRef, PartialTryFromRef, Ref, TryFromPartialRef, TryFromRef},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultMapWireFormat, Groto, PackedEntry, WireFormat,
    groto::{Context, Error, Identifier, PackedMapDecoder, Tag},
  },
  selection::{Selectable, Selector},
  state::State,
};

impl<K, V> DefaultMapWireFormat<Groto> for BTreeMap<K, V> {
  type Format<KM, VM>
    = PackedEntry<KM, VM>
  where
    KM: WireFormat<Groto> + 'static,
    VM: WireFormat<Groto> + 'static;
}

impl<'a, K, V, KW, VW, RB, B> State<PartialRef<'a, RB, B, PackedEntry<KW, VW>, Groto>>
  for BTreeMap<K, V>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: State<PartialRef<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
  V: State<PartialRef<'a, RB, B, VW, Groto>>,
  V::Output: Sized,
  PackedEntry<KW, VW>: WireFormat<Groto> + 'a,
{
  type Output = PackedMapDecoder<'a, K::Output, V::Output, RB, B, KW, VW>;
}

impl<'a, K, KW, V, VW, RB, B> State<Ref<'a, RB, B, PackedEntry<KW, VW>, Groto>> for BTreeMap<K, V>
where
  PackedEntry<KW, VW>: WireFormat<Groto> + 'a,
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: State<Ref<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
  V: State<Ref<'a, RB, B, VW, Groto>>,
  V::Output: Sized,
{
  type Output = PackedMapDecoder<'a, K::Output, V::Output, RB, B, KW, VW>;
}

impl<'a, K, KW, V, VW, RB, B> Decode1<'a, PackedEntry<KW, VW>, RB, B, Groto> for BTreeMap<K, V>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: Ord + Decode1<'a, KW, RB, B, Groto>,
  V: Decode1<'a, VW, RB, B, Groto>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let bytes = src.as_bytes();
    let bytes_len = bytes.len();
    if bytes_len == 0 {
      return Err(Error::buffer_underflow());
    }

    // decode total bytes
    let (mut offset, total_bytes) = decode_u32_varint(bytes)?;

    if bytes_len < offset + total_bytes as usize {
      return Err(Error::buffer_underflow());
    }

    // decode the number of elements
    let (num_elements_size, num_elements) = decode_u32_varint(&bytes[offset..])?;
    offset += num_elements_size;
    if num_elements == 0 {
      return Ok((offset, BTreeMap::new()));
    }

    let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
    let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

    let mut map = BTreeMap::new();
    while map.len() < num_elements as usize && offset < bytes_len {
      let (read, item) = MapEntry::decode_packed_entry(context, src.slice(offset..), &ki, &vi)?;
      offset += read;
      let (k, v) = item.into_components();

      if map.insert(k, v).is_some() && context.err_on_duplicated_map_keys() {
        return Err(Error::custom("duplicated keys in map"));
      }
    }

    if map.len() != num_elements as usize && context.err_on_length_mismatch() {
      return Err(Error::custom(format!(
        "expected {num_elements} elements in map, but got {} elements",
        map.len()
      )));
    }

    Ok((offset, map))
  }
}

impl<K, KW, V, VW> Encode<PackedEntry<KW, VW>, Groto> for BTreeMap<K, V>
where
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  K: Encode<KW, Groto>,
  V: Encode<VW, Groto>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let encoded_len = <Self as Encode<PackedEntry<KW, VW>, Groto>>::encoded_raw_len(self, context);
    let buf_len = buf.len();
    if buf_len < encoded_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let mut offset = 0;
    // encode num of elements
    let num_elems = self.len();
    let num_elems_size = varing::encode_u32_varint_to(num_elems as u32, buf)?;
    offset += num_elems_size;

    let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
    let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

    // encode the elements
    for item in self {
      let item_encoded_len =
        MapEntry::from(item).encode_packed_entry(context, &mut buf[offset..], &ki, &vi)?;
      offset += item_encoded_len;
    }

    Ok(offset)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    let num_elems = self.len();
    let mut len = varing::encoded_u32_varint_len(num_elems as u32);

    let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
    let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

    for item in self {
      len += MapEntry::from(item).encoded_packed_entry_len(context, &ki, &vi);
    }

    len
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let encoded_raw_len =
      <Self as Encode<PackedEntry<KW, VW>, Groto>>::encoded_raw_len(self, context);
    let encoded_len = varing::encoded_u32_varint_len(encoded_raw_len as u32) + encoded_raw_len;

    let buf_len = buf.len();
    if buf_len < encoded_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let mut offset = 0;

    // encode total bytes
    if encoded_len > u32::MAX as usize {
      return Err(Error::too_large(encoded_len, u32::MAX as usize));
    }

    let total_bytes = encoded_raw_len as u32;
    let total_bytes_size = varing::encode_u32_varint_to(total_bytes, buf)?;
    offset += total_bytes_size;

    // encode num of elements
    let num_elems = self.len();
    let num_elems_size = varing::encode_u32_varint_to(num_elems as u32, buf)?;
    offset += num_elems_size;

    let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
    let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

    // encode the elements
    for item in self {
      let item_encoded_len =
        MapEntry::from(item).encode_packed_entry(context, &mut buf[offset..], &ki, &vi)?;
      offset += item_encoded_len;
    }

    Ok(offset)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    let total_bytes = <Self as Encode<PackedEntry<KW, VW>, Groto>>::encoded_raw_len(self, context);
    varing::encoded_u32_varint_len(total_bytes as u32) + total_bytes
  }
}

impl<K, KW, V, VW> PartialEncode<PackedEntry<KW, VW>, Groto> for BTreeMap<K, V>
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

    let encoded_len = <Self as PartialEncode<PackedEntry<KW, VW>, Groto>>::partial_encoded_raw_len(
      self, context, selector,
    );
    let buf_len = buf.len();
    if buf_len < encoded_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let mut offset = 0;
    // encode num of elements
    let num_elems = self.len();
    let num_elems_size = varing::encode_u32_varint_to(num_elems as u32, buf)?;
    offset += num_elems_size;

    let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
    let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

    // encode the elements
    for item in self {
      let item_encoded_len = MapEntry::from(item).partial_encode_packed(
        context,
        &mut buf[offset..],
        &ki,
        &vi,
        selector,
      )?;
      offset += item_encoded_len;
    }

    Ok(offset)
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    let num_elems = self.len();
    let mut len = varing::encoded_u32_varint_len(num_elems as u32);

    let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
    let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

    for item in self {
      len += MapEntry::from(item).partial_encoded_packed_len(context, &ki, &vi, selector);
    }
    len
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

    let encoded_raw_len =
      <Self as PartialEncode<PackedEntry<KW, VW>, Groto>>::partial_encoded_raw_len(
        self, context, selector,
      );
    let encoded_len = varing::encoded_u32_varint_len(encoded_raw_len as u32) + encoded_raw_len;

    let buf_len = buf.len();
    if buf_len < encoded_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let mut offset = 0;

    // encode total bytes
    if encoded_len > u32::MAX as usize {
      return Err(Error::too_large(encoded_len, u32::MAX as usize));
    }

    let total_bytes = encoded_raw_len as u32;
    let total_bytes_size = varing::encode_u32_varint_to(total_bytes, buf)?;
    offset += total_bytes_size;

    // encode num of elements
    let num_elems = self.len();
    let num_elems_size = varing::encode_u32_varint_to(num_elems as u32, buf)?;
    offset += num_elems_size;

    let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
    let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);
    // encode the elements
    for item in self {
      let item_encoded_len = MapEntry::from(item).partial_encode_packed(
        context,
        &mut buf[offset..],
        &ki,
        &vi,
        selector,
      )?;
      offset += item_encoded_len;
    }

    Ok(offset)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    let total_bytes = <Self as PartialEncode<PackedEntry<KW, VW>, Groto>>::partial_encoded_raw_len(
      self, context, selector,
    );
    varing::encoded_u32_varint_len(total_bytes as u32) + total_bytes
  }
}

impl<'a, K, KW, V, VW, RB, B> TryFromRef<'a, RB, B, PackedEntry<KW, VW>, Groto> for BTreeMap<K, V>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, RB, B, KW, Groto> + Ord + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, B, Groto>,
  V: TryFromRef<'a, RB, B, VW, Groto> + 'a,
  V::Output: Sized + Decode1<'a, VW, RB, B, Groto>,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_ref(
    ctx: &'a Context,
    input: <Self as State<Ref<'a, RB, B, PackedEntry<KW, VW>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, RB, B, PackedEntry<KW, VW>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut map = BTreeMap::new();

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

    if map.len() != capacity_hint && ctx.err_on_length_mismatch() {
      return Err(Error::custom(format!(
        "expected {capacity_hint} elements in map, but got {} elements",
        map.len()
      )));
    }

    Ok(map)
  }
}

impl<'a, K, KW, V, VW, RB, B> TryFromPartialRef<'a, RB, B, PackedEntry<KW, VW>, Groto>
  for BTreeMap<K, V>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, RB, B, KW, Groto> + Ord + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, B, Groto>,
  V: TryFromPartialRef<'a, RB, B, VW, Groto> + 'a,
  V::Output: Sized + Decode1<'a, VW, RB, B, Groto>,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_partial_ref(
    ctx: &'a Context,
    input: <Self as State<PartialRef<'a, RB, B, PackedEntry<KW, VW>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, RB, B, PackedEntry<KW, VW>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut map = BTreeMap::new();

    for res in iter {
      match res {
        Ok((_, item)) => {
          let (k, v) = item.try_into_entry()?.into_components();
          let k = K::try_from_partial_ref(ctx, k)?;
          let v = V::try_from_partial_ref(ctx, v)?;
          if map.insert(k, v).is_some() && ctx.err_on_duplicated_set_keys() {
            return Err(Error::custom("duplicated keys in map"));
          }
        }
        Err(e) => return Err(e),
      }
    }

    if map.len() != capacity_hint && ctx.err_on_length_mismatch() {
      return Err(Error::custom(format!(
        "expected {capacity_hint} elements in map, but got {} elements",
        map.len()
      )));
    }

    Ok(map)
  }
}

impl<'a, K, KW, V, VW, RB, B> PartialTryFromRef<'a, RB, B, PackedEntry<KW, VW>, Groto>
  for BTreeMap<K, V>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, RB, B, KW, Groto> + Ord + 'a,
  <K as State<PartialRef<'a, RB, B, KW, Groto>>>::Output:
    Sized + Decode1<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
  <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  V: PartialTryFromRef<'a, RB, B, VW, Groto> + 'a,
  <V as State<PartialRef<'a, RB, B, VW, Groto>>>::Output:
    Sized + Decode1<'a, VW, RB, B, Groto> + Selectable<Groto, Selector = V::Selector>,
  <V as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = V::Selector>,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn partial_try_from_ref(
    input: <Self as State<PartialRef<'a, RB, B, PackedEntry<KW, VW>, Groto>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, Error>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'a, RB, B, PackedEntry<KW, VW>, Groto>>>::Output: Sized,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let Some(mut partial_map) =
      <DefaultPartialMapBuffer<_, _> as Buffer>::with_capacity(capacity_hint)
    else {
      return Err(Error::custom("failed to allocate partial set buffer"));
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
            return Err(Error::custom("capacity exceeded for partial set buffer"));
          }
        }
        Err(e) => return Err(e),
      }
    }

    Ok(partial_map)
  }
}
