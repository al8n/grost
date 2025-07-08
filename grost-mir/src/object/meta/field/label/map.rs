use std::sync::Arc;
use either::Either;
use syn::{parse::{Parse, ParseStream}, token::Paren, Ident, Token, Type};

use super::{Label, parse_type};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapLabel {
  pub(crate) key: Arc<Label>,
  pub(crate) value: Arc<Label>,
  pub(crate) repeated: bool,
}

impl MapLabel {
  /// Returns the key label of the map.
  pub fn key(&self) -> &Label {
    &self.key
  }

  /// Returns the value label of the map.
  pub fn value(&self) -> &Label {
    &self.value
  }

  /// Returns whether the map is marked as repeated.
  pub const fn is_repeated(&self) -> bool {
    self.repeated
  }
}


enum MapLabelParserHelper {
  Key(Label),
  Value(Label),
  Repeated,
  As(Type),
}

impl Parse for MapLabelParserHelper {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.peek(Token![as]) {
      let _: Token![as] = input.parse()?;
      if !input.peek(Token![=]) {
        return Err(syn::Error::new(input.span(), "Expected `as = \"...\"`"));
      }
      let _: Token![=] = input.parse()?;
      let ty = parse_type(input)?;
      return Ok(Self::As(ty));
    }

    if !input.peek(Ident) {
      return Err(syn::Error::new(
        input.span(),
        "Expected one of [key, value, repeated, as]",
      ));
    }

    let ident: Ident = input.parse()?;
    match () {
      () if ident.eq("key") || ident.eq("value") => {
        if !input.peek(Paren) {
          return Err(syn::Error::new(
            ident.span(),
            format!("Expected `(...)` after `{ident}`"),
          ));
        }

        let content;
        syn::parenthesized!(content in input);
        if content.is_empty() {
          return Err(syn::Error::new(
            ident.span(),
            format!("Expected `{ident}(...)`, `{ident}(..., repeated)` or `{ident}(as = \"...\")`"),
          ));
        }

        let label = Label::parse(&content)?;
        if ident.eq("key") {
          Ok(Self::Key(label))
        } else {
          Ok(Self::Value(label))
        }
      }
      () if ident.eq("repeated") => Ok(Self::Repeated),
      _ => Err(syn::Error::new(
        ident.span(),
        format!(
          "Expected one of [scalar, bytes, string, generic, object, enum, union, interface, map, set, list, nullable, repeated, as], found `{ident}`"
        ),
      )),
    }
  }
}

pub(super) struct MapLabelParser(pub(super) Either<MapLabel, Type>);

impl Parse for MapLabelParser {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut key = None;
    let mut value = None;
    let mut repeated = None;
    let mut as_type = None;
    let items = input.parse_terminated(MapLabelParserHelper::parse, Token![,])?;

    for item in items {
      match item {
        MapLabelParserHelper::Key(l) => {
          if let Some(ref old) = key {
            return Err(syn::Error::new(
              input.span(),
              format!(
                "Duplicate key found, expected only one key, found `key({old})` and `key({l})`"
              ),
            ));
          }
          key = Some(l);
        }
        MapLabelParserHelper::Value(l) => {
          if let Some(ref old) = value {
            return Err(syn::Error::new(
              input.span(),
              format!(
                "Duplicate value found, expected only one value, found `value({old})` and `value({l})`"
              ),
            ));
          }
          value = Some(l);
        }
        MapLabelParserHelper::Repeated => {
          if repeated.is_some() {
            return Err(syn::Error::new(
              input.span(),
              "Duplicate `repeated` found, only one `repeated` is allowed",
            ));
          }
          repeated = Some(true);
        }
        MapLabelParserHelper::As(ty) => {
          if as_type.is_some() {
            return Err(syn::Error::new(
              input.span(),
              "Duplicate `as` found, expected only one `as`",
            ));
          }
          as_type = Some(ty);
        }
      }
    }
    let repeated = repeated.unwrap_or(false);

    Ok(match (key, value, repeated, as_type) {
      (Some(key), Some(value), true, None) => Self(Either::Left(MapLabel {
        key: Arc::new(key),
        value: Arc::new(value),
        repeated: true,
      })),
      (Some(key), Some(value), false, None) => Self(Either::Left(MapLabel {
        key: Arc::new(key),
        value: Arc::new(value),
        repeated: false,
      })),
      (None, None, false, Some(ty)) => Self(Either::Right(ty)),
      _ => {
        return Err(syn::Error::new(
          input.span(),
          "Expected `map(key(...), value(...))`, or `map(key(...), value(...), repeated)` or `map(as = \"...\")`",
        ));
      }
    })
  }
}
