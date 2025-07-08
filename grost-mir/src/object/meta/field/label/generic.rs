use syn::{parse::{Parse, ParseStream}, Ident, Token, Type};

use super::parse_type;

/// The value of a generic label, which can either be a marker type or a wire format type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericLabelValue {
  /// A marker type, usually a generic parameter
  Marker(Type),
  /// A wire format type, which is used to indicate how the field should be encode/decode.
  As(Type),
}

impl Parse for GenericLabelValue {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.peek(Token![as]) {
      let _: Token![as] = input.parse()?;
      if !input.peek(Token![=]) {
        return Err(syn::Error::new(input.span(), "Expected `=` after `as`"));
      }
      let _: Token![=] = input.parse()?;
      let ty: Type = input.parse()?;
      return Ok(Self::As(ty));
    }

    if !input.peek(Ident) {
      return Err(syn::Error::new(input.span(), "Expected a `marker = \"...\"` or `as = \"...\"`"));
    }


    let ident: Ident = input.parse()?;
    if ident != "marker" {
      return Err(syn::Error::new(input.span(), "Expected `marker` or `as`"));
    }
    if !input.peek(Token![=]) {
      return Err(syn::Error::new(input.span(), "Expected `=` after `marker`"));
    }
    let _: Token![=] = input.parse()?;

    Ok(Self::Marker(parse_type(input)?))
  }
}

pub(super) struct GenericLabelParser(pub(super) GenericLabelValue);

impl Parse for GenericLabelParser {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut as_type = None;
    let mut marker = None;
    for value in input.parse_terminated(GenericLabelValue::parse, Token![,])? {
      match value {
        GenericLabelValue::Marker(ty) => {
          if marker.is_some() {
            return Err(syn::Error::new(input.span(), "Multiple `marker` values found"));
          }
          marker = Some(ty);
        },
        GenericLabelValue::As(ty) => {
          if as_type.is_some() {
            return Err(syn::Error::new(input.span(), "Multiple `as` values found"));
          }
          as_type = Some(ty);
        },
      }
    }

    match (marker, as_type) {
      (Some(marker), None) => Ok(Self(GenericLabelValue::Marker(marker))),
      (None, Some(as_type)) => Ok(Self(GenericLabelValue::As(as_type))),
      (Some(_), Some(_)) => Err(syn::Error::new(input.span(), "Cannot specify both `marker` and `as` at the same time")),
      (None, None) => Err(syn::Error::new(input.span(), "Expected at least one of `marker = \"...\"` or `as = \"...\"`")),
    }
  }
}