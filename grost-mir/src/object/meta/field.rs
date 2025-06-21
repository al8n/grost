use core::num::NonZeroU32;

use darling::FromMeta;
use syn::{Attribute, Meta, Type};

use crate::utils::{Attributes, Invokable, NestedMeta, SchemaFromMeta};

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
  pub(in crate::object) encode: FieldEncodeFromMeta,
  pub(in crate::object) decode: FieldDecodeFromMeta,
}

impl FromMeta for PartialDecodedFieldFromMeta {
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
      Meta::NameValue(ref value) => Self::from_expr(&value.value),
      Meta::List(ref value) => {
        /// The meta of the partial reference object field
        #[derive(Debug, Default, Clone, FromMeta)]
        struct Helper {
          #[darling(default, rename = "type")]
          ty: Option<Type>,
          #[darling(default, map = "Attributes::into_inner")]
          attrs: Vec<Attribute>,
        }

        let Helper { attrs, ty } =
          Helper::from_list(&NestedMeta::parse_meta_list(value.tokens.clone())?)?;
        Ok(Self { attrs, ty })
      }
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
