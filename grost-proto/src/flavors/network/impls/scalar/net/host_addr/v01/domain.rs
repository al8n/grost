use hostaddr_0_1::Domain;

use crate::{
  IntoTarget, PartialMessage, TypeOwned, TypeRef,
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    Network,
    network::{Context, DecodeError, EncodeError, Unknown, WireType},
  },
};

impl<S> Encode<Network> for Domain<S>
where
  S: Encode<Network>,
{
  fn encode(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    self.as_ref().into_inner().encode(context, wire_type, buf)
  }

  fn encoded_len(&self, context: &Context, wire_type: WireType) -> Result<usize, EncodeError> {
    self.as_ref().into_inner().encoded_len(context, wire_type)
  }

  fn encoded_length_delimited_len(
    &self,
    context: &Context,
    wire_type: WireType,
  ) -> Result<usize, EncodeError> {
    self
      .as_ref()
      .into_inner()
      .encoded_length_delimited_len(context, wire_type)
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    self
      .as_ref()
      .into_inner()
      .encode_length_delimited(context, wire_type, buf)
  }
}

impl<S> PartialEncode<Network> for Domain<S>
where
  S: PartialEncode<Network>,
{
  type Selection = S::Selection;

  fn partial_encode(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError> {
    self
      .as_ref()
      .into_inner()
      .partial_encode(context, wire_type, buf, selection)
  }

  fn partial_encoded_len(
    &self,
    context: &Context,
    wire_type: WireType,
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError> {
    self
      .as_ref()
      .into_inner()
      .partial_encoded_len(context, wire_type, selection)
  }

  fn partial_encoded_length_delimited_len(
    &self,
    context: &Context,
    wire_type: WireType,
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError> {
    self
      .as_ref()
      .into_inner()
      .partial_encoded_length_delimited_len(context, wire_type, selection)
  }

  fn partial_encode_length_delimited(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError> {
    self
      .as_ref()
      .into_inner()
      .partial_encode_length_delimited(context, wire_type, buf, selection)
  }
}
