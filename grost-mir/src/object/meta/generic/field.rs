use core::num::NonZeroU32;
use std::collections::BTreeMap;

use darling::{FromMeta, util::path_to_string};
use syn::{Attribute, Ident, Meta, Path, Type};

use crate::{
  object::{
    Label,
    meta::{FieldConvertFromMeta, FieldDecodeFromMeta, FieldEncodeFromMeta, SelectorFieldFromMeta},
  },
  utils::{Attributes, Invokable, NestedMeta, SchemaFromMeta},
};

#[derive(Debug, Default, Clone, FromMeta)]
pub(in crate::object) struct PartialFieldFlavorFromMeta {
  pub(in crate::object) transform: FieldConvertFromMeta,
  pub(in crate::object) partial_transform: FieldConvertFromMeta,
}

#[derive(Debug, Default, Clone, FromMeta)]
pub(in crate::object) struct PartialRefFieldFlavorFromMeta {
  pub(in crate::object) encode: FieldEncodeFromMeta,
  pub(in crate::object) decode: FieldDecodeFromMeta,
}

#[derive(Debug, Default, Clone)]
pub(in crate::object) struct FieldFlavorFromMeta {
  pub(in crate::object) wire_format: Option<Type>,
  pub(in crate::object) partial_ref: PartialRefFieldFlavorFromMeta,
  pub(in crate::object) partial: PartialFieldFlavorFromMeta,
}

impl FromMeta for FieldFlavorFromMeta {
  fn from_string(value: &str) -> darling::Result<Self> {
    let format = syn::parse_str(value).map_err(darling::Error::from)?;

    Ok(FieldFlavorFromMeta {
      wire_format: Some(format),
      partial_ref: PartialRefFieldFlavorFromMeta::default(),
      partial: PartialFieldFlavorFromMeta::default(),
    })
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    #[derive(Debug, Default, Clone, FromMeta)]
    pub(in crate::object) struct Helper {
      #[darling(default)]
      pub(in crate::object) wire_format: Option<Type>,
      #[darling(default)]
      pub(in crate::object) partial_ref: PartialRefFieldFlavorFromMeta,
      #[darling(default)]
      pub(in crate::object) partial: PartialFieldFlavorFromMeta,
    }

    let Helper {
      wire_format,
      partial_ref,
      partial,
    } = Helper::from_list(items)?;

    Ok(FieldFlavorFromMeta {
      wire_format,
      partial_ref,
      partial,
    })
  }
}

/// The meta of the partial object field
#[derive(Debug, Default, Clone, FromMeta)]
pub struct GenericPartialFieldFromMeta {
  #[darling(default, map = "Attributes::into_inner")]
  pub(in crate::object) attrs: Vec<Attribute>,
}

/// The meta of the partial reference object field
#[derive(Debug, Default, Clone)]
pub struct GenericPartialRefFieldFromMeta {
  pub(in crate::object) copy: bool,
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) ty: Option<Type>,
}

impl FromMeta for GenericPartialRefFieldFromMeta {
  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (match *item {
      Meta::Path(_) => Self::from_word(),
      Meta::NameValue(ref value) => Self::from_expr(&value.value),
      Meta::List(ref value) => {
        #[derive(Debug, Default, Clone, FromMeta)]
        struct Helper {
          #[darling(default)]
          copy: bool,
          #[darling(default, map = "Attributes::into_inner")]
          attrs: Vec<Attribute>,
          #[darling(rename = "type", default)]
          ty: Option<Type>,
        }

        let Helper { copy, attrs, ty } =
          Helper::from_list(&NestedMeta::parse_meta_list(value.tokens.clone())?)?;
        Ok(Self { copy, attrs, ty })
      }
    })
    .map_err(|e| e.with_span(item))
  }
}

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
    flavors: BTreeMap<Ident, FieldFlavorFromMeta>,
    transform: FieldConvertFromMeta,
    partial: GenericPartialFieldFromMeta,
    partial_ref: GenericPartialRefFieldFromMeta,
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
          darling::ast::NestedMeta::Meta(ref meta) => {
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
        .collect::<darling::Result<Vec<_>>>()?;

      return SkipFieldFromMeta::from_list(&skip_meta)
        .map(|SkipFieldFromMeta { default, extra }| Self::Skipped { default, extra });
    }

    let TaggedFieldFromMeta {
      label,
      schema,
      default,
      tag,
      flavors,
      transform,
      partial,
      partial_ref,
      selector,
      copy,
      extra,
    } = TaggedFieldFromMeta::from_list(items)?;

    Ok(Self::Tagged {
      label,
      schema,
      default,
      tag,
      flavors,
      transform,
      partial,
      partial_ref,
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
  flavors: BTreeMap<Ident, FieldFlavorFromMeta>,
  transform: FieldConvertFromMeta,
  partial: GenericPartialFieldFromMeta,
  partial_ref: GenericPartialRefFieldFromMeta,
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
      flavors: BTreeMap<Ident, FieldFlavorFromMeta>,
      #[darling(default)]
      transform: FieldConvertFromMeta,
      #[darling(default)]
      partial: GenericPartialFieldFromMeta,
      #[darling(default)]
      partial_ref: GenericPartialRefFieldFromMeta,
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
      flavors: helper.flavors,
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
