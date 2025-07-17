use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  decode::Decode1,
  encode::Encode,
  flavors::{
    Groto, WireFormat,
    groto::{Context, Error, Identifier},
  },
};

pub struct PartialMapEntry<K, V> {
  key: Option<K>,
  value: Option<V>,
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

  pub(super) fn encode_repeated_entry<KW, VW>(
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
    let encoded_entry_len = self.encoded_entry_len_helper::<KW, VW>(ctx, ki, vi);
    let buf_len = buf.len();
    let mut offset = ei
      .encode_to(buf)
      .map_err(|e| e.update(self.encoded_repeated_entry_len(ctx, ei, ki, vi), buf_len))?;

    offset += varing::encode_u32_varint_to(encoded_entry_len as u32, buf).map_err(|e| {
      Error::from_varint_encode_error(e)
        .update(self.encoded_repeated_entry_len(ctx, ei, ki, vi), buf_len)
    })?;

    let total = offset + encoded_entry_len;
    if total >= buf_len {
      return Err(Error::insufficient_buffer(total, buf_len));
    }

    self.encode_entry_helper(ctx, buf, ki, vi).map(|written| {
      #[cfg(debug_assertions)]
      crate::debug_assert_write_eq::<Self>(written, encoded_entry_len);

      total
    })
  }

  pub(super) fn encoded_repeated_entry_len<KW, VW>(
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
    let encoded_len = self.encoded_entry_len_helper::<KW, VW>(ctx, ki, vi);
    len += varing::encoded_u32_varint_len(encoded_len as u32);
    len += encoded_len;
    len
  }

  pub(super) fn encode_packed_entry<KW, VW>(
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
    let encoded_entry_len = self.encoded_entry_len_helper::<KW, VW>(ctx, ki, vi);
    let buf_len = buf.len();

    let offset = varing::encode_u32_varint_to(encoded_entry_len as u32, buf)
      .map_err(|e| Error::from_varint_encode_error(e).update(encoded_entry_len, buf_len))?;
    let total = offset + encoded_entry_len;
    if total >= buf_len {
      return Err(Error::insufficient_buffer(total, buf_len));
    }

    self.encode_entry_helper(ctx, buf, ki, vi).map(|written| {
      #[cfg(debug_assertions)]
      crate::debug_assert_write_eq::<Self>(written, encoded_entry_len);

      total
    })
  }

  pub(super) fn encoded_packed_entry_len<KW, VW>(
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
    let encoded_len = self.encoded_entry_len_helper::<KW, VW>(ctx, ki, vi);
    let mut len = varing::encoded_u32_varint_len(encoded_len as u32);
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

  pub(super) fn decode_repeated_entry<'de, KW, VW, RB, UB>(
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

  fn encode_entry_helper<KW, VW>(
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
    let encoded_len = self.encoded_entry_len_helper::<KW, VW>(ctx, ki, vi);
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

  fn encoded_entry_len_helper<KW, VW>(
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
    let mut len = 0;
    if let Some(k) = &self.key {
      len += ki.encoded_len() + k.encoded_len(ctx);
    }
    if let Some(v) = &self.value {
      len += vi.encoded_len() + v.encoded_len(ctx);
    }
    len
  }
}
