use either::Either;
use std::rc::Rc;
use syn::{
  Ident, Token, Type,
  parse::{Parse, ParseStream},
};

use super::{Label, parse_type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NullableLabel {
  pub(crate) label: Rc<Label>,
}

impl NullableLabel {
  /// Returns the label of the list-like type.
  pub fn label(&self) -> &Label {
    &self.label
  }
}

pub(super) struct NullableLabelParser(pub(super) Either<NullableLabel, Type>);

enum NullableLabelParserHelper {
  Label(Label),
  As(Type),
}

impl Parse for NullableLabelParserHelper {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.peek(Token![enum]) {
      return Ok(Self::Label(Label::parse(input)?));
    }

    if input.peek(Token![as]) {
      let _: Token![as] = input.parse()?;
      if !input.peek(Token![=]) {
        return Err(syn::Error::new(input.span(), "Expected `=` after `as`"));
      }
      let _: Token![=] = input.parse()?;
      let ty = parse_type(input)?;
      return Ok(Self::As(ty));
    }

    if !input.peek(Ident) {
      return Err(syn::Error::new(
        input.span(),
        "Expected one of [scalar, bytes, string, generic, object, enum, union, interface, map, set, list, nullable, repeated, as]",
      ));
    }

    let ident: Ident = input.fork().parse()?;
    match () {
      () if Label::possible_idents().iter().any(|label| ident.eq(label)) => {
        Label::parse(input).map(Self::Label)
      }
      _ => Err(syn::Error::new(
        ident.span(),
        format!(
          "Expected one of [scalar, bytes, string, generic, object, enum, union, interface, map, set, list, nullable, repeated, as], found `{ident}`"
        ),
      )),
    }
  }
}

impl Parse for NullableLabelParser {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut label = None;
    let mut as_type = None;
    let items = input.parse_terminated(NullableLabelParserHelper::parse, Token![,])?;

    for item in items {
      match item {
        NullableLabelParserHelper::Label(l) => {
          if let Some(ref old) = label {
            return Err(syn::Error::new(
              input.span(),
              format!("Duplicate label found, expected only one label, found `{old}` and `{l}`"),
            ));
          }
          label = Some(l);
        }
        NullableLabelParserHelper::As(ty) => {
          if as_type.is_some() {
            return Err(syn::Error::new(
              input.span(),
              "Duplicate `as` found, expected only one `as`".to_string(),
            ));
          }
          as_type = Some(ty);
        }
      }
    }

    Ok(match (label, as_type) {
      (Some(label), None) => Self(Either::Left(NullableLabel {
        label: Rc::new(label),
      })),
      (None, Some(ty)) => Self(Either::Right(ty)),
      _ => {
        return Err(syn::Error::new(
          input.span(),
          "Expected `nullable(...)` or `nullable(as = \"...\")`",
        ));
      }
    })
  }
}
