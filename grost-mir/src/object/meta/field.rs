use darling::FromMeta;
use syn::{Attribute, Meta, Type};


pub use convert::{FieldConvertFromMeta, FieldDecodeFromMeta, FieldEncodeFromMeta, FieldSkipEncodeOperation};
pub use label::Label;
pub use selector::{FieldSelection, SelectorFieldFromMeta};

use crate::utils::{Attributes, NestedMeta};

mod convert;
mod flavor;
mod label;
mod selector;

/// The meta of the partial object field
#[derive(Debug, Default, Clone)]
pub struct PartialFieldFromMeta {
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) ty: Option<Type>,
  pub(in crate::object) partial_transform: FieldConvertFromMeta,
}

impl FromMeta for PartialFieldFromMeta {
  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (match *item {
      Meta::Path(_) => Self::from_word(),
      Meta::NameValue(ref value) => Self::from_expr(&value.value),
      Meta::List(ref value) => {
        #[derive(Debug, Default, Clone, FromMeta)]
        struct Helper {
          #[darling(default, rename = "type")]
          ty: Option<Type>,
          #[darling(default, map = "Attributes::into_inner")]
          attrs: Vec<Attribute>,
          #[darling(default)]
          partial_transform: FieldConvertFromMeta,
        }

        let Helper { attrs, ty, partial_transform } =
          Helper::from_list(&NestedMeta::parse_meta_list(value.tokens.clone())?)?;
        Ok(Self { attrs, ty, partial_transform })
      }
    })
    .map_err(|e| e.with_span(item))
  }
}
