use darling::FromMeta;
pub use field::*;
pub(super) use indexer::IndexerFromMeta;
pub(super) use selector::{SelectorFromMeta, SelectorIterFromMeta};

pub use concrete::{FieldFromMeta, ObjectFromMeta};
pub use generic::GenericObjectFromMeta;
use syn::Meta;

use crate::utils::{BoolOption, NestedMeta};

mod field;
mod indexer;
mod partial;
mod selector;

/// Concrete object meta, a concrete object means there will only be one flavor and the generated code will not be generic over the flavor type.
pub mod concrete;
/// Generic object meta, a generic object means the generated code will be generic over the flavor type.
pub mod generic;

#[derive(Debug, Default, Clone, darling::FromMeta)]
pub(crate) struct ObjectLabelConvertFromMeta {
  #[darling(default)]
  pub(in crate::object) or_default: BoolOption,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct ObjectConvertFromMeta {
  pub(in crate::object) or_default: BoolOption,
  pub(in crate::object) scalar: ObjectLabelConvertFromMeta,
  pub(in crate::object) bytes: ObjectLabelConvertFromMeta,
  pub(in crate::object) string: ObjectLabelConvertFromMeta,
  pub(in crate::object) object: ObjectLabelConvertFromMeta,
  pub(in crate::object) enumeration: ObjectLabelConvertFromMeta,
  pub(in crate::object) interface: ObjectLabelConvertFromMeta,
  pub(in crate::object) generic: ObjectLabelConvertFromMeta,
  pub(in crate::object) union: ObjectLabelConvertFromMeta,
  pub(in crate::object) map: ObjectLabelConvertFromMeta,
  pub(in crate::object) set: ObjectLabelConvertFromMeta,
  pub(in crate::object) list: ObjectLabelConvertFromMeta,
}

impl FromMeta for ObjectConvertFromMeta {
  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (match *item {
      Meta::Path(_) => Self::from_word(),
      Meta::NameValue(ref value) => Self::from_expr(&value.value),
      Meta::List(ref value) => {
        #[derive(Debug, Default, Clone, darling::FromMeta)]
        struct Helper {
          #[darling(default)]
          or_default: BoolOption,
          #[darling(default)]
          scalar: ObjectLabelConvertFromMeta,
          #[darling(default)]
          bytes: ObjectLabelConvertFromMeta,
          #[darling(default)]
          string: ObjectLabelConvertFromMeta,
          #[darling(default)]
          object: ObjectLabelConvertFromMeta,
          #[darling(default, rename = "enum")]
          enumeration: ObjectLabelConvertFromMeta,
          #[darling(default)]
          interface: ObjectLabelConvertFromMeta,
          #[darling(default)]
          union: ObjectLabelConvertFromMeta,
          #[darling(default)]
          map: ObjectLabelConvertFromMeta,
          #[darling(default)]
          set: ObjectLabelConvertFromMeta,
          #[darling(default)]
          list: ObjectLabelConvertFromMeta,
          #[darling(default)]
          generic: ObjectLabelConvertFromMeta,
        }

        let Helper {
          or_default,
          scalar,
          bytes,
          string,
          object,
          enumeration,
          interface,
          union,
          map,
          set,
          list,
          generic,
        } = Helper::from_list(&NestedMeta::parse_meta_list(value.tokens.clone())?)?;

        Ok(Self {
          or_default,
          scalar,
          bytes,
          string,
          object,
          enumeration,
          interface,
          union,
          map,
          set,
          list,
          generic,
        })
      }
    })
    .map_err(|e| e.with_span(item))
  }
}
