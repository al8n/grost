// use crate::{buffer::Buffer, decode::{Decode, DecodeOwned}, encode::Encode, flavors::network::{Context, DecodeError, EncodeError, Network, Unknown, WireType}, message, partial_encode_primitives};

// use bnum_0_13::*;

// impl<const N: usize> Encode<Network> for BUintD8<N> {
//   fn encode(&self, _: &Context, wire_type: WireType, buf: &mut [u8]) -> Result<usize, EncodeError> {
//     match wire_type {
//       WireType::Varint => varing::bnum::encode_uint_d8_to(*self, buf).map_err(Into::into),
//       WireType::Byte if N == 1 => {
//         if buf.is_empty() {
//           return Err(EncodeError::insufficient_buffer(1, buf.len()));
//         }

//         buf[0] = self.digits()[0];
//         Ok(1)
//       },
//       WireType::Fixed16 if N == 1 => {
//         if buf.len() < 2 {
//           return Err(EncodeError::insufficient_buffer(2, buf.len()));
//         }

//         buf[..2].copy_from_slice(&[self.digits()[0], 0]);
//         Ok(2)
//       },
//       WireType::Fixed32 => {
//         if buf.len() < 4 {
//           return Err(EncodeError::insufficient_buffer(4, buf.len()));
//         }

//         buf[..4].copy_from_slice(&[*self, 0, 0, 0]);
//         Ok(4)
//       },
//       WireType::Fixed64 => {
//         if buf.len() < 8 {
//           return Err(EncodeError::insufficient_buffer(8, buf.len()));
//         }

//         buf[..8].copy_from_slice(&[*self, 0, 0, 0, 0, 0, 0, 0, 0]);
//         Ok(8)
//       },
//       WireType::Fixed128 => {
//         if buf.len() < 16 {
//           return Err(EncodeError::insufficient_buffer(16, buf.len()));
//         }

//         let mut tmp = [0; 16];
//         tmp[0] = *self;

//         buf[..16].copy_from_slice(&tmp);
//         Ok(16)
//       },
//       val => Err(EncodeError::unsupported_wire_type(core::any::type_name::<Self>(), val)),
//     }
//   }

//   fn encoded_len(&self, _: &Context, wire_type: WireType) -> Result<usize, EncodeError> {
//     Ok(match wire_type {
//       WireType::Varint => varing::bnum::encode(*self),
//       WireType::Byte => 1,
//       WireType::Fixed16 => 2,
//       WireType::Fixed32 => 4,
//       WireType::Fixed64 => 8,
//       WireType::Fixed128 => 16,
//       val => return Err(EncodeError::unsupported_wire_type(core::any::type_name::<Self>(), val)),
//     })
//   }

//   fn encoded_length_delimited_len(&self, context: &Context, wire_type: WireType) -> Result<usize, EncodeError> {
//     self.encoded_len(context, wire_type)
//   }

//   fn encode_length_delimited(&self, context: &Context, wire_type: WireType, buf: &mut [u8]) -> Result<usize, EncodeError> {
//     self.encode(context, wire_type, buf)
//   }
// }

// partial_encode_primitives!(Network: u8);

// impl<'de> Decode<'de, Network, Self> for u8 {
//   fn decode<UB>(_: &Context, wire_type: WireType, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
//   where
//     Self: Sized + 'de,
//     UB: Buffer<Unknown<&'de [u8]>> + 'de
//   {
//     decode_u8(wire_type, src)
//   }

//   fn decode_length_delimited<UB>(ctx: &Context, wire_type: WireType, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
//   where
//     Self: Sized + 'de,
//     UB: Buffer<Unknown<&'de [u8]>> + 'de
//   {
//     Self::decode::<UB>(ctx, wire_type, src)
//   }
// }

// impl DecodeOwned<Network, Self> for u8 {
//   fn decode_owned<B, UB>(_: &Context, wire_type: WireType, src: B) -> Result<(usize, Self), DecodeError>
//     where
//       Self: Sized + 'static,
//       B: crate::buffer::BytesBuffer + 'static,
//       UB: Buffer<Unknown<B>> + 'static {
//     decode_u8(wire_type, src.as_bytes())
//   }

//   fn decode_length_delimited_owned<B, UB>(context: &Context, wire_type: WireType, src: B) -> Result<(usize, Self), DecodeError>
//     where
//       Self: Sized + 'static,
//       B: crate::buffer::BytesBuffer + 'static,
//       UB: Buffer<Unknown<B>> + 'static {
//     Self::decode_owned::<B, UB>(context, wire_type, src)
//   }
// }

// fn decode_u8(wire_type: WireType, src: &[u8]) -> Result<(usize, u8), DecodeError> {
//   match wire_type {
//     WireType::Varint => varing::decode_u8_varint(src).map_err(Into::into),
//     WireType::Byte => {
//       if src.is_empty() {
//         return Err(DecodeError::buffer_underflow());
//       }

//       let value = src[0];
//       Ok((1, value))
//     },
//     WireType::Fixed16 => {
//       if src.len() < 2 {
//         return Err(DecodeError::buffer_underflow());
//       }

//       let mut tmp = [0; 2];
//       tmp.copy_from_slice(&src[..2]);
//       Ok((2, tmp[0]))
//     },
//     WireType::Fixed32 => {
//       if src.len() < 4 {
//         return Err(DecodeError::buffer_underflow());
//       }

//       let mut tmp = [0; 4];
//       tmp.copy_from_slice(&src[..4]);
//       Ok((4, tmp[0]))
//     },
//     WireType::Fixed64 => {
//       if src.len() < 8 {
//         return Err(DecodeError::buffer_underflow());
//       }

//       let mut tmp = [0; 8];
//       tmp.copy_from_slice(&src[..8]);
//       Ok((8, tmp[0]))
//     },
//     WireType::Fixed128 => {
//       if src.len() < 16 {
//         return Err(DecodeError::buffer_underflow());
//       }

//       let mut tmp = [0; 16];
//       tmp.copy_from_slice(&src[..16]);
//       Ok((16, tmp[0]))
//     },
//     _ => Err(DecodeError::unsupported_wire_type(core::any::type_name::<u8>(), wire_type)),
//   }
// }

// message!(Network: u8);

use crate::network_varint;
use bnum_0_13::*;

network_varint!(
  BUintD8<N> [const N: usize],
  BUintD16<N> [const N: usize],
  BUintD32<N> [const N: usize],
  BUint<N> [const N: usize],
  BIntD8<N> [const N: usize],
  BIntD16<N> [const N: usize],
  BIntD32<N> [const N: usize],
  BInt<N> [const N: usize],
);
