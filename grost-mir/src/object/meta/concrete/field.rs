use core::num::NonZeroU32;

use darling::{FromMeta, util::path_to_string};
use syn::{Attribute, Meta, Path, Type};

use crate::{
  object::{
    Label,
    meta::{
      FieldConvertFromMeta, FieldDecodeFromMeta, FieldEncodeFromMeta, PartialFieldConvertFromMeta,
      SelectorFieldFromMeta, SkippedFieldFromMeta,
    },
  },
  utils::{Attributes, Invokable, NestedMeta, SchemaFromMeta},
};

/// The meta of the object field
#[derive(Debug, Clone)]
pub enum FieldFromMeta<TO = (), SO = ()> {
  Skipped(Box<SkippedFieldFromMeta<SO>>),
  Tagged(Box<TaggedFieldFromMeta<TO>>),
}

impl<SO, TO> FromMeta for FieldFromMeta<SO, TO>
where
  SO: FromMeta,
  TO: FromMeta,
{
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let skipped = items.iter().any(|item| match item {
      darling::ast::NestedMeta::Lit(_) => false,
      darling::ast::NestedMeta::Meta(meta) => {
        if let Meta::Path(path) = meta {
          path.is_ident("skip")
        } else {
          false
        }
      }
    });

    if skipped {
      let skip_meta = items
        .iter()
        .cloned()
        .filter_map(|item| match item {
          darling::ast::NestedMeta::Lit(_) => Some(Ok(item)),
          darling::ast::NestedMeta::Meta(ref meta) => {
            if let Meta::Path(path) = meta {
              if path.is_ident("skip") {
                None
              } else if is_tagged_field_only_identifiers(path) {
                Some(Err(darling::Error::custom(format!(
                  "`{}` is not supported by skipped field",
                  path_to_string(path)
                ))))
              } else {
                Some(Ok(item))
              }
            } else {
              Some(Ok(item))
            }
          }
        })
        .collect::<darling::Result<Vec<_>>>()?;

      return SkippedFieldFromMeta::from_list(&skip_meta).map(|f| Self::Skipped(Box::new(f)));
    }

    TaggedFieldFromMeta::from_list(items).map(|f| Self::Tagged(Box::new(f)))
  }
}

/// The meta of the partial object field
#[derive(Debug, Default, Clone, FromMeta)]
pub struct PartialFieldFromMeta {
  #[darling(default, map = "Attributes::into_inner")]
  pub(in crate::object) attrs: Vec<Attribute>,
  #[darling(default)]
  pub(in crate::object) transform_ref: PartialFieldConvertFromMeta,
  #[darling(default)]
  pub(in crate::object) partial_transform_ref: PartialFieldConvertFromMeta,
  #[darling(default)]
  pub(in crate::object) partial_transform: PartialFieldConvertFromMeta,
}

/// The meta of the partial reference object field
#[derive(Debug, Default, Clone)]
pub struct PartialRefFieldFromMeta {
  pub(in crate::object) copy: bool,
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) ty: Option<Type>,
  pub(in crate::object) encode: FieldEncodeFromMeta,
  pub(in crate::object) decode: FieldDecodeFromMeta,
}

impl FromMeta for PartialRefFieldFromMeta {
  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (match *item {
      Meta::Path(_) => Self::from_word(),
      Meta::NameValue(ref value) => Self::from_expr(&value.value),
      Meta::List(ref value) => {
        /// The meta of the partial reference object field
        #[derive(Debug, Default, Clone, FromMeta)]
        struct Helper {
          #[darling(default)]
          copy: bool,
          #[darling(default, map = "Attributes::into_inner")]
          attrs: Vec<Attribute>,
          #[darling(default)]
          encode: FieldEncodeFromMeta,
          #[darling(default)]
          decode: FieldDecodeFromMeta,
          #[darling(rename = "type", default)]
          ty: Option<Type>,
        }

        let Helper {
          copy,
          attrs,
          encode,
          decode,
          ty,
        } = Helper::from_list(&NestedMeta::parse_meta_list(value.tokens.clone())?)?;

        Ok(Self {
          copy,
          attrs,
          ty,
          encode,
          decode,
        })
      }
    })
    .map_err(|e| e.with_span(item))
  }
}

#[derive(Debug, Clone)]
pub struct TaggedFieldFromMeta<TO> {
  pub(in crate::object) label: Label,
  pub(in crate::object) schema: SchemaFromMeta,
  pub(in crate::object) default: Option<Invokable>,
  pub(in crate::object) tag: NonZeroU32,
  pub(in crate::object) transform: FieldConvertFromMeta,
  pub(in crate::object) partial: PartialFieldFromMeta,
  pub(in crate::object) partial_ref: PartialRefFieldFromMeta,
  pub(in crate::object) selector: SelectorFieldFromMeta,
  pub(in crate::object) copy: bool,
  pub(in crate::object) wire_format: Option<Type>,
  pub(in crate::object) extra: TO,
}

impl<TO: FromMeta> FromMeta for TaggedFieldFromMeta<TO> {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let mut remaining_items = vec![];
    let mut label = None;

    for item in items {
      if let darling::ast::NestedMeta::Meta(meta) = item {
        if let Meta::Path(path) = meta {
          if Label::possible_idents()
            .iter()
            .any(|ident| path.is_ident(ident))
          {
            if let Some(old) = label {
              return Err(darling::Error::custom(format!(
                "
                Field label has already been specified as `{old}`",
              )));
            }
            label = Some(Label::from_meta(meta)?);
          } else {
            remaining_items.push(item.clone());
          }
        }
      }
    }

    #[derive(Debug, FromMeta)]
    struct Helper<TO> {
      #[darling(default)]
      schema: SchemaFromMeta,
      #[darling(default)]
      default: Option<Invokable>,
      tag: NonZeroU32,
      #[darling(default)]
      transform: FieldConvertFromMeta,
      #[darling(default)]
      partial: PartialFieldFromMeta,
      #[darling(default)]
      partial_ref: PartialRefFieldFromMeta,
      #[darling(default)]
      selector: SelectorFieldFromMeta,
      #[darling(default)]
      copy: bool,
      #[darling(default)]
      wire_format: Option<Type>,
      #[darling(flatten)]
      extra: TO,
    }

    let helper = Helper::from_list(&remaining_items)?;

    Ok(Self {
      label: label.ok_or_else(|| {
        darling::Error::custom(format!(
          "No field label specified, expected one of: {}",
          Label::possible_idents().join(", ")
        ))
      })?,
      schema: helper.schema,
      default: helper.default,
      tag: helper.tag,
      wire_format: helper.wire_format,
      transform: helper.transform,
      partial: helper.partial,
      partial_ref: helper.partial_ref,
      selector: helper.selector,
      copy: helper.copy,
      extra: helper.extra,
    })
  }
}

fn is_tagged_field_only_identifiers(path: &Path) -> bool {
  const POSSIBLE_IDENTIFIERS: &[&str] = &[
    "schema",
    "tag",
    "flavor",
    "convert",
    "partial",
    "partial_ref",
    "selector",
    "copy",
  ];

  if Label::possible_idents()
    .iter()
    .any(|ident| path.is_ident(ident))
  {
    return true;
  }

  POSSIBLE_IDENTIFIERS
    .iter()
    .any(|ident| path.is_ident(ident))
}
