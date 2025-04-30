use num_complex_0_4::Complex;

use crate::{
  decode::{Decode, DecodeOwned}, encode::Encode, flavors::{
    network::{Context, DecodeError, EncodeError, Identifier, Unknown, WireType}, Network
  }, message, partial_encode_primitives, Tag
};

const RE_TAG: Tag = Tag::new(1);
const IM_TAG: Tag = Tag::new(2);

const fn re_identifier(wt: WireType) -> Identifier {
  Identifier::new(wt, RE_TAG)
}

const fn im_identifier(wt: WireType) -> Identifier {
  Identifier::new(wt, IM_TAG)
}

macro_rules! codec {
  ($($wt:ident:$ty:ty), +$(,)?) => {
    $(
      impl Encode<Network> for Complex<$ty> {
        fn encode(
          &self,
          context: &Context,
          wire_type: WireType,
          buf: &mut [u8],
        ) -> Result<usize, EncodeError> {
          match wire_type {
            WireType::Varint => <Self as varing::Varint>::encode(self, buf).map_err(Into::into),
            WireType::LengthDelimited => {
              let mut offset = re_identifier(wire_type).encode_to(buf)?;
              offset += self
                .re
                .encode_length_delimited(context, WireType::$wt, buf)?;
              offset += im_identifier(wire_type).encode_to(buf)?;
              offset += self
                .im
              .encode_length_delimited(context, WireType::$wt, buf)?;
              Ok(offset)
            }
            _ => Err(EncodeError::unsupported_wire_type(
              core::any::type_name::<Self>(),
              wire_type,
            )),
          }
        }
      
        fn encoded_len(&self, context: &Context, wire_type: WireType) -> Result<usize, EncodeError> {
          match wire_type {
            WireType::Varint => Ok(<Self as varing::Varint>::encoded_len(self)),
            WireType::LengthDelimited => {
            let mut offset = re_identifier(wire_type).encoded_len();
            offset += self
              .re
              .encoded_length_delimited_len(context, WireType::$wt)?;
            offset += im_identifier(wire_type).encoded_len();
            offset += self
              .im
              .encoded_length_delimited_len(context, WireType::$wt)?;
            Ok(offset)
            }
            _ => Err(EncodeError::unsupported_wire_type(
              core::any::type_name::<Self>(),
              wire_type,
            )),
          }
        }
      
        fn encoded_length_delimited_len(
          &self,
          context: &Context,
          wire_type: WireType,
        ) -> Result<usize, EncodeError> {
          match wire_type {
            WireType::Varint => Ok(<Self as varing::Varint>::encoded_len(self)),
            WireType::LengthDelimited => {
              let encoded_len = self.encoded_len(context, wire_type)?;
              let length_delimited_len = varing::encoded_u32_varint_len(encoded_len as u32);
              Ok(length_delimited_len + encoded_len)
            }
            _ => Err(EncodeError::unsupported_wire_type(
              core::any::type_name::<Self>(),
              wire_type,
            )),
          }
        }
      
        fn encode_length_delimited(
          &self,
          context: &Context,
          wire_type: WireType,
          buf: &mut [u8],
        ) -> Result<usize, EncodeError> {
          match wire_type {
            WireType::Varint => <Self as varing::Varint>::encode(self, buf).map_err(Into::into),
            WireType::LengthDelimited => {
              let encoded_len = self.encoded_len(context, wire_type)?;
              let mut offset = varing::encode_u32_varint_to(encoded_len as u32, buf)?;
              offset += self.encode(context, wire_type, &mut buf[offset..])?;
              Ok(offset)
            }
            _ => Err(EncodeError::unsupported_wire_type(
              core::any::type_name::<Self>(),
              wire_type,
            )),
          }
        }
      }
      
      partial_encode_primitives!(Network: Complex<$ty>);
      
      impl<'de> Decode<'de, Network, Self> for Complex<$ty> {
        fn decode<UB>(
          context: &Context,
          wire_type: WireType,
          src: &'de [u8],
        ) -> Result<(usize, Self), DecodeError>
        where
          Self: Sized + 'de,
          UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
        {
          match wire_type {
            WireType::Varint => <Self as varing::Varint>::decode(src).map_err(Into::into),
            WireType::LengthDelimited => {
              const RE_IDENTIFIER: Identifier = Identifier::new(WireType::$wt, RE_TAG);
              const IM_IDENTIFIER: Identifier = Identifier::new(WireType::$wt, IM_TAG);

              let mut offset = 0;
              let buf_len = src.len();
              let mut re = None;
              let mut im = None;

              while offset < buf_len {
                let (len, identifier) = Identifier::decode(&src[offset..])?;
                offset += len;
        
                match identifier {
                  RE_IDENTIFIER => {
                    let (len, value) =
                      <$ty>::decode_length_delimited::<()>(context, WireType::$wt, &src[offset..])?;
                    offset += len;
                    re = Some(value);
                  }
                  IM_IDENTIFIER => {
                    let (len, value) =
                      <$ty>::decode_length_delimited::<()>(context, WireType::$wt, &src[offset..])?;
                    offset += len;
                    im = Some(value);
                  }
                  id => {
                    return Err(DecodeError::unknown_identifier(
                      core::any::type_name::<Self>(),
                      id.wire_type(),
                      id.tag(),
                    ));
                  }
                }
              }
        
              let name = core::any::type_name::<Self>();
              Ok((
                offset,
                Complex::new(
                  re.ok_or(DecodeError::field_not_found(
                    name,
                    "re",
                    RE_IDENTIFIER.wire_type(),
                    RE_TAG,
                  ))?,
                  im.ok_or(DecodeError::field_not_found(
                    name,
                    "im",
                    IM_IDENTIFIER.wire_type(),
                    IM_TAG,
                  ))?,
                ),
              ))
            }
            _ => Err(DecodeError::unsupported_wire_type(
              core::any::type_name::<Self>(),
              wire_type,
            )),
          }
        }
      
        fn decode_length_delimited<UB>(
          context: &Context,
          wire_type: WireType,
          src: &'de [u8],
        ) -> Result<(usize, Self), DecodeError>
        where
          Self: Sized + 'de,
          UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
        {
          match wire_type {
            WireType::Varint => <Self as varing::Varint>::decode(src).map_err(Into::into),
            WireType::LengthDelimited => {
              let (len, data_len) = varing::decode_u32_varint(src)?;
              let end = len + data_len as usize;
              if end > src.len() {
                return Err(DecodeError::buffer_underflow());
              }
              let (read, val) = Self::decode::<UB>(context, wire_type, &src[len..end])?;
              #[cfg(debug_assertions)]
              crate::debug_assert_read_eq::<Self>(read, data_len as usize);
          
              Ok((end, val))
            }
            _ => Err(DecodeError::unsupported_wire_type(
              core::any::type_name::<Self>(),
              wire_type,
            )),
          }
        }
      }
      
      impl DecodeOwned<Network, Self> for Complex<$ty> {
        fn decode_owned<B, UB>(
          context: &Context,
          wire_type: WireType,
          src: B,
        ) -> Result<(usize, Self), DecodeError>
        where
          Self: Sized + 'static,
          B: crate::buffer::BytesBuffer + 'static,
          UB: crate::buffer::Buffer<Unknown<B>> + 'static,
        {
          Self::decode::<()>(context, wire_type, src.as_bytes())
        }
      
        fn decode_length_delimited_owned<B, UB>(
          context: &Context,
          wire_type: WireType,
          src: B,
        ) -> Result<(usize, Self), DecodeError>
        where
          Self: Sized + 'static,
          B: crate::buffer::BytesBuffer + 'static,
          UB: crate::buffer::Buffer<Unknown<B>> + 'static,
        {
          Self::decode_length_delimited::<()>(context, wire_type, src.as_bytes())
        }
      }
      
      message!(Network: Complex<$ty>);      
    )*
  };
}

codec!(
  // Fixed32:f32,
  // Fixed64:f64,
  Byte:u8,
  Varint:u16,
  Varint:u32,
  Varint:u64,
  Varint:u128,
  Byte:i8,
  Varint:i16,
  Varint:i32,
  Varint:i64,
  Varint:i128,
);

// #[cfg(feature = "half_2")]
// codec!(Fixed16:half_2::f16);