use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, WireFormat,
    groto::{Context, Error, Identifier},
  },
  selection::Selectable,
};

use super::MapSelector;

pub struct PartialMapEntry<K, V> {
  key: Option<K>,
  value: Option<V>,
}

impl<K, V> Selectable<Groto> for PartialMapEntry<K, V>
where
  K: Selectable<Groto>,
  V: Selectable<Groto>,
{
  type Selector = MapSelector<K::Selector, V::Selector>;
}

impl<K, V> PartialMapEntry<K, V> {
  /// Returns the key of the map entry, if it exists.
  #[inline]
  pub const fn key(&self) -> Option<&K> {
    self.key.as_ref()
  }

  /// Returns the value of the map entry, if it exists.
  #[inline]
  pub const fn value(&self) -> Option<&V> {
    self.value.as_ref()
  }

  /// Consumes the entry and returns its key and value as a tuple.
  #[inline]
  pub fn into_components(self) -> (Option<K>, Option<V>) {
    (self.key, self.value)
  }

  pub(super) fn new(key: Option<K>, value: Option<V>) -> Self {
    Self { key, value }
  }

  #[inline]
  pub(super) fn try_into_entry(self) -> Result<MapEntry<K, V>, Error> {
    let key = self
      .key
      .ok_or_else(|| Error::custom("missing key in map entry"))?;
    let value = self
      .value
      .ok_or_else(|| Error::custom("missing value in map entry"))?;
    Ok(MapEntry { key, value })
  }

  #[inline]
  pub(super) fn and_then<NK, NV, KF, VF>(
    self,
    kf: KF,
    vf: VF,
  ) -> Result<PartialMapEntry<NK, NV>, Error>
  where
    KF: FnOnce(K) -> Result<NK, Error>,
    VF: FnOnce(V) -> Result<NV, Error>,
  {
    let key = self.key.map(kf).transpose()?;
    let value = self.value.map(vf).transpose()?;
    Ok(PartialMapEntry { key, value })
  }

  pub(super) fn encode_repeated<KW, VW>(
    &self,
    ctx: &Context,
    buf: &mut [u8],
    ei: &Identifier,
    ki: &Identifier,
    vi: &Identifier,
  ) -> Result<usize, Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: Encode<KW, Groto>,
    V: Encode<VW, Groto>,
  {
    let encoded_entry_len = self.encoded_len_helper::<KW, VW>(ctx, ki, vi);
    let buf_len = buf.len();
    let mut offset = ei
      .encode_to(buf)
      .map_err(|e| e.update(self.encoded_repeated_len(ctx, ei, ki, vi), buf_len))?;

    offset += varing::encode_u32_varint_to(encoded_entry_len as u32, buf).map_err(|e| {
      Error::from_varint_encode_error(e).update(self.encoded_repeated_len(ctx, ei, ki, vi), buf_len)
    })?;

    let total = offset + encoded_entry_len;
    if total >= buf_len {
      return Err(Error::insufficient_buffer(total, buf_len));
    }

    self.encode_helper(ctx, buf, ki, vi).map(|written| {
      #[cfg(debug_assertions)]
      crate::debug_assert_write_eq::<Self>(written, encoded_entry_len);

      total
    })
  }

  pub(super) fn encoded_repeated_len<KW, VW>(
    &self,
    ctx: &Context,
    ei: &Identifier,
    ki: &Identifier,
    vi: &Identifier,
  ) -> usize
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: Encode<KW, Groto>,
    V: Encode<VW, Groto>,
  {
    let mut len = ei.encoded_len();
    let encoded_len = self.encoded_len_helper::<KW, VW>(ctx, ki, vi);
    len += varing::encoded_u32_varint_len(encoded_len as u32);
    len += encoded_len;
    len
  }

  pub(super) fn encode_packed<KW, VW>(
    &self,
    ctx: &Context,
    buf: &mut [u8],
    ki: &Identifier,
    vi: &Identifier,
  ) -> Result<usize, Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: Encode<KW, Groto>,
    V: Encode<VW, Groto>,
  {
    let encoded_entry_len = self.encoded_len_helper::<KW, VW>(ctx, ki, vi);
    let buf_len = buf.len();

    let offset = varing::encode_u32_varint_to(encoded_entry_len as u32, buf)
      .map_err(|e| Error::from_varint_encode_error(e).update(encoded_entry_len, buf_len))?;
    let total = offset + encoded_entry_len;
    if total >= buf_len {
      return Err(Error::insufficient_buffer(total, buf_len));
    }

    self.encode_helper(ctx, buf, ki, vi).map(|written| {
      #[cfg(debug_assertions)]
      crate::debug_assert_write_eq::<Self>(written, encoded_entry_len);

      total
    })
  }

  pub(super) fn encoded_packed_len<KW, VW>(
    &self,
    ctx: &Context,
    ki: &Identifier,
    vi: &Identifier,
  ) -> usize
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: Encode<KW, Groto>,
    V: Encode<VW, Groto>,
  {
    let encoded_len = self.encoded_len_helper::<KW, VW>(ctx, ki, vi);
    let mut len = varing::encoded_u32_varint_len(encoded_len as u32);
    len += encoded_len;
    len
  }

  pub(super) fn partial_encode_packed<KW, VW>(
    &self,
    ctx: &Context,
    buf: &mut [u8],
    ki: &Identifier,
    vi: &Identifier,
    selector: &MapSelector<K::Selector, V::Selector>,
  ) -> Result<usize, Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: PartialEncode<KW, Groto>,
    V: PartialEncode<VW, Groto>,
  {
    let encoded_entry_len = self.partial_encoded_len_helper::<KW, VW>(ctx, ki, vi, selector);
    let buf_len = buf.len();

    let offset = varing::encode_u32_varint_to(encoded_entry_len as u32, buf)
      .map_err(|e| Error::from_varint_encode_error(e).update(encoded_entry_len, buf_len))?;
    let total = offset + encoded_entry_len;
    if total >= buf_len {
      return Err(Error::insufficient_buffer(total, buf_len));
    }

    self
      .partial_encode_helper(ctx, buf, ki, vi, selector)
      .map(|written| {
        #[cfg(debug_assertions)]
        crate::debug_assert_write_eq::<Self>(written, encoded_entry_len);

        total
      })
  }

  pub(super) fn partial_encoded_packed_len<KW, VW>(
    &self,
    ctx: &Context,
    ki: &Identifier,
    vi: &Identifier,
    selector: &MapSelector<K::Selector, V::Selector>,
  ) -> usize
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: PartialEncode<KW, Groto> + Selectable<Groto>,
    V: PartialEncode<VW, Groto> + Selectable<Groto>,
  {
    let encoded_len = self.partial_encoded_len_helper::<KW, VW>(ctx, ki, vi, selector);
    let mut len = varing::encoded_u32_varint_len(encoded_len as u32);
    len += encoded_len;
    len
  }

  pub(super) fn partial_encode_repeated<KW, VW>(
    &self,
    ctx: &Context,
    buf: &mut [u8],
    ei: &Identifier,
    ki: &Identifier,
    vi: &Identifier,
    selector: &MapSelector<K::Selector, V::Selector>,
  ) -> Result<usize, Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: PartialEncode<KW, Groto>,
    V: PartialEncode<VW, Groto>,
  {
    let encoded_entry_len = self.partial_encoded_len_helper::<KW, VW>(ctx, ki, vi, selector);
    let buf_len = buf.len();
    let mut offset = ei.encode_to(buf).map_err(|e| {
      e.update(
        self.partial_encoded_repeated_len(ctx, ei, ki, vi, selector),
        buf_len,
      )
    })?;

    offset += varing::encode_u32_varint_to(encoded_entry_len as u32, buf).map_err(|e| {
      Error::from_varint_encode_error(e).update(
        self.partial_encoded_repeated_len(ctx, ei, ki, vi, selector),
        buf_len,
      )
    })?;

    let total = offset + encoded_entry_len;
    if total >= buf_len {
      return Err(Error::insufficient_buffer(total, buf_len));
    }

    self
      .partial_encode_helper(ctx, buf, ki, vi, selector)
      .map(|written| {
        #[cfg(debug_assertions)]
        crate::debug_assert_write_eq::<Self>(written, encoded_entry_len);

        total
      })
  }

  pub(super) fn partial_encoded_repeated_len<KW, VW>(
    &self,
    ctx: &Context,
    ei: &Identifier,
    ki: &Identifier,
    vi: &Identifier,
    selector: &MapSelector<K::Selector, V::Selector>,
  ) -> usize
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: PartialEncode<KW, Groto> + Selectable<Groto>,
    V: PartialEncode<VW, Groto> + Selectable<Groto>,
  {
    let mut len = ei.encoded_len();
    let encoded_len = self.partial_encoded_len_helper::<KW, VW>(ctx, ki, vi, selector);
    len += varing::encoded_u32_varint_len(encoded_len as u32);
    len += encoded_len;
    len
  }

  pub(super) fn decode_packed_entry<'de, KW, VW, RB, UB>(
    ctx: &'de Context,
    src: RB,
    ki: &Identifier,
    vi: &Identifier,
  ) -> Result<(usize, Self), Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    RB: ReadBuf + 'de,
    UB: UnknownBuffer<RB, Groto> + 'de,
    K: Decode1<'de, KW, RB, UB, Groto> + 'de,
    V: Decode1<'de, VW, RB, UB, Groto> + 'de,
  {
    // read the length of the entry
    let (len_size, entry_size) = varing::decode_u32_varint(src.as_bytes())?;
    let src_len = src.len();
    let total = len_size + entry_size as usize;
    if total > src_len {
      return Err(Error::buffer_underflow());
    }

    // decode the entry
    Self::decode_entry_helper(ctx, src.slice(len_size..total), ki, vi).map(|(_read, ent)| {
      #[cfg(debug_assertions)]
      crate::debug_assert_read_eq::<Self>(_read, entry_size as usize);

      (total, ent)
    })
  }

  pub(super) fn decode_repeated<'de, KW, VW, RB, UB>(
    parent_name: &'static str,
    ctx: &'de Context,
    src: RB,
    ei: &Identifier,
    ki: &Identifier,
    vi: &Identifier,
  ) -> Result<(usize, Self), Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    RB: ReadBuf + 'de,
    UB: UnknownBuffer<RB, Groto> + 'de,
    K: Decode1<'de, KW, RB, UB, Groto> + 'de,
    V: Decode1<'de, VW, RB, UB, Groto> + 'de,
  {
    let buf_len = src.len();
    let buf = src.as_bytes();

    // read the identifier of the entry
    let (mut offset, identifier) = Identifier::decode(buf)?;
    if identifier.ne(ei) {
      return Err(Error::unknown_identifier(parent_name, identifier));
    }

    if offset >= buf_len {
      return Err(Error::buffer_underflow());
    }

    // read the length of the entry
    let (len_size, entry_size) = varing::decode_u32_varint(&buf[offset..])?;
    offset += len_size;
    let total = offset + entry_size as usize;
    if total > buf_len {
      return Err(Error::buffer_underflow());
    }

    // decode the entry
    Self::decode_entry_helper(ctx, src.slice(offset..total), ki, vi).map(|(_read, ent)| {
      #[cfg(debug_assertions)]
      crate::debug_assert_read_eq::<Self>(_read, entry_size as usize);

      (total, ent)
    })
  }

  fn decode_entry_helper<'de, KW, VW, RB, UB>(
    ctx: &'de Context,
    src: RB,
    ki: &Identifier,
    vi: &Identifier,
  ) -> Result<(usize, Self), Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    RB: ReadBuf + 'de,
    UB: UnknownBuffer<RB, Groto> + 'de,
    K: Decode1<'de, KW, RB, UB, Groto> + 'de,
    V: Decode1<'de, VW, RB, UB, Groto> + 'de,
  {
    let buf_len = src.len();
    let buf = src.as_bytes();
    let mut offset = 0;

    let mut k = None;
    let mut v = None;

    while offset < buf_len {
      let (identifier_size, identifier) = Identifier::decode(&buf[offset..])?;
      offset += identifier_size;

      if offset >= buf_len {
        return Err(Error::buffer_underflow());
      }

      match () {
        () if identifier.eq(ki) => {
          if k.is_some() {
            return Err(Error::custom("duplicate key found when decoding map entry"));
          }
          let (klen, key) = K::decode(ctx, src.slice(offset..))?;
          offset += klen;
          k = Some(key);
        }
        () if identifier.eq(vi) => {
          if v.is_some() {
            return Err(Error::custom(
              "duplicate value found when decoding map entry",
            ));
          }
          let (vlen, value) = V::decode(ctx, src.slice(offset..))?;
          offset += vlen;
          v = Some(value);
        }
        _ => {
          return Err(Error::unknown_identifier(
            core::any::type_name::<Self>(),
            identifier,
          ));
        }
      }
    }

    Ok((offset, Self { key: k, value: v }))
  }

  fn encode_helper<KW, VW>(
    &self,
    ctx: &Context,
    buf: &mut [u8],
    ki: &Identifier,
    vi: &Identifier,
  ) -> Result<usize, Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: Encode<KW, Groto>,
    V: Encode<VW, Groto>,
  {
    let encoded_len = self.encoded_len_helper::<KW, VW>(ctx, ki, vi);
    let buf_len = buf.len();
    if encoded_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let mut offset = 0;
    if let Some(k) = &self.key {
      offset += ki.encode_to(buf)?;
      if offset >= buf_len {
        return Err(Error::buffer_underflow());
      }
      offset += k.encode(ctx, &mut buf[offset..])?;
    }

    if let Some(v) = &self.value {
      if offset >= buf_len {
        return Err(Error::buffer_underflow());
      }
      offset += vi.encode_to(&mut buf[offset..])?;

      if offset >= buf_len {
        return Err(Error::buffer_underflow());
      }

      offset += v.encode(ctx, &mut buf[offset..])?;
    }

    #[cfg(debug_assertions)]
    crate::debug_assert_write_eq::<Self>(offset, encoded_len);

    Ok(offset)
  }

  fn encoded_len_helper<KW, VW>(&self, ctx: &Context, ki: &Identifier, vi: &Identifier) -> usize
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: Encode<KW, Groto>,
    V: Encode<VW, Groto>,
  {
    let mut len = 0;
    if let Some(k) = &self.key {
      len += ki.encoded_len() + k.encoded_len(ctx);
    }
    if let Some(v) = &self.value {
      len += vi.encoded_len() + v.encoded_len(ctx);
    }
    len
  }

  fn partial_encode_helper<KW, VW>(
    &self,
    ctx: &Context,
    buf: &mut [u8],
    ki: &Identifier,
    vi: &Identifier,
    selector: &MapSelector<K::Selector, V::Selector>,
  ) -> Result<usize, Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: PartialEncode<KW, Groto>,
    V: PartialEncode<VW, Groto>,
  {
    let encoded_len = self.partial_encoded_len_helper::<KW, VW>(ctx, ki, vi, selector);
    let buf_len = buf.len();
    if encoded_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let mut offset = 0;

    if let Some(ref k) = self.key {
      offset += ki.encode_to(buf)?;
      if offset >= buf_len {
        return Err(Error::buffer_underflow());
      }
      offset += k.partial_encode(ctx, &mut buf[offset..], selector.key())?;
    }

    if let Some(ref v) = self.value {
      if offset >= buf_len {
        return Err(Error::buffer_underflow());
      }
      offset += vi.encode_to(&mut buf[offset..])?;

      if offset >= buf_len {
        return Err(Error::buffer_underflow());
      }

      offset += v.partial_encode(ctx, &mut buf[offset..], selector.value())?;
    }

    #[cfg(debug_assertions)]
    crate::debug_assert_write_eq::<Self>(offset, encoded_len);

    Ok(offset)
  }

  fn partial_encoded_len_helper<KW, VW>(
    &self,
    ctx: &Context,
    ki: &Identifier,
    vi: &Identifier,
    selector: &MapSelector<K::Selector, V::Selector>,
  ) -> usize
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: PartialEncode<KW, Groto> + Selectable<Groto>,
    V: PartialEncode<VW, Groto> + Selectable<Groto>,
  {
    let mut len = 0;
    if let Some(ref k) = self.key {
      len += ki.encoded_len() + k.partial_encoded_len(ctx, selector.key());
    }

    if let Some(ref v) = self.value {
      len += vi.encoded_len() + v.partial_encoded_len(ctx, selector.value());
    }
    len
  }
}

pub struct MapEntry<K, V> {
  key: K,
  value: V,
}

impl<K, V> From<(K, V)> for MapEntry<K, V> {
  fn from((key, value): (K, V)) -> Self {
    Self { key, value }
  }
}

impl<K, V> From<MapEntry<K, V>> for (K, V) {
  fn from(entry: MapEntry<K, V>) -> Self {
    entry.into_components()
  }
}

impl<K, V> MapEntry<K, V> {
  /// Returns the key of the map entry, if it exists.
  #[inline]
  pub const fn key(&self) -> &K {
    &self.key
  }

  /// Returns the value of the map entry, if it exists.
  #[inline]
  pub const fn value(&self) -> &V {
    &self.value
  }

  /// Consumes the entry and returns its key and value as a tuple.
  #[inline]
  pub fn into_components(self) -> (K, V) {
    (self.key, self.value)
  }

  pub(super) fn new(key: K, value: V) -> Self {
    Self { key, value }
  }

  pub(super) fn encode_repeated<KW, VW>(
    &self,
    ctx: &Context,
    buf: &mut [u8],
    ei: &Identifier,
    ki: &Identifier,
    vi: &Identifier,
  ) -> Result<usize, Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: Encode<KW, Groto>,
    V: Encode<VW, Groto>,
  {
    let encoded_entry_len = self.encoded_len_helper::<KW, VW>(ctx, ki, vi);
    let buf_len = buf.len();
    let mut offset = ei
      .encode_to(buf)
      .map_err(|e| e.update(self.encoded_repeated_len(ctx, ei, ki, vi), buf_len))?;

    offset += varing::encode_u32_varint_to(encoded_entry_len as u32, buf).map_err(|e| {
      Error::from_varint_encode_error(e).update(self.encoded_repeated_len(ctx, ei, ki, vi), buf_len)
    })?;

    let total = offset + encoded_entry_len;
    if total >= buf_len {
      return Err(Error::insufficient_buffer(total, buf_len));
    }

    self.encode_helper(ctx, buf, ki, vi).map(|written| {
      #[cfg(debug_assertions)]
      crate::debug_assert_write_eq::<Self>(written, encoded_entry_len);

      total
    })
  }

  pub(super) fn encoded_repeated_len<KW, VW>(
    &self,
    ctx: &Context,
    ei: &Identifier,
    ki: &Identifier,
    vi: &Identifier,
  ) -> usize
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: Encode<KW, Groto>,
    V: Encode<VW, Groto>,
  {
    let mut len = ei.encoded_len();
    let encoded_len = self.encoded_len_helper::<KW, VW>(ctx, ki, vi);
    len += varing::encoded_u32_varint_len(encoded_len as u32);
    len += encoded_len;
    len
  }

  pub(super) fn encode_packed<KW, VW>(
    &self,
    ctx: &Context,
    buf: &mut [u8],
    ki: &Identifier,
    vi: &Identifier,
  ) -> Result<usize, Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: Encode<KW, Groto>,
    V: Encode<VW, Groto>,
  {
    let encoded_entry_len = self.encoded_len_helper::<KW, VW>(ctx, ki, vi);
    let buf_len = buf.len();

    let offset = varing::encode_u32_varint_to(encoded_entry_len as u32, buf)
      .map_err(|e| Error::from_varint_encode_error(e).update(encoded_entry_len, buf_len))?;
    let total = offset + encoded_entry_len;
    if total >= buf_len {
      return Err(Error::insufficient_buffer(total, buf_len));
    }

    self.encode_helper(ctx, buf, ki, vi).map(|written| {
      #[cfg(debug_assertions)]
      crate::debug_assert_write_eq::<Self>(written, encoded_entry_len);

      total
    })
  }

  pub(super) fn encoded_packed_len<KW, VW>(
    &self,
    ctx: &Context,
    ki: &Identifier,
    vi: &Identifier,
  ) -> usize
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: Encode<KW, Groto>,
    V: Encode<VW, Groto>,
  {
    let encoded_len = self.encoded_len_helper::<KW, VW>(ctx, ki, vi);
    let mut len = varing::encoded_u32_varint_len(encoded_len as u32);
    len += encoded_len;
    len
  }

  pub(super) fn partial_encode_packed<KW, VW>(
    &self,
    ctx: &Context,
    buf: &mut [u8],
    ki: &Identifier,
    vi: &Identifier,
    selector: &MapSelector<K::Selector, V::Selector>,
  ) -> Result<usize, Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: PartialEncode<KW, Groto>,
    V: PartialEncode<VW, Groto>,
  {
    let encoded_entry_len = self.partial_encoded_len_helper::<KW, VW>(ctx, ki, vi, selector);
    let buf_len = buf.len();

    let offset = varing::encode_u32_varint_to(encoded_entry_len as u32, buf)
      .map_err(|e| Error::from_varint_encode_error(e).update(encoded_entry_len, buf_len))?;
    let total = offset + encoded_entry_len;
    if total >= buf_len {
      return Err(Error::insufficient_buffer(total, buf_len));
    }

    self
      .partial_encode_helper(ctx, buf, ki, vi, selector)
      .map(|written| {
        #[cfg(debug_assertions)]
        crate::debug_assert_write_eq::<Self>(written, encoded_entry_len);

        total
      })
  }

  pub(super) fn partial_encoded_packed_len<KW, VW>(
    &self,
    ctx: &Context,
    ki: &Identifier,
    vi: &Identifier,
    selector: &MapSelector<K::Selector, V::Selector>,
  ) -> usize
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: PartialEncode<KW, Groto> + Selectable<Groto>,
    V: PartialEncode<VW, Groto> + Selectable<Groto>,
  {
    let encoded_len = self.partial_encoded_len_helper::<KW, VW>(ctx, ki, vi, selector);
    let mut len = varing::encoded_u32_varint_len(encoded_len as u32);
    len += encoded_len;
    len
  }

  pub(super) fn partial_encode_repeated<KW, VW>(
    &self,
    ctx: &Context,
    buf: &mut [u8],
    ei: &Identifier,
    ki: &Identifier,
    vi: &Identifier,
    selector: &MapSelector<K::Selector, V::Selector>,
  ) -> Result<usize, Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: PartialEncode<KW, Groto>,
    V: PartialEncode<VW, Groto>,
  {
    let encoded_entry_len = self.partial_encoded_len_helper::<KW, VW>(ctx, ki, vi, selector);
    let buf_len = buf.len();
    let mut offset = ei.encode_to(buf).map_err(|e| {
      e.update(
        self.partial_encoded_repeated_len(ctx, ei, ki, vi, selector),
        buf_len,
      )
    })?;

    offset += varing::encode_u32_varint_to(encoded_entry_len as u32, buf).map_err(|e| {
      Error::from_varint_encode_error(e).update(
        self.partial_encoded_repeated_len(ctx, ei, ki, vi, selector),
        buf_len,
      )
    })?;

    let total = offset + encoded_entry_len;
    if total >= buf_len {
      return Err(Error::insufficient_buffer(total, buf_len));
    }

    self
      .partial_encode_helper(ctx, buf, ki, vi, selector)
      .map(|written| {
        #[cfg(debug_assertions)]
        crate::debug_assert_write_eq::<Self>(written, encoded_entry_len);

        total
      })
  }

  pub(super) fn partial_encoded_repeated_len<KW, VW>(
    &self,
    ctx: &Context,
    ei: &Identifier,
    ki: &Identifier,
    vi: &Identifier,
    selector: &MapSelector<K::Selector, V::Selector>,
  ) -> usize
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: PartialEncode<KW, Groto> + Selectable<Groto>,
    V: PartialEncode<VW, Groto> + Selectable<Groto>,
  {
    let mut len = ei.encoded_len();
    let encoded_len = self.partial_encoded_len_helper::<KW, VW>(ctx, ki, vi, selector);
    len += varing::encoded_u32_varint_len(encoded_len as u32);
    len += encoded_len;
    len
  }

  pub(super) fn decode_packed_entry<'de, KW, VW, RB, UB>(
    ctx: &'de Context,
    src: RB,
    ki: &Identifier,
    vi: &Identifier,
  ) -> Result<(usize, Self), Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    RB: ReadBuf + 'de,
    UB: UnknownBuffer<RB, Groto> + 'de,
    K: Decode1<'de, KW, RB, UB, Groto> + 'de,
    V: Decode1<'de, VW, RB, UB, Groto> + 'de,
  {
    // read the length of the entry
    let (len_size, entry_size) = varing::decode_u32_varint(src.as_bytes())?;
    let src_len = src.len();
    let total = len_size + entry_size as usize;
    if total > src_len {
      return Err(Error::buffer_underflow());
    }

    // decode the entry
    Self::decode_entry_helper(ctx, src.slice(len_size..total), ki, vi).map(|(_read, ent)| {
      #[cfg(debug_assertions)]
      crate::debug_assert_read_eq::<Self>(_read, entry_size as usize);

      (total, ent)
    })
  }

  /// None means we should stop.
  pub(super) fn decode_repeated<'de, KW, VW, RB, UB>(
    ctx: &'de Context,
    src: RB,
    ei: &Identifier,
    ki: &Identifier,
    vi: &Identifier,
  ) -> Result<(usize, Option<Self>), Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    RB: ReadBuf + 'de,
    UB: UnknownBuffer<RB, Groto> + 'de,
    K: Decode1<'de, KW, RB, UB, Groto> + 'de,
    V: Decode1<'de, VW, RB, UB, Groto> + 'de,
  {
    let buf_len = src.len();
    let buf = src.as_bytes();

    // read the identifier of the entry
    let (mut offset, identifier) = Identifier::decode(buf)?;
    if identifier.ne(ei) {
      return Ok((offset, None));
    }

    if offset >= buf_len {
      return Err(Error::buffer_underflow());
    }

    // read the length of the entry
    let (len_size, entry_size) = varing::decode_u32_varint(&buf[offset..])?;
    offset += len_size;
    let total = offset + entry_size as usize;
    if total > buf_len {
      return Err(Error::buffer_underflow());
    }

    // decode the entry
    Self::decode_entry_helper(ctx, src.slice(offset..total), ki, vi).map(|(_read, ent)| {
      #[cfg(debug_assertions)]
      crate::debug_assert_read_eq::<Self>(_read, entry_size as usize);

      (total, Some(ent))
    })
  }

  fn decode_entry_helper<'de, KW, VW, RB, UB>(
    ctx: &'de Context,
    src: RB,
    ki: &Identifier,
    vi: &Identifier,
  ) -> Result<(usize, Self), Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    RB: ReadBuf + 'de,
    UB: UnknownBuffer<RB, Groto> + 'de,
    K: Decode1<'de, KW, RB, UB, Groto> + 'de,
    V: Decode1<'de, VW, RB, UB, Groto> + 'de,
  {
    let buf_len = src.len();
    let buf = src.as_bytes();
    let mut offset = 0;

    let mut k = None;
    let mut v = None;

    while offset < buf_len {
      let (identifier_size, identifier) = Identifier::decode(&buf[offset..])?;
      offset += identifier_size;

      if offset >= buf_len {
        return Err(Error::buffer_underflow());
      }

      match () {
        () if identifier.eq(ki) => {
          if k.is_some() {
            return Err(Error::custom("duplicate key found when decoding map entry"));
          }
          let (klen, key) = K::decode(ctx, src.slice(offset..))?;
          offset += klen;
          k = Some(key);
        }
        () if identifier.eq(vi) => {
          if v.is_some() {
            return Err(Error::custom(
              "duplicate value found when decoding map entry",
            ));
          }
          let (vlen, value) = V::decode(ctx, src.slice(offset..))?;
          offset += vlen;
          v = Some(value);
        }
        _ => {
          return Err(Error::unknown_identifier(
            core::any::type_name::<Self>(),
            identifier,
          ));
        }
      }
    }

    let Some(k) = k else {
      return Err(Error::custom("missing key in map entry"));
    };

    let Some(v) = v else {
      return Err(Error::custom("missing value in map entry"));
    };

    Ok((offset, Self { key: k, value: v }))
  }

  fn encode_helper<KW, VW>(
    &self,
    ctx: &Context,
    buf: &mut [u8],
    ki: &Identifier,
    vi: &Identifier,
  ) -> Result<usize, Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: Encode<KW, Groto>,
    V: Encode<VW, Groto>,
  {
    let encoded_len = self.encoded_len_helper::<KW, VW>(ctx, ki, vi);
    let buf_len = buf.len();
    if encoded_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let mut offset = 0;
    offset += ki.encode_to(buf)?;
    if offset >= buf_len {
      return Err(Error::buffer_underflow());
    }
    offset += self.key.encode(ctx, &mut buf[offset..])?;

    if offset >= buf_len {
      return Err(Error::buffer_underflow());
    }
    offset += vi.encode_to(&mut buf[offset..])?;

    if offset >= buf_len {
      return Err(Error::buffer_underflow());
    }

    offset += self.value.encode(ctx, &mut buf[offset..])?;

    #[cfg(debug_assertions)]
    crate::debug_assert_write_eq::<Self>(offset, encoded_len);

    Ok(offset)
  }

  fn encoded_len_helper<KW, VW>(&self, ctx: &Context, ki: &Identifier, vi: &Identifier) -> usize
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: Encode<KW, Groto>,
    V: Encode<VW, Groto>,
  {
    let mut len = 0;
    len += ki.encoded_len() + self.key.encoded_len(ctx);
    len += vi.encoded_len() + self.value.encoded_len(ctx);
    len
  }

  fn partial_encode_helper<KW, VW>(
    &self,
    ctx: &Context,
    buf: &mut [u8],
    ki: &Identifier,
    vi: &Identifier,
    selector: &MapSelector<K::Selector, V::Selector>,
  ) -> Result<usize, Error>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: PartialEncode<KW, Groto>,
    V: PartialEncode<VW, Groto>,
  {
    let encoded_len = self.partial_encoded_len_helper::<KW, VW>(ctx, ki, vi, selector);
    let buf_len = buf.len();
    if encoded_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let mut offset = 0;
    offset += ki.encode_to(buf)?;
    if offset >= buf_len {
      return Err(Error::buffer_underflow());
    }
    offset += self
      .key
      .partial_encode(ctx, &mut buf[offset..], selector.key())?;

    if offset >= buf_len {
      return Err(Error::buffer_underflow());
    }
    offset += vi.encode_to(&mut buf[offset..])?;

    if offset >= buf_len {
      return Err(Error::buffer_underflow());
    }

    offset += self
      .value
      .partial_encode(ctx, &mut buf[offset..], selector.value())?;

    #[cfg(debug_assertions)]
    crate::debug_assert_write_eq::<Self>(offset, encoded_len);

    Ok(offset)
  }

  fn partial_encoded_len_helper<KW, VW>(
    &self,
    ctx: &Context,
    ki: &Identifier,
    vi: &Identifier,
    selector: &MapSelector<K::Selector, V::Selector>,
  ) -> usize
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    K: PartialEncode<KW, Groto> + Selectable<Groto>,
    V: PartialEncode<VW, Groto> + Selectable<Groto>,
  {
    let mut len = 0;
    len += ki.encoded_len() + self.key.partial_encoded_len(ctx, selector.key());
    len += vi.encoded_len() + self.value.partial_encoded_len(ctx, selector.value());
    len
  }
}
