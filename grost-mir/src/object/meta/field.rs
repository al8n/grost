use core::num::NonZeroU32;

use darling::FromMeta;
use syn::{Attribute, Meta, Type, parse::Parser};

use crate::utils::{Attributes, Invokable, NestedMetaWithTypeField, SchemaFromMeta};

pub(in crate::object) use flavor::{
  DecodeFromMeta as FieldDecodeFromMeta, EncodeFromMeta as FieldEncodeFromMeta,
};

pub use convert::ConvertFromMeta;
pub use flavor::FieldFlavorFromMeta;
pub use label::Label;
pub use selector::{FieldSelection, SelectorFieldFromMeta};

mod convert;
mod flavor;
mod label;
mod selector;

/// The meta of the partial reference object field
#[derive(Debug, Default, Clone)]
pub struct PartialDecodedFieldFromMeta {
  pub(in crate::object) copy: bool,
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) ty: Option<Type>,
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
pub struct PartialFieldFromMeta {
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) ty: Option<Type>,
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

/// The meta of the object field
#[derive(Debug, Clone)]
pub struct FieldFromMeta {
  pub label: Option<Label>,
  pub schema: SchemaFromMeta,
  pub default: Option<Invokable>,
  pub tag: Option<NonZeroU32>,
  pub flavor: FieldFlavorFromMeta,
  pub convert: ConvertFromMeta,
  pub partial: PartialFieldFromMeta,
  pub partial_decoded: PartialDecodedFieldFromMeta,
  pub selector: SelectorFieldFromMeta,
  pub copy: bool,
  pub skip: bool,
}
