use either::Either;
use std::rc::Rc;
use syn::{
  Ident, Token, Type,
  parse::{Parse, ParseStream},
};

use super::{Label, parse_type};

#[derive(Debug, Clone)]
pub struct ListLikeLabel {
  pub(crate) label: Rc<Label>,
  pub(crate) repeated: bool,
}

impl ListLikeLabel {
  /// Returns the label of the list-like type.
  pub fn label(&self) -> &Label {
    &self.label
  }

  /// Returns whether the list-like type is marked as repeated.
  pub fn is_repeated(&self) -> bool {
    self.repeated
  }
}

pub(super) struct ListLikeLabelParser<const TAG: u8>(pub(super) Either<ListLikeLabel, Type>);

enum ListLikeLabelParserHelper<const TAG: u8> {
  Label(Label),
  Repeated,
  As(Type),
}

impl<const TAG: u8> Parse for ListLikeLabelParserHelper<TAG> {
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

impl<const TAG: u8> Parse for ListLikeLabelParser<TAG> {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut label = None;
    let mut repeated = None;
    let mut as_type = None;
    let items = input.parse_terminated(ListLikeLabelParserHelper::<TAG>::parse, Token![,])?;

    for item in items {
      match item {
        ListLikeLabelParserHelper::Label(l) => {
          if let Some(ref old) = label {
            return Err(syn::Error::new(
              input.span(),
              format!("Duplicate label found, expected only one label, found `{old}` and `{l}`"),
            ));
          }
          label = Some(l);
        }
        ListLikeLabelParserHelper::Repeated => {
          if repeated.is_some() {
            return Err(syn::Error::new(
              input.span(),
              "Duplicate `repeated` found, only one `repeated` is allowed",
            ));
          }
          repeated = Some(true);
        }
        ListLikeLabelParserHelper::As(ty) => {
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
    let repeated = repeated.unwrap_or(false);

    let key = if TAG == super::LIST_TAG {
      "list"
    } else {
      "set"
    };

    Ok(match (label, repeated, as_type) {
      (Some(label), true, None) => Self(Either::Left(ListLikeLabel {
        label: Rc::new(label),
        repeated: true,
      })),
      (Some(label), false, None) => Self(Either::Left(ListLikeLabel {
        label: Rc::new(label),
        repeated: false,
      })),
      (None, false, Some(ty)) => Self(Either::Right(ty)),
      _ => {
        return Err(syn::Error::new(
          input.span(),
          format!("Expected `{key}(...)`, or `{key}(..., repeated)` or `{key}(as = \"...\")`",),
        ));
      }
    })
  }
}
