use num_rational_0_4::Ratio;

use crate::{
  decode::{Decode, DecodeOwned}, encode::Encode, flavors::{
    network::{Context, DecodeError, EncodeError, Identifier, Unknown, WireType}, Network
  }, message, partial_encode_scalar, Tag
};

const NUMER_TAG: Tag = Tag::new(1);
const DENOM_TAG: Tag = Tag::new(2);

const fn re_identifier(wt: WireType) -> Identifier {
  Identifier::new(wt, NUMER_TAG)
}

const fn im_identifier(wt: WireType) -> Identifier {
  Identifier::new(wt, DENOM_TAG)
}

macro_rules! codec {
  ($($wt:ident:$ty:ty), +$(,)?) => {
    $(
      impl Encode<Network> for Ratio<$ty> {
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
                .numer()
                .encode_length_delimited(context, WireType::$wt, buf)?;
              offset += im_identifier(wire_type).encode_to(buf)?;
              offset += self
                .denom()
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
                .numer()
                .encoded_length_delimited_len(context, WireType::$wt)?;
              offset += im_identifier(wire_type).encoded_len();
              offset += self
                .denom()
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
      
      partial_encode_scalar!(Network: Ratio<$ty>);
      
      impl<'de> Decode<'de, Network, Self> for Ratio<$ty> {
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
              const NUMER_IDENTIFIER: Identifier = Identifier::new(WireType::$wt, NUMER_TAG);
              const DENOM_IDENTIFIER: Identifier = Identifier::new(WireType::$wt, DENOM_TAG);

              let mut offset = 0;
              let buf_len = src.len();
              let mut numer = None;
              let mut denom = None;

              while offset < buf_len {
                let (len, identifier) = Identifier::decode(&src[offset..])?;
                offset += len;
        
                match identifier {
                  NUMER_IDENTIFIER => {
                    let (len, value) =
                      <$ty>::decode_length_delimited::<()>(context, WireType::$wt, &src[offset..])?;
                    offset += len;
                    numer = Some(value);
                  }
                  DENOM_IDENTIFIER => {
                    let (len, value) =
                      <$ty>::decode_length_delimited::<()>(context, WireType::$wt, &src[offset..])?;
                    offset += len;
                    denom = Some(value);
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
                Ratio::new_raw(
                  numer.ok_or(DecodeError::field_not_found(
                    name,
                    "numer",
                    NUMER_IDENTIFIER.wire_type(),
                    NUMER_TAG,
                  ))?,
                  denom.ok_or(DecodeError::field_not_found(
                    name,
                    "denom",
                    DENOM_IDENTIFIER.wire_type(),
                    DENOM_TAG,
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
      
      impl DecodeOwned<Network, Self> for Ratio<$ty> {
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
      
      message!(Network: Ratio<$ty>);      
    )*
  };
}

codec!(
  Byte:u8,
  Varint:u16,
  Varint:u32,
  Varint:u64,
  Byte:i8,
  Varint:i16,
  Varint:i32,
  Varint:i64,
);

#[cfg(any(feature = "ruint_1", feature = "bnum_0_13"))]
codec!(
  Varint:u128,
  Varint:i128,
);
