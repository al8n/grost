use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{PartialRef, Ref, TryFromPartialRef, TryFromRef},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, PackedEntry, WireFormat,
    groto::{Context, Error, Identifier, PackedMapDecoder, PartialMapBuffer, PartialMapEntry, Tag},
  },
  selection::Selector,
  state::State,
};

impl<'a, K, V, KW, VW, RB, UB, PB> State<PartialRef<'a, RB, UB, PackedEntry<KW, VW>, Groto>>
  for PartialMapBuffer<K, V, PB>
{
  type Output = PackedMapDecoder<'a, K, V, RB, UB, KW, VW>;
}

impl<'a, K, V, KW, VW, RB, UB, PB> State<Ref<'a, RB, UB, PackedEntry<KW, VW>, Groto>>
  for PartialMapBuffer<K, V, PB>
{
  type Output = PackedMapDecoder<'a, K, V, RB, UB, KW, VW>;
}

impl<K, V, KW, VW, PB> Encode<PackedEntry<KW, VW>, Groto> for PartialMapBuffer<K, V, PB>
where
  K: Encode<KW, Groto>,
  V: Encode<VW, Groto>,
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
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
    for item in self.iter() {
      let item_encoded_len = item.encode_packed(context, &mut buf[offset..], &ki, &vi)?;
      offset += item_encoded_len;
    }

    Ok(offset)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    let num_elems = self.len();
    let mut len = varing::encoded_u32_varint_len(num_elems as u32);

    let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
    let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

    for item in self.iter() {
      len += item.encoded_packed_len(context, &ki, &vi);
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
    for item in self.iter() {
      let item_encoded_len = item.encode_packed(context, &mut buf[offset..], &ki, &vi)?;
      offset += item_encoded_len;
    }

    Ok(offset)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    let total_bytes = <Self as Encode<PackedEntry<KW, VW>, Groto>>::encoded_raw_len(self, context);
    varing::encoded_u32_varint_len(total_bytes as u32) + total_bytes
  }
}

impl<K, V, KW, VW, PB> PartialEncode<PackedEntry<KW, VW>, Groto> for PartialMapBuffer<K, V, PB>
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
    for item in self.iter() {
      let item_encoded_len =
        item.partial_encode_packed(context, &mut buf[offset..], &ki, &vi, selector)?;
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

    for item in self.iter() {
      len += item.partial_encoded_packed_len(context, &ki, &vi, selector);
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
    for item in self.iter() {
      let item_encoded_len =
        item.partial_encode_packed(context, &mut buf[offset..], &ki, &vi, selector)?;
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

impl<'a, K, KW, V, VW, RB, UB, PB> Decode1<'a, PackedEntry<KW, VW>, RB, UB, Groto>
  for PartialMapBuffer<K, V, PB>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: Decode1<'a, KW, RB, UB, Groto>,
  V: Decode1<'a, VW, RB, UB, Groto>,
  PB: Buffer<Item = PartialMapEntry<K, V>>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    UB: UnknownBuffer<RB, Groto> + 'a,
  {
    todo!()
  }
}

impl<'de, K, V, RB, UB, PB, KW, VW> TryFromRef<'de, RB, UB, PackedEntry<KW, VW>, Groto>
  for PartialMapBuffer<K, V, PB>
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
    input: <Self as State<Ref<'de, RB, UB, PackedEntry<KW, VW>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'de, RB, UB, PackedEntry<KW, VW>, Groto>>>::Output: Sized,
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

impl<'de, K, V, RB, UB, PB, KW, VW> TryFromPartialRef<'de, RB, UB, PackedEntry<KW, VW>, Groto>
  for PartialMapBuffer<K, V, PB>
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
    input: <Self as State<PartialRef<'de, RB, UB, PackedEntry<KW, VW>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'de, RB, UB, PackedEntry<KW, VW>, Groto>>>::Output: Sized,
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
