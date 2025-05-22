use heapless_0_9::{LenType, Vec};

use crate::{
  IntoTarget, Message, PartialMessage, TypeOwned, TypeRef,
  decode::{Decode, DecodeOwned},
  encode::{Encode, PartialEncode},
  flavors::{
    Network,
    network::{Context, DecodeError, EncodeError, Unknown, WireType},
  },
};

const CUSTOM_ERR: &str = "insufficient capacity";

impl<const N: usize, L: LenType> Encode<Network> for Vec<u8, N, L> {
  fn encode(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    Ok(match wire_type {
      WireType::Zst if N == 0 => 0,
      WireType::LengthDelimited => <[u8]>::encode(self.as_slice(), context, wire_type, buf)?,
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
      WireType::LengthDelimited => <[u8]>::encoded_len(self.as_slice(), context, wire_type)?,
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
        <[u8]>::encoded_length_delimited_len(self.as_slice(), context, wire_type)?
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
        <[u8]>::encode_length_delimited(self.as_slice(), context, wire_type, buf)?
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

impl<const N: usize, L: LenType> PartialEncode<Network> for Vec<u8, N, L> {
  type Selection = ();

  fn partial_encode(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
    _: &Self::Selector,
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Network>>::encode(self, context, wire_type, buf)
  }

  fn partial_encoded_len(
    &self,
    context: &Context,
    wire_type: WireType,
    _: &Self::Selector,
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Network>>::encoded_len(self, context, wire_type)
  }

  fn partial_encoded_length_delimited_len(
    &self,
    context: &Context,
    wire_type: WireType,
    _: &Self::Selector,
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Network>>::encoded_length_delimited_len(self, context, wire_type)
  }

  fn partial_encode_length_delimited(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
    _: &Self::Selector,
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Network>>::encode_length_delimited(self, context, wire_type, buf)
  }
}

impl<'de, const N: usize, L: LenType> Decode<'de, Network, Self> for Vec<u8, N, L> {
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
      WireType::Zst if N == 0 => Ok((0, Self::new())),
      WireType::LengthDelimited => {
        let (len, slice) = <[u8]>::decode::<UB>(context, wire_type, src)?;
        let vec = Vec::from_slice(slice).map_err(|_| DecodeError::custom(CUSTOM_ERR))?;
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
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    match wire_type {
      WireType::Zst if N == 0 => Ok((0, Self::new())),
      WireType::LengthDelimited => {
        let (len, slice) = <[u8]>::decode_length_delimited::<UB>(context, wire_type, src)?;
        let vec = Vec::from_slice(slice).map_err(|_| DecodeError::custom(CUSTOM_ERR))?;
        Ok((len, vec))
      }
      wt => Err(DecodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wt,
      )),
    }
  }
}

impl<const N: usize, L: LenType + 'static> DecodeOwned<Network, Self> for Vec<u8, N, L> {
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
    <Self as Decode<Network, Self>>::decode::<()>(context, wire_type, src.as_bytes())
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
    <Self as Decode<Network, Self>>::decode_length_delimited::<()>(
      context,
      wire_type,
      src.as_bytes(),
    )
  }
}

impl<const N: usize, L: LenType> IntoTarget<Network, Self> for Vec<u8, N, L> {
  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

impl<const N: usize, L: LenType> TypeOwned<Network, Self> for Vec<u8, N, L> {
  fn to(&self) -> Result<Self, DecodeError> {
    Ok(self.clone())
  }
}

impl<const N: usize, L: LenType> IntoTarget<Network, Vec<u8, N, L>> for &[u8] {
  fn into_target(self) -> Result<Vec<u8, N, L>, DecodeError> {
    Vec::from_slice(self).map_err(|_| DecodeError::custom(CUSTOM_ERR))
  }
}

impl<const N: usize, L: LenType> TypeRef<Network, Vec<u8, N, L>> for &[u8] {
  fn to(&self) -> Result<Vec<u8, N, L>, DecodeError> {
    Vec::from_slice(self).map_err(|_| DecodeError::custom(CUSTOM_ERR))
  }
}

impl<const N: usize, L: LenType> PartialMessage<Network> for Vec<u8, N, L> {
  type UnknownBuffer<B> = ();

  type Decoded<'a>
    = &'a [u8]
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

impl<const N: usize, L: LenType> Message<Network> for Vec<u8, N, L> {
  type Partial = Self;

  type Decoded<'a>
    = &'a [u8]
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
