// use grost_proto::flavors::Flavor;

// use crate::{BytesBuffer, Wirable};

// /// A buffer that stores the [`Unknown`] data type.
// ///
// /// This trait is used for forward and backward compatibility for structs.
// pub trait UnknownBuffer<B: ?Sized, F: Flavor> {
//   /// Creates a new buffer.
//   fn new() -> Self;

//   /// Pushes the unknown data type to the buffer, if the buffer is full,
//   /// the given value will be returned back.
//   fn push(&mut self, value: Unknown<B, F>) -> Option<Unknown<B, F>>
//   where
//     B: Sized;

//   /// Returns the capacity of the buffer.
//   fn capacity(&self) -> usize;

//   /// Returns the length of the buffer.
//   fn len(&self) -> usize;

//   /// Returns a slice of the unknown data type.
//   fn as_slice(&self) -> &[Unknown<B, F>]
//   where
//     B: Sized;

//   /// Returns `true` if the buffer is empty.
//   fn is_empty(&self) -> bool {
//     self.len() == 0
//   }
// }

// impl<B: ?Sized, F: Flavor> UnknownBuffer<B, F> for () {
//   fn new() -> Self {}

//   fn push(&mut self, value: Unknown<B, F>) -> Option<Unknown<B, F>>
//   where
//     B: Sized,
//   {
//     Some(value)
//   }

//   fn capacity(&self) -> usize {
//     0
//   }

//   fn len(&self) -> usize {
//     0
//   }

//   fn as_slice(&self) -> &[Unknown<B, F>]
//   where
//     B: Sized,
//   {
//     &[]
//   }
// }

// #[cfg(any(feature = "std", feature = "alloc"))]
// const _: () = {
//   use std::vec::Vec;

//   impl<B, F: Flavor> UnknownBuffer<B, F> for Vec<Unknown<B, F>> {
//     fn new() -> Self {
//       Vec::new()
//     }

//     fn push(&mut self, value: Unknown<B, F>) -> Option<Unknown<B, F>> {
//       self.push(value);
//       None
//     }

//     fn capacity(&self) -> usize {
//       self.capacity()
//     }

//     fn len(&self) -> usize {
//       self.len()
//     }

//     fn as_slice(&self) -> &[Unknown<B, F>]
//     where
//       B: Sized,
//     {
//       self.as_slice()
//     }

//     fn is_empty(&self) -> bool {
//       self.is_empty()
//     }
//   }
// };

// #[cfg(feature = "heapless_0_8")]
// const _: () = {
//   use heapless_0_8::Vec;

//   impl<B, F: Flavor, const N: usize> UnknownBuffer<B, F> for Vec<Unknown<B, F>, N> {
//     fn new() -> Self {
//       Vec::new()
//     }

//     fn push(&mut self, value: Unknown<B, F>) -> Option<Unknown<B, F>> {
//       self.push(value).err()
//     }

//     fn capacity(&self) -> usize {
//       self.capacity()
//     }

//     fn as_slice(&self) -> &[Unknown<B, F>]
//     where
//       B: Sized,
//     {
//       self.as_ref()
//     }

//     fn len(&self) -> usize {
//       self.as_slice().len()
//     }

//     fn is_empty(&self) -> bool {
//       self.is_empty()
//     }
//   }
// };

// /// The unknown type, used for forward and backward compatibility.
// /// The data is stored as a byte array, including the wire type and the tag,
// /// and the data.
// ///
// /// When the older version trying to decode the newer version, for the tag that
// /// is not recognized, it will be stored in this variant. And when the older version
// /// trying to forward the data, the data stored in this variant will be forwarded
// /// as is.
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// pub struct Unknown<B: ?Sized, F: Flavor> {
//   tag: F::Tag,
//   wire_type: F::WireType,
//   data_offset: usize,
//   data: B,
// }

// impl<B: ?Sized, F: Flavor> Unknown<B, F> {
//   /// Returns the tag of the unknown data type.
//   #[inline]
//   pub const fn tag(&self) -> F::Tag {
//     self.tag
//   }

//   /// Returns the wire type of the unknown data type.
//   #[inline]
//   pub const fn wire_type(&self) -> F::WireType {
//     self.wire_type
//   }

//   /// Returns the actual data of the unknown data type.
//   ///
//   /// Note: The data does not include the wire type and the tag.
//   /// If you want to access the original data, use [`raw`] instead.
//   #[inline]
//   pub fn data(&self) -> &[u8]
//   where
//     B: AsRef<[u8]>,
//   {
//     &self.data.as_ref()[self.data_offset..]
//   }

//   /// Returns the raw data of the unknown data type.
//   ///
//   /// Note: The data includes the wire type and the tag.
//   /// If you want to access the actual data, use [`data`] instead.
//   #[inline]
//   pub fn raw(&self) -> &[u8]
//   where
//     B: BytesBuffer,
//   {
//     self.data.as_bytes()
//   }

//   /// Encodes the unknown data type.
//   pub fn encode(&self, dst: &mut [u8]) -> Result<usize, EncodeError>
//   where
//     B: AsRef<[u8]>,
//   {
//     let data = self.data.as_ref();
//     let data_len = data.len();
//     let dst_len = dst.len();
//     if data_len > dst_len {
//       return Err(EncodeError::insufficient_buffer(data_len, dst_len));
//     }

//     dst[..data_len].copy_from_slice(data);
//     Ok(data_len)
//   }

//   /// Decodes the unknown data type.
//   pub fn decode(identifier: Identifier, buf: B) -> Result<(usize, Self), DecodeError>
//   where
//     B: BytesBuffer + Sized,
//   {
//     macro_rules! slice {
//       ($end:ident, $buf_len:ident, $buf:ident) => {{
//         if $end == $buf_len {
//           $buf
//         } else {
//           $buf.slice(..$end)
//         }
//       }};
//     }

//     macro_rules! consume_fixed {
//       ($size:literal, $identifier_len:ident) => {{
//         let end = $identifier_len + $size;
//         let buf_len = buf.len();
//         if end > buf_len {
//           return Err(DecodeError::buffer_underflow());
//         }

//         Ok((
//           end,
//           Self {
//             identifier,
//             data_offset: $identifier_len,
//             data: slice!(end, buf_len, buf),
//           },
//         ))
//       }};
//     }

//     let src = buf.as_bytes();
//     let identifier_len = identifier.encoded_len();
//     let mut offset = 0;
//     match identifier.wire_type() {
//       WireType::LengthDelimited => {
//         if identifier_len >= src.len() {
//           return Err(DecodeError::buffer_underflow());
//         }

//         let (size_len, size) = varing::decode_u32_varint(&src[identifier_len..])?;
//         offset += size_len;
//         let end = offset + size as usize;

//         if end > src.len() {
//           return Err(DecodeError::buffer_underflow());
//         }

//         let data = buf.slice(..end);
//         Ok((
//           end,
//           Self {
//             identifier,
//             data_offset: identifier_len + size_len,
//             data,
//           },
//         ))
//       }
//       WireType::Varint => {
//         let buf_len = buf.len();
//         if identifier_len >= buf_len {
//           return Err(DecodeError::buffer_underflow());
//         }

//         let size_len = varing::consume_varint(&src[identifier_len..])?;
//         let end = identifier_len + size_len;
//         Ok((
//           end,
//           Self {
//             identifier,
//             data_offset: identifier_len,
//             data: slice!(end, buf_len, buf),
//           },
//         ))
//       }
//       WireType::Byte => consume_fixed!(1, identifier_len),
//       WireType::Fixed16 => consume_fixed!(2, identifier_len),
//       WireType::Fixed32 => consume_fixed!(4, identifier_len),
//       WireType::Fixed64 => consume_fixed!(8, identifier_len),
//       WireType::Fixed128 => consume_fixed!(16, identifier_len),
//       WireType::Zst => consume_fixed!(0, identifier_len),
//     }
//   }

//   /// Returns the length of the encoded data.
//   #[inline]
//   pub fn encoded_len(&self) -> usize
//   where
//     B: AsRef<[u8]>,
//   {
//     self.data.as_ref().len()
//   }

//   /// Converts the unknown data type to an borrowed type.
//   pub fn borrow(&self) -> Unknown<&B, F> {
//     Unknown {
//       tag: self.tag,
//       wire_type: self.wire_type,
//       data_offset: self.data_offset,
//       data: &self.data,
//     }
//   }
// }

// impl<B, F> Unknown<B, F>
// where
//   B: super::BytesBuffer,
//   F: Flavor,
// {
//   /// Converts the `Unknown<B>` to `Unknown<N>`.
//   pub fn map<'a, N>(&'a self) -> Unknown<N, F>
//   where
//     N: From<&'a [u8]>,
//   {
//     Unknown {
//       tag: self.tag,
//       wire_type: self.wire_type,
//       data_offset: self.data_offset,
//       data: N::from(self.raw()),
//     }
//   }

//   /// Converts the `Unknown<B>` to `Unknown<N>`.
//   pub fn consume_map<N>(self) -> Unknown<N, F>
//   where
//     N: From<B>,
//   {
//     Unknown {
//       tag: self.tag,
//       wire_type: self.wire_type,
//       data_offset: self.data_offset,
//       data: N::from(self.data),
//     }
//   }
// }
