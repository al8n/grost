use std::sync::Arc;

use darling::{FromMeta, ast::NestedMeta, util::SpannedValue};
use syn::Meta;

/// The specification of the type of a field.
#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::IsVariant)]
pub enum Label {
  /// Specifies the field is optional
  Optional(Option<Arc<Label>>),
  /// Specifies the field is repeated, e.g. `Vec<T>`, `[T]`, etc.
  Repeated(Option<Arc<Label>>),
  /// Specifies the field is a map
  Map {
    key: Option<Arc<Label>>,
    value: Option<Arc<Label>>,
  },
}

impl Label {
  fn from_hint(type_hint: &FieldLabel) -> Option<Self> {
    match type_hint {
      FieldLabel::Required => None,
      FieldLabel::Optional(inner) => Some(Self::Optional(Self::from_hint(inner).map(Arc::new))),
      FieldLabel::Repeated(inner) => Some(Self::Repeated(Self::from_hint(inner).map(Arc::new))),
      FieldLabel::Map { key, value } => Some(Self::Map {
        key: key.as_ref().and_then(|k| Self::from_hint(k).map(Arc::new)),
        value: value
          .as_ref()
          .and_then(|v| Self::from_hint(v).map(Arc::new)),
      }),
    }
  }
}

#[derive(Debug, Clone)]
pub(super) enum FieldLabel {
  Required,
  Optional(Box<FieldLabel>),
  Repeated(Box<FieldLabel>),
  Map {
    key: Option<Box<FieldLabel>>,
    value: Option<Box<FieldLabel>>,
  },
}

impl FromMeta for FieldLabel {
  fn from_meta(meta: &Meta) -> darling::Result<Self> {
    match meta {
      Meta::NameValue(_) => Err(darling::Error::unsupported_format("name value")),
      Meta::Path(path) => {
        // Check if the path is one of the expected types
        let ident = path.get_ident().ok_or_else(|| {
          darling::Error::custom("expected one of [optional, repeated, map]").with_span(meta)
        })?;

        match () {
          () if ident.eq("optional") => Ok(Self::Optional(Box::new(Self::Required))),
          () if ident.eq("repeated") => Ok(Self::Repeated(Box::new(Self::Required))),
          () if ident.eq("map") => Ok(Self::Map {
            key: None,
            value: None,
          }),
          _ => Err(darling::Error::custom(format!(
            "unknown type specification: `{ident}`"
          ))),
        }
      }
      Meta::List(meta) => {
        // Self::from_list(&NestedMeta::parse_meta_list(meta.tokens.clone())?[..])
        // Check what the list represents based on its path
        let path = meta
          .path
          .get_ident()
          .ok_or_else(|| darling::Error::custom("expected one of [optional, repeated, map]"))?;

        let nested_items = NestedMeta::parse_meta_list(meta.tokens.clone())?;

        match () {
          () if path.eq("optional") => {
            // optional(inner)
            let inner = Self::parse_single_inner(&nested_items)?;
            Ok(Self::Optional(Box::new(inner)))
          }
          () if path.eq("repeated") => {
            // repeated(inner)
            let inner = Self::parse_single_inner(&nested_items)?;
            Ok(Self::Repeated(Box::new(inner)))
          }
          () if path.eq("map") => {
            // map(key(...), value(...))
            Self::parse_map(&nested_items)
          }
          _ => Err(darling::Error::custom(format!(
            "unknown type specification: `{path}`"
          ))),
        }
      }
    }
  }

  fn from_nested_meta(nested: &NestedMeta) -> darling::Result<Self> {
    match nested {
      NestedMeta::Meta(meta) => Self::from_meta(meta),
      NestedMeta::Lit(lit) => Err(darling::Error::unexpected_lit_type(lit)),
    }
  }
}

impl FieldLabel {
  fn parse_single_inner(items: &[NestedMeta]) -> darling::Result<Self> {
    if items.is_empty() {
      // No inner type specified, default to Required
      Ok(Self::Required)
    } else if items.len() == 1 {
      // Single inner type
      Self::from_nested_meta(&items[0])
    } else {
      Err(darling::Error::custom("expected at most one inner type"))
    }
  }

  fn parse_map(items: &[NestedMeta]) -> darling::Result<Self> {
    let expected_key_or_value_err =
      |span| darling::Error::custom("expected 'key' or 'value' in map").with_span(span);

    let mut key_hint = None;
    let mut value_hint = None;

    for item in items {
      match item {
        NestedMeta::Meta(Meta::List(list)) => {
          let name = list
            .path
            .get_ident()
            .ok_or_else(|| expected_key_or_value_err(list))?;

          match () {
            () if name.eq("key") => {
              if key_hint.is_some() {
                return Err(darling::Error::custom("duplicate key specification").with_span(list));
              }
              let key_items = NestedMeta::parse_meta_list(list.tokens.clone())?;
              key_hint = Some(Self::parse_single_inner(&key_items)?);
            }
            () if name.eq("value") => {
              if value_hint.is_some() {
                return Err(
                  darling::Error::custom("duplicate value specification").with_span(list),
                );
              }
              let value_items = NestedMeta::parse_meta_list(list.tokens.clone())?;
              value_hint = Some(Self::parse_single_inner(&value_items)?);
            }
            _ => return Err(expected_key_or_value_err(list)),
          }
        }
        NestedMeta::Meta(Meta::Path(path)) => {
          let ident = path
            .require_ident()
            .map_err(|e| darling::Error::from(e).with_span(item))?;
          darling::Error::custom(format!("a single {} cannot be used in map", ident))
            .with_span(item);
        }
        _ => return Err(darling::Error::custom("invalid map argument")),
      }
    }

    Ok(Self::Map {
      key: key_hint.map(Box::new),
      value: value_hint.map(Box::new),
    })
  }
}

#[derive(Debug, Clone)]
struct OptionalFieldLabel(FieldLabel);

impl core::ops::Deref for OptionalFieldLabel {
  type Target = FieldLabel;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl FromMeta for OptionalFieldLabel {
  fn from_word() -> darling::Result<Self> {
    Ok(Self(FieldLabel::Optional(Box::new(FieldLabel::Required))))
  }

  fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
    if items.is_empty() {
      return Ok(Self(FieldLabel::Optional(Box::new(FieldLabel::Required))));
    }

    if items.len() > 1 {
      return Err(darling::Error::custom(
        "optional(...) can only have one type specification",
      ));
    }

    let item = &items[0];
    let inner = FieldLabel::from_nested_meta(item)?;
    Ok(Self(inner))
  }
}

#[derive(Debug, Clone)]
struct RepeatedFieldLabel(FieldLabel);

impl core::ops::Deref for RepeatedFieldLabel {
  type Target = FieldLabel;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl FromMeta for RepeatedFieldLabel {
  fn from_word() -> darling::Result<Self> {
    Ok(Self(FieldLabel::Repeated(Box::new(FieldLabel::Required))))
  }

  fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
    if items.is_empty() {
      return Ok(Self(FieldLabel::Repeated(Box::new(FieldLabel::Required))));
    }

    if items.len() > 1 {
      return Err(darling::Error::custom(
        "repeated(...) can only have one type specification",
      ));
    }

    let item = &items[0];
    let inner = FieldLabel::from_nested_meta(item)?;
    Ok(Self(inner))
  }
}

#[derive(Debug, Clone)]
struct MapFieldLabel(FieldLabel);

impl core::ops::Deref for MapFieldLabel {
  type Target = FieldLabel;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl FromMeta for MapFieldLabel {
  fn from_word() -> darling::Result<Self> {
    Ok(Self(FieldLabel::Map {
      key: None,
      value: None,
    }))
  }

  fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
    if items.is_empty() {
      return Ok(Self(FieldLabel::Map {
        key: None,
        value: None,
      }));
    }
    let inner = FieldLabel::parse_map(items)?;
    Ok(Self(inner))
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
#[darling(and_then = "Self::validate")]
pub(super) struct FieldLabelFromMeta {
  #[darling(default)]
  optional: Option<SpannedValue<OptionalFieldLabel>>,
  #[darling(default)]
  repeated: Option<SpannedValue<RepeatedFieldLabel>>,
  #[darling(default)]
  map: Option<SpannedValue<MapFieldLabel>>,
}

impl FieldLabelFromMeta {
  pub(super) fn into_label(self) -> Option<Label> {
    if let Some(optional) = self.optional {
      return Some(Label::Optional(Label::from_hint(&optional.0).map(Arc::new)));
    }
    if let Some(repeated) = self.repeated {
      return Some(Label::Repeated(Label::from_hint(&repeated.0).map(Arc::new)));
    }
    if let Some(map) = self.map {
      return Label::from_hint(&map.0);
    }

    None
  }

  pub(super) fn validate(self) -> darling::Result<Self> {
    let mut num_specifications = 0;
    let mut specifications = [None, None, None];
    if let Some(ref optional) = self.optional {
      num_specifications += 1;
      specifications[0] = match optional.span().source_text() {
        Some(text) => Some(format!("optional({text})")),
        None => Some("optional".to_string()),
      };
    }

    if let Some(ref repeated) = self.repeated {
      num_specifications += 1;
      specifications[1] = match repeated.span().source_text() {
        Some(text) => Some(format!("repeated({text})")),
        None => Some("repeated".to_string()),
      };
    }

    if let Some(ref map) = self.map {
      num_specifications += 1;
      specifications[2] = match map.span().source_text() {
        Some(text) => Some(format!("map({text})")),
        None => Some("map".to_string()),
      };
    }

    if num_specifications <= 1 {
      return Ok(self);
    }

    let specs = specifications
      .into_iter()
      .flatten()
      .map(|s| format!("`{s}`"))
      .collect::<Vec<_>>()
      .join(", ");

    Err(darling::Error::custom(format!(
      "only one type specification is allowed for a field, found: {specs}"
    )))
  }
}
