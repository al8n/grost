use syn::{
  Ident, Token, Type,
  parse::{Parse, ParseStream},
};

use super::parse_type;

/// The value of a generic label, which can either be a marker type or a wire format type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericLabelValueVariant {
  /// A marker type, usually a generic parameter
  Marker {
    /// The marker type
    marker: Type,
    /// The type to be marked
    marked: Option<Type>,
  },
  /// A wire format type, which is used to indicate how the field should be encode/decode.
  As {
    /// The wire format type
    wire_format: Type,
    /// The type to be marked
    marked: Option<Type>,
  },
}

impl GenericLabelValueVariant {
  /// Returns the marker type, if it exists.
  pub fn marker(&self) -> Option<&Type> {
    match self {
      Self::Marker { marker, .. } => Some(marker),
      _ => None,
    }
  }

  /// Returns the wire format type, if it exists.
  pub fn wire_format(&self) -> Option<&Type> {
    match self {
      Self::As { wire_format, .. } => Some(wire_format),
      _ => None,
    }
  }

  /// Returns the marked type, if it exists.
  pub fn marked(&self) -> Option<&Type> {
    match self {
      Self::Marker { marked, .. } | Self::As { marked, .. } => marked.as_ref(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum GenericLabelValueParser {
  Marker(Type),
  As(Type),
  Type(Type),
}

impl Parse for GenericLabelValueParser {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.peek(Token![as]) {
      let _: Token![as] = input.parse()?;
      if !input.peek(Token![=]) {
        return Err(syn::Error::new(input.span(), "Expected `=` after `as`"));
      }
      let _: Token![=] = input.parse()?;
      let ty: Type = parse_type(input)?;
      return Ok(Self::As(ty));
    }

    if input.peek(Token![type]) {
      let _: Token![type] = input.parse()?;
      if !input.peek(Token![=]) {
        return Err(syn::Error::new(input.span(), "Expected `=` after `type`"));
      }
      let _: Token![=] = input.parse()?;
      let ty: Type = parse_type(input)?;
      return Ok(Self::Type(ty));
    }

    if !input.peek(Ident) {
      return Err(syn::Error::new(
        input.span(),
        "Expected a `marker = \"...\"` or `as = \"...\"`",
      ));
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

#[derive(Debug, Clone)]
pub struct GenericLabelValue {
  pub(super) value: GenericLabelValueVariant,
  pub(super) span: proc_macro2::Span,
}

impl GenericLabelValue {
  /// Returns the span of the label value.
  pub fn span(&self) -> proc_macro2::Span {
    self.span
  }

  /// Returns the marker type, if it exists.
  pub fn marker(&self) -> Option<&Type> {
    match &self.value {
      GenericLabelValueVariant::Marker { marker, .. } => Some(marker),
      _ => None,
    }
  }

  /// Returns the wire format type, if it exists.
  pub fn wire_format(&self) -> Option<&Type> {
    match &self.value {
      GenericLabelValueVariant::As { wire_format, .. } => Some(wire_format),
      _ => None,
    }
  }

  /// Returns the marked type, if it exists.
  pub fn marked(&self) -> Option<&Type> {
    match &self.value {
      GenericLabelValueVariant::Marker { marked, .. } => marked.as_ref(),
      GenericLabelValueVariant::As { marked, .. } => marked.as_ref(),
    }
  }
}

impl Parse for GenericLabelValue {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut as_type = None;
    let mut marker = None;
    let mut ty = None;
    for value in input.parse_terminated(GenericLabelValueParser::parse, Token![,])? {
      match value {
        GenericLabelValueParser::Marker(ty) => {
          if marker.is_some() {
            return Err(syn::Error::new(
              input.span(),
              "Multiple `marker` values found",
            ));
          }
          marker = Some(ty);
        }
        GenericLabelValueParser::As(ty) => {
          if as_type.is_some() {
            return Err(syn::Error::new(input.span(), "Multiple `as` values found"));
          }
          as_type = Some(ty);
        }
        GenericLabelValueParser::Type(typ) => {
          if ty.is_some() {
            return Err(syn::Error::new(
              input.span(),
              "Multiple `type` values found",
            ));
          }
          ty = Some(typ);
        }
      }
    }

    match (marker, as_type) {
      (Some(marker), None) => Ok(Self {
        value: GenericLabelValueVariant::Marker { marker, marked: ty },
        span: input.span(),
      }),
      (None, Some(as_type)) => Ok(Self {
        value: GenericLabelValueVariant::As {
          wire_format: as_type,
          marked: ty,
        },
        span: input.span(),
      }),
      (Some(_), Some(_)) => Err(syn::Error::new(
        input.span(),
        "Cannot specify both `marker` and `as` at the same time",
      )),
      (None, None) => Err(syn::Error::new(
        input.span(),
        "Expected at least one of `marker = \"...\"` or `as = \"...\"`",
      )),
    }
  }
}
