use core::num::NonZeroU32;

use darling::FromMeta;
use syn::{Attribute, Meta, Type, parse::Parser};

use crate::meta::{Attributes, NestedMetaWithTypeField, SchemaFromMeta};

pub(crate) use convert::ConvertFromMeta;
pub(crate) use flavor::{
  DecodeFromMeta as FieldDecodeFromMeta, EncodeFromMeta as FieldEncodeFromMeta, FieldFlavorFromMeta,
};
pub use label::Label;
pub use selector::Selection;
pub(crate) use selector::SelectorFieldFromMeta;

mod convert;
mod flavor;
mod label;
mod selector;

/// The meta of the partial reference object field
#[derive(Debug, Default, Clone)]
pub(crate) struct PartialDecodedFieldFromMeta {
  pub(crate) copy: bool,
  pub(crate) attrs: Vec<Attribute>,
  pub(crate) ty: Option<Type>,
}

impl FromMeta for PartialDecodedFieldFromMeta {
  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (match *item {
      Meta::Path(_) => Self::from_word(),
      Meta::List(ref value) => {
        let punctuated =
          syn::punctuated::Punctuated::<NestedMetaWithTypeField, syn::Token![,]>::parse_terminated
            .parse2(value.tokens.clone())?;

        let mut nested_meta = Vec::new();
        let mut ty = None;
        for item in punctuated {
          match item {
            NestedMetaWithTypeField::Type(t) => {
              if ty.is_some() {
                return Err(darling::Error::duplicate_field("type"));
              }
              ty = Some(t);
            }
            NestedMetaWithTypeField::Nested(value) => {
              nested_meta.push(value);
            }
          }
        }

        /// The meta of the partial reference object field
        #[derive(Debug, Default, Clone, FromMeta)]
        struct Helper {
          copy: bool,
          #[darling(default, map = "Attributes::into_inner")]
          attrs: Vec<Attribute>,
        }

        let Helper { copy, attrs } = Helper::from_list(&nested_meta)?;
        Ok(Self { copy, attrs, ty })
      }
      Meta::NameValue(ref value) => Self::from_expr(&value.value),
    })
    .map_err(|e| e.with_span(item))
  }
}

/// The meta of the partial object field
#[derive(Debug, Default, Clone)]
pub(crate) struct PartialFieldFromMeta {
  pub(crate) attrs: Vec<Attribute>,
  pub(crate) ty: Option<Type>,
}

impl FromMeta for PartialFieldFromMeta {
  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (match *item {
      Meta::Path(_) => Self::from_word(),
      Meta::List(ref value) => {
        let punctuated =
          syn::punctuated::Punctuated::<NestedMetaWithTypeField, syn::Token![,]>::parse_terminated
            .parse2(value.tokens.clone())?;

        let mut nested_meta = Vec::new();
        let mut ty = None;
        for item in punctuated {
          match item {
            NestedMetaWithTypeField::Type(t) => {
              if ty.is_some() {
                return Err(darling::Error::duplicate_field("type"));
              }
              ty = Some(t);
            }
            NestedMetaWithTypeField::Nested(value) => {
              nested_meta.push(value);
            }
          }
        }

        /// The meta of the partial reference object field
        #[derive(Debug, Default, Clone, FromMeta)]
        struct Helper {
          #[darling(default, map = "Attributes::into_inner")]
          attrs: Vec<Attribute>,
        }

        let Helper { attrs } = Helper::from_list(&nested_meta)?;
        Ok(Self { attrs, ty })
      }
      Meta::NameValue(ref value) => Self::from_expr(&value.value),
    })
    .map_err(|e| e.with_span(item))
  }
}

#[allow(clippy::large_enum_variant)]
enum FieldNestedMeta {
  Label(Label),
  Nested(darling::ast::NestedMeta),
}

impl syn::parse::Parse for FieldNestedMeta {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    if Label::peek(&input)? {
      let label: Label = input.parse()?;
      Ok(Self::Label(label))
    } else {
      darling::ast::NestedMeta::parse(input).map(Self::Nested)
    }
  }
}

/// The meta of the object field
#[derive(Debug, Clone, FromMeta)]
struct FieldFromMetaHelper {
  #[darling(default)]
  schema: SchemaFromMeta,
  #[darling(default)]
  default: Option<syn::Path>,
  #[darling(default)]
  tag: Option<NonZeroU32>,
  #[darling(default)]
  flavor: flavor::FieldFlavorFromMeta,
  #[darling(default)]
  convert: convert::ConvertFromMeta,
  #[darling(default)]
  partial: PartialFieldFromMeta,
  #[darling(default)]
  partial_decoded: PartialDecodedFieldFromMeta,
  #[darling(default)]
  selector: SelectorFieldFromMeta,
  #[darling(default)]
  copy: bool,
  #[darling(default)]
  skip: bool,
}

/// The meta of the object field
#[derive(Debug, Clone)]
pub struct FieldFromMeta {
  pub(crate) label: Label,
  pub(crate) schema: SchemaFromMeta,
  pub(crate) default: Option<syn::Path>,
  pub(crate) tag: Option<NonZeroU32>,
  pub(crate) flavor: flavor::FieldFlavorFromMeta,
  pub(crate) convert: convert::ConvertFromMeta,
  pub(crate) partial: PartialFieldFromMeta,
  pub(crate) partial_decoded: PartialDecodedFieldFromMeta,
  pub(crate) selector: SelectorFieldFromMeta,
  pub(crate) copy: bool,
  pub(crate) skip: bool,
}

impl FromMeta for FieldFromMeta {
  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (match item {
      Meta::Path(_) => Self::from_word(),
      Meta::NameValue(value) => Self::from_expr(&value.value),
      Meta::List(value) => {
        let punctuated =
          syn::punctuated::Punctuated::<FieldNestedMeta, syn::Token![,]>::parse_terminated
            .parse2(value.tokens.clone())?;

        let mut nested_meta = Vec::new();
        let mut label: Option<Label> = None;
        for item in punctuated {
          match item {
            FieldNestedMeta::Label(l) => {
              if let Some(ref label) = label {
                return Err(darling::Error::custom(
                  format!(
                    "Cannot specify both `{label}` and `{l}` at the same time.",
                  )
                ));
              }
              label = Some(l);
            }
            FieldNestedMeta::Nested(value) => {
              nested_meta.push(value);
            }
          }
        }

        let FieldFromMetaHelper { schema, default, tag, flavor, convert, partial, partial_decoded, selector, copy, skip } = FieldFromMetaHelper::from_list(&nested_meta)?;
        Ok(Self {
          label: label.ok_or_else(|| darling::Error::custom("Expected one of [scalar, bytes, string, object, enum, union, interface, map, set, list, optional] to be specified for a field"))?,
            schema,
            default,
            tag,
            flavor,
            convert,
            partial,
            partial_decoded,
            selector,
            copy,
            skip,
        })
      }
    })
    .map_err(|e| e.with_span(item))
  }
}
