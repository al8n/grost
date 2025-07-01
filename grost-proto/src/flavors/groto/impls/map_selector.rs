// use crate::{
//   decode::{Decode, DecodeOwned},
//   encode::{Encode, PartialEncode},
//   flavors::{
//     DefaultWireFormat, Groto, WireFormat,
//     groto::{Context, DecodeError, EncodeError, Identifier, LengthDelimited, Tag, Unknown},
//   },
//   map::MapSelector,
//   selector::{Selectable, Selector},
// };

// const KEY_TAG: Tag = Tag::new(1);
// const VALUE_TAG: Tag = Tag::new(2);

// const fn key_identifier<K>() -> Identifier
// where
//   K: WireFormat<Groto>,
// {
//   Identifier::new(K::WIRE_TYPE, KEY_TAG)
// }

// const fn value_identifier<V>() -> Identifier
// where
//   V: WireFormat<Groto>,
// {
//   Identifier::new(V::WIRE_TYPE, VALUE_TAG)
// }

// impl<K, V> Selectable<Groto> for MapSelector<K, V> {
//   type Selector = ();
// }

// impl<K, V> Encode<LengthDelimited, Groto> for MapSelector<K, V>
// where
//   K: Selector<Groto> + DefaultWireFormat<Groto> + Encode<Groto, K::Format>,
//   V: Selector<Groto> + DefaultWireFormat<Groto> + Encode<Groto, V::Format>,
// {
//   fn encode(&self, ctx: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
//     let kid = key_identifier::<K::Format>();
//     let vid = value_identifier::<V::Format>();

//     let mut offset = kid.encode_to(buf)?;
//     offset += self
//       .key()
//       .encode_length_delimited(ctx, &mut buf[offset..])?;
//     offset += vid.encode_to(&mut buf[offset..])?;
//     offset += self
//       .value()
//       .encode_length_delimited(ctx, &mut buf[offset..])?;

//     #[cfg(debug_assertions)]
//     crate::debug_assert_write_eq::<(K, V)>(offset, self.encoded_len(ctx));
//     Ok(offset)
//   }

//   fn encoded_len(&self, context: &Context) -> usize {
//     let kid = key_identifier::<K::Format>();
//     let vid = value_identifier::<V::Format>();

//     let mut len = kid.encoded_len();
//     len += self.key().encoded_length_delimited_len(context);
//     len += vid.encoded_len();
//     len += self.value().encoded_length_delimited_len(context);

//     len
//   }

//   fn encoded_length_delimited_len(&self, context: &Context) -> usize {
//     let encoded_len = self.encoded_len(context);
//     let len = varing::encoded_u32_varint_len(encoded_len as u32);
//     len + encoded_len
//   }

//   fn encode_length_delimited(
//     &self,
//     context: &Context,
//     buf: &mut [u8],
//   ) -> Result<usize, EncodeError> {
//     let encoded_len = self.encoded_len(context);
//     let len = varing::encode_u32_varint_to(encoded_len as u32, buf)?;
//     let total = len + encoded_len;
//     if total > buf.len() {
//       return Err(EncodeError::insufficient_buffer(total, buf.len()));
//     }

//     self.encode(context, &mut buf[len..]).map(|_| total)
//   }
// }

// impl<K, V> PartialEncode<LengthDelimited, Groto> for MapSelector<K, V>
// where
//   K: Selector<Groto> + DefaultWireFormat<Groto> + PartialEncode<Groto, K::Format>,
//   V: Selector<Groto> + DefaultWireFormat<Groto> + PartialEncode<Groto, V::Format>,
// {
//   fn partial_encode(
//     &self,
//     context: &Context,
//     buf: &mut [u8],
//     _: &Self::Selector,
//   ) -> Result<usize, EncodeError> {
//     <Self as Encode<LengthDelimited, Groto>>::encode(self, context, buf)
//   }

//   fn partial_encoded_len(&self, context: &Context, _: &Self::Selector) -> usize {
//     <Self as Encode<LengthDelimited, Groto>>::encoded_len(self, context)
//   }

//   fn partial_encoded_length_delimited_len(&self, context: &Context, _: &Self::Selector) -> usize {
//     <Self as Encode<LengthDelimited, Groto>>::encoded_length_delimited_len(self, context)
//   }

//   fn partial_encode_length_delimited(
//     &self,
//     context: &Context,
//     buf: &mut [u8],
//     _: &Self::Selector,
//   ) -> Result<usize, EncodeError> {
//     <Self as Encode<LengthDelimited, Groto>>::encode_length_delimited(self, context, buf)
//   }
// }

// impl<'de, K, V> Decode<'de, Groto, LengthDelimited, Self> for MapSelector<K, V>
// where
//   K: Selector<Groto> + DefaultWireFormat<Groto> + Decode<'de, Groto, K::Format, K>,
//   V: Selector<Groto> + DefaultWireFormat<Groto> + Decode<'de, Groto, V::Format, V>,
// {
//   fn decode<UB>(context: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
//   where
//     Self: Sized + 'de,
//     UB: crate::buffer::Buffer<Unknown<B>> + 'de,
//   {
//     let mut offset = 0;
//     let buf_len = src.len();
//     let kid = key_identifier::<K::Format>();
//     let vid = value_identifier::<V::Format>();
//     let mut key: Option<K> = None;
//     let mut value: Option<V> = None;

//     while offset < buf_len {
//       let (read, identifier) = Identifier::decode(src)?;
//       offset += read;
//       match () {
//         () if identifier == kid => {
//           let (read, decoded) = K::decode::<UB>(context, &src[offset..])?;
//           offset += read;
//           if offset > buf_len {
//             return Err(DecodeError::buffer_underflow());
//           }
//           if key.is_some() {
//             return Err(DecodeError::duplicate_field(
//               core::any::type_name::<Self>(),
//               "key",
//               kid,
//             ));
//           }

//           key = Some(decoded);
//         }
//         () if identifier == vid => {
//           let (read, decoded) = V::decode::<UB>(context, &src[offset..])?;
//           offset += read;
//           if offset > buf_len {
//             return Err(DecodeError::buffer_underflow());
//           }
//           if value.is_some() {
//             return Err(DecodeError::duplicate_field(
//               core::any::type_name::<Self>(),
//               "value",
//               vid,
//             ));
//           }
//           value = Some(decoded);
//         }
//         _ => {
//           return Err(DecodeError::unknown_identifier(
//             core::any::type_name::<Self>(),
//             identifier,
//           ));
//         }
//       }
//     }

//     Ok((offset, Self::new(key, value)))
//   }

//   fn decode_length_delimited<UB>(
//     context: &Context,
//     src: &'de [u8],
//   ) -> Result<(usize, Self), DecodeError>
//   where
//     Self: Sized + 'de,
//     UB: crate::buffer::Buffer<Unknown<B>> + 'de,
//   {
//     let (mut offset, data_len) = varing::decode_u32_varint(src)?;
//     let data_len = data_len as usize;
//     let total = offset + data_len;
//     if total > src.len() {
//       return Err(DecodeError::buffer_underflow());
//     }
//     let (read, data) = Self::decode::<UB>(context, &src[offset..])?;
//     offset += read;

//     #[cfg(debug_assertions)]
//     crate::debug_assert_read_eq::<Self>(offset, total);

//     Ok((total, data))
//   }
// }

// impl<K, V> DecodeOwned<Groto, LengthDelimited, Self> for MapSelector<K, V>
// where
//   K: Selector<Groto> + DefaultWireFormat<Groto> + DecodeOwned<Groto, K::Format, K>,
//   V: Selector<Groto> + DefaultWireFormat<Groto> + DecodeOwned<Groto, V::Format, V>,
// {
//   fn decode_owned<B, UB>(context: &Context, src: B) -> Result<(usize, Self), DecodeError>
//   where
//     Self: Sized,
//     B: crate::buffer::BytesBuffer + 'static,
//     UB: crate::buffer::Buffer<Unknown<B>> + 'static,
//   {
//     let mut offset = 0;
//     let buf_len = src.len();
//     let kid = key_identifier::<K::Format>();
//     let vid = value_identifier::<V::Format>();
//     let mut key: Option<K> = None;
//     let mut value: Option<V> = None;
//     let buf = src.as_bytes();

//     while offset < buf_len {
//       let (read, identifier) = Identifier::decode(&buf[offset..])?;
//       offset += read;
//       match () {
//         () if identifier == kid => {
//           let (read, decoded) = K::decode_owned::<B, UB>(context, src.slice(offset..))?;
//           offset += read;
//           if offset > buf_len {
//             return Err(DecodeError::buffer_underflow());
//           }
//           if key.is_some() {
//             return Err(DecodeError::duplicate_field(
//               core::any::type_name::<Self>(),
//               "key",
//               kid,
//             ));
//           }

//           key = Some(decoded);
//         }
//         () if identifier == vid => {
//           let (read, decoded) = V::decode_owned::<B, UB>(context, src.slice(offset..))?;
//           offset += read;
//           if offset > buf_len {
//             return Err(DecodeError::buffer_underflow());
//           }
//           if value.is_some() {
//             return Err(DecodeError::duplicate_field(
//               core::any::type_name::<Self>(),
//               "value",
//               vid,
//             ));
//           }
//           value = Some(decoded);
//         }
//         _ => {
//           return Err(DecodeError::unknown_identifier(
//             core::any::type_name::<Self>(),
//             identifier,
//           ));
//         }
//       }
//     }

//     Ok((offset, Self::new(key, value)))
//   }

//   fn decode_length_delimited_owned<B, UB>(
//     context: &Context,
//     src: B,
//   ) -> Result<(usize, Self), DecodeError>
//   where
//     Self: Sized,
//     B: crate::buffer::BytesBuffer + 'static,
//     UB: crate::buffer::Buffer<Unknown<B>> + 'static,
//   {
//     let (mut offset, data_len) = varing::decode_u32_varint(src.as_bytes())?;
//     let data_len = data_len as usize;
//     let total = offset + data_len;
//     if total > src.len() {
//       return Err(DecodeError::buffer_underflow());
//     }
//     let (read, data) = Self::decode_owned::<B, UB>(context, src.slice(offset..))?;
//     offset += read;

//     #[cfg(debug_assertions)]
//     crate::debug_assert_read_eq::<Self>(offset, total);

//     Ok((total, data))
//   }
// }
