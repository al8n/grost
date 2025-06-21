use core::num::NonZeroU32;

use darling::{FromMeta, util::path_to_string};
use syn::{Meta, Path};

use crate::{object::{meta::{ConvertFromMeta, GenericFieldFlavorFromMeta, PartialDecodedFieldFromMeta, PartialFieldFromMeta, SelectorFieldFromMeta}, Label}, utils::{Invokable, SchemaFromMeta}};

/// The meta of the object field
#[derive(Debug, Clone)]
pub enum GenericFieldFromMeta<TO = (), SO = ()> {
  Skipped {
    default: Option<Invokable>,
    extra: SO,
  },
  Tagged {
    label: Label,
    schema: SchemaFromMeta,
    default: Option<Invokable>,
    tag: NonZeroU32,
    flavor: GenericFieldFlavorFromMeta,
    convert: ConvertFromMeta,
    partial: PartialFieldFromMeta,
    partial_decoded: PartialDecodedFieldFromMeta,
    selector: SelectorFieldFromMeta,
    copy: bool,
    extra: TO,
  },
}

impl<SO, TO> FromMeta for GenericFieldFromMeta<SO, TO>
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
      #[derive(Debug, FromMeta)]
      struct SkipFieldFromMeta<M> {
        #[darling(default)]
        default: Option<Invokable>,
        #[darling(flatten)]
        extra: M,
      }

      let skip_meta = items
        .iter()
        .cloned()
        .filter_map(|item| match item {
          darling::ast::NestedMeta::Lit(_) => Some(Ok(item)),
          darling::ast::NestedMeta::Meta(meta) => {
            if let Meta::Path(path) = meta {
              if path.is_ident("skip") {
                return None;
              } else if is_tagged_field_only_identifiers(path) {
                return Some(Err(darling::Error::custom(format!(
                  "`{}` is not supported by skipped field",
                  path_to_string(path)
                ))));
              } else {
                return Some(Ok(item));
              }
            } else {
              Some(Ok(item))
            }
          }
        })
        .collect::<darling::Result<Vec<_>>>();

      return SkipFieldFromMeta::from_list(&skip_meta)
        .map(|SkipFieldFromMeta { default, extra }| Self::Skipped { default, extra });
    }

    let TaggedFieldFromMeta {
      label,
      schema,
      default,
      tag,
      flavor,
      convert,
      partial,
      partial_decoded,
      selector,
      copy,
      extra,
    } = TaggedFieldFromMeta::from_list(items)?;

    Ok(Self::Tagged {
      label,
      schema,
      default,
      tag,
      flavor,
      convert,
      partial,
      partial_decoded,
      selector,
      copy,
      extra,
    })
  }
}

#[derive(Debug, Clone)]
struct TaggedFieldFromMeta<TO> {
  label: Label,
  schema: SchemaFromMeta,
  default: Option<Invokable>,
  tag: NonZeroU32,
  flavor: GenericFieldFlavorFromMeta,
  convert: ConvertFromMeta,
  partial: PartialFieldFromMeta,
  partial_decoded: PartialDecodedFieldFromMeta,
  selector: SelectorFieldFromMeta,
  copy: bool,
  extra: TO,
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
            label = Some(Label::from_meta(item)?);
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
      #[darling(default)]
      tag: NonZeroU32,
      #[darling(default)]
      flavor: GenericFieldFlavorFromMeta,
      #[darling(default)]
      convert: ConvertFromMeta,
      #[darling(default)]
      partial: PartialFieldFromMeta,
      #[darling(default)]
      partial_decoded: PartialDecodedFieldFromMeta,
      #[darling(default)]
      selector: SelectorFieldFromMeta,
      #[darling(default)]
      copy: bool,
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
      flavor: helper.flavor,
      convert: helper.convert,
      partial: helper.partial,
      partial_decoded: helper.partial_decoded,
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
    "partial_decoded",
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
