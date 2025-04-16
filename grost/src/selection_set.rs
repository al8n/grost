// use core::num::NonZeroUsize;

// #[cfg(feature = "bytes_1")]
// use bytes_1 as bytes;

// use grost_types::Identifier;
// use varing::{Varint, utils::Buffer};

// use crate::{IntoTarget, TypeBorrowed};

// use super::{
//   Decode, DecodeOwned, Encode, Message, Tag, TypeOwned, TypeRef, Unknown, UnknownRef,
//   UnknownRefBuffer, Wirable, WireType,
//   error::{DecodeError, EncodeError},
// };

// /// A selection set.
// #[derive(
//   Debug,
//   Default,
//   Copy,
//   Clone,
//   PartialEq,
//   Eq,
//   Hash,
//   derive_more::IsVariant,
//   derive_more::Unwrap,
//   derive_more::TryUnwrap,
// )]
// #[unwrap(ref, ref_mut)]
// #[try_unwrap(ref, ref_mut)]
// #[non_exhaustive]
// pub enum SelectionSet<S, B> {
//   /// Select all fields.
//   #[default]
//   All,
//   /// Select no fields.
//   None,
//   /// Select specific fields.
//   Set(S),
//   /// The unknown selection set.
//   Unknown(Unknown<B>),
// }

// type U32VarintBuffer = Buffer<{ <u32 as Varint>::MAX_ENCODED_LEN + 1 }>;

// const ALL_TAG: Tag = Tag::new(1);
// const NONE_TAG: Tag = Tag::new(2);
// const SET_TAG: Tag = Tag::new(3);
// const ALL_MERGED: Identifier = Identifier::new(WireType::Zst, ALL_TAG);
// const NONE_MERGED: Identifier = Identifier::new(WireType::Zst, NONE_TAG);
// const ALL_ENCODED_LEN: usize = ALL_MERGED.encoded_len();
// const NONE_ENCODED_LEN: usize = NONE_MERGED.encoded_len();
// const ALL_ENCODED: U32VarintBuffer = ALL_MERGED.encode();
// const NONE_ENCODED: U32VarintBuffer = NONE_MERGED.encode();

// impl<S, B> SelectionSet<S, B>
// where
//   S: Wirable,
// {
//   const SET_MERGED: Identifier = Identifier::new(S::WIRE_TYPE, SET_TAG);
//   const SET_MERGED_ENCODED_LEN: usize = Self::SET_MERGED.encoded_len();
//   const SET_MERGED_ENCODED: U32VarintBuffer = Self::SET_MERGED.encode();
// }

// impl<S, B> Message for SelectionSet<S, B>
// where
//   S: Message,
//   B: for<'a> From<&'a [u8]> + AsRef<[u8]> + Clone,
// {
//   type Encoded<'a>
//     = EncodedSelectionSet<'a, S::Encoded<'a>>
//   where
//     Self: Sized + 'a;

//   type Borrowed<'a>
//     = SelectionSet<S::Borrowed<'a>, &'a B>
//   where
//     Self: 'a;

//   #[cfg(any(feature = "std", feature = "alloc"))]
//   type EncodedOwned
//     = SelectionSet<S::EncodedOwned, bytes::Bytes>
//   where
//     Self: Sized + 'static;

//   #[cfg(not(any(feature = "std", feature = "alloc")))]
//   type EncodedOwned
//     = SelectionSet<S::EncodedOwned, B>
//   where
//     Self: Sized + 'static;
// }

// impl<S, B> Wirable for SelectionSet<S, B> where S: Wirable {}

// macro_rules! encode {
//   ($this:ident($buf:ident)) => {{
//     match $this {
//       Self::All => {
//         let buf_len = $buf.len();
//         if buf_len < ALL_ENCODED_LEN {
//           return Err(EncodeError::insufficient_buffer(ALL_ENCODED_LEN, buf_len));
//         }

//         $buf[..ALL_ENCODED_LEN].copy_from_slice(&ALL_ENCODED);
//         Ok(ALL_ENCODED_LEN)
//       }
//       Self::None => {
//         let buf_len = $buf.len();
//         if buf_len < NONE_ENCODED_LEN {
//           return Err(EncodeError::insufficient_buffer(NONE_ENCODED_LEN, buf_len));
//         }

//         $buf[..NONE_ENCODED_LEN].copy_from_slice(&NONE_ENCODED);
//         Ok(NONE_ENCODED_LEN)
//       }
//       Self::Set(set) => {
//         let buf_len = $buf.len();
//         let set_len = set.encoded_len_with_identifier();
//         let total = Self::SET_MERGED_ENCODED_LEN + set_len;
//         if buf_len < total {
//           return Err(EncodeError::insufficient_buffer(total, buf_len));
//         }

//         $buf[..Self::SET_MERGED_ENCODED_LEN].copy_from_slice(&Self::SET_MERGED_ENCODED);
//         set.encode_with_identifier(&mut $buf[Self::SET_MERGED_ENCODED_LEN..])?;
//         Ok(total)
//       }
//       Self::Unknown(unknown) => unknown.encode($buf),
//     }
//   }};
// }

// macro_rules! encoded_len {
//   ($this:ident()) => {{
//     match $this {
//       Self::All => ALL_ENCODED_LEN,
//       Self::None => NONE_ENCODED_LEN,
//       Self::Set(set) => Self::SET_MERGED_ENCODED_LEN + set.encoded_len_with_identifier(),
//       Self::Unknown(unknown) => unknown.encoded_len(),
//     }
//   }};
// }

// macro_rules! decode {
//   ($ty:ident::$de:ident($buf:ident,$unknown_buffer:ident $(, $converter:ident)?)) => {{
//     let (mut offset, merged) = Identifier::decode($buf)?;

//     match merged {
//       ALL_MERGED => Ok((offset, Self::All)),
//       NONE_MERGED => Ok((offset, Self::None)),
//       val if val == Self::SET_MERGED => {
//         let (set_offset, set) = S::decode_length_prefix(&$buf[offset..], $unknown_buffer)?;
//         offset += set_offset;
//         Ok((offset, Self::Set(set)))
//       }
//       _ => {
//         let (readed, unknown) = $ty::$de($buf)?;
//         if $unknown_buffer.push(unknown).is_some() {
//           return Err(DecodeError::unknown_buffer_overflow(
//             $unknown_buffer.len(),
//             NonZeroUsize::new($unknown_buffer.len() + 1).unwrap(),
//           ));
//         }
//         Ok((readed, Self::Unknown(unknown $(.$converter())?)))
//       }
//     }
//   }};
// }

// impl<S, B> Encode for SelectionSet<S, B>
// where
//   S: Encode,
//   B: AsRef<[u8]>,
// {
//   fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
//     encode!(self(buf))
//   }

//   fn encoded_len(&self) -> usize {
//     encoded_len!(self())
//   }
// }

// impl<'a, S, U> Decode<'a> for SelectionSet<S, U>
// where
//   S: Decode<'a> + 'a,
//   U: From<&'a [u8]>,
// {
//   fn decode<B>(buf: &'a [u8], unknown_buffer: &mut B) -> Result<(usize, Self), DecodeError>
//   where
//     B: super::UnknownRefBuffer<'a>,
//   {
//     decode!(UnknownRef::decode(buf, unknown_buffer, to_owned))
//   }
// }

// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// #[non_exhaustive]
// pub enum EncodedSelectionSet<'a, S> {
//   All,
//   None,
//   Set(S),
//   Unknown(UnknownRef<'a>),
// }

// impl<S> EncodedSelectionSet<'_, S>
// where
//   S: Wirable,
// {
//   const SET_MERGED: Identifier = Identifier::new(S::WIRE_TYPE, SET_TAG);
//   const SET_MERGED_ENCODED_LEN: usize = Self::SET_MERGED.encoded_len();
//   const SET_MERGED_ENCODED: U32VarintBuffer = Self::SET_MERGED.encode();
// }

// impl<S> Wirable for EncodedSelectionSet<'_, S> {}

// impl<S> Encode for EncodedSelectionSet<'_, S>
// where
//   S: Encode,
// {
//   fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
//     encode!(self(buf))
//   }

//   fn encoded_len(&self) -> usize {
//     encoded_len!(self())
//   }
// }

// impl<'de, S> Decode<'de> for EncodedSelectionSet<'de, S>
// where
//   S: Decode<'de>,
// {
//   fn decode<B>(buf: &'de [u8], unknown_buffer: &mut B) -> Result<(usize, Self), DecodeError>
//   where
//     Self: Sized + 'de,
//     B: UnknownRefBuffer<'de>,
//   {
//     decode!(UnknownRef::decode(buf, unknown_buffer))
//   }
// }

// impl<'a, S, B> IntoTarget<SelectionSet<S, B>> for EncodedSelectionSet<'a, S::Encoded<'a>>
// where
//   S: Message,
//   B: From<&'a [u8]>,
// {
//   fn into_target(self) -> Result<SelectionSet<S, B>, DecodeError> {
//     Ok(match self {
//       Self::All => SelectionSet::All,
//       Self::None => SelectionSet::None,
//       Self::Set(set) => return set.into_target().map(SelectionSet::Set),
//       Self::Unknown(unknown) => SelectionSet::Unknown(unknown.to_owned()),
//     })
//   }
// }

// impl<'a, S, B> TypeRef<SelectionSet<S, B>> for EncodedSelectionSet<'a, S::Encoded<'a>>
// where
//   S: Message,
//   B: From<&'a [u8]>,
// {
//   fn to(&self) -> Result<SelectionSet<S, B>, DecodeError> {
//     Ok(match self {
//       Self::All => SelectionSet::All,
//       Self::None => SelectionSet::None,
//       Self::Set(set) => return set.to().map(SelectionSet::Set),
//       Self::Unknown(unknown) => SelectionSet::Unknown(unknown.to_owned()),
//     })
//   }
// }

// impl<'a, S, B> TypeBorrowed<'a, SelectionSet<S, B>> for SelectionSet<S::Borrowed<'a>, &'a B>
// where
//   S: Message,
// {
//   fn from_borrow(value: &'a SelectionSet<S, B>) -> Self {
//     match value {
//       SelectionSet::All => SelectionSet::All,
//       SelectionSet::None => SelectionSet::None,
//       SelectionSet::Set(set) => {
//         SelectionSet::Set(<S::Borrowed<'_> as TypeBorrowed<'_, _>>::from_borrow(set))
//       }
//       SelectionSet::Unknown(unknown) => SelectionSet::Unknown(unknown.borrow()),
//     }
//   }
// }

// #[cfg(any(feature = "std", feature = "alloc"))]
// impl<S, B> IntoTarget<SelectionSet<S, B>> for SelectionSet<S::EncodedOwned, bytes::Bytes>
// where
//   S: Message,
//   B: for<'a> From<&'a [u8]>,
// {
//   fn into_target(self) -> Result<SelectionSet<S, B>, DecodeError> {
//     Ok(match self {
//       Self::All => SelectionSet::All,
//       Self::None => SelectionSet::None,
//       Self::Set(set) => return set.into_target().map(SelectionSet::Set),
//       Self::Unknown(unknown) => SelectionSet::Unknown(unknown.map()),
//     })
//   }
// }

// #[cfg(not(any(feature = "std", feature = "alloc")))]
// impl<S, B> IntoTarget<SelectionSet<S, B>> for SelectionSet<S::EncodedOwned, B>
// where
//   S: Message,
//   B: for<'a> From<&'a [u8]>,
// {
//   fn into_target(self) -> Result<SelectionSet<S, B>, DecodeError> {
//     Ok(match self {
//       Self::All => SelectionSet::All,
//       Self::None => SelectionSet::None,
//       Self::Set(set) => return set.into_target().map(SelectionSet::Set),
//       Self::Unknown(unknown) => SelectionSet::Unknown(unknown),
//     })
//   }
// }

// #[cfg(any(feature = "std", feature = "alloc"))]
// impl<S, B> TypeOwned<SelectionSet<S, B>> for SelectionSet<S::EncodedOwned, bytes::Bytes>
// where
//   S: Message,
//   B: for<'a> From<&'a [u8]>,
// {
//   fn to(&self) -> Result<SelectionSet<S, B>, DecodeError> {
//     Ok(match self {
//       Self::All => SelectionSet::All,
//       Self::None => SelectionSet::None,
//       Self::Set(set) => return set.to().map(SelectionSet::Set),
//       Self::Unknown(unknown) => SelectionSet::Unknown(unknown.map()),
//     })
//   }
// }

// #[cfg(not(any(feature = "std", feature = "alloc")))]
// impl<S, B> TypeOwned<SelectionSet<S, B>> for SelectionSet<S::EncodedOwned, B>
// where
//   S: Message,
//   B: for<'a> From<&'a [u8]> + Clone,
// {
//   fn to(&self) -> Result<SelectionSet<S, B>, DecodeError> {
//     Ok(match self {
//       Self::All => SelectionSet::All,
//       Self::None => SelectionSet::None,
//       Self::Set(set) => return set.to().map(SelectionSet::Set),
//       Self::Unknown(unknown) => SelectionSet::Unknown(unknown.clone()),
//     })
//   }
// }

// #[cfg(any(feature = "std", feature = "alloc"))]
// impl<S> DecodeOwned for SelectionSet<S, bytes::Bytes>
// where
//   S: DecodeOwned,
// {
//   #[cfg(any(feature = "std", feature = "alloc"))]
//   fn decode_from_bytes<U>(
//     buf: bytes::Bytes,
//     unknown_buffer: &mut U,
//   ) -> Result<(usize, Self), DecodeError>
//   where
//     U: crate::UnknownBuffer<bytes::Bytes>,
//   {
//     let (mut offset, merged) = Identifier::decode(&buf)?;

//     match merged {
//       ALL_MERGED => Ok((offset, Self::All)),
//       NONE_MERGED => Ok((offset, Self::None)),
//       val if val == Self::SET_MERGED => {
//         let (set_offset, set) =
//           S::decode_length_prefix_from_bytes(buf.slice(offset..), unknown_buffer)?;
//         offset += set_offset;
//         Ok((offset, Self::Set(set)))
//       }
//       _ => {
//         let (readed, unknown) = Unknown::decode_owned(&buf)?;
//         if unknown_buffer.push(unknown.clone()).is_some() {
//           return Err(DecodeError::unknown_buffer_overflow(
//             unknown_buffer.len(),
//             NonZeroUsize::new(unknown_buffer.len() + 1).unwrap(),
//           ));
//         }
//         Ok((readed, Self::Unknown(unknown)))
//       }
//     }
//   }
// }

// #[cfg(not(any(feature = "std", feature = "alloc")))]
// impl<S, B> DecodeOwned for SelectionSet<S, B>
// where
//   S: DecodeOwned,
//   B: for<'a> From<&'a [u8]> + Clone + 'static,
// {
// }
