use core::str::FromStr;

use heapless_0_9::{LenType, String};

use crate::{
  IntoTarget, Message, PartialMessage, TypeOwned, TypeRef,
  decode::{Decode, DecodeOwned},
  encode::{Encode, PartialEncode},
  flavors::{
    Groto,
    groto::{Context, DecodeError, EncodeError, Unknown, WireType},
  },
};

const CUSTOM_ERR: &str = "insufficient capacity";

impl<const N: usize, L: LenType> Encode<Groto> for String<N, L> {
  fn encode(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    Ok(match wire_type {
      WireType::Zst if N == 0 => 0,
      WireType::LengthDelimited => <str>::encode(self.as_str(), context, wire_type, buf)?,
      wt => {
        return Err(EncodeError::unsupported_wire_type(
          core::any::type_name::<Self>(),
          wt,
        ));
      }
    })
  }

  fn encoded_len(&self, context: &Context, wire_type: WireType) -> Result<usize, EncodeError> {
    Ok(match wire_type {
      WireType::Zst if N == 0 => 0,
      WireType::LengthDelimited => <str>::encoded_len(self.as_str(), context, wire_type)?,
      wt => {
        return Err(EncodeError::unsupported_wire_type(
          core::any::type_name::<Self>(),
          wt,
        ));
      }
    })
  }

  fn encoded_length_delimited_len(
    &self,
    context: &Context,
    wire_type: WireType,
  ) -> Result<usize, EncodeError> {
    Ok(match wire_type {
      WireType::Zst if N == 0 => 0,
      WireType::LengthDelimited => {
        <str>::encoded_length_delimited_len(self.as_str(), context, wire_type)?
      }
      wt => {
        return Err(EncodeError::unsupported_wire_type(
          core::any::type_name::<Self>(),
          wt,
        ));
      }
    })
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    Ok(match wire_type {
      WireType::Zst if N == 0 => 0,
      WireType::LengthDelimited => {
        <str>::encode_length_delimited(self.as_str(), context, wire_type, buf)?
      }
      wt => {
        return Err(EncodeError::unsupported_wire_type(
          core::any::type_name::<Self>(),
          wt,
        ));
      }
    })
  }
}

impl<const N: usize, L: LenType> PartialEncode<Groto> for String<N, L> {
  type Selection = ();

  fn partial_encode<WB>(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
    _: &Self::Selector,
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Groto>>::encode(self, context, wire_type, buf)
  }

  fn partial_encoded_len(
    &self,
    context: &Context,
    wire_type: WireType,
    _: &Self::Selector,
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Groto>>::encoded_len(self, context, wire_type)
  }

  fn partial_encoded_length_delimited_len(
    &self,
    context: &Context,
    wire_type: WireType,
    _: &Self::Selector,
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Groto>>::encoded_length_delimited_len(self, context, wire_type)
  }

  fn partial_encode_length_delimited(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
    _: &Self::Selector,
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Groto>>::encode_length_delimited(self, context, wire_type, buf)
  }
}

impl<'de, const N: usize, L: LenType> Decode<'de, Groto, Self> for String<N, L> {
  fn decode<UB>(
    context: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<B>> + 'de,
  {
    match wire_type {
      WireType::Zst if N == 0 => Ok((0, Self::new())),
      WireType::LengthDelimited => {
        let (len, slice) = <str>::decode::<UB>(context, wire_type, src)?;
        let vec = String::from_str(slice).map_err(|_| DecodeError::custom(CUSTOM_ERR))?;
        Ok((len, vec))
      }
      wt => Err(DecodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wt,
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
    UB: crate::buffer::Buffer<Unknown<B>> + 'de,
  {
    match wire_type {
      WireType::Zst if N == 0 => Ok((0, Self::new())),
      WireType::LengthDelimited => {
        let (len, slice) = <str>::decode_length_delimited::<UB>(context, wire_type, src)?;
        let vec = String::from_str(slice).map_err(|_| DecodeError::custom(CUSTOM_ERR))?;
        Ok((len, vec))
      }
      wt => Err(DecodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wt,
      )),
    }
  }
}

impl<const N: usize, L: LenType + 'static> DecodeOwned<Groto, Self> for String<N, L> {
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
    <Self as Decode<Groto, Self>>::decode::<()>(context, wire_type, src.remaining_slice())
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
    <Self as Decode<Groto, Self>>::decode_length_delimited::<()>(
      context,
      wire_type,
      src.remaining_slice(),
    )
  }
}

impl<const N: usize, L: LenType> IntoTarget<Groto, Self> for String<N, L> {
  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

impl<const N: usize, L: LenType> TypeOwned<Groto, Self> for String<N, L> {
  fn to(&self) -> Result<Self, DecodeError> {
    Ok(self.clone())
  }
}

impl<const N: usize, L: LenType> IntoTarget<Groto, String<N, L>> for &str {
  fn into_target(self) -> Result<String<N, L>, DecodeError> {
    String::from_str(self).map_err(|_| DecodeError::custom(CUSTOM_ERR))
  }
}

impl<const N: usize, L: LenType> TypeRef<Groto, String<N, L>> for &str {
  fn to(&self) -> Result<String<N, L>, DecodeError> {
    String::from_str(self).map_err(|_| DecodeError::custom(CUSTOM_ERR))
  }
}

impl<const N: usize, L: LenType> PartialMessage<Groto> for String<N, L> {
  type UnknownBuffer<B> = ();

  type PartialRef<'a>
    = &'a str
  where
    Self: Sized + 'a;

  type Borrowed<'a>
    = &'a Self
  where
    Self: 'a;

  type EncodedOwned
    = Self
  where
    Self: Sized + 'static;
}

impl<const N: usize, L: LenType> Message<Groto> for String<N, L> {
  type Partial = Self;

  type PartialRef<'a>
    = &'a str
  where
    Self: Sized + 'a;

  type Borrowed<'a>
    = &'a Self
  where
    Self: 'a;

  type EncodedOwned
    = Self
  where
    Self: Sized + 'static;
}
