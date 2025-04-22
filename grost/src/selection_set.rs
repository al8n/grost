// use core::num::NonZeroUsize;

// #[cfg(feature = "bytes_1")]
// use bytes_1 as bytes;

// use grost_proto::Identifier;
// use varing::{Varint, utils::BytesBuffer};

// use crate::{IntoTarget, TypeBorrowed};

// use super::{
//   Decode, DecodeOwned, Encode, Message, Tag, TypeOwned, TypeRef, Unknown, UnknownRef,
//   UnknownRefBytesBuffer, Wirable, WireType,
//   error::{DecodeError, EncodeError},
// };

use grost_proto::{flavors::{network::WireType, Flavor}, unknown::Unknown, Tag};
use varing::Varint;

use crate::flavors::network::{EncodeError, Identifier};

pub struct SelectionSet<S, UB> {
  selection: S,
  unknown_buffer: Option<UB>,
}

impl<S, UB> SelectionSet<S, UB> {
  /// Creates a new selection set with the given selection and unknown buffer.
  #[inline]
  pub const fn new(selection: S, unknown_buffer: Option<UB>) -> Self {
    Self { selection, unknown_buffer }
  }

  /// Returns the selection set.
  #[inline]
  pub const fn selection(&self) -> &S {
    &self.selection
  }

  /// Returns the unknown buffer.
  #[inline]
  pub const fn unknown_buffer(&self) -> Option<&UB> {
    self.unknown_buffer.as_ref()
  }

  /// Consumes the selection set and returns the components.
  #[inline]
  pub fn into_components(self) -> (S, Option<UB>) {
    (self.selection, self.unknown_buffer)
  }
}

const ALL_TAG: Tag = Tag::new(1);
const NONE_TAG: Tag = Tag::new(2);
const SELECT_TAG: Tag = Tag::new(3);
const UNSELECT_TAG: Tag = Tag::new(4);
const SELECT_ONE_TAG: Tag = Tag::new(5);
const UNSELECT_ONE_TAG: Tag = Tag::new(6);

const ALL_IDENTIFIER: Identifier = Identifier::new(WireType::Zst, ALL_TAG);
const NONE_IDENTIFIER: Identifier = Identifier::new(WireType::Zst, NONE_TAG);
const SELECT_IDENTIFIER: Identifier = Identifier::new(WireType::LengthDelimited, SELECT_TAG);
const UNSELECT_IDENTIFIER: Identifier = Identifier::new(WireType::LengthDelimited, UNSELECT_TAG);
const SELECT_ONE_IDENTIFIER: Identifier = Identifier::new(WireType::Varint, SELECT_ONE_TAG);
const UNSELECT_ONE_IDENTIFIER: Identifier = Identifier::new(WireType::Varint, UNSELECT_ONE_TAG);

const ALL_IDENTIFIER_ENCODED_LEN: usize = ALL_IDENTIFIER.encoded_len();
const NONE_IDENTIFIER_ENCODED_LEN: usize = NONE_IDENTIFIER.encoded_len();
const SELECT_IDENTIFIER_ENCODED_LEN: usize = SELECT_IDENTIFIER.encoded_len();
const UNSELECT_IDENTIFIER_ENCODED_LEN: usize = UNSELECT_IDENTIFIER.encoded_len();
const SELECT_ONE_IDENTIFIER_ENCODED_LEN: usize = SELECT_ONE_IDENTIFIER.encoded_len();
const UNSELECT_ONE_IDENTIFIER_ENCODED_LEN: usize = UNSELECT_ONE_IDENTIFIER.encoded_len();

const ALL_IDENTIFIER_ENCODED: &[u8] = ALL_IDENTIFIER.encode().as_slice();
const NONE_IDENTIFIER_ENCODED: &[u8] = NONE_IDENTIFIER.encode().as_slice();
const SELECT_IDENTIFIER_ENCODED: &[u8] = SELECT_IDENTIFIER.encode().as_slice();
const UNSELECT_IDENTIFIER_ENCODED: &[u8] = UNSELECT_IDENTIFIER.encode().as_slice();
const SELECT_ONE_IDENTIFIER_ENCODED: &[u8] = SELECT_ONE_IDENTIFIER.encode().as_slice();
const UNSELECT_ONE_IDENTIFIER_ENCODED: &[u8] = UNSELECT_ONE_IDENTIFIER.encode().as_slice();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Repr<S> {
  All,
  None,
  Select(S),
  Unselect(S),
}

/// A selection set encoder
pub struct SelectionEncoder<S> {
  repr: Repr<S>,
}

impl<S> SelectionEncoder<S> {
  /// Creates a new selection encoder with the given representation.
  #[inline]
  pub const fn all() -> Self {
    Self { repr: Repr::<S>::All }
  }

  /// Creates a new selection encoder with the given representation.
  #[inline]
  pub const fn none() -> Self {
    Self { repr: Repr::<S>::None }
  }

  /// Creates a new selection encoder with the given representation.
  #[inline]
  pub const fn select(set: S) -> Self {
    Self { repr: Repr::<S>::Select(set) }
  }

  /// Creates a new selection encoder with the given representation.
  #[inline]
  pub const fn unselect(set: S) -> Self {
    Self { repr: Repr::<S>::Unselect(set) }
  }
}

impl<S> SelectionEncoder<S>
where
  S: IntoIterator<Item = Tag> + Clone,
  S::IntoIter: ExactSizeIterator,
{
  /// Returns the encoded length of the selection.
  pub fn encoded_len(&self) -> usize {
    macro_rules! len {
      ($iter:ident, $len:ident) => {{
        let data_len = $iter.map(|t| t.get().encoded_len()).sum::<usize>();
        let len_size = (data_len as u32).encoded_len(); 
        $len + len_size + data_len
      }};
    }

    match &self.repr {
      Repr::All => ALL_IDENTIFIER_ENCODED_LEN,
      Repr::None => NONE_IDENTIFIER_ENCODED_LEN,
      Repr::Select(set) => {
        let mut iter = set.clone().into_iter();
        match iter.len() {
          0 => ALL_IDENTIFIER_ENCODED_LEN,
          1 => SELECT_ONE_IDENTIFIER_ENCODED_LEN + iter.next().unwrap().get().encoded_len(),
          _ => len!(iter, SELECT_IDENTIFIER_ENCODED_LEN),
        }
      },
      Repr::Unselect(set) => {
        let mut iter = set.clone().into_iter();
        match iter.len() {
          0 => NONE_IDENTIFIER_ENCODED_LEN,
          1 => UNSELECT_ONE_IDENTIFIER_ENCODED_LEN + iter.next().unwrap().get().encoded_len(),
          _ => len!(iter, UNSELECT_IDENTIFIER_ENCODED_LEN),
        }
      },
    }
  }

  /// Encodes the selection into the given buffer.
  pub fn encode(&self, dst: &mut [u8]) -> Result<usize, EncodeError> {
    macro_rules! encode {
      (@all) => {{
        if dst.len() < ALL_IDENTIFIER_ENCODED_LEN {
          return Err(EncodeError::insufficient_buffer(ALL_IDENTIFIER_ENCODED_LEN, dst.len()));
        }

        dst[..ALL_IDENTIFIER_ENCODED_LEN].copy_from_slice(ALL_IDENTIFIER_ENCODED);
        Ok(ALL_IDENTIFIER_ENCODED_LEN)
      }};
      (@none) => {{
        if dst.len() < NONE_IDENTIFIER_ENCODED_LEN {
          return Err(EncodeError::insufficient_buffer(NONE_IDENTIFIER_ENCODED_LEN, dst.len()));
        }

        dst[..NONE_IDENTIFIER_ENCODED_LEN].copy_from_slice(NONE_IDENTIFIER_ENCODED);
        Ok(NONE_IDENTIFIER_ENCODED_LEN)
      }};
      (@one $iter:ident, $len:ident) => {{
        if dst.len() < $len {
          return Err(EncodeError::insufficient_buffer(
            self.encoded_len(),
            dst.len(),
          ));
        }

        $iter.next().unwrap().get().encode(&mut dst[$len..]).map_err(|e| EncodeError::from_varint_error(e).update(self.encoded_len(), dst.len()))
      }};
      (@many $set:ident, $iter:ident, $encoded_len:ident, $encoded:ident) => {{
        let mut offset = 0;
        if dst.len() < $encoded_len {
          return Err(EncodeError::insufficient_buffer(
            self.encoded_len(),
            dst.len(),
          ));
        }
        dst[..$encoded_len].copy_from_slice($encoded);
        offset += $encoded_len;
        let data_len = $iter.map(|t| t.get().encoded_len()).sum::<usize>();
        let write = (data_len as u32).encode(&mut dst[offset..]).map_err(|e| EncodeError::from_varint_error(e).update(self.encoded_len(), dst.len()))?;
        offset += write;

        for tag in $set.clone() {
          let write = tag.get().encode(&mut dst[offset..]).map_err(|e| EncodeError::from_varint_error(e).update(self.encoded_len(), dst.len()))?;
          offset += write;
        }
        Ok(offset)
      }};
    }

    match &self.repr {
      Repr::All => encode!(@all),
      Repr::None => encode!(@none),
      Repr::Select(set) => {
        let mut iter = set.clone().into_iter();
        match iter.len() {
          0 => encode!(@all),
          1 => encode!(@one iter, SELECT_ONE_IDENTIFIER_ENCODED_LEN),
          _ => encode!(@many set, iter, SELECT_IDENTIFIER_ENCODED_LEN, SELECT_IDENTIFIER_ENCODED), 
        }
      },
      Repr::Unselect(set) => {
        let mut iter = set.clone().into_iter();
        match iter.len() {
          0 => encode!(@none),
          1 => encode!(@one iter, UNSELECT_ONE_IDENTIFIER_ENCODED_LEN),
          _ => encode!(@many set, iter, UNSELECT_IDENTIFIER_ENCODED_LEN, UNSELECT_IDENTIFIER_ENCODED),
        }
      },
    }
  }
}

/// A selection kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionIdentifier {
  /// Select all fields
  All,
  /// Select no fields
  None,
  /// Select specific fields
  Select,
  /// Unselect specific fields
  Unselect,
  /// Select one specific field
  SelectOne,
  /// Unselect one specific field
  UnselectOne,
  /// Unknown selection identifier
  Unknown(u32),
}

pub struct SelectionDecoder {
  
}

impl SelectionDecoder {
  /// Decodes the selection identifier from the given buffer.
  pub fn decode_identifier<F: Flavor>(src: &[u8]) -> Result<(usize, SelectionIdentifier), grost_proto::DecodeError<F>> {
    let (read, val) = u32::decode(src)?;
    let identifier = Identifier::from_u32(val);

    Ok(match identifier {
      ALL_IDENTIFIER => (read, SelectionIdentifier::All),
      NONE_IDENTIFIER => (read, SelectionIdentifier::None),
      SELECT_IDENTIFIER => (read, SelectionIdentifier::Select),
      UNSELECT_IDENTIFIER => (read, SelectionIdentifier::Unselect),
      SELECT_ONE_IDENTIFIER => (read, SelectionIdentifier::SelectOne),
      UNSELECT_ONE_IDENTIFIER => (read, SelectionIdentifier::UnselectOne),
      _ => (read, SelectionIdentifier::Unknown(val)),
    })
  }
}

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

// type U32VarintBytesBuffer = BytesBuffer<{ <u32 as Varint>::MAX_ENCODED_LEN + 1 }>;

// const ALL_TAG: Tag = Tag::new(1);
// const NONE_TAG: Tag = Tag::new(2);
// const SET_TAG: Tag = Tag::new(3);
// const ALL_IDENTIFIER: Identifier = Identifier::new(WireType::Zst, ALL_TAG);
// const NONE_IDENTIFIER: Identifier = Identifier::new(WireType::Zst, NONE_TAG);
// const ALL_ENCODED_LEN: usize = ALL_IDENTIFIER.encoded_len();
// const NONE_ENCODED_LEN: usize = NONE_IDENTIFIER.encoded_len();
// const ALL_ENCODED: U32VarintBytesBuffer = ALL_IDENTIFIER.encode();
// const NONE_ENCODED: U32VarintBytesBuffer = NONE_IDENTIFIER.encode();

// impl<S, B> SelectionSet<S, B>
// where
//   S: Wirable,
// {
//   const SET_IDENTIFIER: Identifier = Identifier::new(S::WIRE_TYPE, SET_TAG);
//   const SET_IDENTIFIER_ENCODED_LEN: usize = Self::SET_IDENTIFIER.encoded_len();
//   const SET_IDENTIFIER_ENCODED: U32VarintBytesBuffer = Self::SET_IDENTIFIER.encode();
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
//         let total = Self::SET_IDENTIFIER_ENCODED_LEN + set_len;
//         if buf_len < total {
//           return Err(EncodeError::insufficient_buffer(total, buf_len));
//         }

//         $buf[..Self::SET_IDENTIFIER_ENCODED_LEN].copy_from_slice(&Self::SET_IDENTIFIER_ENCODED);
//         set.encode_with_identifier(&mut $buf[Self::SET_IDENTIFIER_ENCODED_LEN..])?;
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
//       Self::Set(set) => Self::SET_IDENTIFIER_ENCODED_LEN + set.encoded_len_with_identifier(),
//       Self::Unknown(unknown) => unknown.encoded_len(),
//     }
//   }};
// }

// macro_rules! decode {
//   ($ty:ident::$de:ident($buf:ident,$unknown_buffer:ident $(, $converter:ident)?)) => {{
//     let (mut offset, merged) = Identifier::decode($buf)?;

//     match merged {
//       ALL_IDENTIFIER => Ok((offset, Self::All)),
//       NONE_IDENTIFIER => Ok((offset, Self::None)),
//       val if val == Self::SET_IDENTIFIER => {
//         let (set_offset, set) = S::decode_length_prefix(&$buf[offset..], $unknown_buffer)?;
//         offset += set_offset;
//         Ok((offset, Self::Set(set)))
//       }
//       _ => {
//         let (readed, unknown) = $ty::$de($buf)?;
//         if $unknown_buffer.push(unknown).is_some() {
//           return Err(DecodeError::buffer_overflow(
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
//     B: super::UnknownRefBytesBuffer<'a>,
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
//   const SET_IDENTIFIER: Identifier = Identifier::new(S::WIRE_TYPE, SET_TAG);
//   const SET_IDENTIFIER_ENCODED_LEN: usize = Self::SET_IDENTIFIER.encoded_len();
//   const SET_IDENTIFIER_ENCODED: U32VarintBytesBuffer = Self::SET_IDENTIFIER.encode();
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
//     B: UnknownRefBytesBuffer<'de>,
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
//       ALL_IDENTIFIER => Ok((offset, Self::All)),
//       NONE_IDENTIFIER => Ok((offset, Self::None)),
//       val if val == Self::SET_IDENTIFIER => {
//         let (set_offset, set) =
//           S::decode_length_prefix_from_bytes(buf.slice(offset..), unknown_buffer)?;
//         offset += set_offset;
//         Ok((offset, Self::Set(set)))
//       }
//       _ => {
//         let (readed, unknown) = Unknown::decode_owned(&buf)?;
//         if unknown_buffer.push(unknown.clone()).is_some() {
//           return Err(DecodeError::buffer_overflow(
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
